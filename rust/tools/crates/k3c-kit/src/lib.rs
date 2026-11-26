//! Gemeinsamer Code der Rust-Werkzeuge (Entscheidung 014, Leitfaden `docs/standards/rust-werkzeuge.md`): Port von
//! `tools/workkit` und `engine/conlog`. Die Go-Fassung ist das Orakel; Verhalten, Dateiformate und Ausgaben bleiben gleich.
//!
//! - [`logger`]: Gegenstück zu Go `log/slog` (Level, Senken, Attribute).
//! - [`conlog`]: lesbare, farbige Konsolenzeile je Eintrag (`engine/conlog`).
//! - [`console`]: Konsolenpuffer je Quelle und Zeilen-Writer (`workkit/console`).
//! - [`applog`]: eigenes Log eines Werkzeugs als JSON-Datei plus Spiegel in den Konsolenpuffer (`workkit/applog`).

pub mod applog;
pub mod conlog;
pub mod console;
pub mod logger;

#[cfg(test)]
pub(crate) mod testutil;
