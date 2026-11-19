//! Welt-Reittiere (RU62.8, B-970, docs/rules/monarch.md § 7; Entscheidung 013: neue Mechanik nur in Rust, ohne Go-Orakel):
//! Eine Welt aus `mounts.json › worlds` hat ein eigenes Reittier mit einer Stärke. Auf der Oberfläche jeder Insel steht
//! ein Gatter; wer dort `price` Münzen einwirft, reitet das Reittier bis zum Ende der Welt. Daten: data/mounts.json
//! (`k3c-data/mounts.rs`).
//!
//! Das gekaufte Reittier steht je Spieler (`Player::mount`, Katalog-ID; leer = Standard-Pferd) und im Spielstand
//! (`ISLAND_SAVE_VERSION` 11); das Gatter wird aus dem Insel-Seed berechnet (`k3c-rng`), nie gespeichert und ohne Eingriff
//! in den Level-Generator. `standard` und Welten ohne Eintrag haben weder Gatter noch Katalog (Verhalten wie heute).
//!
//! - `mod.rs`: Einschalten der Insel, Platz des Gatters, Tempo- und Sprintfaktor des Spielers.
//! - `buy.rs`: der Kauf am Gatter (Spielerschritt in `economy/players.rs`).
use std::sync::Arc;

use k3c_data::Data;
use k3c_rng::Rng;

use crate::island::Island;
use crate::state::mount_view::MountGate;
use crate::state::{World, WorldValues};

pub mod buy;

#[cfg(test)]
mod buy_tests;
#[cfg(test)]
mod gate_tests;
#[cfg(test)]
mod keep_tests;
#[cfg(test)]
mod save_tests;
#[cfg(test)]
mod testutil;

/// Abstand des Gatters zum Rand der Welt in Units.
pub const EDGE_MARGIN_UNITS: f64 = 10.0;

/// Das Welt-Reittier (Katalog-ID) der Welt `world_id`; `None` = Welt ohne eigenes Reittier.
pub fn world_mount<'a>(data: &'a Data, world_id: &str) -> Option<&'a str> {
    data.mounts.worlds.get(world_id).map(String::as_str)
}

/// Tempofaktor und Sprintfaktor des Monarchen `pi`: die Werte seines gekauften Welt-Reittiers, sonst die des
/// Standard-Reittiers aus den Daten der Welt. Ein unbekanntes oder in dieser Stufe nicht geführtes Reittier zählt nicht.
pub fn mount_factors(w: &World, v: &WorldValues, pi: usize) -> (f64, f64) {
    let own = &w.players[pi].mount;
    if !own.is_empty()
        && let Some(e) = w.mounts_cfg.as_ref().and_then(|c| c.entry(own))
    {
        return (e.speed_factor, e.sprint_factor);
    }
    let m = crate::monarch::mount_of(v, pi);
    (m.speed_factor, m.sprint_factor)
}

/// Der Ort des Gatters (Units): deterministisch aus Insel-Seed und Tiefe, mindestens `min_dist` vom Hub und
/// `EDGE_MARGIN_UNITS` vom Rand. Gibt es links und rechts vom Hub Platz, entscheidet die Länge der Streifen; passt keiner,
/// steht das Gatter am weiter entfernten Rand.
pub fn gate_x(seed: &str, depth: i64, width: f64, hub_x: f64, min_dist: f64) -> f64 {
    let (lo, hi) = (EDGE_MARGIN_UNITS, (width - EDGE_MARGIN_UNITS).max(EDGE_MARGIN_UNITS));
    let left = (hub_x - min_dist >= lo).then_some((lo, (hub_x - min_dist).min(hi)));
    let right = (hub_x + min_dist <= hi).then_some(((hub_x + min_dist).max(lo), hi));
    let len = |s: Option<(f64, f64)>| s.map_or(0.0, |(a, b)| (b - a).max(0.0));
    let total = len(left) + len(right);
    let mut rng = Rng::new(&format!("{seed}|mountGate|{depth}"));
    let r = rng.next();
    if total <= 0.0 {
        return match (left, right) {
            (Some((a, _)), _) => a,
            (_, Some((_, b))) => b,
            _ if (hub_x - lo).abs() >= (hi - hub_x).abs() => lo,
            _ => hi,
        };
    }
    let mut t = r * total;
    for (a, b) in [left, right].into_iter().flatten() {
        let l = (b - a).max(0.0);
        if t <= l {
            return a + t;
        }
        t -= l;
    }
    hi
}

impl Island {
    /// Schaltet die Welt-Reittiere der Insel ein (nur Welten aus mounts.json; Aufruf beim Bauen der Insel): Katalog in alle
    /// Stufen, das Gatter in die Oberflächen-Stufe (Tiefe 0). Ohne Eintrag der Welt geschieht nichts.
    pub fn enable_mounts(&mut self, data: &Data) {
        let Some(id) = world_mount(data, self.world_id()) else {
            return;
        };
        let Some(entry) = data.mounts.entry(id) else {
            return;
        };
        let cfg = Arc::new(data.mounts.clone());
        let gate = &data.mounts.mount_gate;
        let seed = self.seed.clone();
        for w in &mut self.stages {
            w.mounts_cfg = Some(Arc::clone(&cfg));
            if w.biome.depth == 0 {
                w.mount_gate = Some(MountGate {
                    x: gate_x(&seed, 0, w.width_units, w.hub_x, gate.min_hub_distance_units),
                    range: gate.range_units,
                    price: entry.price,
                    mount: entry.id.clone(),
                    sprite: entry.sprite.clone(),
                });
            }
        }
    }
}
