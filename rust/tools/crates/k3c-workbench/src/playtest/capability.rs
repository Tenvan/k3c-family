//! Ist die Auswertung möglich? (`Check`, `Capability` und `ModelPath` aus `transcribe.go` und `transcribe_job.go`;
//! `PlaytestCheck` braucht sie schon vor der Auswertung aus RU69.14.)

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use super::devices::{Probe, strings};
use super::ffmpeg_find::LookPath;
use super::no_ffmpeg;

pub(super) const MODEL_FILE: &str = "ggml-large-v3.bin";
pub(super) const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp";
/// SHA-256 von [`MODEL_FILE`] (3095033483 Byte).
pub(super) const MODEL_SHA: &str = "64d182b440b98d5203c4f9bd541544d84c605196c4f7b845dfa11fb23594d1e2";
/// Pfad der Modelldatei; sonst `models/<MODEL_FILE>`, sonst `models/whisper/*.bin`.
pub const MODEL_ENV: &str = "K3C_WHISPER_MODEL";
/// Kleinste ffmpeg-Hauptversion.
const MIN_FFMPEG: u32 = 8;

static VERSION: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"ffmpeg version\s+n?([0-9]+)\.?([0-9]*)[.0-9A-Za-z_-]*").ok());

/// Ist die Auswertung möglich, und sonst warum nicht (mit Hinweis, was zu tun ist).
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub ready: bool,
    /// Version, leer wenn ffmpeg fehlt.
    pub ffmpeg: String,
    pub whisper: bool,
    pub model: String,
    pub model_ok: bool,
    /// Größe der Modelldatei in MB (0: fehlt).
    pub model_mb: u64,
    pub message: String,
}

/// Die erste Datei `*.bin` eines Ordners (alphabetisch).
fn first_bin(dir: &Path) -> Option<PathBuf> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| name.ends_with(".bin"))
        .collect();
    names.sort();
    names.first().map(|name| dir.join(name))
}

/// Die Modelldatei: `K3C_WHISPER_MODEL` (`env`), dann `models/<MODEL_FILE>`, dann `models/whisper/*.bin`. Ohne Datei
/// der erste Kandidat (für die Meldung). Nichts wird automatisch geladen.
pub fn model_path(root: &Path, env: &str) -> String {
    if !env.trim().is_empty() {
        return env.trim().to_owned();
    }
    let text = |path: PathBuf| path.to_string_lossy().into_owned();
    let default = root.join("models").join(MODEL_FILE);
    if default.is_file() {
        return text(default);
    }
    // Sonst irgendein Modell, damit eine andere Datei die Auswertung nicht sperrt.
    [root.join("models"), root.join("models").join("whisper")]
        .iter()
        .find_map(|dir| first_bin(dir))
        .map_or_else(|| text(default), text)
}

/// Version und Hauptversion aus `ffmpeg -version` (0, wenn nicht erkennbar; Git-Builds ohne Zahl).
fn parse_version(out: &str) -> (String, u32) {
    let Some(found) = VERSION.as_ref().and_then(|re| re.captures(out)) else {
        return (String::new(), 0);
    };
    let major = found.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
    let whole = found.get(0).map_or("", |m| m.as_str());
    let version = whole.trim_start_matches("ffmpeg version").split_whitespace().next();
    (version.unwrap_or_default().to_owned(), major)
}

/// Steht der Audio-Filter `whisper` in `ffmpeg -filters`?
fn has_whisper(filters: &str) -> bool {
    filters
        .split('\n')
        .any(|line| line.split_whitespace().nth(1) == Some("whisper"))
}

/// Prüft ffmpeg (Version ab 8, Filter `whisper`) und die Modelldatei; `message` nennt, was zu tun ist. Blockiert.
pub fn check(look_path: &LookPath, run: &Probe, model: &str) -> Capability {
    let mut c = Capability {
        model: model.to_owned(),
        ..Capability::default()
    };
    if let Ok(meta) = std::fs::metadata(model)
        && !meta.is_dir()
    {
        (c.model_ok, c.model_mb) = (true, meta.len() >> 20);
    }
    let ffmpeg = match look_path("ffmpeg") {
        Ok(path) => path,
        Err(e) => {
            c.message = no_ffmpeg(e).to_string();
            return c;
        }
    };
    let (out, _) = run(&strings(&[&ffmpeg, "-hide_banner", "-version"]));
    let (version, major) = parse_version(&out);
    c.ffmpeg.clone_from(&version);
    if major < MIN_FFMPEG {
        let shown = if version.is_empty() { "(unbekannt)" } else { &version };
        c.message = format!(
            "ffmpeg {shown} ist zu alt, nötig ist Version {MIN_FFMPEG} oder neuer mit whisper-Filter (gyan full build)"
        );
        return c;
    }
    let (filters, _) = run(&strings(&[&ffmpeg, "-hide_banner", "-filters"]));
    c.whisper = has_whisper(&filters);
    if !c.whisper {
        c.message = format!(
            "dieses ffmpeg hat keinen whisper-Filter (ffmpeg {version}); die Version „full build“ von gyan.dev installieren"
        );
        return c;
    }
    if !c.model_ok {
        c.message = format!(
            "Modell fehlt: {model}. Datei {MODEL_FILE} (3095033483 Byte, SHA-256 {MODEL_SHA}) von {MODEL_URL} von Hand \
             laden und nach models/ legen (oder models/whisper/*.bin bzw. Pfad in {MODEL_ENV}); die Workbench lädt \
             nichts selbst"
        );
        return c;
    }
    c.ready = true;
    c
}

#[cfg(test)]
#[path = "capability_tests.rs"]
mod tests;
