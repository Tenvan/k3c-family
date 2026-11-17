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
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "events.json";

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
    Ok(())
}
