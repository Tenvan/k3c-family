//! Geschichte je Welt und Insel (Port von `engine/sim/story.go`, B-965, docs/rules/welten.md § 2.2): Kunstsprache mit
//! eingestreuten echten Wörtern, feste Zeilen in der Welt-Datei. Alles ist optional; fehlt `story` oder ein Eintrag, gibt es
//! dort keine Zeilen. Ein Eintrag ist eine Liste aus 1 bis 2 Zeilen. Prüfung: `check.rs`, Wendepunkte (reine Funktionen):
//! `points.rs`.
//!
//! `StoryLimits` (`data/story.json`) stehen in `k3c_data::story` (`Data::story`); Gos Paket-Variable `storyLimits` fällt
//! weg, `check_story` bekommt die Daten. Die Felder `start`, `end`, `arrival`, `boss` und `done` sind `Option<Vec<_>>`,
//! weil Go zwischen fehlend (`nil`) und leer (`[]`) unterscheidet: Ein vorhandener, leerer Eintrag wird geprüft und abgelehnt.

pub mod check;
pub mod points;

#[cfg(test)]
mod check_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(test)]
mod points_tests;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

pub use check::check_story;
pub use points::{
    island_story_arrival, island_story_boss, island_story_defeated, island_story_victory, story_arrival, story_boss,
    story_defeated, story_start, story_victory,
};

use crate::island::keys::Keys;

/// Go: `WorldStory`, `story` der Welt: Wortschatz der Kunstsprache und die Zeilen für Start, Etappen, Ende und Flaschenpost.
#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct WorldStory {
    pub lexicon: Vec<String>,
    pub start: Option<Vec<String>>,
    /// Etappen-Nummer (Dezimalzahl) → Zeilen beim Abschluss der Etappe.
    pub etappen: BTreeMap<String, Vec<String>>,
    pub end: Option<Vec<String>>,
    /// Nur Format und Prüfung, genutzt erst von der Flaschenpost (B-971).
    pub bottles: Vec<Vec<String>>,
}

/// Go: `IslandStory`, `story` der Insel: Ankunft, vor dem Endboss und Abschlusszeile nach dem Sieg über ihn.
#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct IslandStory {
    pub arrival: Option<Vec<String>>,
    pub boss: Option<Vec<String>>,
    pub done: Option<Vec<String>>,
}

/// Go: `strictDecode` für `story`: Schlüssel der obersten Ebene exakt aus `fields` (Groß-/Kleinschreibung zählt), dann Lesen
/// in `T`; `null` lässt die Geschichte leer wie in Go. Doppelte Schlüssel sieht ein `Value` nicht mehr (`Keys::from_value`).
fn strict_decode<T: for<'de> Deserialize<'de> + Default>(raw: &Value, fields: &[&str]) -> Result<T, String> {
    let wrap = |e: String| format!("story: {e}");
    Keys::from_value(raw).check(fields).map_err(wrap)?;
    match raw {
        Value::Null => Ok(T::default()),
        Value::Object(_) => T::deserialize(raw).map_err(|e| wrap(format!("kein gültiges JSON: {e}"))),
        _ => Err(wrap("kein gültiges JSON: kein Objekt".to_string())),
    }
}

/// Go: `decodeWorldStory`: liest `story` der Welt streng; fehlt das Feld, ist die Geschichte leer.
pub fn decode_world_story(raw: &Value) -> Result<WorldStory, String> {
    strict_decode(raw, &["lexicon", "start", "etappen", "end", "bottles"])
}

/// Go: `decodeIslandStory`: liest `story` der Insel streng; fehlt das Feld, ist die Geschichte leer.
pub fn decode_island_story(raw: &Value) -> Result<IslandStory, String> {
    strict_decode(raw, &["arrival", "boss", "done"])
}
