//! Neue Vorschläge anhängen (B-673, Go `suggestions_add.go`): eine Funktion für Studio (Chat, Recherche-Übernahme) und
//! Workbench. Jeder Eintrag durchläuft `check_suggestion`; ungültige und doppelte (nach Datei-URL) werden mit Grund
//! übersprungen, die übrigen in einem Schritt atomar geschrieben. Geladen wird nichts.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::future::Future;
use std::io;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::CONTENT_LENGTH;
use reqwest::{Method, StatusCode};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::error::{Error, Result};
use crate::fsutil;
use crate::import_download::MAX_HOPS;
use crate::net::{DownloadClient, parse_with_host, send_following, url_path};
use crate::register::{Register, load};
use crate::research::from_object;
use crate::suggestions::{
    SUGGESTION_SOURCES, SUGGESTIONS_PATH, Suggestion, Suggestions, check_suggestion, is_https, suggestion_targets,
};
use crate::suggestions_write::lock_suggestions;
use crate::vorrat_download::{vorrat_redirect, vorrat_url_allowed};

/// Frist je HEAD-Anfrage (Größe einer Datei).
const HEAD_TIMEOUT: Duration = Duration::from_secs(10);

/// Die Vorschlagsliste hat sich während der Prüfung geändert, es wurde nichts geschrieben.
pub const ERR_SUGGESTIONS_CHANGED: &str =
    "die Vorschlagsliste wurde währenddessen geändert, nichts geschrieben: bitte neu laden";

/// Liefert die Größe der Datei unter der Adresse in Bytes (im Betrieb per HEAD, im Test ein Ersatz).
pub type SizeFunc = Arc<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<i64>> + Send>> + Send + Sync>;

/// Kandidat, der nicht angehängt wird, mit Grund.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SkippedSuggestion {
    #[serde(rename = "titel")]
    pub title: String,
    #[serde(rename = "dateiUrl")]
    pub file_url: String,
    #[serde(rename = "grund")]
    pub reason: String,
}

/// Ergebnis (oder Plan) eines Anhängens.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct SuggestionAdd {
    #[serde(rename = "uebernommen")]
    pub added: Vec<Suggestion>,
    #[serde(rename = "uebersprungen")]
    pub skipped: Vec<SkippedSuggestion>,
    pub bytes: i64,
}

/// Fragt die Größe per HEAD (Content-Length); das Ziel muss eine erlaubte Quelle sein, Weiterleitungen werden geprüft.
pub fn head_size(client: DownloadClient) -> SizeFunc {
    Arc::new(move |raw_url: String| {
        let client = client.clone();
        Box::pin(async move { head(&client, &raw_url).await })
    })
}

async fn head(client: &DownloadClient, raw_url: &str) -> Result<i64> {
    let url = parse_with_host(raw_url).ok_or_else(|| Error::Msg(format!("download nur über https: {raw_url:?}")))?;
    vorrat_url_allowed(&url)?;
    let request = send_following(client, Method::HEAD, &url, MAX_HOPS, vorrat_redirect);
    // Ein hängender Host blockiert nicht den ganzen Lauf.
    let response = tokio::time::timeout(HEAD_TIMEOUT, request)
        .await
        .map_err(|_| Error::Msg("zeitüberschreitung beim Laden".to_string()))??;
    if response.status() != StatusCode::OK {
        return Err(Error::Msg(format!("HEAD: HTTP {}", response.status().as_u16())));
    }
    // Die Kopfzeile selbst: `content_length()` meldet bei HEAD die Länge des leeren Körpers.
    response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<i64>().ok())
        .filter(|&n| n > 0)
        .ok_or_else(|| Error::Msg("HEAD: keine Größe angegeben".to_string()))
}

/// Liest die Liste samt Prüfsumme ihrer Bytes (leer, wenn die Datei fehlt).
fn load_suggestions_stamped(root: &Path) -> Result<(Suggestions, String)> {
    let bytes = match fs::read(root.join(SUGGESTIONS_PATH)) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok((Suggestions::default(), String::new()));
        }
        Err(e) => return Err(Error::Io(e)),
    };
    let list = from_object(&bytes).map_err(|e| Error::Msg(format!("{SUGGESTIONS_PATH}: {e}")))?;
    let sum = Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect();
    Ok((list, sum))
}

/// Prüft die Kandidaten gegen Register und Liste und nennt, was angehängt würde; schreibt nichts. Ohne `size` bleibt
/// eine fehlende Größe fehlend.
pub async fn plan_suggestions(root: &Path, input: &[Suggestion], size: Option<&SizeFunc>) -> Result<SuggestionAdd> {
    Ok(plan(root, input, size).await?.0)
}

/// Hängt die gültigen Kandidaten als „offen“ an die Liste und schreibt sie atomar; die übrigen stehen mit Grund in
/// `skipped`. Fehlt die Größe, fragt `size` die Datei. Ändert sich die Datei währenddessen:
/// [`ERR_SUGGESTIONS_CHANGED`].
///
/// Grenze: Die Sperre gilt nur im eigenen Prozess; schreibt ein anderer Prozess genau zwischen Vergleich und
/// `write_atomic`, gewinnt der spätere Schreiber. Bewusst ohne Lock-Datei, weil das Fenster sehr klein ist.
pub async fn add_suggestions(root: &Path, input: &[Suggestion], size: Option<&SizeFunc>) -> Result<SuggestionAdd> {
    let (result, mut list, stamp) = plan(root, input, size).await?;
    if result.added.is_empty() {
        return Ok(result);
    }
    let _guard = lock_suggestions();
    if load_suggestions_stamped(root)?.1 != stamp {
        return Err(Error::Msg(ERR_SUGGESTIONS_CHANGED.to_string()));
    }
    list.list.extend(result.added.iter().cloned());
    list.save(root)?;
    Ok(result)
}

/// Wogegen ein Kandidat geprüft wird.
struct Known {
    register: Register,
    targets: BTreeMap<String, String>,
    urls: HashSet<String>,
}

async fn plan(
    root: &Path,
    input: &[Suggestion],
    size: Option<&SizeFunc>,
) -> Result<(SuggestionAdd, Suggestions, String)> {
    let (list, stamp) = load_suggestions_stamped(root)?;
    let register = load(root)?;
    let mut known = Known {
        targets: suggestion_targets(&register),
        register,
        urls: list.list.iter().map(|s| s.file_url.clone()).collect(),
    };
    let mut next = list
        .list
        .iter()
        .filter_map(|s| s.id.strip_prefix('v').unwrap_or(&s.id).parse::<i64>().ok())
        .fold(0, i64::max);
    let mut result = SuggestionAdd::default();
    for candidate in input {
        match prepare(candidate.clone(), &known, size).await {
            Ok(mut s) => {
                next = next.saturating_add(1);
                s.id = format!("v{next:03}");
                known.urls.insert(s.file_url.clone());
                result.bytes = result.bytes.saturating_add(s.size);
                result.added.push(s);
            }
            Err(e) => result.skipped.push(SkippedSuggestion {
                title: candidate.title.clone(),
                file_url: candidate.file_url.clone(),
                reason: e.to_string(),
            }),
        }
    }
    Ok((result, list, stamp))
}

/// Ergänzt Vorgaben (Status offen, Priorität 2, Dateiname, Größe) und prüft den Kandidaten.
async fn prepare(mut s: Suggestion, known: &Known, size: Option<&SizeFunc>) -> Result<Suggestion> {
    let fail = |text: String| Err(Error::Msg(text));
    s.file_url = s.file_url.trim().to_string();
    if known.urls.contains(&s.file_url) {
        return fail("Datei-URL schon in der Liste".to_string());
    }
    let source = s.source.to_lowercase();
    let Some(name) = SUGGESTION_SOURCES.iter().find(|n| n.to_lowercase() == source) else {
        return fail("Quelle nicht erlaubt".to_string());
    };
    s.source = (*name).to_string();
    let slot_kind = || known.register.find(&s.slot).map(|slot| &slot.kind);
    let Some(kind) = known.targets.get(&s.slot).or_else(slot_kind) else {
        return fail(format!("slot {:?} nicht im Register", s.slot));
    };
    if &s.kind != kind {
        return fail(format!("art {:?} passt nicht zum Slot ({kind})", s.kind));
    }
    s.status = "offen".to_string();
    if s.priority == 0 {
        s.priority = 2;
    }
    if s.file_name.is_empty()
        && let Some(path) = url_path(&s.file_url)
    {
        s.file_name = fsutil::path_base(&path).to_string();
    }
    if s.size <= 0
        && let Some(size) = size
        && is_https(&s.page)
        && is_https(&s.file_url)
    {
        s.size = size(s.file_url.clone())
            .await
            .map_err(|e| Error::Msg(format!("Größe nicht ermittelbar: {e}")))?;
    }
    s.id = "neu".to_string();
    check_suggestion(&s)?;
    let url =
        parse_with_host(&s.file_url).ok_or_else(|| Error::Msg(format!("download nur über https: {:?}", s.file_url)))?;
    vorrat_url_allowed(&url)?;
    Ok(s)
}

#[cfg(test)]
#[path = "suggestions_add_tests.rs"]
mod tests;
