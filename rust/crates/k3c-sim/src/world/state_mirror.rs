//! Zustands-Spiegel (Port von `engine/sim/world_state_mirror.go`): Boss-Phase, Warnkreis, Nacht-Events und der Wechselpunkt
//! der Insel werden am Ende von `step_island` in den Zustand der Stufe geschrieben, damit das Protokoll sie mitträgt.
//! `WarnZone`, `ActiveEvent` und `SwitchState` stehen in `state/shared.rs`.
//!
//! Abweichung von Go: Die Insel steht nicht in der Stufe (Zustandsmodell, Entscheidung 4). `mirror_state` bekommt darum den
//! Index der Stufe und den laufenden Endboss-Kampf der Insel (Go: `w.island.endboss`); `mirror_switch` nimmt die Insel selbst.

use std::sync::Arc;

use super::pending::event_active;
use super::vals;
use crate::boss::data::boss_by_id;
use crate::boss::endboss::{EndbossFight, boss_ability_of};
use crate::island::Island;
use crate::pending::unit_x;
use crate::state::shared::{ActiveEvent, SwitchState, WarnZone, WorldValues};
use crate::state::{IslandRef, World};

/// Go: `mirrorState`: setzt Boss-Phase, Warnkreis und Nacht-Events der Stufe `stage` (Ende von `StepIsland`). `endboss` ist
/// der laufende Endboss-Kampf der Insel (`None` ohne Insel oder ohne Kampf).
pub fn mirror_state(w: &mut World, stage: usize, endboss: Option<&EndbossFight>) {
    let v = vals(w);
    for ei in 0..w.enemies.len() {
        if w.enemies[ei].boss {
            mirror_boss(w, &v, ei, stage, endboss);
        }
    }
    w.night_events.clear();
    for ev in &v.events {
        if event_active(w, ev) {
            let seconds_left = w.cycle.seconds_left;
            w.night_events.push(ActiveEvent {
                id: ev.id.clone(),
                seconds_left,
            });
        }
    }
}

/// Go: `mirrorBoss`: Phase (ab 1, nur der laufende Endboss) und Warnkreis (nur bei der Fähigkeit `aoe`) des Gegners `ei`.
fn mirror_boss(w: &mut World, v: &Arc<WorldValues>, ei: usize, stage: usize, endboss: Option<&EndbossFight>) {
    let (id, kind, x, aoe_in) = {
        let e = &w.enemies[ei];
        (e.id, e.kind.clone(), e.x, e.aoe_in)
    };
    let phase = endboss
        .filter(|f| f.home == stage && f.boss_id == id)
        .map_or(0, |f| f.phase as i64 + 1);
    let mut warn = None;
    if let Some(b) = boss_by_id(&v.bosses, &kind) {
        // `boss_ability_of` liest die Phase aus der Insel; nur der Endboss braucht sie
        let isl = endboss.filter(|_| b.kind == "end").map(|f| IslandRef {
            endboss: Some(f.clone()),
            ..IslandRef::default()
        });
        let a = boss_ability_of(b, isl.as_ref());
        if a.r#type == "aoe" {
            warn = Some(WarnZone {
                x: unit_x(x),
                r: a.radius,
                r#in: aoe_in,
            });
        }
    }
    let e = &mut w.enemies[ei];
    e.phase = phase;
    e.warn = warn;
}

/// Go: `mirrorSwitch`: setzt den Wechselzustand der Insel in allen Stufen (nach `step_island_switch`).
pub fn mirror_switch(isl: &mut Island) {
    let s = isl.gate_open.then(|| {
        let mut completed = isl.completed.clone();
        completed.push(isl.island_id.clone());
        SwitchState {
            open: true,
            progress: isl.switch_progress.min(1.0),
            ready: isl.switch_ready,
            targets: isl.targets(),
            choice: isl.chosen_target(),
            completed,
        }
    });
    for w in &mut isl.stages {
        w.island_switch = s.clone();
    }
}
