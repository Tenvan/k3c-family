//! Ausrüstung von oben (ECO1.7, B-1197): Seit ECO1.6 steht die Werkstatt (Schwert, Hammer) nur in Stufe 0. Ein freier
//! Bürger einer tieferen Stufe, die selbst keine Werkstatt hat, geht dann zur Treppe nach oben (Auftrag `ascend`,
//! `unit/jobs.rs`), kommt oben an der Treppe nach unten heraus und holt dort wie jeder Bürger Schwert oder Hammer aus dem
//! Regal (`weapon_to_fetch`). Mit der Ausrüstung (oder ohne, wenn oben nichts mehr zu holen ist) kehrt er in seine Stufe
//! zurück. Liegt Ausrüstung am Boden der eigenen Stufe, holt ihn der Bürger wie bisher von dort (`pickup`).
//!
//! Nur mit den nativen Regeln (`WorldValues::native_rules`); die Go-Läufe kennen die Besorgung nicht. Der Weg ist
//! deterministisch: Stufen und Truppen in Index-Reihenfolge, je Stufe höchstens ein Bürger zugleich unterwegs nach oben.

use super::Island;
use crate::state::links::{fighters, troop_limit};
use crate::state::{Troop, World};

/// Der Auftrag, der den Bürger zur Treppe nach oben führt; `amount` 1 heißt: an der Treppe angekommen.
pub const ASCEND: &str = "ascend";

/// Wie viele Bürger der Stufe gerade den Auftrag `typ` haben.
fn fetching(w: &World, typ: &str) -> i64 {
    let n = w
        .troops
        .iter()
        .filter(|t| t.job.as_ref().is_some_and(|j| j.r#type == typ))
        .count();
    n as i64
}

/// Liegt im Regal einer gebauten Werkstatt etwas, das noch kein Bürger oben holt und ein Bürger von unten holen darf
/// (Hammer immer, Schwert nur unter dem Truppen-Limit wie in `weapon_to_fetch`)?
fn surface_gear(w: &World) -> bool {
    let room = fighters(w) < troop_limit(w);
    let (swords, hammers) = w
        .sites
        .iter()
        .filter(|s| s.kind == "workshop" && s.state == "built")
        .fold((0, 0), |(a, b), s| (a + s.swords, b + s.hammers));
    hammers > fetching(w, "fetchHammer") || (room && swords > fetching(w, "fetchSword"))
}

/// Die gebaute Treppe der Art `kind` in `w` (Ort), `None` ohne.
pub fn built_stairs(w: &World, kind: &str) -> Option<f64> {
    w.sites
        .iter()
        .find(|s| s.kind == kind && s.state == "built")
        .map(|s| s.x)
}

/// `IslandRef::gear_above` der Stufe `i`: oben gibt es etwas zu holen, die Stufe hat keine eigene Werkstatt und eine
/// gebaute Treppe nach oben; nur mit nativen Regeln.
pub fn gear_above(isl: &Island, i: usize) -> bool {
    if i == 0 || !isl.values().native_rules {
        return false;
    }
    let w = &isl.stages[i];
    let own = w.sites.iter().any(|s| s.kind == "workshop" && s.state == "built");
    !own && built_stairs(w, "stairsUp").is_some() && surface_gear(&isl.stages[0])
}

/// Hat der Bürger oben etwas geholt (Beruf, Kämpfer) oder gibt es für ihn nichts zu tun?
fn done_upstairs(t: &Troop) -> bool {
    let equipped = t.kind != "peasant" || t.hammer;
    equipped || t.job.is_none()
}

/// Ein Schritt der Besorgungen nach den Stufenschritten: erst die Rückkehr aus Stufe 0, dann der Aufstieg derer, die an
/// der Treppe stehen. Neu Angekommene warten einen Takt, damit sie oben einen Auftrag finden, bevor sie zurückgeschickt
/// werden.
pub fn step_errands(isl: &mut Island) {
    if isl.stages.len() < 2 || !isl.values().native_rules {
        return;
    }
    let mut down: Vec<(usize, Troop)> = Vec::new();
    let mut ti = 0;
    while ti < isl.stages[0].troops.len() {
        let t = &isl.stages[0].troops[ti];
        if t.errand_from > 0 && !t.fleeing && done_upstairs(t) {
            down.push(take(&mut isl.stages[0], ti));
        } else {
            ti += 1;
        }
    }
    let mut up: Vec<Troop> = Vec::new();
    for i in 1..isl.stages.len() {
        let mut ti = 0;
        while ti < isl.stages[i].troops.len() {
            let arrived = isl.stages[i].troops[ti]
                .job
                .as_ref()
                .is_some_and(|j| j.r#type == ASCEND && j.amount == 1);
            if arrived {
                let (_, mut t) = take(&mut isl.stages[i], ti);
                t.errand_from = i as i64 + 1;
                up.push(t);
            } else {
                ti += 1;
            }
        }
    }
    for (i, t) in down {
        let x = built_stairs(&isl.stages[i], "stairsUp").unwrap_or(isl.stages[i].hub_x);
        place(&mut isl.stages[i], t, x, 0);
    }
    for t in up {
        let x = built_stairs(&isl.stages[0], "stairsDown").unwrap_or(isl.stages[0].hub_x);
        let from = t.errand_from;
        place(&mut isl.stages[0], t, x, from);
    }
}

/// Nimmt die Truppe `ti` aus der Stufe; gibt ihre Aufträge frei und löst sie aus Gruppe und Schlange. Liefert mit ihr die
/// Herkunft (`errand_from - 1`, bei der Rückkehr das Ziel).
fn take(w: &mut World, ti: usize) -> (usize, Troop) {
    crate::unit::jobs::release_job(w, ti);
    crate::combat::leave_squad(w, ti);
    let t = w.troops.remove(ti);
    (usize::try_from(t.errand_from - 1).unwrap_or(0), t)
}

/// Stellt die Truppe als neue Truppe der Stufe an die Treppe `x`: neue ID, ohne Auftrag; Art, Leben und Beruf bleiben.
fn place(w: &mut World, old: Troop, x: f64, errand_from: i64) {
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: old.kind,
        x,
        hp: old.hp,
        max_hp: old.max_hp,
        anchor_x: x,
        target_x: x,
        profession: old.profession,
        hammer: old.hammer,
        errand_from,
        ..Troop::default()
    });
}
