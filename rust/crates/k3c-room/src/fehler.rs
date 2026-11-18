//! Fehler des Raum-Crates (Leitfaden § 3): ein `enum` je Crate, Meldungstexte wie in Go (`engine/store`).
use std::fmt;

/// Fehler von Spielständen, Berichten und Raum. `errors.Is` aus Go wird ein `match` auf die Variante.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// `ErrSlot`: der Name des Spielstands ist ungültig.
    Slot,
    /// `ErrNotFound`: nicht gefunden.
    NotFound,
    /// `ErrInvalid`: ungültiger Inhalt.
    Invalid,
    /// `ErrTooLarge`: zu groß.
    TooLarge,
    /// Dateisystem-Fehler (Text des Betriebssystems, Code in `os`).
    Io(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Slot => f.write_str("ungültiger Slot"),
            Fehler::NotFound => f.write_str("nicht gefunden"),
            Fehler::Invalid => f.write_str("ungültiger Inhalt"),
            Fehler::TooLarge => f.write_str("zu groß"),
            Fehler::Io(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}

impl From<std::io::Error> for Fehler {
    fn from(e: std::io::Error) -> Self {
        Fehler::Io(e.to_string())
    }
}
