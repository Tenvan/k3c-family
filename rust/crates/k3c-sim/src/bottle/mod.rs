//! Flaschenpost (GS4.6, B-971, B-1261, R1 bis R8): Am Tagesanbruch liegt auf Stufe 1 an einem Inselende eine Flasche, sobald
//! die Insel ein Ziel hat und der Rhythmus es erlaubt. Ein lebender Monarch hebt sie auf; danach markiert die Insel ein Ziel
//! auf irgendeiner Stufe, bis ein Monarch dort ist oder die Truhe weg ist. Liegt die Flasche beim Abend noch, ist sie weg.
//! Werte aus data/bottle.json (`World::bottle_cfg`, beim Bau der Insel gesetzt). Der Zielpool ist die Liste aller noch
//! nicht gefundenen Truhen über alle Stufen; Erscheinen (`at_dawn`) und Aufheben laufen je Stufe bzw. über die Insel.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use k3c_data::Data;
use k3c_data::bottle::BottleData;
use k3c_rng::{Rng, pick};

use crate::events::emit;
use crate::island::Island;
use crate::state::World;
use crate::state::common::event;
use serde_json::json;

/// Ein markiertes Ziel: die Truhe an `x` auf Stufe `stage` (Index in `Island::stages`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BottleTarget {
    pub stage: usize,
    pub x: f64,
}

/// Der Zustand der Flaschenpost der Insel. Liegt auf der Stufe mit Tiefe 0. Nicht im Spielstand außer `BottleSave`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BottleState {
    /// Ort der liegenden Flasche in Units; `None` = keine Flasche liegt.
    pub ashore: Option<f64>,
    /// Markiertes Ziel (Stufe und Ort); `None` = keine Markierung.
    pub target: Option<BottleTarget>,
    /// Tag, an dem die letzte Flasche erschien (zählt den Rhythmus, R1).
    pub last_day: Option<i64>,
    /// Zahl der aufgehobenen Flaschen (R7, für die Geschichtszeile).
    pub found: i64,
    /// Zahl der Geschichtszeilen der Welt (`story.bottles`), aus der Welt beim Bau gesetzt; nicht im Spielstand.
    pub lines: usize,
    /// Zahl der Ziele im Zielpool der Insel, vor dem Stufentakt gesetzt (`prepare`); nicht im Spielstand.
    pub pool_len: usize,
}

/// Der Spielstand-Teil der Flaschenpost (Version 14, R8). Ohne Zustand fehlt das Feld.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BottleSave {
    /// Ort der liegenden Flasche in Units.
    #[serde(rename = "ashore", skip_serializing_if = "Option::is_none")]
    pub ashore: Option<f64>,
    /// Ort des markierten Ziels in Units.
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
    /// Stufe des markierten Ziels (B-1261); fehlt, wenn 0.
    #[serde(rename = "targetStage", skip_serializing_if = "is_zero")]
    pub target_stage: i64,
    /// Tag der letzten Flasche.
    #[serde(rename = "lastDay", skip_serializing_if = "Option::is_none")]
    pub last_day: Option<i64>,
    /// Zahl der aufgehobenen Flaschen.
    #[serde(rename = "found", skip_serializing_if = "is_zero")]
    pub found: i64,
}

fn is_zero(x: &i64) -> bool {
    *x == 0
}

impl BottleSave {
    /// Leer, wenn keine Flasche und keine Markierung läuft und nichts gefunden wurde.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl BottleState {
    /// Der Spielstand-Teil dieses Zustands.
    pub fn to_save(&self) -> BottleSave {
        BottleSave {
            ashore: self.ashore,
            target: self.target.map(|t| t.x),
            target_stage: self.target.map_or(0, |t| t.stage as i64),
            last_day: self.last_day,
            found: self.found,
        }
    }

    /// Der Zustand aus dem Spielstand-Teil.
    pub fn from_save(s: &BottleSave) -> BottleState {
        BottleState {
            ashore: s.ashore,
            target: s.target.map(|x| BottleTarget {
                stage: s.target_stage as usize,
                x,
            }),
            last_day: s.last_day,
            found: s.found,
            lines: 0,
            pool_len: 0,
        }
    }
}

impl Island {
    /// Schaltet die Flaschenpost der Insel ein (Aufruf beim Bau der Insel): Konfiguration nur auf der Stufe der Tiefe 0.
    pub fn enable_bottle(&mut self, data: &Data) {
        let cfg = Arc::new(data.bottle.clone());
        let lines = self.world.story.bottles.len();
        for w in &mut self.stages {
            enable(w, Arc::clone(&cfg));
            if w.biome.depth == 0 {
                w.bottle.lines = lines;
            }
        }
    }
}

/// Die Konfiguration einer Stufe setzen, nur auf der Stufe der Tiefe 0.
pub fn enable(w: &mut World, cfg: Arc<BottleData>) {
    if w.biome.depth == 0 {
        w.bottle_cfg = Some(cfg);
    }
}

/// Der Index der Stufe mit Tiefe 0 (hier liegt die Flasche).
fn home(isl: &Island) -> Option<usize> {
    isl.stages.iter().position(|w| w.biome.depth == 0)
}

/// Ein lebender Monarch (gesteuert, nicht gefallen) an `x` innerhalb `range`; bei Gleichstand der höchste Index (R4).
fn living_near(w: &World, x: f64, range: f64) -> Option<i64> {
    w.players
        .iter()
        .filter(|p| !p.free && crate::crown::crowned(p) && (p.x - x).abs() < range)
        .map(|p| p.index)
        .max()
}

/// Der Zielpool der Insel (R5): jede noch nicht gefundene Truhe über alle Stufen, in fester Reihenfolge.
pub fn pool(isl: &Island) -> Vec<BottleTarget> {
    isl.stages
        .iter()
        .enumerate()
        .flat_map(|(stage, w)| {
            w.pickups
                .iter()
                .filter(|p| p.kind == "chest")
                .map(move |p| BottleTarget { stage, x: p.x })
        })
        .collect()
}

/// Ist `a` dieselbe Truhe wie `b` (Stufe und Ort)?
fn same(a: &BottleTarget, b: &BottleTarget) -> bool {
    a.stage == b.stage && (a.x - b.x).abs() < f64::EPSILON
}

/// Vor dem Stufentakt: setzt die Zahl der Ziele, damit der Morgen (`at_dawn`) weiß, ob eine Flasche gilt (R3).
pub fn prepare(isl: &mut Island) {
    let n = pool(isl).len();
    if let Some(h) = home(isl) {
        isl.stages[h].bottle.pool_len = n;
    }
}

/// Morgengrauen (aus `dawn_break`): erscheint die Flasche, wenn Rhythmus, Ziel und Ort es erlauben (R1 bis R3).
pub fn at_dawn(w: &mut World) {
    let Some(cfg) = w.bottle_cfg.clone() else {
        return;
    };
    let day = w.cycle.day;
    let rhythmus = day >= cfg.first_day && w.bottle.last_day.is_none_or(|l| day - l >= cfg.every_days);
    if w.peace || w.bottle.ashore.is_some() || w.bottle.target.is_some() || !rhythmus || w.bottle.pool_len == 0 {
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

/// Je Takt nach dem Stufentakt: aufheben (R4), Ziel setzen (R5), Ziel erreichen oder verlieren (R6).
pub fn step_bottle(isl: &mut Island) {
    let Some(h) = home(isl) else {
        return;
    };
    let Some(cfg) = isl.stages[h].bottle_cfg.clone() else {
        return;
    };
    let pool = pool(isl);
    if let Some(x) = isl.stages[h].bottle.ashore
        && let Some(player) = living_near(&isl.stages[h], x, cfg.pickup_range_units)
    {
        {
            let w = &mut isl.stages[h];
            w.bottle.ashore = None;
            w.bottle.found += 1;
            if !pool.is_empty() {
                let mut r = Rng::new(&format!("{}:bottle-target:{}", w.seed, w.bottle.found));
                w.bottle.target = Some(pick(&mut r, &pool));
            }
            // R7: die n-te Flasche nennt die Zeile `(n - 1) mod Anzahl`; ohne Zeilen -1
            let line = if w.bottle.lines == 0 {
                -1
            } else {
                (w.bottle.found - 1) % w.bottle.lines as i64
            };
            emit(
                w,
                "bottleFound",
                event(json!({"player": player, "n": w.bottle.found, "line": line})),
            );
        }
    }
    let Some(t) = isl.stages[h].bottle.target else {
        return;
    };
    if !pool.iter().any(|c| same(c, &t)) {
        // R6: die Truhe ist weg (von wem auch immer gefunden): Markierung endet, mit Ereignis
        isl.stages[h].bottle.target = None;
        emit(
            &mut isl.stages[h],
            "bottleTargetReached",
            event(json!({"player": -1, "gone": true})),
        );
    } else if let Some(player) = living_near(&isl.stages[t.stage], t.x, cfg.target_range_units) {
        isl.stages[h].bottle.target = None;
        emit(
            &mut isl.stages[h],
            "bottleTargetReached",
            event(json!({"player": player})),
        );
    }
}

#[cfg(test)]
#[path = "bottle_tests.rs"]
mod bottle_tests;
