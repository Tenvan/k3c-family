//! Regelsatz des Generators. Gos Paket-Variablen (`guardRules`, `ruinRules`, `ruinBuildings`, `hubSites`,
//! `payGapUnits`, `sitePlacement`) gibt es in Rust nicht; `Rules` hält sie, `generate` bekommt sie als Parameter.

use std::collections::BTreeMap;

use k3c_data::Data;
use k3c_data::guards::GuardRules;
use k3c_data::hub::SitePlacementData;
use k3c_data::ruins::RuinRules;

use crate::error::Fehler;
use crate::lines::HubSiteRef;

/// Go: `buildingInfo`, der Teil eines Eintrags aus `data/buildings.json`, den die Ruinen brauchen.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuildingInfo {
    pub hp: f64,
    /// Breite des Pflanzstreifens einer Plantage (0 ohne Plantage).
    pub strip_units: f64,
}

/// Alle Werte, die Gos Paketstart aus `data/` liest.
#[derive(Debug, Clone, PartialEq)]
pub struct Rules {
    pub guard: GuardRules,
    pub ruin: RuinRules,
    pub ruin_buildings: BTreeMap<String, BuildingInfo>,
    /// Plätze der Basis: `hub.sites` gefolgt von `hub.islandSites`.
    pub hub_sites: Vec<HubSiteRef>,
    /// Mindestabstand zweier Zahlziele: 2 × `payRangeUnits` (Go: `payGapUnits`).
    pub pay_gap_units: f64,
    pub site_placement: SitePlacementData,
}

impl Rules {
    /// Baut den Regelsatz aus den geladenen (und geprüften) Daten.
    pub fn from_data(d: &Data) -> Result<Rules, Fehler> {
        let ruin_buildings = d
            .buildings
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    BuildingInfo {
                        hp: v.hp,
                        strip_units: v.plantation.strip_units,
                    },
                )
            })
            .collect();
        let hub_sites = d
            .hub
            .sites
            .iter()
            .chain(&d.hub.island_sites)
            .map(|s| HubSiteRef {
                kind: s.kind.clone(),
                offset: s.offset_units,
            })
            .collect();
        Ok(Rules {
            guard: d.guards.clone(),
            ruin: d.ruins.clone(),
            ruin_buildings,
            hub_sites,
            pay_gap_units: 2.0 * d.economy.pay_range_units,
            site_placement: d.hub.site_placement.clone(),
        })
    }
}
