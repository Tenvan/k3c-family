//! Takt des Einsiedlers einer Insel (RU62.10): nach den Stufen und vor dem Wechsel (`step_island`). Ein Einwurf an der Hütte
//! (`pay.rs`, Stufe der Tiefe 1) startet den Bau auf dem Hub-Platz (Stufe der Tiefe 0): kein Bauer, kein Material, nur die
//! Zeit `buildSeconds` zählt. Ein zerstörtes Bauwerk (`destroy_site` setzt den Platz auf leer) wird bei Tag ohne neue Zahlung
//! wieder gebaut. Ohne Einsiedler (`cfg` leer, auch `standard`) kehrt der Takt sofort zurück, ohne zu allozieren.
use k3c_data::hermits::HermitEntry;
use serde_json::json;

use super::{BUILDING_STATE, publish};
use crate::events::{build_progress_event, emit, unit_x};
use crate::island::Island;
use crate::state::common::event;

/// Die Spielzeit, in der ein zerstörtes Bauwerk wieder gebaut wird.
const REBUILD_PHASE: &str = "day";

/// Ein Takt des Einsiedlers der Insel.
pub fn step_hermit(isl: &mut Island, dt: f64) {
    let Some(cfg) = isl.hermit.cfg.clone() else {
        return;
    };
    let Some(entry) = cfg.entry(&isl.hermit.id) else {
        return;
    };
    let pending = isl.stages.iter().any(|w| w.hermit.as_ref().is_some_and(|h| h.pending));
    for w in &mut isl.stages {
        if let Some(h) = w.hermit.as_mut() {
            h.pending = false;
        }
    }
    let Some((wi, si)) = isl.hall_site() else {
        return;
    };
    let unpaid = isl.stages[wi].sites[si].state == "unpaid";
    let building = isl.stages[wi].sites[si].state == BUILDING_STATE;
    if unpaid && pending && !isl.hermit.paid {
        isl.hermit.paid = true;
        begin(isl, wi, si, entry, false);
    } else if unpaid && isl.hermit.paid && isl.stages[wi].cycle.phase == REBUILD_PHASE {
        begin(isl, wi, si, entry, true);
    } else if building {
        advance(isl, wi, si, entry, dt);
    }
    publish(isl);
}

/// Der Bau beginnt: der Platz ist leer, wird bezahlt (oder wieder aufgebaut) und läuft mit Fortschritt 0.
fn begin(isl: &mut Island, wi: usize, si: usize, entry: &HermitEntry, rebuild: bool) {
    let w = &mut isl.stages[wi];
    let s = &mut w.sites[si];
    s.state = BUILDING_STATE.to_string();
    s.build_progress = 0.0;
    s.paid_gold = 0;
    s.hp = 0.0;
    s.max_hp = entry.hp; // `destroy_site` setzt max_hp mangels Eintrag in buildings.json auf 0
    let (id, x) = (s.id, unit_x(s.x));
    let ev = event(json!({
        "site": id, "kind": entry.building, "hermit": entry.id, "x": x, "rebuild": rebuild
    }));
    emit(w, "hermitBuilding", ev);
}

/// Die Bauzeit läuft; mit 100 % steht das Bauwerk (Ereignis `built` wie bei jedem Bau).
fn advance(isl: &mut Island, wi: usize, si: usize, entry: &HermitEntry, dt: f64) {
    let w = &mut isl.stages[wi];
    let (id, before) = (w.sites[si].id, w.sites[si].build_progress);
    let progress = before + dt / entry.build_seconds.max(0.1);
    w.sites[si].build_progress = progress;
    build_progress_event(w, id, before);
    if progress < 1.0 {
        return;
    }
    let s = &mut w.sites[si];
    s.state = "built".to_string();
    s.build_progress = 1.0;
    s.hp = s.max_hp;
    let (kind, x) = (s.kind.clone(), unit_x(s.x));
    w.events.push(event(json!({"type": "built", "kind": kind, "x": x})));
}
