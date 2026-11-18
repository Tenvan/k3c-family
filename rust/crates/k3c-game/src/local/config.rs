//! Einstellungen der Session (Go: `Config`, `DefaultSavesDir` aus `session.go`), Zeit, Log und Uhr.

use std::fmt::Display;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use k3c_room::room::{Level, Log, NullLog};
use k3c_room::store::Stamp;

use super::watched::lock;

/// Zeitstempel der Spielstände und Insel-ID (nicht Spiellogik); der Zeit-Typ des Raums.
pub type Zeit = Stamp;
/// Ziel der Meldungen der Session (`room::Log`).
pub use k3c_room::room::Log as LogSink;

/// Config sind die Einstellungen der Session.
#[derive(Default)]
pub struct Config {
    /// Ordner der Spielstände; leer = `default_saves_dir()`
    pub dir: String,
    /// `None` = verwerfen
    pub log: Option<Box<dyn LogSink + Send>>,
    /// Uhr der Spielstände und der Insel-ID; `None` = Wanduhr
    pub now: Option<Box<dyn Fn() -> Zeit>>,
    /// Aufnahme-Modus. Go wartet im Takt auf die Schreib-Goroutine; `k3c-room` speichert synchron, das Warten entfällt
    /// (RU27.2).
    pub recording: bool,
}

/// Das Log der Session und ihres Managers: ein Ziel, von beiden geteilt.
#[derive(Clone)]
pub struct LogHandle(Arc<Mutex<Box<dyn LogSink + Send>>>);

impl LogHandle {
    /// `None` verwirft alle Meldungen (Go: Logger auf `io.Discard`).
    pub fn new(sink: Option<Box<dyn LogSink + Send>>) -> LogHandle {
        LogHandle(Arc::new(Mutex::new(sink.unwrap_or_else(|| Box::new(NullLog)))))
    }
}

impl Log for LogHandle {
    fn log(&mut self, level: Level, msg: &str, fields: &[(&str, &dyn Display)]) {
        lock(&self.0).log(level, msg, fields);
    }
}

/// DefaultSavesDir ist der Ordner `saves/` neben der EXE (Entscheidung B-793); ohne EXE-Pfad `saves/` im Arbeitsordner.
pub fn default_saves_dir() -> String {
    match std::env::current_exe() {
        Ok(exe) => exe
            .parent()
            .map_or_else(|| PathBuf::from("saves"), |d| d.join("saves"))
            .to_string_lossy()
            .into_owned(),
        Err(_) => "saves".to_string(),
    }
}

/// Die Wanduhr als `Zeit` (Standard von `Config::now`).
pub(super) fn wall_clock() -> Zeit {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    Stamp::new(i64::try_from(d.as_secs()).unwrap_or(i64::MAX), d.subsec_nanos())
}

/// Eine Uhr für `Saves` und `Manager` (sie wollen `FnMut() -> Stamp`).
pub(super) fn clock(now: &Arc<dyn Fn() -> Zeit>) -> impl FnMut() -> Stamp + 'static {
    let now = Arc::clone(now);
    move || now()
}
