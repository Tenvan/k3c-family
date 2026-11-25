//! Fehler des Crates (Go: `error` in `engine/balance`); die Meldungstexte folgen Go.

use std::fmt;

/// Fehler des Balancing-Crates.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// Eine Daten-Datei fehlt oder passt nicht (Go: `balance: data/<datei>: <grund>`).
    Daten(String),
    /// Ein Profil passt nicht zum Szenario (Go: `checkPlayers`).
    Profil(String),
    /// Die Hilfen eines Szenarios passen nicht (Go: `checkHelps`).
    Hilfen(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Daten(s) | Fehler::Profil(s) | Fehler::Hilfen(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}
