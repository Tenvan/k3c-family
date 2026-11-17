//! data/goals.json: Go `sim.Goals` (engine/sim/data.go), Startwerte der Siegvarianten.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.Goals`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Goals {
    #[serde(rename = "Gold", alias = "gold")]
    pub gold: i64,
    #[serde(rename = "Days", alias = "days")]
    pub days: i64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "goals.json";

/// `load` liest goals.json wie `loadFrom` in engine/sim (ohne eigene Prüfung).
pub fn load(raw: &str) -> Result<Goals, Fehler> {
    decode("data/goals.json", raw)
}
