//! Download in den Vorrat (Go `vorrat.go`): Eintrag prüfen, nur erlaubte Quellen (jede Weiterleitung neu), erst in eine
//! Temp-Datei, dann umbenennen und in den Index eintragen.

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset, SecondsFormat, Utc};
use reqwest::{Method, StatusCode};
use sha2::{Digest, Sha256};
use url::Url;

use crate::error::{Error, Result};
use crate::fsutil::{self, TempFile};
use crate::import::{check_signature, slug};
use crate::import_download::MAX_HOPS;
use crate::net::{DownloadClient, bounded, is_test_origin, parse_with_host, send_following};
use crate::register::{KIND_GRAPHIC, KIND_SOUND};
use crate::suggestions::{MAX_SUGGESTION_BYTES, Suggestion, VORRAT_EXT, suggestion_license_allowed};
use crate::vorrat::{
    VORRAT_BLOCKED, VORRAT_DROPPED, VorratEntry, change_vorrat, load_vorrat, lock_vorrat, vorrat_path,
};

/// Hosts der Quellen (samt Unterdomänen), von denen der Vorrat lädt; auch jedes Redirect-Ziel muss dazugehören, damit
/// eine Quelle nicht auf Klartext, Loopback oder Geräte im Heimnetz umleiten kann (SSRF).
pub const VORRAT_HOSTS: &[&str] = &["opengameart.org", "kenney.nl", "freesound.org", "itch.io", "itch.zone"];

/// Meldet die bisher geladenen Bytes eines Downloads.
pub type Progress = Arc<dyn Fn(i64) + Send + Sync>;

/// Ordner je Art unter `.work/vorrat/`.
fn kind_dir(kind: &str) -> Option<&'static str> {
    match kind {
        KIND_GRAPHIC => Some("grafik"),
        KIND_SOUND => Some("audio"),
        _ => None,
    }
}

/// Prüft einen Eintrag vor dem Download: nicht gesperrt oder verworfen, Lizenz nach Q70, Größe im Limit, https-Adresse
/// und erlaubte Endung. Liefert die Adresse und die Endung.
pub fn check_vorrat_download(s: &Suggestion) -> Result<(Url, String)> {
    let id = &s.id;
    let fail = |text: String| Err(Error::Msg(text));
    if !suggestion_license_allowed(&s.license) {
        return fail(format!("{id}: Lizenz {} nicht erlaubt (Q70)", s.license));
    }
    if s.status == VORRAT_BLOCKED || s.status == VORRAT_DROPPED {
        return fail(format!("{id} ist {}", s.status));
    }
    if s.size <= 0 || s.size > MAX_SUGGESTION_BYTES {
        return fail(format!("{id}: Größe {} außerhalb 1 … {MAX_SUGGESTION_BYTES}", s.size));
    }
    if kind_dir(&s.kind).is_none() {
        return fail(format!("{id}: Art {:?} unbekannt", s.kind));
    }
    let Some(url) = parse_with_host(&s.file_url) else {
        return fail(format!("{id}: download nur über https: {:?}", s.file_url));
    };
    if let Err(e) = vorrat_url_allowed(&url) {
        return fail(format!("{id}: {e}"));
    }
    let ext = fsutil::path_ext(&s.file_name).to_lowercase();
    if !ext.strip_prefix('.').is_some_and(|e| VORRAT_EXT.contains(&e)) {
        return fail(format!(
            "{id}: Endung {ext:?} nicht erlaubt (png, ogg, mp3, wav, zip, 7z)"
        ));
    }
    Ok((url, ext))
}

/// Nur https und nur ein Host aus [`VORRAT_HOSTS`].
pub fn vorrat_url_allowed(url: &Url) -> Result<()> {
    if is_test_origin(url) {
        return Ok(());
    }
    let host = url.host_str().unwrap_or_default();
    if url.scheme() != "https" || host.is_empty() {
        return Err(Error::Msg(format!("download nur über https: {:?}", url.as_str())));
    }
    let allowed = VORRAT_HOSTS.iter().any(|a| {
        host.strip_suffix(a)
            .is_some_and(|sub| sub.is_empty() || sub.ends_with('.'))
    });
    if !allowed {
        return Err(Error::Msg(format!("host {host:?} ist keine erlaubte Quelle")));
    }
    Ok(())
}

/// Weiterleitungs-Prüfung der Downloads: jedes Ziel wie die Start-URL, wie Go höchstens vier Sprünge (`len(via) >= 5`).
pub fn vorrat_redirect(url: &Url, hops: usize) -> Result<()> {
    if hops >= 5 {
        return Err(Error::Msg("zu viele Weiterleitungen".to_string()));
    }
    vorrat_url_allowed(url)
}

/// Lädt einen Eintrag in den Vorrat (Größe laut Liste, Limit, Signatur). Ein Fehler lässt nichts zurück; ist der
/// Eintrag schon geladen und passt die Prüfsumme der Datei, passiert nichts. `client` darf nicht selbst weiterleiten
/// ([`crate::net::client`]).
pub async fn vorrat_download(
    root: &Path,
    s: &Suggestion,
    client: &DownloadClient,
    now: DateTime<FixedOffset>,
    progress: Option<Progress>,
) -> Result<VorratEntry> {
    let (url, ext) = check_vorrat_download(s)?;
    if let Some(entry) = vorrat_have(root, &s.id) {
        return Ok(entry);
    }
    let rel = target_rel(s, &ext);
    let full = vorrat_path(root, &rel);
    let folder = full.parent().unwrap_or(root);
    fs::create_dir_all(folder)?;
    let (tmp, sha256) = fetch_vorrat(client, &url, folder, progress).await?;
    check_vorrat_file(tmp.path(), &ext, s.size).map_err(|e| Error::Msg(format!("{}: {e}", s.id)))?;
    fs::rename(tmp.path(), &full)?;
    let entry = VorratEntry {
        id: s.id.clone(),
        source: s.source.clone(),
        license: s.license.clone(),
        size: s.size,
        sha256,
        loaded: now.with_timezone(&Utc).to_rfc3339_opts(SecondsFormat::Secs, true),
        file: rel,
        public: String::new(),
    };
    let stored = change_vorrat(root, |index| {
        index.put(entry.clone());
        Ok(())
    });
    if stored.is_err() {
        let _ = fsutil::remove(&full);
    }
    stored.map(|()| entry)
}

/// `<art>/<quelle>/<id>-<name><endung>`; alle Teile aus der Liste laufen durch [`slug`].
fn target_rel(s: &Suggestion, ext: &str) -> String {
    let base = fsutil::path_base(&s.file_name);
    let stem = base.strip_suffix(fsutil::path_ext(&s.file_name)).unwrap_or(base);
    let dir = kind_dir(&s.kind).unwrap_or_default();
    let name = slug(&format!("{}-{stem}", s.id));
    format!("{dir}/{}/{name}{ext}", slug(&s.source))
}

/// Eintrag im Index, Datei vorhanden und ihre Prüfsumme passt.
fn vorrat_have(root: &Path, id: &str) -> Option<VorratEntry> {
    let index = {
        let _guard = lock_vorrat();
        load_vorrat(root).ok()?
    };
    let entry = index.find(id)?;
    let sum = file_sha256(&vorrat_path(root, &entry.file)).ok()?;
    (sum == entry.sha256).then(|| entry.clone())
}

fn hex(hash: Sha256) -> String {
    hash.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn file_sha256(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = vec![0; 32 << 10];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            return Ok(hex(hash));
        }
        hash.update(&buffer[..n]);
    }
}

/// Lädt `url` in eine Temp-Datei in `dir` (höchstens [`MAX_SUGGESTION_BYTES`]) und liefert sie mit ihrer SHA-256; bei
/// einem Fehler oder Abbruch verschwindet die Temp-Datei.
async fn fetch_vorrat(
    client: &DownloadClient,
    url: &Url,
    dir: &Path,
    progress: Option<Progress>,
) -> Result<(TempFile, String)> {
    let mut response = send_following(client, Method::GET, url, MAX_HOPS, vorrat_redirect).await?;
    if response.status() != StatusCode::OK {
        return Err(Error::Msg(format!("download: HTTP {}", response.status().as_u16())));
    }
    let (file, path) = fsutil::create_temp(dir, ".vorrat-")?;
    // Reihenfolge: `file` schließt vor dem Löschen durch `tmp`.
    let tmp = TempFile::new(path);
    let mut file = file;
    let mut hash = Sha256::new();
    let mut count = CountWriter { n: 0, progress };
    while let Some(chunk) = bounded(response.chunk()).await? {
        // Vor dem Schreiben prüfen: nie mehr als die Grenze auf der Platte.
        if count.n.saturating_add(chunk.len() as i64) > MAX_SUGGESTION_BYTES {
            return Err(Error::Msg(format!(
                "datei größer als {} MB",
                MAX_SUGGESTION_BYTES >> 20
            )));
        }
        file.write_all(&chunk)?;
        hash.update(&chunk);
        count.add(chunk.len());
    }
    file.sync_all()?;
    drop(file);
    fsutil::make_readable(tmp.path());
    Ok((tmp, hex(hash)))
}

/// Zählt die geschriebenen Bytes und meldet die Summe.
struct CountWriter {
    n: i64,
    progress: Option<Progress>,
}

impl CountWriter {
    fn add(&mut self, len: usize) {
        self.n += len as i64;
        if let Some(report) = &self.progress {
            report(self.n);
        }
    }
}

/// Prüft die Größe laut Liste und die Signatur (Kopf der Datei) zur Endung.
fn check_vorrat_file(file: &Path, ext: &str, want: i64) -> Result<()> {
    let size = fs::metadata(file)?.len();
    if i64::try_from(size) != Ok(want) {
        return Err(Error::Msg(format!("größe {size} statt {want} laut Liste")));
    }
    let mut head = Vec::new();
    File::open(file)?.take(16).read_to_end(&mut head)?;
    vorrat_signature(&head, ext)
}

/// Pakete (zip, 7z) selbst, Bilder und Töne wie der Import.
fn vorrat_signature(head: &[u8], ext: &str) -> Result<()> {
    let (magic, kind): (&[u8], &str) = match ext {
        ".zip" => (b"PK\x03\x04", "zip"),
        ".7z" => (b"7z\xBC\xAF\x27\x1C", "7z"),
        _ => return check_signature(head, ext),
    };
    if !head.starts_with(magic) {
        return Err(Error::Msg(format!("inhalt ist kein {kind}")));
    }
    Ok(())
}

#[cfg(test)]
#[path = "vorrat_tests.rs"]
mod tests;
