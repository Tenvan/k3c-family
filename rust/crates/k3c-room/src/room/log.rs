//! Log des Raums ohne `slog`: `k3c-room` hängt nicht an `k3c-platform` (das zieht Fenster- und Grafik-Abhängigkeiten in
//! einen Sim-Crate). Der Aufrufer (RU27) verdrahtet `k3c_platform::log`. Jede Meldung beginnt mit einem Emoji aus
//! `CLAUDE.md` (Leitfaden § 10), Texte und Feld-Reihenfolge wie in Go.
use std::fmt::Display;

/// Stufe einer Meldung (Go `slog.Level`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

/// Ziel der Meldungen. `fields` sind Name/Wert-Paare in Go-Reihenfolge.
pub trait Log {
    fn log(&mut self, level: Level, msg: &str, fields: &[(&str, &dyn Display)]);

    fn info(&mut self, msg: &str, fields: &[(&str, &dyn Display)]) {
        self.log(Level::Info, msg, fields);
    }
    fn warn(&mut self, msg: &str, fields: &[(&str, &dyn Display)]) {
        self.log(Level::Warn, msg, fields);
    }
    fn error(&mut self, msg: &str, fields: &[(&str, &dyn Display)]) {
        self.log(Level::Error, msg, fields);
    }
}

/// Der Standard: verwirft alles (Go: Logger auf `io.Discard`).
#[derive(Debug, Clone, Copy, Default)]
pub struct NullLog;

impl Log for NullLog {
    fn log(&mut self, _level: Level, _msg: &str, _fields: &[(&str, &dyn Display)]) {}
}

/// Sammelt die Zeilen als Text `Meldung name=wert …` (Tests).
#[derive(Debug, Clone, Default)]
pub struct RecordingLog {
    pub lines: Vec<String>,
}

impl Log for RecordingLog {
    fn log(&mut self, level: Level, msg: &str, fields: &[(&str, &dyn Display)]) {
        let mut line = format!("{level:?} {msg}");
        for (k, v) in fields {
            line.push_str(&format!(" {k}={v}"));
        }
        self.lines.push(line);
    }
}

/// Kürzt eine ID auf acht Zeichen (nach Zeichen, nicht nach Bytes).
pub fn short(id: &str) -> String {
    id.chars().take(8).collect()
}

/// Eine Zahlenliste wie Go `%v`: `[0 1 2]`.
pub fn list_text(items: &[i64]) -> String {
    let parts: Vec<String> = items.iter().map(i64::to_string).collect();
    format!("[{}]", parts.join(" "))
}
