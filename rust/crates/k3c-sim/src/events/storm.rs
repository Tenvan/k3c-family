//! Welt-Events Überfall, Sandsturm und Nebel (B-991, B-992, docs/rules/bosse.md § 2). Sie stehen in `data/events.json`, sind
//! in der Basis aus (`everyNights: 0`) und laufen nur in der Nacht des Events an der Oberfläche (Tiefe 0):
//! - Überfall (`raid`): zur Welle kommt eine schnelle Gruppe der Gegnerart `kind`, Anteil `share` der Wellengröße.
//! - Sandsturm (`sandstorm`): Tempo aller × `speedFactor`, Fernkampf-Reichweite aller × `rangeFactor`.
//! - Nebel (`fog`): Reichweite nur der Verteidiger × `rangeFactor` (Gegner unverändert).
//!
//! Sandsturm und Nebel setzen `World::weather_speed` und `weather_range` nach dem Taifun (die Faktoren multiplizieren sich)
//! und `weather_range_def` nur für die Verteidiger. Die Belohnung des Überfalls zahlt der Tagesanbruch wie beim Vollmond.
use k3c_data::events::{NightEvent, find_event};

use super::moon::{event_active, event_in_night, stage_played};
use crate::enemies::waves::spawn_orders;
use crate::state::links::{scatter_coins, values};
use crate::state::{QueuedSpawn, World};

/// Das Event `id`, wenn es in der Nacht läuft, die die Stufe gerade hat. Liest nur den Wertesatz (Arc), ohne Allokation.
fn running<'a>(w: &World, events: &'a [NightEvent], id: &str) -> Option<&'a NightEvent> {
    if w.cycle.phase != "night" {
        return None;
    }
    find_event(events, id).filter(|ev| event_active(w, ev))
}

/// Setzt die Faktoren des Sandsturms und des Nebels je Takt (Aufruf nach `taifun::update`). Ohne Wetter-Plan setzt niemand
/// die Faktoren zurück; darum beginnen sie dann neu bei 1.0, damit sich nichts über Takte aufschaukelt.
pub fn update(w: &mut World) {
    if w.weather_cfg.is_none() {
        w.weather_speed = 1.0;
        w.weather_range = 1.0;
    }
    w.weather_range_def = 1.0;
    // Frieden (Festival, B-973): keine Welt-Events mit Wirkung; ihre Meldung entfällt dort ebenfalls
    if w.biome.depth != 0 || w.peace {
        return;
    }
    let v = values(w);
    if let Some(s) = running(w, &v.events, "sandstorm") {
        w.weather_speed *= s.speed_factor;
        w.weather_range *= s.range_factor;
    }
    if let Some(f) = running(w, &v.events, "fog") {
        w.weather_range_def = f.range_factor;
    }
}

/// Überfall: zur Welle der Tiefe 0 kommt in der Event-Nacht eine Gruppe der Art `raid.kind` (Anteil `share` der Wellengröße,
/// aufgerundet, mindestens 1), über die Portale, mit Tempo × `raid.speedFactor` (`step_spawns`). Nur Tag/Nacht-Welten mit
/// Portalen, wie der Vollmond (`moon::full_moon_extra`).
pub fn queue_raid(w: &mut World, wave_size: usize, hp: f64, damage: f64) {
    let v = values(w);
    let Some(raid) = find_event(&v.events, "raid") else {
        return;
    };
    let frieden = w.peace;
    if frieden
        || w.biome.cycle.r#type != "dayNight"
        || w.biome.depth != 0
        || w.portals.is_empty()
        || !event_active(w, raid)
    {
        return;
    }
    let n = ((wave_size as f64) * raid.share).ceil().max(1.0) as usize;
    let picks = vec![raid.kind.clone(); n];
    let orders = spawn_orders(&v, &picks, &w.portals, &mut w.rng);
    for s in orders {
        w.spawn_queue.push(QueuedSpawn {
            kind: s.kind,
            x: s.x,
            at: w.time + s.delay,
            hp_factor: hp,
            damage_factor: damage,
            speed_factor: raid.speed_factor,
        });
    }
}

/// Überfall-Belohnung bei Tagesanbruch: wie beim Vollmond, wenn die Burg in dieser Nacht hielt und die Stufe bespielt war.
pub fn raid_reward(w: &mut World, night_no: i64) {
    let v = values(w);
    let Some(raid) = find_event(&v.events, "raid") else {
        return;
    };
    if raid.reward_gold > 0
        && w.biome.depth == 0
        && w.biome.cycle.r#type == "dayNight"
        && event_in_night(w, raid, night_no)
        && w.castle_fell_night != night_no
        && stage_played(w)
        && !w.portals.is_empty()
    {
        let x = w.hub_x;
        scatter_coins(w, x, raid.reward_gold);
    }
}
