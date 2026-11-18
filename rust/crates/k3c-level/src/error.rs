//! Fehler des Level-Generators; der Text folgt Go (`fmt.Errorf`).

use std::fmt;

/// Fehler beim Aufbau der Regeln oder bei Planung und Erzeugung eines Levels.
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// Die Spiel-Daten lassen sich nicht laden.
    Daten(k3c_data::Fehler),
    /// Prüfung oder Erzeugung schlägt fehl (Meldungstext wie in Go).
    Level(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Daten(e) => e.fmt(f),
            Fehler::Level(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}

impl From<k3c_data::Fehler> for Fehler {
    fn from(e: k3c_data::Fehler) -> Self {
        Fehler::Daten(e)
    }
}
