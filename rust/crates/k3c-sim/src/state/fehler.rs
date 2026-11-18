//! Fehler der Zustands-Schicht (Leitfaden § 3): ein `enum` je Crate, der Meldungstext wie in Go.

use std::fmt;

/// Fehler beim Lesen und Prüfen eines Spielstands; der Text beginnt wie in Go mit `spielstand:`.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// Ein Spielstand ist ungültig (`spielstand: …`).
    Spielstand(String),
    /// Dev-Aktion: In der Stufe lebt schon ein Boss oder der Endboss-Kampf der Insel läuft (Go: `ErrDevBossPresent`).
    DevBossPresent,
    /// Dev-Aktion ungültig: unbekannte Boss-Id, unbekannter Spieler oder Stufe ohne Portal (Go: `errDevInvalid`).
    DevInvalid,
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Spielstand(s) => f.write_str(s),
            Fehler::DevBossPresent => f.write_str("boss schon da"),
            Fehler::DevInvalid => f.write_str("dev-aktion ungültig"),
        }
    }
}

impl std::error::Error for Fehler {}
