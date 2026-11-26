//! Einfaches Datei-Log des Spikes: jede Zeile mit Millisekunden seit Prozessstart, Emoji vorn (Projektregel).
//! Ziel: `<spike>/logs/spike.log` (oder `SPIKE_LOG_DIR`), damit Command, Event und MCP ohne Fenster belegbar sind.

use std::fs::{File, OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

static START: OnceLock<Instant> = OnceLock::new();
static FILE: OnceLock<Mutex<Option<File>>> = OnceLock::new();

/// Spike-Ordner (`.work/spike-tauri`), zur Bauzeit aus dem Manifest abgeleitet.
pub fn spike_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// Startzeit merken; früh in `main` aufrufen.
pub fn init() {
    START.get_or_init(Instant::now);
    let dir = std::env::var_os("SPIKE_LOG_DIR").map(PathBuf::from).unwrap_or_else(|| spike_dir().join("logs"));
    let file = create_dir_all(&dir)
        .ok()
        .and_then(|_| OpenOptions::new().create(true).append(true).open(dir.join("spike.log")).ok());
    FILE.get_or_init(|| Mutex::new(file));
}

/// Millisekunden seit `init`.
pub fn since_start_ms() -> u128 {
    START.get_or_init(Instant::now).elapsed().as_millis()
}

/// Eine Zeile in Datei und stderr schreiben.
pub fn line(msg: &str) {
    let epoch = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
    let text = format!("{epoch} +{}ms pid={} {msg}\n", since_start_ms(), std::process::id());
    eprint!("{text}");
    if let Some(m) = FILE.get() {
        if let Ok(mut guard) = m.lock() {
            if let Some(f) = guard.as_mut() {
                let _ = f.write_all(text.as_bytes());
            }
        }
    }
}
