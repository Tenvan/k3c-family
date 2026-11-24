//! Flaschenpost (GS4.6, B-971, R1 bis R6): Am Tagesanbruch liegt auf Stufe 1 an einem Inselende eine Flasche, sobald die
//! Insel ein Ziel hat (hier: eine Doppel-Truhe der Stufe, `coop.chests`) und der Rhythmus es erlaubt. Ein lebender Monarch
//! hebt sie auf; danach markiert die Insel ein Ziel, bis ein Monarch dort ist. Liegt die Flasche beim Abend noch, ist sie
//! weg. Werte aus data/bottle.json (`World::bottle_cfg`, beim Bau der Insel gesetzt). Der Spielstand (R8) und die
//! Geschichtszeilen (R7) folgen in den nächsten Schritten.

use std::sync::Arc;

use k3c_data::Data;
use k3c_data::bottle::BottleData;
use k3c_rng::{Rng, pick};

use crate::events::emit;
use crate::island::Island;
use crate::state::World;
use crate::state::common::event;
use serde_json::json;

/// Der Zustand der Flaschenpost einer Stufe. Nicht im Spielstand (folgt in GS4.6 Schritt 3).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BottleState {
    /// Ort der liegenden Flasche in Units; `None` = keine Flasche liegt.
    pub ashore: Option<f64>,
    /// Ort des markierten Ziels in Units; `None` = keine Markierung.
    pub target: Option<f64>,
    /// Tag, an dem die letzte Flasche erschien (zählt den Rhythmus, R1).
    pub last_day: Option<i64>,
    /// Zahl der aufgehobenen Flaschen (R7, für die Geschichtszeile).
    pub found: i64,
}

impl Island {
    /// Schaltet die Flaschenpost der Insel ein (Aufruf beim Bau der Insel): Konfiguration nur auf der Stufe der Tiefe 0.
    pub fn enable_bottle(&mut self, data: &Data) {
        let cfg = Arc::new(data.bottle.clone());
        for w in &mut self.stages {
            enable(w, Arc::clone(&cfg));
        }
    }
}

/// Die Konfiguration einer Stufe setzen, nur auf der Stufe der Tiefe 0.
pub fn enable(w: &mut World, cfg: Arc<BottleData>) {
    if w.biome.depth == 0 {
        w.bottle_cfg = Some(cfg);
    }
}

/// Ein lebender Monarch (gesteuert, nicht gefallen) an `x` innerhalb `range`.
fn living_near(w: &World, x: f64, range: f64) -> Option<i64> {
    w.players
        .iter()
        .find(|p| !p.free && crate::crown::crowned(p) && (p.x - x).abs() < range)
        .map(|p| p.index)
}

/// Ziele der Insel: die wirksamen Doppel-Truhen der Stufe (R5).
fn targets(w: &World) -> Vec<f64> {
    w.coop.chests.iter().map(|c| c.x).collect()
}

/// Morgengrauen (aus `dawn_break`): erscheint die Flasche, wenn Rhythmus, Ziel und Ort es erlauben (R1 bis R3).
pub fn at_dawn(w: &mut World) {
    let Some(cfg) = w.bottle_cfg.clone() else {
        return;
    };
    let day = w.cycle.day;
    let rhythmus = day >= cfg.first_day && w.bottle.last_day.is_none_or(|l| day - l >= cfg.every_days);
    if w.peace || w.bottle.ashore.is_some() || w.bottle.target.is_some() || !rhythmus || targets(w).is_empty() {
        return;
    }
    let mut r = Rng::new(&format!("{}:bottle:{day}", w.seed));
    let margin = cfg.edge_margin_min_units + r.next() * (cfg.edge_margin_max_units - cfg.edge_margin_min_units);
    let x = if r.next() < 0.5 { margin } else { w.width_units - margin };
    w.bottle.ashore = Some(x);
    w.bottle.last_day = Some(day);
    emit(w, "bottleAshore", event(json!({"x": x, "day": day})));
}

/// Abend (aus `step_cycle`): liegt die Flasche noch, verschwindet sie (R4); der Rhythmus zählt weiter.
pub fn at_dusk(w: &mut World) {
    if w.bottle.ashore.take().is_some() {
        emit(w, "bottleLost", event(json!({})));
    }
}

/// Je Takt: aufheben (R4), Ziel setzen (R5) und Ziel erreichen (R6).
pub fn step_pickup(w: &mut World) {
    let Some(cfg) = w.bottle_cfg.clone() else {
        return;
    };
    if let Some(x) = w.bottle.ashore {
        if let Some(player) = living_near(w, x, cfg.pickup_range_units) {
            w.bottle.ashore = None;
            w.bottle.found += 1;
            let pool = targets(w);
            if !pool.is_empty() {
                let mut r = Rng::new(&format!("{}:bottle-target:{}", w.seed, w.bottle.found));
                w.bottle.target = Some(pick(&mut r, &pool));
            }
            emit(w, "bottleFound", event(json!({"player": player, "n": w.bottle.found})));
        }
    }
    if let Some(t) = w.bottle.target {
        if !targets(w).iter().any(|c| (c - t).abs() < f64::EPSILON) {
            w.bottle.target = None; // das Ziel besteht nicht mehr (R6)
        } else if let Some(player) = living_near(w, t, cfg.target_range_units) {
            w.bottle.target = None;
            emit(w, "bottleTargetReached", event(json!({"player": player})));
        }
    }
}

#[cfg(test)]
#[path = "bottle_tests.rs"]
mod bottle_tests;
