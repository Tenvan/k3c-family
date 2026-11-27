//! Bausteine der Auswertung (Port von `transcribe.go`, B-1223): Aufruf des ffmpeg-Filters `whisper`, Sprachen, wählbare
//! Modelle und das Lesen der Rohausgabe. `Capability` steht in `capability.rs`, der Auftrag in `transcribe_job.rs`.

use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use super::capability::model_path;
use super::stats::Segment;
use super::{DIR_FRAMES, Error, FILE_VOICE, null_default};

/// Rohausgabe des Filters.
pub(super) const FILE_RAW_TMP: &str = "transcript.part";

/// Stückgröße in Sekunden, die der Filter am Stück erkennt. Der Standard (3 s) zerhackt Sätze und lässt das Modell bei
/// Pausen „Vielen Dank.“ erfinden; 20 bis 30 s erkannten die Sätze am besten, whisper.cpp arbeitet in Fenstern von 30 s.
const WHISPER_QUEUE: &str = "30";

/// Sprachen der Auswahl (Standard de).
const LANGUAGES: [&str; 3] = ["de", "en", "auto"];

/// Aufruf im Session-Ordner `dir` (relative Pfade, damit `C:` im Filter nicht gequotet werden muss); das Ergebnis des
/// Filters steht roh in `dest` (Zeilen JSON, Zeiten in ms ab Dateianfang).
pub fn whisper_args(ffmpeg: &str, dir: &Path, model: &str, language: &str, dest: &str) -> Vec<String> {
    let model = filter_path(dir, Path::new(model));
    let filter =
        format!("whisper=model={model}:language={language}:queue={WHISPER_QUEUE}:format=json:destination={dest}");
    [
        ffmpeg,
        "-hide_banner",
        "-loglevel",
        "warning",
        "-nostats",
        "-y",
        "-i",
        FILE_VOICE,
        "-vn",
        "-af",
        &filter,
        "-f",
        "null",
        "-",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// Prüft die Sprache; leer ist de.
pub(super) fn parse_language(language: &str) -> Result<String, Error> {
    let language = language.trim();
    if language.is_empty() {
        return Ok("de".to_owned());
    }
    if LANGUAGES.contains(&language) {
        return Ok(language.to_owned());
    }
    let allowed = LANGUAGES.join(", ");
    let shown = crate::planning::store::quoted(language);
    Err(Error::Other(format!("Sprache {shown} ungültig: erlaubt {allowed}")))
}

/// Eine gefundene Modelldatei für die Auswahl der Seite. `id` ist, was die Seite zurückgibt: der Pfad relativ zur
/// Repo-Wurzel mit `/` oder `env` für `K3C_WHISPER_MODEL`; nie ein Pfad vom Client.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size_mb: u64,
    pub default: bool,
    #[serde(skip)]
    pub path: String,
}

/// Die Dateien `*.bin` eines Ordners, nach Namen sortiert.
fn bins(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| name.ends_with(".bin"))
        .collect();
    names.sort();
    names
}

/// Die Dateien aus `K3C_WHISPER_MODEL` (`env`), `models/*.bin` und `models/whisper/*.bin`; der Standard
/// ([`model_path`]) ist markiert.
pub fn list_models(root: &Path, env: &str) -> Vec<ModelInfo> {
    let default = model_path(root, env);
    let mut out = Vec::new();
    let mut add = |id: String, path: String| {
        let Ok(meta) = std::fs::metadata(&path) else { return };
        if meta.is_dir() {
            return;
        }
        let name = Path::new(&path).file_name().map(|n| n.to_string_lossy().into_owned());
        out.push(ModelInfo {
            id,
            name: name.unwrap_or_default(),
            size_mb: meta.len() >> 20,
            default: path == default,
            path,
        });
    };
    if !env.trim().is_empty() {
        add("env".to_owned(), env.trim().to_owned());
    }
    for (prefix, dir) in [
        ("models", root.join("models")),
        ("models/whisper", root.join("models").join("whisper")),
    ] {
        for name in bins(&dir) {
            add(
                format!("{prefix}/{name}"),
                dir.join(&name).to_string_lossy().into_owned(),
            );
        }
    }
    out
}

/// Wie Go `filepath.Rel` auf bereinigten Pfaden; `None`, wenn es keinen relativen Weg gibt (anderes Laufwerk).
fn relative(base: &Path, target: &Path) -> Option<String> {
    let parts = |path: &Path| {
        let mut anchor = String::new();
        let mut names: Vec<String> = Vec::new();
        for part in path.components() {
            match part {
                Component::Prefix(p) => anchor.push_str(&p.as_os_str().to_string_lossy().to_lowercase()),
                Component::RootDir => anchor.push('/'),
                Component::CurDir => {}
                Component::ParentDir if names.last().is_some_and(|n| n != "..") => {
                    names.pop();
                }
                other => names.push(other.as_os_str().to_string_lossy().into_owned()),
            }
        }
        (anchor, names)
    };
    let ((base_anchor, base), (target_anchor, target)) = (parts(base), parts(target));
    if base_anchor != target_anchor {
        return None;
    }
    let same = base
        .iter()
        .zip(&target)
        .take_while(|(a, b)| {
            if cfg!(windows) {
                a.to_lowercase() == b.to_lowercase()
            } else {
                a == b
            }
        })
        .count();
    if base.iter().skip(same).any(|name| name == "..") {
        return None;
    }
    let up = std::iter::repeat_n("..", base.len() - same);
    let out: Vec<&str> = up.chain(target.iter().skip(same).map(String::as_str)).collect();
    Some(if out.is_empty() { ".".to_owned() } else { out.join("/") })
}

/// Die Modelldatei relativ zum Arbeitsordner; auf anderem Laufwerk bleibt der Pfad absolut, dann wird `:` für den
/// Filter-Parser maskiert.
fn filter_path(dir: &Path, model: &Path) -> String {
    relative(dir, model).unwrap_or_else(|| model.to_string_lossy().replace('\\', "/").replace(':', r"\\:"))
}

/// Pfad des Bilds zu Marker `nr` (1-basiert, Zeilennummer in `markers.jsonl`), relativ zur Session.
pub fn frame_name(nr: i64) -> String {
    format!("{DIR_FRAMES}/marker-{nr}.jpg")
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct RawSegment {
    #[serde(deserialize_with = "null_default")]
    start: i64,
    #[serde(deserialize_with = "null_default")]
    end: i64,
    #[serde(deserialize_with = "null_default")]
    text: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Wrapped {
    #[serde(deserialize_with = "null_default")]
    segments: Vec<Option<RawSegment>>,
}

/// Liest die Rohausgabe des Filters (eine JSON-Zeile je Satz, auch Feld oder `{"segments":…}`) und rechnet die Zeiten ab
/// Dateianfang mit `audio_start` in Unix-ms um; Leeres entfällt.
pub(super) fn parse_segments(raw: &[u8], audio_start: i64) -> Result<Vec<Segment>, Error> {
    let failed = |e: serde_json::Error| Error::Other(format!("Transkript nicht lesbar: {e}"));
    let text = String::from_utf8_lossy(raw);
    let text = text.trim();
    let list: Vec<Option<RawSegment>> = if text.starts_with('[') {
        serde_json::from_str(text).map_err(failed)?
    } else if text.starts_with(r#"{"segments""#) {
        serde_json::from_str::<Wrapped>(text).map_err(failed)?.segments
    } else {
        text.split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).map_err(failed))
            .collect::<Result<_, _>>()?
    };
    Ok(list
        .into_iter()
        .flatten()
        .filter(|s| !s.text.trim().is_empty())
        .map(|s| Segment {
            start_ms: audio_start.saturating_add(s.start),
            end_ms: audio_start.saturating_add(s.end),
            text: s.text.trim().to_owned(),
        })
        .collect())
}

#[cfg(test)]
#[path = "transcribe_tests.rs"]
mod tests;
