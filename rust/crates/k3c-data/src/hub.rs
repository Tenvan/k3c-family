//! data/hub.json: Go `sim.HubData` (engine/sim/data.go); Kosten und Stufen wie bei den Gebäuden.
//!
//! Die Prüfung einer hub.json (`hub_validate.go`, Gebäude-Planer) braucht die Sim und kommt mit ihr (B-1024).

use serde::{Deserialize, Serialize};

use crate::buildings::LevelData;
use crate::{Fehler, decode};

/// Go: `sim.HubData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubData {
    #[serde(rename = "Levels", alias = "levels")]
    pub levels: Vec<LevelData>,
    #[serde(rename = "Sites", alias = "sites")]
    pub sites: Vec<HubSite>,
    #[serde(rename = "IslandSites", alias = "islandSites")]
    pub island_sites: Vec<HubSite>,
    #[serde(rename = "IslandStartStock", alias = "islandStartStock")]
    pub island_start_stock: Stock,
    #[serde(rename = "SitePlacement", alias = "sitePlacement")]
    pub site_placement: SitePlacementData,
    #[serde(rename = "Merchant", alias = "merchant")]
    pub merchant: HubDataMerchant,
    #[serde(rename = "CastleRadiusUnits", alias = "castleRadiusUnits")]
    pub castle_radius_units: f64,
    #[serde(rename = "HomeRadiusUnits", alias = "homeRadiusUnits")]
    pub home_radius_units: f64,
    #[serde(rename = "StartTroops", alias = "startTroops")]
    pub start_troops: HubDataStartTroops,
    #[serde(rename = "CastleSpareHammer", alias = "castleSpareHammer")]
    pub castle_spare_hammer: bool,
    #[serde(rename = "Travel", alias = "travel")]
    pub travel: HubDataTravel,
}

/// Go: `sim.HubSite`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubSite {
    #[serde(rename = "Kind", alias = "kind")]
    pub kind: String,
    #[serde(rename = "OffsetUnits", alias = "offsetUnits")]
    pub offset_units: f64,
    #[serde(rename = "FromDepth", alias = "fromDepth")]
    pub from_depth: i64,
    #[serde(rename = "NeedsDeeper", alias = "needsDeeper")]
    pub needs_deeper: bool,
    #[serde(rename = "HubLevel", alias = "hubLevel")]
    pub hub_level: i64,
    /// Produktion: steht nur ganz oben (Tiefe 0), unten bauen nur Kampfgebäude (ECO1.6, B-1196). Nur die nativen Regeln
    /// (`WorldValues::native_rules`) werten es aus; die Go-Läufe kennen das Feld nicht.
    #[serde(rename = "TopOnly", alias = "topOnly")]
    pub top_only: bool,
}

/// Go: `sim.Stock`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Stock {
    #[serde(rename = "wood")]
    pub wood: i64,
    #[serde(rename = "stone")]
    pub stone: i64,
    #[serde(rename = "copper")]
    pub copper: i64,
    #[serde(rename = "iron", skip_serializing_if = "crate::omit::is_zero_i64")]
    pub iron: i64,
    #[serde(rename = "crystal", skip_serializing_if = "crate::omit::is_zero_i64")]
    pub crystal: i64,
}

/// Go: `sim.SitePlacementData`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SitePlacementData {
    #[serde(rename = "Zone1WallUnits", alias = "zone1WallUnits")]
    pub zone1_wall_units: f64,
    #[serde(rename = "Zone1TowerUnits", alias = "zone1TowerUnits")]
    pub zone1_tower_units: f64,
    #[serde(rename = "OutsideUnits", alias = "outsideUnits")]
    pub outside_units: f64,
}

/// Go: `struct { BuyOffsetUnits float64; SellOffsetUnits float64; OnlyDepth int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubDataMerchant {
    #[serde(rename = "BuyOffsetUnits", alias = "buyOffsetUnits")]
    pub buy_offset_units: f64,
    #[serde(rename = "SellOffsetUnits", alias = "sellOffsetUnits")]
    pub sell_offset_units: f64,
    #[serde(rename = "OnlyDepth", alias = "onlyDepth")]
    pub only_depth: i64,
}

/// Go: `struct { Peasant int; Craftsman int; Archer int }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubDataStartTroops {
    #[serde(rename = "Peasant", alias = "peasant")]
    pub peasant: i64,
    #[serde(rename = "Craftsman", alias = "craftsman")]
    pub craftsman: i64,
    #[serde(rename = "Archer", alias = "archer")]
    pub archer: i64,
}

/// Go: `struct { RangeUnits float64; Seconds float64 }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubDataTravel {
    #[serde(rename = "RangeUnits", alias = "rangeUnits")]
    pub range_units: f64,
    #[serde(rename = "Seconds", alias = "seconds")]
    pub seconds: f64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "hub.json";

/// `load` liest hub.json wie `loadFrom` in engine/sim.
pub fn load(raw: &str) -> Result<HubData, Fehler> {
    decode("data/hub.json", raw)
}
