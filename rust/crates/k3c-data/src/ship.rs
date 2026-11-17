//! data/ship.json: Go `sim.shipFile` (engine/sim/ship.go); den Platz `ship` daraus baut die Sim (`withShip`).

use serde::{Deserialize, Serialize};

use crate::buildings::Cost;
use crate::{Fehler, decode};

/// Go: `detvec.shipFile`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ShipFile {
    #[serde(rename = "Stages", alias = "stages")]
    pub stages: Vec<ShipFileStages>,
}

/// Go: `struct { Cost sim.Cost; HP float64; BuildSeconds float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ShipFileStages {
    #[serde(rename = "Cost", alias = "cost")]
    pub cost: Cost,
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "BuildSeconds", alias = "buildSeconds")]
    pub build_seconds: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "ship.json";

/// `load` liest ship.json wie `loadFrom` in engine/sim (ohne eigene Prüfung).
pub fn load(raw: &str) -> Result<ShipFile, Fehler> {
    decode("data/ship.json", raw)
}
