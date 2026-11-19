//! Spiel-Daten: typisierte Strukturen für data/*.json, wertgleich zu den Go-Strukturen (RU3/AC-04).
//!
//! Feldnamen der Ausgabe wie Gos `encoding/json` (ohne Tag der Go-Name, z. B. `HPFactor`); gelesen wird der
//! Schlüssel der Datei (Go vergleicht ohne Groß- und Kleinschreibung, hier über `alias`). Unbekannte Felder
//! überliest Rust wie Go; nur die Balancing-Dateien lehnen sie ab (Go: `DisallowUnknownFields`).

use std::fmt;

pub mod balance_hints;
pub mod balance_targets;
pub mod biome;
pub mod bosses;
pub mod buildings;
pub mod cargo;
pub mod coop;
pub mod data;
pub mod difficulty;
pub mod economy;
pub mod enemies;
pub mod events;
pub mod goals;
pub mod guards;
pub mod hermits;
pub mod hub;
pub mod island_sizes;
pub mod light;
pub mod look;
pub mod look_world;
pub mod monarch;
pub mod mounts;
pub mod native;
mod omit;
pub mod ordered;
pub mod paper;
pub mod render;
pub mod ruins;
pub mod ship;
pub mod sounds;
pub mod source;
pub mod story;
pub mod stresstests;
pub mod troops;
pub mod waves;
pub mod weather;
pub mod world;

pub use data::{Data, load_all};
pub use source::Source;

/// Fehler beim Laden der Daten; der Text folgt Go (Datei, Feld, Grund).
#[derive(Debug, Clone, PartialEq)]
pub enum Fehler {
    /// Die Datei fehlt oder ist nicht lesbar.
    Lesen(String),
    /// Das JSON passt nicht zur Struktur (Text von `serde_json`, nicht von Go).
    Json(String),
    /// Eine Prüfung beim Laden schlägt fehl (Text wie in Go).
    Pruefung(String),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fehler::Lesen(s) | Fehler::Json(s) | Fehler::Pruefung(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for Fehler {}

/// `decode` liest raw in T; ein Fehler bekommt das Präfix wie Gos Lade-Funktion (z. B. `data/bosses.json`).
pub(crate) fn decode<T: serde::de::DeserializeOwned>(prefix: &str, raw: &str) -> Result<T, Fehler> {
    serde_json::from_str(raw).map_err(|e| Fehler::Json(format!("{prefix}: {e}")))
}

/// `check` macht aus einem Prüf-Text mit Präfix einen `Fehler::Pruefung`.
pub(crate) fn check(prefix: &str, r: Result<(), String>) -> Result<(), Fehler> {
    r.map_err(|e| Fehler::Pruefung(format!("{prefix}: {e}")))
}

#[cfg(test)]
mod source_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod tree_rest_tests;
#[cfg(test)]
mod tree_tests;
