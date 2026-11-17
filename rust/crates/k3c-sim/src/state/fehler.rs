//! Fehler der Zustands-Schicht (Leitfaden § 3): ein `enum` je Crate, der Meldungstext wie in Go.

use std::fmt;

/// Fehler beim Lesen und Prüfen eines Spielstands; der Text beginnt wie in Go mit `spielstand:`.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// Ein Spielstand ist ungültig (`spielstand: …`).
    Spielstand(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Spielstand(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}
