//! data/troops.json: Go `map[string]sim.TroopData` (engine/sim/data.go).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::buildings::Cost;
use crate::{Fehler, decode};

/// Go: `sim.TroopData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TroopData {
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "Speed", alias = "speed")]
    pub speed: f64,
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "Range", alias = "range")]
    pub range: f64,
    #[serde(rename = "AttacksPerSecond", alias = "attacksPerSecond")]
    pub attacks_per_second: f64,
    #[serde(rename = "Cost", alias = "cost")]
    pub cost: Cost,
    #[serde(rename = "RecruitCost", alias = "recruitCost")]
    pub recruit_cost: Cost,
    #[serde(rename = "Professions", alias = "professions")]
    pub professions: Option<BTreeMap<String, ProfessionData>>,
    #[serde(rename = "PostUnits", alias = "postUnits")]
    pub post_units: f64,
    #[serde(rename = "UpgradeFrom", alias = "upgradeFrom")]
    pub upgrade_from: String,
    #[serde(rename = "FetchEnemyDistance", alias = "fetchEnemyDistance")]
    pub fetch_enemy_distance: f64,
}

/// Go: `sim.ProfessionData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfessionData {
    #[serde(rename = "Cost", alias = "cost")]
    pub cost: Cost,
    #[serde(rename = "WorkFactor", alias = "workFactor")]
    pub work_factor: f64,
    #[serde(rename = "Resources", alias = "resources")]
    pub resources: Vec<String>,
    #[serde(rename = "MaxPerBuilding", alias = "maxPerBuilding")]
    pub max_per_building: i64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "troops.json";

/// `load` liest troops.json wie `loadFrom` in engine/sim (ohne eigene Prüfung).
pub fn load(raw: &str) -> Result<BTreeMap<String, TroopData>, Fehler> {
    decode("data/troops.json", raw)
}
