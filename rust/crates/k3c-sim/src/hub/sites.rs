//! Prüfung der Hub-Plätze eines Wertesatzes (Teil-Port von `engine/sim/world_hub_sites.go`: `checkWorldHubSites`,
//! `checkSiteBounds`, `RuleBounds`; welten.md § 1.3, B-983). `level_sites` steht in `world/mod.rs` (RU14.1),
//! `check_world_plans` in `world/hub_sites.rs`. Gos Paket-Variable `biomes` kommt als Parameter.

use std::collections::BTreeMap;

use k3c_data::biome::Biome;
use k3c_data::buildings::BuildingData;
use k3c_data::economy::EconomyData;
use k3c_data::hub::HubData;
use k3c_det::golden::go_float;

use super::validate::{HubFinding, all_sites, check_hub_levels, check_site_kinds, site_name};
use super::validate_layout::check_gaps;

/// Platz außerhalb der Hub-Breite.
pub const RULE_BOUNDS: &str = "hub-breite";

/// Go: `checkWorldHubSites`: prüft die Plätze eines Wertesatzes (fail closed): Hub-Stufen, Arten, Pflichtplätze, ein Platz
/// je Bau, Lage innerhalb der Hub-Breite und Abstand der Zahlziele (2 × payRangeUnits der Welt). Die Meldung nennt Datei,
/// Regel und Platz; der Aufrufer ergänzt die Welt.
pub fn check_world_hub_sites(
    h: &HubData,
    eco: &EconomyData,
    bs: &BTreeMap<String, BuildingData>,
    biomes: &[Biome],
) -> Result<(), String> {
    let mut out = check_hub_levels(h);
    out.extend(check_site_kinds(h, bs));
    out.extend(check_site_bounds(h, biomes));
    if out.is_empty() {
        out = check_gaps(h, bs, eco.pay_range_units, biomes);
    }
    match out.first() {
        Some(f) => Err(format!("data/hub.json: {f}")),
        None => Ok(()),
    }
}

/// Go: `checkSiteBounds`: Jeder Platz liegt innerhalb der halben Hub-Breite des schmalsten Bioms (die Insel-Plätze stehen
/// im Hub).
pub fn check_site_bounds(h: &HubData, biomes: &[Biome]) -> Vec<HubFinding> {
    let half = biomes.iter().fold(f64::INFINITY, |m, b| m.min(b.hub_width_units / 2.0));
    all_sites(h)
        .filter(|s| s.offset_units.abs() >= half)
        .map(|s| {
            HubFinding::new(
                RULE_BOUNDS,
                site_name(s),
                format!(
                    "offsetUnits {} liegt nicht innerhalb der Hub-Breite (±{})",
                    go_float(s.offset_units),
                    go_float(half)
                ),
            )
        })
        .collect()
}
