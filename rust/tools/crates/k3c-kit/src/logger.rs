//! Strukturiertes Logging, Gegenstück zu Go `log/slog`: ein [`Logger`] gibt jeden [`Record`] an seine [`Sink`]s weiter.
//! Senken sind die Konsolenzeile ([`crate::conlog::ConsoleSink`]) und die JSON-Datei ([`crate::applog`]).
//!
//! ```
//! use k3c_kit::logger::Logger;
//! use serde_json::json;
//!
//! let log = Logger::discard();
//! let url = "http://127.0.0.1:5280/mcp";
//! log.info("🔁 server neu gestartet", &[("ns", json!("mcp")), ("url", json!(url))]);
//! ```
//!
//! `WithGroup` aus slog ist nicht portiert (kein Aufrufer in Workbench, Studio und workkit).

use std::fmt;
use std::sync::Arc;

use chrono::{DateTime, Local, Offset, Timelike};
use serde_json::Value;

/// Stufe eines Eintrags; Reihenfolge und Werte wie slog (−4, 0, 4, 8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    /// Zahlenwert wie `slog.Level`.
    pub fn value(self) -> i32 {
        match self {
            Level::Debug => -4,
            Level::Info => 0,
            Level::Warn => 4,
            Level::Error => 8,
        }
    }

    /// Name wie `slog.Level.String()`: `DEBUG`, `INFO`, `WARN`, `ERROR`.
    pub fn name(self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Attribute eines Eintrags in Reihenfolge (doppelte Schlüssel bleiben, wie slog).
pub type Attrs = Vec<(String, Value)>;

/// Ein Eintrag: Attribute aus [`Logger::with`] stehen vor denen des Aufrufs.
#[derive(Clone, Debug)]
pub struct Record {
    pub time: DateTime<Local>,
    pub level: Level,
    pub msg: String,
    pub attrs: Attrs,
}

/// Ziel von Einträgen (slog.Handler ohne Gruppen).
pub trait Sink: Send + Sync {
    /// Ob die Senke Einträge dieser Stufe schreibt.
    fn enabled(&self, level: Level) -> bool;
    /// Schreibt einen Eintrag.
    fn handle(&self, r: &Record) -> std::io::Result<()>;
}

/// Logger mit festen Senken und vorab gesetzten Attributen; günstig zu klonen.
#[derive(Clone)]
pub struct Logger {
    sinks: Arc<Vec<Box<dyn Sink>>>,
    attrs: Arc<Attrs>,
}

impl Logger {
    /// Logger über diese Senken (jeder Eintrag geht an alle, die ihn wollen).
    pub fn new(sinks: Vec<Box<dyn Sink>>) -> Logger {
        Logger {
            sinks: Arc::new(sinks),
            attrs: Arc::new(Vec::new()),
        }
    }

    /// Logger, der nichts schreibt (Tests, Server ohne Log).
    pub fn discard() -> Logger {
        Logger::new(Vec::new())
    }

    /// Neuer Logger mit zusätzlichen Attributen vor denen jedes Aufrufs (slog `Logger.With`).
    pub fn with(&self, attrs: &[(&str, Value)]) -> Logger {
        let mut all = (*self.attrs).clone();
        all.extend(attrs.iter().map(|(k, v)| ((*k).to_string(), v.clone())));
        Logger {
            sinks: Arc::clone(&self.sinks),
            attrs: Arc::new(all),
        }
    }

    /// Ob irgendeine Senke diese Stufe schreibt.
    pub fn enabled(&self, level: Level) -> bool {
        self.sinks.iter().any(|s| s.enabled(level))
    }

    /// Schreibt einen Eintrag an alle Senken, die die Stufe wollen; Fehler einer Senke werden verworfen (wie slog).
    pub fn log(&self, level: Level, msg: &str, attrs: &[(&str, Value)]) {
        if !self.enabled(level) {
            return;
        }
        let mut all = (*self.attrs).clone();
        all.extend(attrs.iter().map(|(k, v)| ((*k).to_string(), v.clone())));
        let r = Record {
            time: Local::now(),
            level,
            msg: msg.to_string(),
            attrs: all,
        };
        for s in self.sinks.iter().filter(|s| s.enabled(level)) {
            let _ = s.handle(&r);
        }
    }

    pub fn debug(&self, msg: &str, attrs: &[(&str, Value)]) {
        self.log(Level::Debug, msg, attrs);
    }

    pub fn info(&self, msg: &str, attrs: &[(&str, Value)]) {
        self.log(Level::Info, msg, attrs);
    }

    pub fn warn(&self, msg: &str, attrs: &[(&str, Value)]) {
        self.log(Level::Warn, msg, attrs);
    }

    pub fn error(&self, msg: &str, attrs: &[(&str, Value)]) {
        self.log(Level::Error, msg, attrs);
    }
}

/// RFC 3339 mit genau drei Nachkommastellen und Offset, bei Offset 0 `Z` (Go `2006-01-02T15:04:05.000Z07:00`).
pub fn time_millis(t: &DateTime<Local>) -> String {
    let frac = format!(".{:03}", t.nanosecond() % 1_000_000_000 / 1_000_000);
    format!("{}{frac}{}", t.format("%Y-%m-%dT%H:%M:%S"), offset(t))
}

/// RFC 3339 mit Nanosekunden ohne Nullen am Ende (Go `time.RFC3339Nano`), wie slog `JSONHandler` die Zeit schreibt.
pub fn time_nanos(t: &DateTime<Local>) -> String {
    let nanos = t.nanosecond() % 1_000_000_000;
    let frac = if nanos == 0 {
        String::new()
    } else {
        let digits = format!("{nanos:09}");
        format!(".{}", digits.trim_end_matches('0'))
    };
    format!("{}{frac}{}", t.format("%Y-%m-%dT%H:%M:%S"), offset(t))
}

/// Offset wie Go `Z07:00`.
fn offset(t: &DateTime<Local>) -> String {
    if t.offset().fix().local_minus_utc() == 0 {
        return "Z".to_string();
    }
    t.format("%:z").to_string()
}

/// Wert als Text wie `slog.Value.String()`: Texte roh, Zahlen wie Go (`strconv` mit Format `g`), sonst kompaktes JSON.
pub fn value_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "<nil>".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => match (n.as_i64(), n.as_u64(), n.as_f64()) {
            (Some(i), _, _) => i.to_string(),
            (_, Some(u), _) => u.to_string(),
            (_, _, Some(f)) => go_float_g(f),
            _ => n.to_string(),
        },
        Value::Array(_) | Value::Object(_) => v.to_string(),
    }
}

/// Kürzeste Darstellung wie Go `strconv.FormatFloat(f, 'g', -1, 64)`: Exponent-Schreibweise ab 1e6 und unter 1e-4
/// (`1e+06`, `1.5e-05`), sonst Dezimalschreibweise.
pub(crate) fn go_float_g(f: f64) -> String {
    if f == 0.0 || !f.is_finite() {
        return format!("{f}");
    }
    let sci = format!("{f:e}");
    let (mant, exp) = sci.split_once('e').unwrap_or((sci.as_str(), "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    if (-4..6).contains(&exp) {
        return format!("{f}");
    }
    let sign = if exp < 0 { '-' } else { '+' };
    format!("{mant}e{sign}{:02}", exp.abs())
}

#[cfg(test)]
#[path = "logger_tests.rs"]
mod tests;
