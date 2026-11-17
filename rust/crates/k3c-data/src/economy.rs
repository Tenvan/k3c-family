//! data/economy.json: Go `sim.economyData` (engine/sim/world_values.go) mit Sammelgut, Adern, Händler und Kampf.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.economyData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyData {
    #[serde(rename = "Purse", alias = "purse")]
    pub purse: EconomyDataPurse,
    #[serde(rename = "PayRangeUnits", alias = "payRangeUnits")]
    pub pay_range_units: f64,
    #[serde(rename = "PayIntervalSeconds", alias = "payIntervalSeconds")]
    pub pay_interval_seconds: f64,
    #[serde(rename = "PickupRangeUnits", alias = "pickupRangeUnits")]
    pub pickup_range_units: f64,
    #[serde(rename = "DropPickupDelaySeconds", alias = "dropPickupDelaySeconds")]
    pub drop_pickup_delay_seconds: f64,
    #[serde(rename = "DawnGoldPerPlayer", alias = "dawnGoldPerPlayer")]
    pub dawn_gold_per_player: i64,
    #[serde(rename = "DawnGoldPerDepth", alias = "dawnGoldPerDepth")]
    pub dawn_gold_per_depth: i64,
    #[serde(rename = "ChestGold", alias = "chestGold")]
    pub chest_gold: [i64; 2],
    #[serde(rename = "ChestFountain", alias = "chestFountain")]
    pub chest_fountain: EconomyDataChestFountain,
    #[serde(rename = "TreeGold", alias = "treeGold")]
    pub tree_gold: [i64; 2],
    #[serde(rename = "EnemyGoldFactor", alias = "enemyGoldFactor")]
    pub enemy_gold_factor: f64,
    #[serde(rename = "EnemyResourceDrop", alias = "enemyResourceDrop")]
    pub enemy_resource_drop: EconomyDataEnemyResourceDrop,
    #[serde(rename = "Gatherables", alias = "gatherables")]
    pub gatherables: BTreeMap<String, Gatherable>,
    #[serde(rename = "Veins", alias = "veins")]
    pub veins: BTreeMap<String, VeinData>,
    #[serde(rename = "Storage", alias = "storage")]
    pub storage: EconomyDataStorage,
    #[serde(rename = "RecruitCamp", alias = "recruitCamp")]
    pub recruit_camp: EconomyDataRecruitCamp,
    #[serde(rename = "Merchant", alias = "merchant")]
    pub merchant: MerchantData,
    #[serde(rename = "Combat", alias = "combat")]
    pub combat: CombatData,
}

/// Go: `struct { StartGold int; MaxGold int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyDataPurse {
    #[serde(rename = "StartGold", alias = "startGold")]
    pub start_gold: i64,
    #[serde(rename = "MaxGold", alias = "maxGold")]
    pub max_gold: i64,
}

/// Go: `struct { FlightSeconds float64; SpreadUnits float64; StaggerSeconds float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyDataChestFountain {
    #[serde(rename = "FlightSeconds", alias = "flightSeconds")]
    pub flight_seconds: f64,
    #[serde(rename = "SpreadUnits", alias = "spreadUnits")]
    pub spread_units: f64,
    #[serde(rename = "StaggerSeconds", alias = "staggerSeconds")]
    pub stagger_seconds: f64,
}

/// Go: `struct { Chance float64; Amount int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyDataEnemyResourceDrop {
    #[serde(rename = "Chance", alias = "chance")]
    pub chance: f64,
    #[serde(rename = "Amount", alias = "amount")]
    pub amount: i64,
}

/// Go: `sim.Gatherable`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Gatherable {
    #[serde(rename = "Resource", alias = "resource")]
    pub resource: String,
    #[serde(rename = "Amount", alias = "amount")]
    pub amount: i64,
    #[serde(rename = "WorkSeconds", alias = "workSeconds")]
    pub work_seconds: f64,
    #[serde(rename = "MarkCost", alias = "markCost")]
    pub mark_cost: i64,
}

/// Go: `sim.VeinData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VeinData {
    #[serde(flatten)]
    pub gatherable: Gatherable,
    #[serde(rename = "MaxWorkers", alias = "maxWorkers")]
    pub max_workers: i64,
}

/// Go: `struct { BasePerHub int; PerStorage int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyDataStorage {
    #[serde(rename = "BasePerHub", alias = "basePerHub")]
    pub base_per_hub: i64,
    #[serde(rename = "PerStorage", alias = "perStorage")]
    pub per_storage: i64,
}

/// Go: `struct { MaxVagrants int; RespawnSeconds float64; WanderUnits float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyDataRecruitCamp {
    #[serde(rename = "MaxVagrants", alias = "maxVagrants")]
    pub max_vagrants: i64,
    #[serde(rename = "RespawnSeconds", alias = "respawnSeconds")]
    pub respawn_seconds: f64,
    #[serde(rename = "WanderUnits", alias = "wanderUnits")]
    pub wander_units: f64,
}

/// Go: `sim.MerchantData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MerchantData {
    #[serde(rename = "HP", alias = "hp")]
    pub hp: f64,
    #[serde(rename = "EveryDays", alias = "everyDays")]
    pub every_days: i64,
    #[serde(rename = "EveryDaysWithTavern", alias = "everyDaysWithTavern")]
    pub every_days_with_tavern: i64,
    #[serde(rename = "StayDays", alias = "stayDays")]
    pub stay_days: i64,
    #[serde(rename = "Material", alias = "material")]
    pub material: i64,
    #[serde(rename = "Gold", alias = "gold")]
    pub gold: i64,
    #[serde(rename = "Materials", alias = "materials")]
    pub materials: Vec<String>,
}

/// Go: `sim.combatData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CombatData {
    #[serde(rename = "ShieldCost", alias = "shieldCost")]
    pub shield_cost: i64,
    #[serde(rename = "TriggerCost", alias = "triggerCost")]
    pub trigger_cost: i64,
    #[serde(rename = "BannerCost", alias = "bannerCost")]
    pub banner_cost: i64,
    #[serde(rename = "TroopSoldiers", alias = "troopSoldiers")]
    pub troop_soldiers: i64,
    #[serde(rename = "TroopArchers", alias = "troopArchers")]
    pub troop_archers: i64,
    #[serde(rename = "EngageUnits", alias = "engageUnits")]
    pub engage_units: f64,
    #[serde(rename = "ShieldOffsetUnits", alias = "shieldOffsetUnits")]
    pub shield_offset_units: f64,
    #[serde(rename = "GateOutUnits", alias = "gateOutUnits")]
    pub gate_out_units: f64,
    #[serde(rename = "SpotClearUnits", alias = "spotClearUnits")]
    pub spot_clear_units: f64,
    #[serde(rename = "ShieldMinHubLevel", alias = "shieldMinHubLevel")]
    pub shield_min_hub_level: i64,
    #[serde(rename = "ShieldStackMax", alias = "shieldStackMax")]
    pub shield_stack_max: i64,
    #[serde(rename = "BehindUnits", alias = "behindUnits")]
    pub behind_units: f64,
    #[serde(rename = "StandRangeFactor", alias = "standRangeFactor")]
    pub stand_range_factor: f64,
    #[serde(rename = "FollowUnits", alias = "followUnits")]
    pub follow_units: f64,
    #[serde(rename = "ShieldArriveUnits", alias = "shieldArriveUnits")]
    pub shield_arrive_units: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "economy.json";

/// `load` liest economy.json wie `loadFrom` in engine/sim (ohne eigene Prüfung).
pub fn load(raw: &str) -> Result<EconomyData, Fehler> {
    decode("data/economy.json", raw)
}
