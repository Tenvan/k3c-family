//! Fehler des Rigs: Meldungstexte wie in Go (`fmt.Errorf`).

use std::fmt;

/// Fehler des Rig-Pakets; der Text ist die Meldung von Go.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fehler {
    /// Datei fehlt oder ist nicht lesbar („… fehlt: <pfad> (erst `task native:rig:export` ausführen): …“).
    Fehlt(String),
    /// Datei ist kein gültiges JSON („… kaputt: <pfad>: …“).
    Kaputt(String),
    /// Regelverstoß im Rig oder in der Animation (`animation-unbekannt`, `pflicht-teil-fehlt`, Eltern, Kreis).
    Rig(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fehler::Fehlt(s) | Fehler::Kaputt(s) | Fehler::Rig(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}
