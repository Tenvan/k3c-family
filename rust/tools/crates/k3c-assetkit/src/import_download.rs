//! Treffer-Download (Go `DownloadHit` aus `import.go`): nur https, nur erlaubte Hosts, jede Weiterleitung geprüft,
//! Größe und Signatur wie beim Import.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, FixedOffset};
use reqwest::{Method, StatusCode};
use url::Url;

use crate::error::{Error, Result};
use crate::fsutil::{self, TempFile};
use crate::import::{MAX_IMPORT_BYTES, import_file_on, import_target};
use crate::net::{DownloadClient, bounded, is_test_origin, parse_with_host, percent_decode, send_following};
use crate::research::LICENSE_CHECK;

/// Quellen mit direktem Download und maschinenlesbarer Lizenz. Nur sie lädt das Werkzeug selbst; Links aus Treffern
/// (auch von Agenten) erreichen so keine internen Adressen (SSRF).
pub const DOWNLOAD_HOSTS: &[&str] = &["upload.wikimedia.org"];

/// Erlaubte ausdrückliche Ports einer Download-URL; ohne Port ist immer erlaubt.
pub const DOWNLOAD_PORTS: &[u16] = &[443];

/// Obergrenze für [`send_following`]; die Prüffunktionen selbst brechen wie Go schon früher ab.
pub(crate) const MAX_HOPS: usize = 5;

fn host_list() -> String {
    let mut hosts = DOWNLOAD_HOSTS.to_vec();
    hosts.sort_unstable();
    hosts.join(", ")
}

/// Host samt ausdrücklichem Port (Go `u.Host`).
fn host_port(url: &Url) -> String {
    let host = url.host_str().unwrap_or_default();
    url.port()
        .map_or_else(|| host.to_string(), |port| format!("{host}:{port}"))
}

/// Übernimmt einen Treffer mit direktem Download und erkannter Lizenz ohne Handarbeit. `client` darf nicht selbst
/// weiterleiten ([`crate::net::client`]).
pub async fn download_hit(
    root: &Path,
    slot_id: &str,
    hit_id: &str,
    client: &DownloadClient,
    now: DateTime<FixedOffset>,
) -> Result<String> {
    download_hit_on(root, slot_id, hit_id, None, client, now).await
}

/// [`download_hit`] mit der Stand-Regel von [`import_file_on`].
pub(crate) async fn download_hit_on(
    root: &Path,
    slot_id: &str,
    hit_id: &str,
    stand: Option<&str>,
    client: &DownloadClient,
    now: DateTime<FixedOffset>,
) -> Result<String> {
    let (_, hit) = import_target(root, slot_id, hit_id)?;
    if hit.download.is_empty() || hit.license_status == LICENSE_CHECK {
        return Err(Error::Msg(format!(
            "treffer {hit_id}: kein direkter Download mit erkannter Lizenz, bitte manuell laden und importieren"
        )));
    }
    let url = parse_with_host(&hit.download)
        .filter(|u| u.scheme() == "https" || is_test_origin(u))
        .ok_or_else(|| Error::Msg(format!("download nur über https: {:?}", hit.download)))?;
    if !download_allowed(&url) {
        return Err(Error::Msg(format!(
            "download nur von {}, nicht von {:?}: bitte manuell laden und importieren",
            host_list(),
            host_port(&url)
        )));
    }
    let tmp = fetch_to_temp(client, root, &url, &temp_suffix(&url)).await?;
    import_file_on(root, slot_id, hit_id, tmp.path(), stand, now)
}

/// Prüft jedes Weiterleitungsziel wie die Start-URL (https, Host aus [`DOWNLOAD_HOSTS`]), wie Go höchstens vier
/// Sprünge (`len(via) >= 5`) (SSRF, B-688).
fn download_redirect(url: &Url, hops: usize) -> Result<()> {
    if hops >= 5 {
        return Err(Error::Msg("zu viele Weiterleitungen".to_string()));
    }
    if (url.scheme() != "https" && !is_test_origin(url)) || !download_allowed(url) {
        let host = host_port(url);
        log::warn!("🚫 Weiterleitung auf nicht erlaubten Host abgelehnt host={host}");
        return Err(Error::Msg(format!(
            "weiterleitung auf {host:?} abgelehnt: nur https und {}",
            host_list()
        )));
    }
    Ok(())
}

/// Host aus [`DOWNLOAD_HOSTS`], erlaubter Port, keine Zugangsdaten in der URL (B-694).
pub(crate) fn download_allowed(url: &Url) -> bool {
    if !url.username().is_empty() || url.password().is_some() {
        return false;
    }
    is_test_origin(url)
        || (url.port().is_none_or(|p| DOWNLOAD_PORTS.contains(&p))
            && url.host_str().is_some_and(|h| DOWNLOAD_HOSTS.contains(&h)))
}

/// Wegwerf-Ordner im Repo (`.work/tmp`, Regel „Schreiben nur im Repo“, 🧑 2026-11-18).
fn work_tmp(root: &Path) -> io::Result<PathBuf> {
    let dir = root.join(".work").join("tmp");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Endung der Temp-Datei aus dem dekodierten Pfad der Adresse. Netzdaten: nur ein Punkt mit Buchstaben und Ziffern
/// gelangt in den Dateinamen (kein `\`, kein `:`); alles andere ergibt keine Endung und scheitert dann beim Import.
fn temp_suffix(url: &Url) -> String {
    let path = percent_decode(url.path());
    let ext = fsutil::path_ext(&path).to_lowercase();
    let plain = ext.len() <= 16 && ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric());
    if plain { ext } else { String::new() }
}

/// Lädt höchstens ein Byte mehr als erlaubt nach `.work/tmp`; die Prüfung beim Import erkennt die Überschreitung.
async fn fetch_to_temp(client: &DownloadClient, root: &Path, url: &Url, suffix: &str) -> Result<TempFile> {
    let mut response = send_following(client, Method::GET, url, MAX_HOPS, download_redirect).await?;
    if response.status() != StatusCode::OK {
        return Err(Error::Msg(format!("download: HTTP {}", response.status().as_u16())));
    }
    let (file, path) = fsutil::create_temp_with(&work_tmp(root)?, "k3c-import-", suffix)?;
    // Reihenfolge: `file` schließt vor dem Löschen durch `tmp`.
    let tmp = TempFile::new(path);
    let mut file = file;
    let mut left = MAX_IMPORT_BYTES + 1;
    while left > 0 {
        let Some(chunk) = bounded(response.chunk()).await? else {
            break;
        };
        let take = chunk.len().min(left);
        file.write_all(&chunk[..take])?;
        left -= take;
    }
    Ok(tmp)
}
