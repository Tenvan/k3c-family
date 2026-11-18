//! Stufenwechsel (Port von `engine/sim/world_travel.go`, Port von src/world/sim/travel.ts): Der Tiefen-Eingang am
//! Levelende führt nach unten, gebaute Treppen im Hub verbinden die Hubs. Alle gesteuerten, lebenden Monarchen müssen
//! gemeinsam am selben Punkt stehen; freie Monarchen (B-059) zählen nicht und reisen mit.
//!
//! Go: `hasDepth` liest die Paket-Variable `biomes`; hier die Biom-Liste der eingebetteten Daten (`base`). Ohne lesbare
//! Daten gibt es keine Tiefe und damit keinen Reisepunkt (der Fehler meldet `base()` an anderer Stelle).

use super::base::base;
use super::vals;
use crate::crown::crowned;
use crate::state::data::has_depth;
use crate::state::{Travel, World};

/// Go: `travelPoint` (world_travel.go): ein Reisepunkt einer Stufe (`via`: exit, stairsUp, stairsDown).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TravelPoint {
    pub x: f64,
    pub to_depth: i64,
    pub via: String,
}

/// Gibt es ein Biom dieser Tiefe (Go: `hasDepth`)?
fn depth_exists(depth: i64) -> bool {
    base().is_ok_and(|b| has_depth(&b.data.biomes, depth))
}

/// Go: `travelPoints` (world_travel.go): die Reisepunkte der Stufe (Tiefen-Eingang nach unten, gebaute Treppen).
pub fn travel_points(w: &World) -> Vec<TravelPoint> {
    let depth = w.biome.depth;
    let mut points = Vec::new();
    if depth_exists(depth + 1) {
        for e in w.level.entities.iter().filter(|e| e.kind == "exit") {
            points.push(TravelPoint {
                x: e.x,
                to_depth: depth + 1,
                via: "exit".into(),
            });
        }
    }
    for s in w.sites.iter().filter(|s| s.state == "built") {
        if s.kind == "stairsUp" && depth_exists(depth - 1) {
            points.push(TravelPoint {
                x: s.x,
                to_depth: depth - 1,
                via: "stairsUp".into(),
            });
        }
        if s.kind == "stairsDown" && depth_exists(depth + 1) {
            points.push(TravelPoint {
                x: s.x,
                to_depth: depth + 1,
                via: "stairsDown".into(),
            });
        }
    }
    points
}

/// Go: `readyAt`: Stehen alle Entscheider am Punkt? Ohne Entscheider gibt es keinen Wechsel. `deciders` sind Indizes in
/// `w.players`.
pub fn ready_at(w: &World, deciders: &[usize], x: f64, range_units: f64) -> bool {
    deciders.iter().all(|&pi| (w.players[pi].x - x).abs() <= range_units) && !deciders.is_empty()
}

/// Go: `stepTravel` (world_travel.go): startet und rechnet den gemeinsamen Stufenwechsel.
pub fn step_travel(w: &mut World, dt: f64) {
    if w.no_travel {
        return;
    }
    let deciders: Vec<usize> = (0..w.players.len())
        .filter(|&pi| crowned(&w.players[pi]) && !w.players[pi].free)
        .collect();
    let v = vals(w);
    let range = v.hub.travel.range_units;
    let Some(point) = travel_points(w)
        .into_iter()
        .find(|pt| ready_at(w, &deciders, pt.x, range))
    else {
        w.travel = None;
        return;
    };
    if w.travel.as_ref().is_none_or(|t| t.x != point.x) {
        w.travel = Some(Travel {
            x: point.x,
            to_depth: point.to_depth,
            via: point.via,
            progress: 0.0,
        });
    }
    if let Some(t) = w.travel.as_mut() {
        t.progress = (t.progress + dt / v.hub.travel.seconds).min(1.0);
    }
}
