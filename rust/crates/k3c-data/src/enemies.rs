//! data/enemies.json: Go `map[string]sim.EnemyData` (engine/sim/enemies_traits.go, `loadEnemies`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.EnemyData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EnemyData {
    #[serde(rename = "Name", alias = "name")]
    pub name: String,
    #[serde(rename = "Tier", alias = "tier")]
    pub tier: String,
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "Speed", alias = "speed")]
    pub speed: f64,
    #[serde(rename = "Range", alias = "range")]
    pub range: f64,
    #[serde(rename = "Gold", alias = "gold")]
    pub gold: [i64; 2],
    #[serde(rename = "Traits", alias = "traits")]
    pub traits: Vec<String>,
    #[serde(rename = "AttacksPerSecond", alias = "attacksPerSecond")]
    pub attacks_per_second: f64,
    #[serde(rename = "Aoe", alias = "aoe")]
    pub aoe: EnemyDataAoe,
    #[serde(rename = "SwarmSize", alias = "swarmSize")]
    pub swarm_size: i64,
    #[serde(rename = "Phases", alias = "phases")]
    pub phases: EnemyDataPhases,
    #[serde(rename = "KiteDistance", alias = "kiteDistance")]
    pub kite_distance: f64,
    #[serde(rename = "GrabRange", alias = "grabRange")]
    pub grab_range: f64,
}

/// Go: `struct { Radius float64; IntervalSeconds float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EnemyDataAoe {
    #[serde(rename = "Radius", alias = "radius")]
    pub radius: f64,
    #[serde(rename = "IntervalSeconds", alias = "intervalSeconds")]
    pub interval_seconds: f64,
}

/// Go: `struct { EverySeconds float64; DurationSeconds float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EnemyDataPhases {
    #[serde(rename = "EverySeconds", alias = "everySeconds")]
    pub every_seconds: f64,
    #[serde(rename = "DurationSeconds", alias = "durationSeconds")]
    pub duration_seconds: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "enemies.json";

/// Bekannte Traits (engine/sim `knownTraits`).
pub const KNOWN_TRAITS: [&str; 13] = [
    "stealsGold",
    "prefersTroops",
    "prefersBuildings",
    "prefersTowers",
    "prefersMonarch",
    "fleesAtHalfHp",
    "ignoresWalls",
    "ranged",
    "flying",
    "swarm",
    "aoe",
    "phases",
    "kiting",
];

/// `load` liest enemies.json und prüft Name und Traits wie `loadEnemies` (Gegner sortiert nach Art).
pub fn load(raw: &str) -> Result<BTreeMap<String, EnemyData>, Fehler> {
    let v: BTreeMap<String, EnemyData> = decode("data/enemies.json", raw)?;
    for (kind, e) in &v {
        if e.name.trim().is_empty() {
            return Err(Fehler::Pruefung(format!(
                "data/enemies.json: {kind}: name fehlt oder ist leer"
            )));
        }
        for t in &e.traits {
            if !KNOWN_TRAITS.contains(&t.as_str()) {
                return Err(Fehler::Pruefung(format!(
                    "data/enemies.json: {kind}: unbekanntes Trait {t:?}"
                )));
            }
        }
    }
    Ok(v)
}
