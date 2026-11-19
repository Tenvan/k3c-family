//! Siegvarianten der Raum-Optionen (Port von `engine/sim/island_victory.go`, B-102, docs/rules/stufen.md § 3): Das Ziel der
//! Insel (`options.goal`) beendet das Spiel mit dem Ereignis `victory`, genau einmal (`Island::won`). Danach rechnet die
//! Simulation weiter; was der Raum anzeigt, entscheidet das Protokoll (K4). Jede Variante gilt nur auf der letzten Insel der
//! Daten (Kampagnensieg); vorher öffnet der Endboss den Inselwechsel (`switch.rs`).

use std::collections::BTreeSet;

use serde_json::Value;

use super::Island;
use crate::state::links::{emit, is_vein};
use crate::state::{Event, World};

/// Go: `shipKind`, die Art des Schiffs-Platzes (ship.go): dient dem Inselwechsel, nicht dem Sieg.
const SHIP_KIND: &str = "ship";

/// Go: `checkVictory` prüft das Ziel der Insel nach dem Tick aller Stufen und meldet den Sieg in Stufe 0.
pub fn check_victory(isl: &mut Island) {
    if isl.won || isl.stages.is_empty() || !isl.is_last() || !goal_reached(isl) {
        return;
    }
    isl.won = true;
    let mut ev = Event::new();
    ev.insert("goal".to_string(), Value::from(isl.options.goal.clone()));
    ev.insert("day".to_string(), Value::from(isl.stages[0].cycle.day));
    emit(&mut isl.stages[0], "victory", ev);
}

/// Go: `goalReached`.
fn goal_reached(isl: &mut Island) -> bool {
    match isl.options.goal.as_str() {
        "endboss" => isl.endboss_defeated,
        "gold" => isl.gold_collected >= isl.values().goals.gold,
        "days" => isl.stages[0].cycle.day > isl.values().goals.days,
        "mineAll" => all_mined(isl),
        "buildAll" => all_built(isl),
        _ => false,
    }
}

/// Go: `allMined`: Kein endliches Ressourcenobjekt steht mehr. Endlich sind Objekte aus dem Level, nicht Adern
/// (world_vein.go) und nicht Plantage-Bäume (building_plantation.go). Eine Insel, auf der nie eines stand, gewinnt nie
/// (`had_finite`).
pub fn all_mined(isl: &mut Island) -> bool {
    let mut finite = false;
    for w in &isl.stages {
        let planted = planted_trees(w);
        let v = w.vals.clone().unwrap_or_else(|| crate::state::links::values(w));
        if w.nodes.iter().any(|n| !is_vein(&v, n) && !planted.contains(&n.id)) {
            finite = true;
            break;
        }
    }
    if finite {
        isl.had_finite = true;
        return false;
    }
    isl.had_finite
}

/// Go: `plantedTrees`, die Knoten-IDs der Bäume der Plantagen einer Stufe (Go: `map[*ResourceNode]bool`).
pub fn planted_trees(w: &World) -> BTreeSet<i64> {
    w.sites
        .iter()
        .flat_map(|s| s.plots.iter())
        .filter_map(|p| p.tree)
        .collect()
}

/// Go: `allBuilt`: Alle Bauplätze aller Stufen sind gebaut; eine Insel ohne Bauplatz gewinnt nie.
pub fn all_built(isl: &Island) -> bool {
    let mut n = 0;
    for w in &isl.stages {
        for s in &w.sites {
            if s.kind == SHIP_KIND || isl.hermit.is_hall(s) {
                continue; // das Schiff dient dem Inselwechsel, nicht dem Sieg (letzte Insel: nutzlos); das Bauwerk des Einsiedlers ist freiwillig (RU62.10)
            }
            if s.state != "built" {
                return false;
            }
            n += 1;
        }
    }
    n > 0
}
