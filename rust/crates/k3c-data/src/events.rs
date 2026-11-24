//! data/events.json: Go `sim.eventsFile` mit `[]sim.NightEvent` (engine/sim/events_values.go).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::enemies::EnemyData;
use crate::{Fehler, decode};

/// Go: `struct { Events []sim.NightEvent }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EventsFile {
    #[serde(rename = "Events", alias = "events")]
    pub events: Vec<NightEvent>,
}

/// Go: `sim.NightEvent`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NightEvent {
    #[serde(rename = "ID", alias = "id")]
    pub id: String,
    #[serde(rename = "EveryNights", alias = "everyNights")]
    pub every_nights: i64,
    #[serde(rename = "Kind", alias = "kind")]
    pub kind: String,
    #[serde(rename = "WolfShare", alias = "wolfShare")]
    pub wolf_share: f64,
    #[serde(rename = "Elite", alias = "elite")]
    pub elite: String,
    #[serde(rename = "RewardGold", alias = "rewardGold")]
    pub reward_gold: i64,
    #[serde(rename = "DamageFactor", alias = "damageFactor")]
    pub damage_factor: f64,
    #[serde(rename = "DropFactor", alias = "dropFactor")]
    pub drop_factor: f64,
    #[serde(rename = "EveryVisits", alias = "everyVisits")]
    pub every_visits: i64,
    #[serde(rename = "RewardMaterial", alias = "rewardMaterial")]
    pub reward_material: i64,
    /// Überfall (B-991): Anteil der Gegner an der Wellengröße (aufgerundet, mindestens 1).
    #[serde(rename = "Share", alias = "share", skip_serializing_if = "is_zero")]
    pub share: f64,
    /// Überfall und Sandsturm (B-991, B-992): Tempo-Faktor.
    #[serde(rename = "SpeedFactor", alias = "speedFactor", skip_serializing_if = "is_zero")]
    pub speed_factor: f64,
    /// Sandsturm und Nebel (B-992): Reichweiten-Faktor der Fernkämpfer.
    #[serde(rename = "RangeFactor", alias = "rangeFactor", skip_serializing_if = "is_zero")]
    pub range_factor: f64,
}

/// Nur gesetzte Welt-Werte erscheinen im Schreibbild: Die Go-Golden kennen sie nicht (events_wie_go).
fn is_zero(v: &f64) -> bool {
    *v == 0.0
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "events.json";
/// `WELT_FILE` ist der Name der Welt-Events (Überfall, Sandsturm, Nebel) unter data/; `load_all` hängt sie an `FILE` an.
pub const WELT_FILE: &str = "welt-events.json";

/// `load` liest events.json wie `loadEvents` (ohne Prüfung; die macht `check_night_events`).
pub fn load(raw: &str) -> Result<EventsFile, Fehler> {
    decode("data/events.json", raw)
}

/// `find_event` liefert das Event id (None, wenn es fehlt).
pub fn find_event<'a>(events: &'a [NightEvent], id: &str) -> Option<&'a NightEvent> {
    events.iter().find(|e| e.id == id)
}

/// `check_night_events` prüft die Events gegen die Gegnerarten wie Gos `checkNightEvents`: Vollmond, Blutmond und
/// Händler-Überfall vorhanden, Abstände ≥ 0 (0 = aus), Gegnerarten bekannt, Faktoren > 0, Mengen ≥ 0.
pub fn check_night_events(events: &[NightEvent], enemies: &BTreeMap<String, EnemyData>) -> Result<(), Fehler> {
    let fail = |s: String| Err(Fehler::Pruefung(format!("data/events.json: {s}")));
    let (Some(full), Some(blood), Some(raid)) = (
        find_event(events, "fullMoon"),
        find_event(events, "bloodMoon"),
        find_event(events, "merchantRaid"),
    ) else {
        return fail("fullMoon, bloodMoon und merchantRaid fehlen".into());
    };
    for ev in events {
        if ev.every_nights < 0 || ev.every_visits < 0 {
            return fail(format!("{}: Abstand muss >= 0 sein (0 = aus)", ev.id));
        }
    }
    if !enemies.contains_key(&full.kind) || !enemies.contains_key(&full.elite) {
        return fail(format!(
            "fullMoon nennt unbekannte Gegner {:?}, {:?}",
            full.kind, full.elite
        ));
    }
    if full.wolf_share < 0.0 || full.reward_gold < 0 || raid.reward_material < 0 {
        return fail("Anteil und Belohnung müssen >= 0 sein".into());
    }
    if blood.damage_factor <= 0.0 || blood.drop_factor <= 0.0 {
        return fail("bloodMoon braucht Faktoren > 0".into());
    }
    // Welt-Events (B-991, B-992): fehlen sie, gilt nichts; stehen sie da, müssen die Werte stimmen
    if let Some(r) = find_event(events, "raid")
        && (r.share <= 0.0 || r.speed_factor <= 0.0 || !enemies.contains_key(&r.kind))
    {
        return fail(format!(
            "raid braucht Anteil und Tempo > 0 und eine bekannte Gegnerart ({:?})",
            r.kind
        ));
    }
    if let Some(s) = find_event(events, "sandstorm")
        && (s.speed_factor <= 0.0 || s.range_factor <= 0.0)
    {
        return fail("sandstorm braucht Tempo- und Reichweiten-Faktor > 0".into());
    }
    if let Some(f) = find_event(events, "fog")
        && f.range_factor <= 0.0
    {
        return fail("fog braucht einen Reichweiten-Faktor > 0".into());
    }
    Ok(())
}
