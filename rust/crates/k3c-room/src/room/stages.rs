//! Stufen der Insel im Raum: Monarch `i` ist der Insel-Spieler mit Index `i`; jedes Gerät bekommt Level und Zustand jeder
//! Stufe, in der einer seiner Slots steht (B-176). Port der Teile von `engine/room/stages.go`, die `join` braucht
//! (vorgezogen aus RU24; `observer_stages` stammt aus `observer.go`).
use std::collections::BTreeMap;

use k3c_sim::island::Island;
use k3c_sim::state::World;
use serde_json::Value;

use super::env::Env;
use super::types::{Device, MAX_MONARCHS, Monarch, MonarchState, Room};

/// Die Stufen aller Monarchen, aufsteigend und ohne Doppelte; ohne Monarchen die Startstufe.
pub(super) fn observer_stages(isl: &Island, monarchs: usize, start: usize) -> Vec<usize> {
    if monarchs == 0 {
        return vec![start];
    }
    let mut out: Vec<usize> = (0..monarchs).filter_map(|i| isl.stage_of(i as i64)).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Die Stufen der Slots des Geräts, aufsteigend und ohne Doppelte; beim Beobachter die aller Monarchen.
fn device_stages(isl: &Island, monarchs: usize, start: usize, d: &Device) -> Vec<usize> {
    if d.observer {
        return observer_stages(isl, monarchs, start);
    }
    let mut out: Vec<usize> = d.slots.values().filter_map(|&idx| isl.stage_of(idx as i64)).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Baut den Zustand der Stufe `w` über `Env::snapshot` (Go `Room.snapshot`).
fn snapshot(env: &Env, isl: &Island, timescale: i64, paused: bool, w: &World) -> Value {
    let Some(build) = env.snapshot else {
        return serde_json::to_value(w).unwrap_or(Value::Null);
    };
    // ohne Dev-Mode fehlt devTimescale im Zustand
    let scale = if env.dev { timescale.max(1) } else { 0 };
    let mut out = build(w, scale, paused);
    if let (Value::Object(m), false) = (&mut out, isl.options.grade.is_empty()) {
        // Grad des Raums (B-080, KB2.6); die Dev-Aktion grade ändert ihn ab der nächsten Welle
        m.insert("grade".into(), Value::String(isl.options.grade.clone()));
    }
    out
}

impl Room {
    /// Die Stufe für neue Slots des Geräts: die des Monarchen im kleinsten Slot, ohne Slots die Startstufe des Raums.
    pub fn device_stage(&self, id: &str) -> usize {
        let first = self.devices.get(id).and_then(|d| d.slots.iter().next());
        match first {
            None => self.start,
            Some((_, &idx)) => self.isl.stage_of(idx as i64).unwrap_or(self.start),
        }
    }

    /// Schickt dem Gerät den Zustand jeder seiner Stufen (aufsteigend), einer neuen Stufe vorher das Level; Stufen ohne
    /// Slot fallen aus der Menge, ihr Strom endet. `states` sind die in diesem Tick schon gebauten Zustände je Stufe
    /// (`None`: nur für dieses Gerät bauen). `true`: Mindestens ein Level wurde gesendet.
    pub fn push_state(&mut self, env: &Env, id: &str, mut states: Option<&mut BTreeMap<usize, Value>>) -> bool {
        let Room {
            isl,
            devices,
            monarchs,
            start,
            tick,
            timescale,
            paused,
            ..
        } = self;
        let Some(d) = devices.get_mut(id) else {
            return false;
        };
        let now = device_stages(isl, monarchs.len(), *start, d);
        d.stages.retain(|s| now.contains(s));
        let mut level_sent = false;
        for s in now {
            let Some(w) = isl.stages.get(s) else { continue };
            if d.stages.insert(s) {
                d.peer.level(s as i64, w.biome.depth, &w.level);
                level_sent = true;
            }
            match states.as_deref_mut() {
                Some(m) => {
                    let st = m.entry(s).or_insert_with(|| snapshot(env, isl, *timescale, *paused, w));
                    d.peer.state(*tick, s as i64, st);
                }
                None => d
                    .peer
                    .state(*tick, s as i64, &snapshot(env, isl, *timescale, *paused, w)),
            }
        }
        level_sent
    }

    /// Die Stufen aller Monarchen (Index = Slot), zum Erkennen eines Stufenwechsels im Tick.
    pub fn depths(&self) -> Vec<Option<usize>> {
        (0..self.monarchs.len()).map(|i| self.isl.stage_of(i as i64)).collect()
    }
}

/// Die Startstufe einer geöffneten Insel: die Stufe des ersten Spielers, sonst die mit dieser Tiefe.
pub fn start_stage(isl: &Island, depth: i64) -> usize {
    if let Some(p) = isl.players().first() {
        return isl.stage_of(p.index).unwrap_or(0);
    }
    isl.stages.iter().position(|w| w.biome.depth == depth).unwrap_or(0)
}

/// Trägt die Spieler eines geladenen Stands als freie Monarchen ein. Annahme: Indizes 0…n−1 ohne Lücke (Stände aus dem
/// Raum sind so); sonst `None`.
pub fn free_monarchs(isl: &Island) -> Option<Vec<Monarch>> {
    let ps = isl.players();
    let ok = ps.len() <= MAX_MONARCHS && ps.iter().enumerate().all(|(i, p)| p.index == i as i64);
    ok.then(|| {
        ps.iter()
            .map(|_| Monarch {
                state: MonarchState::Free,
                ..Monarch::default()
            })
            .collect()
    })
}
