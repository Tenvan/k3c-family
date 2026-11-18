//! Schiff (Port von `engine/sim/ship.go`; BAS, B-637, docs/rules/geplante-chunks.md § 10, wirtschaft.md § 8): Der
//! Hafen-Chunk (`HARBOR_KIND`) trägt den Platz `ship`. Er wird wie jeder Bau bezahlt (Gold, Material aus dem Vorrat, ein
//! Bauer baut); die Abschnitte aus data/ship.json sind seine Ausbau-Stufen (Kiel = Bau, Rumpf und Segel = Stufe 2 und 3,
//! hub_level.go), unabhängig von der Hub-Stufe. Gegner greifen den Platz nicht an. Fertig ist das Schiff mit der letzten
//! Stufe; mit besiegtem Endboss öffnet es den Inselwechsel (island/switch.rs).

use std::collections::BTreeMap;

use k3c_data::buildings::{BuildingData, Cost, LevelData};
use k3c_data::ship::ShipFile;
use k3c_level::level::HARBOR_KIND;

use crate::island::Island;
use crate::links::next_level;
use crate::state::island_ref::IslandRef;
use crate::state::{Site, World};

use super::links::{level_of, site_cost, upgrade_payable};
use super::{empty_site, vals};

/// Go: `shipKind`, die Art des Schiffs-Platzes.
pub const SHIP_KIND: &str = "ship";

/// Go: `withShip`: trägt den Platz `ship` aus data/ship.json in die Gebäude ein: Kosten und Werte des ersten Abschnitts am
/// Platz, die Abschnitte als Stufen. Ohne Abschnitte ist es ein Fehler (Go: `panic`).
pub fn with_ship(b: &mut BTreeMap<String, BuildingData>, f: &ShipFile) -> Result<(), String> {
    let levels: Vec<LevelData> = f
        .stages
        .iter()
        .map(|st| LevelData {
            cost: st.cost.clone(),
            hp: st.hp,
            build_seconds: st.build_seconds,
            ..LevelData::default()
        })
        .collect();
    let Some(first) = levels.first() else {
        return Err("ship.json: keine Abschnitte".to_string());
    };
    let ship = BuildingData {
        hp: first.hp,
        build_seconds: first.build_seconds,
        cost: first.cost.clone(),
        levels: levels.clone(),
        ..BuildingData::default()
    };
    b.insert(SHIP_KIND.to_string(), ship);
    Ok(())
}

/// Go: `addShipSite`: legt den Schiffs-Platz an den Hafen der Stufe (letzter Platz der Liste, die Plätze davor behalten
/// ihre IDs).
pub fn add_ship_site(w: &mut World) {
    let harbors: Vec<f64> = w
        .level
        .entities
        .iter()
        .filter(|e| e.kind == HARBOR_KIND)
        .map(|e| e.x)
        .collect();
    for x in harbors {
        let site = empty_site(w, SHIP_KIND, x);
        w.sites.push(site);
    }
}

/// Go: `Island.harbor`: die Stufe mit dem Hafen und der Platz des Schiffs darin (Indizes in `stages` und `sites`); `None`
/// ohne Hafen.
pub fn harbor(isl: &Island) -> Option<(usize, usize)> {
    isl.stages
        .iter()
        .enumerate()
        .find_map(|(wi, w)| w.sites.iter().position(|s| s.kind == SHIP_KIND).map(|si| (wi, si)))
}

/// Go: `shipComplete`: Alle Abschnitte des Schiffs sind gebaut.
pub fn ship_complete(w: &World, s: &Site) -> bool {
    let stages = vals(w).buildings.get(SHIP_KIND).map_or(0, |b| b.levels.len() as i64);
    s.state == "built" && s.upgrade.is_empty() && level_of(w, s) >= stages
}

/// Go: `ShipNextCost`: der Preis des nächsten Schiffs-Abschnitts am Platz `s` (Kiel am unbezahlten Platz, danach Rumpf und
/// Segel); `None`, wenn der Platz kein Schiff ist, nichts mehr zu bauen ist oder gerade ein Ausbau läuft. Nur gelesen, für
/// die Bots des Balancing-Testers (BS9).
pub fn ship_next_cost(w: &World, isl: Option<&IslandRef>, s: &Site) -> Option<Cost> {
    if s.kind != SHIP_KIND {
        None
    } else if s.state == "unpaid" {
        Some(site_cost(w, isl, s))
    } else if upgrade_payable(w, s) {
        Some(next_level(w, isl, s).map_or_else(Cost::default, |l| l.cost))
    } else {
        None
    }
}
