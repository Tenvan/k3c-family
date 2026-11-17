//! data/buildings.json: Go `map[string]sim.BuildingData` (engine/sim/data.go, `building_*.go`).
//!
//! Go hängt beim Laden den Platz `ship` aus ship.json an (`withShip`); das ist Logik der Sim und gehört nicht hierher.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.BuildingData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildingData {
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "BuildSeconds", alias = "buildSeconds")]
    pub build_seconds: f64,
    #[serde(rename = "Cost", alias = "cost")]
    pub cost: Cost,
    #[serde(rename = "CostByDepth", alias = "costByDepth")]
    pub cost_by_depth: Vec<Cost>,
    #[serde(rename = "ArcherSlots", alias = "archerSlots")]
    pub archer_slots: i64,
    #[serde(rename = "RangeBonus", alias = "rangeBonus")]
    pub range_bonus: i64,
    #[serde(rename = "BowRack", alias = "bowRack")]
    pub bow_rack: i64,
    #[serde(rename = "SwordRack", alias = "swordRack")]
    pub sword_rack: i64,
    #[serde(rename = "HammerRack", alias = "hammerRack")]
    pub hammer_rack: i64,
    #[serde(rename = "HammerCost", alias = "hammerCost")]
    pub hammer_cost: Cost,
    #[serde(rename = "Offers", alias = "offers")]
    pub offers: Vec<Offer>,
    #[serde(rename = "CraftSeconds", alias = "craftSeconds")]
    pub craft_seconds: Option<BTreeMap<String, f64>>,
    #[serde(rename = "Levels", alias = "levels")]
    pub levels: Vec<LevelData>,
    #[serde(rename = "Plantation", alias = "plantation")]
    pub plantation: Plantation,
    #[serde(rename = "TroopLimit", alias = "troopLimit")]
    pub troop_limit: TroopLimit,
    #[serde(rename = "Vagrants", alias = "vagrants")]
    pub vagrants: Vagrants,
    #[serde(rename = "Heal", alias = "heal")]
    pub heal: Heal,
    #[serde(rename = "Spell", alias = "spell")]
    pub spell: Spell,
    #[serde(rename = "Armor", alias = "armor")]
    pub armor: Vec<ArmorStage>,
}

/// Go: `sim.Cost`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Cost {
    #[serde(rename = "Gold", alias = "gold")]
    pub gold: i64,
    #[serde(rename = "Wood", alias = "wood")]
    pub wood: i64,
    #[serde(rename = "Stone", alias = "stone")]
    pub stone: i64,
    #[serde(rename = "Copper", alias = "copper")]
    pub copper: i64,
    #[serde(rename = "Iron", alias = "iron")]
    pub iron: i64,
    #[serde(rename = "Crystal", alias = "crystal")]
    pub crystal: i64,
}

/// Go: `sim.Offer`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Offer {
    #[serde(rename = "Kind", alias = "kind")]
    pub kind: String,
    #[serde(rename = "DX", alias = "dx")]
    pub dx: f64,
}

/// Go: `sim.LevelData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LevelData {
    #[serde(rename = "Cost", alias = "cost")]
    pub cost: Cost,
    #[serde(rename = "Gate", alias = "gate")]
    pub gate: Cost,
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "BuildSeconds", alias = "buildSeconds")]
    pub build_seconds: f64,
    #[serde(rename = "Steps", alias = "steps")]
    pub steps: Vec<LevelData>,
}

/// Go: `sim.Plantation`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Plantation {
    #[serde(rename = "Slots", alias = "slots")]
    pub slots: i64,
    #[serde(rename = "GrowSeconds", alias = "growSeconds")]
    pub grow_seconds: f64,
    #[serde(rename = "StripUnits", alias = "stripUnits")]
    pub strip_units: f64,
}

/// Go: `sim.TroopLimit`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TroopLimit {
    #[serde(rename = "Base", alias = "base")]
    pub base: i64,
    #[serde(rename = "PerBuilding", alias = "perBuilding")]
    pub per_building: i64,
}

/// Go: `sim.Vagrants`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Vagrants {
    #[serde(rename = "PerDawn", alias = "perDawn")]
    pub per_dawn: i64,
    #[serde(rename = "Max", alias = "max")]
    pub max: i64,
    #[serde(rename = "WanderUnits", alias = "wanderUnits")]
    pub wander_units: f64,
}

/// Go: `sim.Heal`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Heal {
    #[serde(rename = "HPPerSecond", alias = "hpPerSecond")]
    pub hp_per_second: f64,
    #[serde(rename = "RadiusUnits", alias = "radiusUnits")]
    pub radius_units: f64,
}

/// Go: `sim.Spell`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Spell {
    #[serde(rename = "FromLevel", alias = "fromLevel")]
    pub from_level: i64,
    #[serde(rename = "Damage", alias = "damage")]
    pub damage: f64,
    #[serde(rename = "RadiusUnits", alias = "radiusUnits")]
    pub radius_units: f64,
    #[serde(rename = "RangeUnits", alias = "rangeUnits")]
    pub range_units: f64,
    #[serde(rename = "IntervalSeconds", alias = "intervalSeconds")]
    pub interval_seconds: f64,
}

/// Go: `sim.ArmorStage`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ArmorStage {
    #[serde(rename = "HubLevel", alias = "hubLevel")]
    pub hub_level: i64,
    #[serde(rename = "HPBonus", alias = "hpBonus")]
    pub hp_bonus: f64,
    #[serde(rename = "Cost", alias = "cost")]
    pub cost: Cost,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "buildings.json";

/// `load` liest buildings.json wie `loadFrom` in engine/sim (ohne eigene Prüfung).
pub fn load(raw: &str) -> Result<BTreeMap<String, BuildingData>, Fehler> {
    decode("data/buildings.json", raw)
}
