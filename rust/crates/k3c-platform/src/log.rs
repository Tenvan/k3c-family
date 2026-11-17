//! Log nach stderr mit Emoji-Regel (`CLAUDE.md` › Logging): Jede Meldung beginnt mit einem Emoji aus [`EMOJIS`].
//! Format wie `slog`-Text: Stufe, Meldung, dann `schlüssel=wert` je Feld.

use std::fmt::Display;
use std::io::Write;

/// Erlaubte Emojis am Anfang einer Meldung (je ein Codepunkt, kein U+FE0F), Reihenfolge wie in `CLAUDE.md`.
pub const EMOJIS: [char; 30] = [
    '🚀', '🛑', '🔁', '✅', '❌', '💥', '🚫', '⏳', '🐢', '🔌', '👑', '👋', '💾', '📂', '🩹', '🧹', '📨', '📄', '🔒',
    '🚦', '🤒', '💓', '🏰', '🪜', '⏩', '🌙', '🏹', '🪙', '🔨', '🐛',
];

/// Stufe einer Meldung.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn name(self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

/// Wahr, wenn `msg` mit einem erlaubten Emoji beginnt.
pub fn starts_with_emoji(msg: &str) -> bool {
    msg.chars().next().is_some_and(|c| EMOJIS.contains(&c)) && !msg.contains('\u{FE0F}')
}

/// Formatiert eine Zeile: `WARN 🤒 Meldung grund=…`.
pub fn line(level: Level, msg: &str, fields: &[(&str, &dyn Display)]) -> String {
    let mut s = format!("{} {msg}", level.name());
    for (k, v) in fields {
        s.push_str(&format!(" {k}={v}"));
    }
    s
}

/// Schreibt eine Meldung nach stderr; im Debug-Build prüft es die Emoji-Regel.
pub fn log(level: Level, msg: &str, fields: &[(&str, &dyn Display)]) {
    debug_assert!(starts_with_emoji(msg), "Log-Meldung ohne Emoji aus der Liste: {msg}");
    let _ = writeln!(std::io::stderr(), "{}", line(level, msg, fields));
}

pub fn debug(msg: &str, fields: &[(&str, &dyn Display)]) {
    log(Level::Debug, msg, fields);
}

pub fn info(msg: &str, fields: &[(&str, &dyn Display)]) {
    log(Level::Info, msg, fields);
}

pub fn warn(msg: &str, fields: &[(&str, &dyn Display)]) {
    log(Level::Warn, msg, fields);
}

pub fn error(msg: &str, fields: &[(&str, &dyn Display)]) {
    log(Level::Error, msg, fields);
}
