//! Feedback-Ereignisse (Port von `engine/sim/events.go`, B-139, Beschluss Q08): kurze Felder, damit im Mittel ≤ 200 Byte je
//! Tick und Client reichen. Orte `x` in Units; auf einer Insel trägt jedes Ereignis zusätzlich `stage` (`step_island`).
//! Die Liste aller Ereignis-Typen mit ihren Feldern steht im Kopfkommentar von `engine/sim/events.go`.

pub mod merchant;
pub mod moon;
pub mod storm;
pub mod values;

#[cfg(test)]
mod budget_tests;
#[cfg(test)]
mod island_tests;
#[cfg(test)]
mod location_tests;
#[cfg(test)]
mod merchant_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(test)]
mod moon_stages_tests;
#[cfg(test)]
mod moon_tests;
#[cfg(test)]
mod storm_tests;
#[cfg(test)]
pub(crate) mod testutil;

use serde_json::json;

use crate::economy::PayTarget;
use crate::state::common::event;
use crate::state::{Event, Projectile, World};

/// Go: `maxEventsPerTick`: die Obergrenze K je Tick und Stufe. Vorläufig, 🧑 bestätigt mit der Freigabe bzw. F4
/// (Benchmark). `kill` hat Priorität (K1.2, B-128/AC-04): Jeder Tod eines Gegners wird gemeldet.
pub const MAX_EVENTS_PER_TICK: usize = 32;

/// Go: `emit`: hängt ein Ereignis an. Es liest nur und verbraucht kein rng, damit sich der Spielverlauf nicht ändert.
pub fn emit(w: &mut World, typ: &str, mut fields: Event) {
    fields.insert("type".to_string(), json!(typ));
    w.events.push(fields);
}

/// Go: `unitX`: rundet einen Ort auf eine Nachkommastelle (kürzer im Protokoll; nur für Ereignisse, nie für den Zustand).
pub fn unit_x(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

/// Go: `hitEvent`: meldet wirksamen Schaden an einem Ziel.
pub fn hit_event(w: &mut World, target: &str, id: i64, x: f64, damage: f64) {
    let ev = event(json!({"x": unit_x(x), "target": target, "id": id, "damage": damage}));
    emit(w, "hit", ev);
}

/// Go: `arrowEvent`: meldet ein neues Geschoss `p` (noch nicht in `w.projectiles`) vom Schützen `from`.
pub fn arrow_event(w: &mut World, p: &Projectile, from: i64) {
    let ev = event(json!({"from": from, "to": p.target_id, "x": unit_x(p.x), "team": p.team}));
    emit(w, "arrow", ev);
}

/// Go: `coinGiveEvent`: meldet eine Münze, die ein Ziel bezahlt (Bauplatz, Landstreicher, Markierung einer Ressource).
pub fn coin_give_event(w: &mut World, pi: usize, t: &PayTarget) {
    let to = match t {
        PayTarget::Site(_) => "site",
        PayTarget::Troop(_) => "recruit",
        PayTarget::Sword(_) | PayTarget::Combat { .. } => "offer",
        _ => "mark",
    };
    let p = &w.players[pi];
    let ev = event(json!({"player": p.index, "x": unit_x(p.x), "to": to}));
    emit(w, "coinGive", ev);
}

/// Go: `buildProgressEvent`: meldet das Überschreiten von 25, 50 und 75 % (100 % ist `built`), höchstens eins je Schritt.
pub fn build_progress_event(w: &mut World, site_id: i64, before: f64) {
    let Some(s) = w.site(site_id) else {
        return;
    };
    let step = (s.build_progress * 4.0).floor();
    if step > (before * 4.0).floor() && (1.0..=3.0).contains(&step) {
        let ev = event(json!({"site": s.id, "kind": s.kind, "x": unit_x(s.x), "percent": step as i64 * 25}));
        emit(w, "buildProgress", ev);
    }
}

/// Go: `priorityEvent`: Tod und Bau gehen bei Überlauf vor (B-139/AC-03).
pub fn priority_event(ev: &Event) -> bool {
    matches!(
        ev.get("type").and_then(|t| t.as_str()),
        Some("crownFallen" | "castleFallen" | "built" | "buildProgress" | "destroyed" | "kill")
    )
}

/// Go: `capEvents`: begrenzt die Ereignisse eines Ticks auf `MAX_EVENTS_PER_TICK`: zuerst alle mit Priorität, dann die
/// übrigen in ihrer Reihenfolge; die Reihenfolge der behaltenen bleibt. `events_dropped` zählt die Verworfenen.
pub fn cap_events(w: &mut World) {
    w.events_dropped = 0;
    if w.events.len() <= MAX_EVENTS_PER_TICK {
        return;
    }
    let mut keep = vec![false; w.events.len()];
    let mut room = MAX_EVENTS_PER_TICK;
    for prio in [true, false] {
        for (i, ev) in w.events.iter().enumerate() {
            if room > 0 && priority_event(ev) == prio {
                keep[i] = true;
                room -= 1;
            }
        }
    }
    let total = w.events.len();
    let mut flags = keep.into_iter();
    w.events.retain(|_| flags.next().unwrap_or(false));
    w.events_dropped = (total - w.events.len()) as i64;
}
