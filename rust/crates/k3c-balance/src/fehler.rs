//! Fehler des Crates (Go: `error` in `engine/balance`); die Meldungstexte folgen Go.

use std::fmt;

use crate::parallel::Crash;

/// Fehler des Balancing-Crates.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// Eine Daten-Datei fehlt oder passt nicht (Go: `balance: data/<datei>: <grund>`).
    Daten(String),
    /// Ein Profil passt nicht zum Szenario (Go: `checkPlayers`).
    Profil(String),
    /// Die Hilfen eines Szenarios passen nicht (Go: `checkHelps`).
    Hilfen(String),
    /// Die Matrix ist ungültig (Go: `matrix: …` in `Matrix.Scenarios`).
    Matrix(String),
    /// Die Simulation lehnt die Insel, die Welt oder den Grad ab (Text von `k3c-sim`).
    Sim(String),
    /// Ein Lauf oder `progress` ist abgestürzt und hat die übrigen Läufe angehalten (Go: `panic(*Crash)` in `runAll`).
    Abbruch(Crash),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Daten(s) | Fehler::Profil(s) | Fehler::Hilfen(s) | Fehler::Matrix(s) | Fehler::Sim(s) => {
                f.write_str(s)
            }
            Fehler::Abbruch(c) => c.fmt(f),
        }
    }
}

impl std::error::Error for Fehler {}
