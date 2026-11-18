//! Raum-Optionen der Insel (Port von `engine/sim/island_options.go`, B-101). Rumpf aus RU10.1: nur der Typ, die Funktionen
//! (`validate_options`, `set_options`, `default_options_for` …) kommen mit RU10.2.

use serde::{Deserialize, Serialize};

/// Go: `IslandOptions`, die Optionen einer Insel. Grad: dev, easy, normal, hard, ultra.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandOptions {
    #[serde(rename = "grade")]
    pub grade: String,
    /// endboss, gold, days, mineAll, buildAll.
    #[serde(rename = "goal")]
    pub goal: String,
    /// resources, stage (Kronenraub ist in jedem Grad Game Over, kein Modus).
    #[serde(rename = "defeat")]
    pub defeat: String,
}
