//! Spieltests (Port von `internal/playtest`, B-1218): Ablage `playtests/<JJJJ-MM-TT>-<nr>/`, Geräte und Aufnahme.
//! Ordner-Vertrag: `session.json`, `voice.wav`, `video.mkv`, `markers.jsonl`, `survey.json`, `transcript.json`,
//! `frames/`, `findings.json`; Zeitstempel überall Unix-Millisekunden der Systemuhr. Prozesse (ffmpeg, Spiel) entstehen
//! nur über austauschbare Funktionen ([`Spawn`], [`Probe`]), damit Tests Attrappen einsetzen.

mod capability;
mod devices;
mod encoders;
mod exec;
mod ffmpeg_find;
mod frames;
mod level;
#[allow(clippy::module_inception)]
mod playtest;
mod record;
mod start;
mod stats;
mod watch;

#[cfg(test)]
pub(crate) mod rig_tests;

use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

pub use capability::{Capability, MODEL_ENV, check, model_path};
pub use devices::{Monitor, Probe, check_devices};
pub use encoders::{PickCache, pick_encoder};
pub use exec::{Devices, Proc, Spawn, exec_spawn, game_finder, list_devices, run_output};
pub use ffmpeg_find::{FFMPEG_ENV, LookPath, find_ffmpeg};
pub use frames::{capture_start, frame_args, frame_at};
pub use playtest::{Action, File, Meta, Plan, Session, Store};
pub use record::{Config, Options, Recorder, Status, ffmpeg_args, game_args, voice_args};
pub use stats::{EvalStats, Segment, Transcript, make_stats, save_stats};
pub use watch::Stats;

/// Dateien des Ordner-Vertrags, die das Modul kennt.
const FILE_META: &str = "session.json";
const FILE_VIDEO: &str = "video.mkv";
const FILE_VOICE: &str = "voice.wav";
const FILE_MARKERS: &str = "markers.jsonl";
const FILE_TRANSCRIPT: &str = "transcript.json";
const FILE_FINDINGS: &str = "findings.json";
const DIR_FRAMES: &str = "frames";

/// Fehler; die Oberfläche zeigt den Text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BadId,
    BadAction,
    NotFound,
    Running,
    NotConfirmed,
    Busy,
    NotRunning,
    /// Die geprüften Orte, wenn die Suche sie nennt.
    NoFfmpeg(Vec<String>),
    NoGame,
    NoMic,
    BadMonitor,
    NoVideo,
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Error::BadId => "ungültige Session-Kennung",
            Error::BadAction => "unbekannte Aktion (video, session oder ausgewertet)",
            Error::NotFound => "Session oder Datei nicht gefunden",
            Error::Running => "Aufnahme läuft, Löschen gesperrt",
            Error::NotConfirmed => "Löschen nicht bestätigt",
            Error::Busy => "Aufnahme läuft schon",
            Error::NotRunning => "keine Aufnahme aktiv",
            Error::NoFfmpeg(tried) => {
                f.write_str("ffmpeg nicht gefunden (ffmpeg installieren)")?;
                if tried.is_empty() {
                    return Ok(());
                }
                let places = tried.join(", ");
                return write!(f, "; geprüft: {places}; oder {FFMPEG_ENV} auf die ffmpeg.exe setzen");
            }
            Error::NoGame => "k3c-game.exe nicht gefunden (task build)",
            Error::NoMic => "kein Mikrofon gewählt",
            Error::BadMonitor => "ungültiger Monitor (0 bis 15)",
            Error::NoVideo => "video.mkv fehlt (gelöscht?), kein Bild möglich",
            Error::Other(text) => text,
        };
        f.write_str(text)
    }
}

/// Fehler der Suche als [`Error::NoFfmpeg`]; hat sie die geprüften Orte genannt, bleibt der Text.
fn no_ffmpeg(error: Error) -> Error {
    match error {
        Error::NoFfmpeg(_) => error,
        _ => Error::NoFfmpeg(Vec::new()),
    }
}

/// Meldung ins Log der App: Stufe (`info`, `warn`), Text, Felder.
pub type LogFn = Arc<dyn Fn(&str, &str, &[(&str, serde_json::Value)]) + Send + Sync>;

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Ein einmal gesetzter Wert, auf den mehrere warten (Go: `chan` und `close`).
pub struct Signal<T> {
    value: Mutex<Option<T>>,
    changed: Condvar,
}

impl<T: Clone> Signal<T> {
    pub fn new() -> Arc<Signal<T>> {
        Arc::new(Signal {
            value: Mutex::new(None),
            changed: Condvar::new(),
        })
    }

    /// Der erste Wert gewinnt.
    pub fn set(&self, value: T) {
        lock(&self.value).get_or_insert(value);
        self.changed.notify_all();
    }

    pub fn is_set(&self) -> bool {
        lock(&self.value).is_some()
    }

    /// Blockiert bis zum Wert.
    pub fn wait(&self) -> T {
        let mut guard = lock(&self.value);
        loop {
            if let Some(value) = guard.as_ref() {
                return value.clone();
            }
            guard = self.changed.wait(guard).unwrap_or_else(PoisonError::into_inner);
        }
    }

    /// Wartet höchstens `limit`.
    pub fn wait_timeout(&self, limit: Duration) -> Option<T> {
        let guard = lock(&self.value);
        let (guard, _) = self
            .changed
            .wait_timeout_while(guard, limit, |value| value.is_none())
            .unwrap_or_else(PoisonError::into_inner);
        guard.clone()
    }
}

/// JSON wie Go `json.MarshalIndent(v, "", "  ")`: zwei Leerzeichen, `<`, `>`, `&`, U+2028 und U+2029 maskiert (sie
/// stehen in gültigem JSON nur innerhalb von Zeichenketten).
fn go_json(value: &impl serde::Serialize) -> serde_json::Result<String> {
    let mut out = serde_json::to_string_pretty(value)?;
    for (raw, code) in [
        ('<', "003c"),
        ('>', "003e"),
        ('&', "0026"),
        ('\u{2028}', "2028"),
        ('\u{2029}', "2029"),
    ] {
        if out.contains(raw) {
            out = out.replace(raw, &format!("\\u{code}"));
        }
    }
    Ok(out)
}

/// `null` gilt wie ein fehlendes Feld (Go lässt den Nullwert stehen).
fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let value: Option<T> = serde::Deserialize::deserialize(d)?;
    Ok(value.unwrap_or_default())
}
