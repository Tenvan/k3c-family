//! Events der Nacht (Port von `engine/sim/events_moon.go`, B-131, docs/rules/bosse.md § 2): Vollmond jede 7. Nacht (mehr
//! Wölfe, ein Alpha-Wolf, Belohnung bei Tagesanbruch), Blutmond jede 13. (Schaden und Gold-Drop aller Gegner höher). Werte in
//! data/events.json. Sie gelten für die Nacht des globalen Zyklus und nur auf Inseln; Campaign und einzelne Welten
//! (Golden-Läufe) bleiben unverändert. `w.island` liest Rust aus dem Spiegel `World::island_events`.

use k3c_data::events::NightEvent;
use serde_json::json;

use super::emit;
use super::values::night;
use crate::enemies::waves::{SpawnOrder, spawn_orders};
use crate::state::World;
use crate::state::common::event;
use crate::state::links::{scatter_coins, values};

/// Go: `eventInNight`: Das Event läuft in Nacht `n` der Insel (Nacht n = `cycle.day` n, die Nacht gehört zum Tag davor), nach
/// Rhythmus oder erzwungen von der Dev-Aktion event (`DevFlags::nights`).
pub fn event_in_night(w: &World, ev: &NightEvent, n: i64) -> bool {
    let Some(isl) = w.island_events.as_ref() else {
        return false;
    };
    if n <= 0 {
        return false;
    }
    // ohne Nacht-Rhythmus: merchantRaid
    (ev.every_nights > 0 && n % ev.every_nights == 0) || isl.nights.get(&ev.id).copied().unwrap_or(0) == n
}

/// Go: `eventActive`: Das Event läuft jetzt (Phase night).
pub fn event_active(w: &World, ev: &NightEvent) -> bool {
    w.cycle.phase == "night" && event_in_night(w, ev, w.cycle.day)
}

/// Go: `eventStage`: Die Stufe meldet die Events der Insel (Stufe 0), damit jedes Event je Nacht genau einmal erscheint.
fn event_stage(w: &World) -> bool {
    w.island_events.as_ref().is_some_and(|i| i.first_stage)
}

/// Go: `nightEventsStart`: meldet bei Einbruch der Nacht die Events dieser Nacht (`step_cycle`).
pub fn night_events_start(w: &mut World) {
    if !event_stage(w) {
        return;
    }
    for ev in values(w).events.iter() {
        if event_active(w, ev) {
            let e = event(json!({"event": ev.id, "day": w.cycle.day}));
            emit(w, "eventStarted", e);
        }
    }
}

/// Go: `nightEventsEnd`: meldet bei Tagesanbruch das Ende der Events der vergangenen Nacht (nur Stufe 0) und zahlt die
/// Vollmond-Belohnung in jeder bespielten Stufe, deren Burg in dieser Nacht nicht gefallen ist, an ihren Hub (B-379,
/// `step_cycle`, Phase dawn). Der Übergang in den Morgen kommt je Stufe und Nacht einmal, also auch die Belohnung.
pub fn night_events_end(w: &mut World) {
    let night_no = w.cycle.day - 1;
    if event_stage(w) {
        for ev in values(w).events.iter() {
            if event_in_night(w, ev, night_no) {
                let e = event(json!({"event": ev.id, "day": night_no}));
                emit(w, "eventEnded", e);
            }
        }
    }
    super::storm::raid_reward(w, night_no); // Überfall (B-991), wie der Vollmond
    let Some(full) = night(w, "fullMoon") else {
        return;
    };
    // nur in Stufen mit Portalen (kampf.md § 2)
    if event_in_night(w, &full, night_no) && w.castle_fell_night != night_no && stage_played(w) && !w.portals.is_empty()
    {
        let x = w.hub_x;
        scatter_coins(w, x, full.reward_gold);
    }
}

/// Go: `stagePlayed`: In der Stufe steht mindestens ein gesteuerter Monarch (Getrennte, `Player::free`, zählen nicht).
pub fn stage_played(w: &World) -> bool {
    w.players.iter().any(|p| !p.free)
}

/// Go: `fullMoonExtra`: In einer Vollmondnacht kommen zur Welle einer Tag/Nacht-Stufe zusätzliche Gegner der Art Kind
/// (Anteil der gezogenen, aufgerundet) und genau ein Elite-Gegner (`start_wave`).
pub fn full_moon_extra(w: &mut World, plan: &[SpawnOrder]) -> Vec<SpawnOrder> {
    let Some(full) = night(w, "fullMoon") else {
        return Vec::new();
    };
    if w.biome.cycle.r#type != "dayNight" || !event_active(w, &full) || w.portals.is_empty() {
        return Vec::new();
    }
    let n = plan.iter().filter(|s| s.kind == full.kind).count();
    let mut picks = vec![full.elite.clone()];
    let extra = (n as f64 * full.wolf_share).ceil() as usize;
    picks.extend(std::iter::repeat_n(full.kind.clone(), extra));
    let v = values(w);
    spawn_orders(&v, &picks, &w.portals, &mut w.rng)
}

/// Go: `moonDamage`: der Schaden eines Gegnerangriffs, im Blutmond mit dem Faktor der Daten (attack, Flächenangriff und
/// Splitter der Bosse, B-380); die Bodenfläche (Hazard) bleibt ohne Faktor.
pub fn moon_damage(w: &World, damage: f64) -> f64 {
    match night(w, "bloodMoon") {
        Some(blood) if event_active(w, &blood) => damage * blood.damage_factor,
        _ => damage,
    }
}

/// Go: `moonDrop`: der Gold-Drop eines besiegten Gegners, im Blutmond mit dem Faktor der Daten (`enemy_drop`).
pub fn moon_drop(w: &World, gold: i64) -> i64 {
    match night(w, "bloodMoon") {
        Some(blood) if event_active(w, &blood) => (gold as f64 * blood.drop_factor).round() as i64,
        _ => gold,
    }
}
