//! Marker-Zeile, Drosselung und Frame-Zeiten (B-1220). Reine Rechnung ohne Datei und Fenster.

use std::collections::{HashMap, VecDeque};

use serde_json::json;

/// Mindestabstand zweier Marker desselben Spielers in ms.
pub const MIN_GAP_MS: u64 = 1000;
/// Fenster der schlechtesten Frame-Zeit in ms.
pub const WORST_WINDOW_MS: u64 = 5000;

/// Spielkontext eines Spielers zum Zeitpunkt des Markers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Context {
    /// Welt-ID der aktuellen Insel (leer = Standard-Welt)
    pub world: String,
    /// Etappe der Insel (>= 1), 0 = unbekannt
    pub island: i64,
    pub island_id: String,
    /// Stufe (`Seat::stage`)
    pub stage: i64,
    pub night: bool,
    /// day, dusk, night, dawn; leer = unbekannt
    pub phase: String,
    /// Tag-Nummer des Tag/Nacht-Zyklus
    pub day: i64,
}

/// Ein Marker, wie er als Zeile in `markers.jsonl` steht.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    /// Unix-ms der Systemuhr
    pub ts: u64,
    /// 1 oder 2
    pub player: i64,
    pub ctx: Context,
    /// schlechteste Frame-Zeit der letzten 5 s in ms
    pub worst_frame_ms: f64,
}

impl Marker {
    /// Eine Zeile JSON ohne Zeilenende; `kind` ist immer `pad` (der Tastatur-Ersatz F9 zählt als Pad-Marker).
    pub fn to_line(&self) -> String {
        let c = &self.ctx;
        json!({
            "ts": self.ts,
            "kind": "pad",
            "player": self.player,
            "world": c.world,
            "island": c.island,
            "island_id": c.island_id,
            "stage": c.stage,
            "night": c.night,
            "phase": c.phase,
            "day": c.day,
            "worst_frame_ms": (self.worst_frame_ms * 100.0).round() / 100.0,
        })
        .to_string()
    }
}

/// Höchstens ein Marker je Sekunde und Spieler.
#[derive(Debug, Default)]
pub struct RateLimit {
    last: HashMap<i64, u64>,
}

impl RateLimit {
    /// Erlaubt den Marker von `player` zur Zeit `now_ms` und merkt ihn sich; sonst `false`.
    pub fn allow(&mut self, player: i64, now_ms: u64) -> bool {
        if let Some(&t) = self.last.get(&player)
            && now_ms.saturating_sub(t) < MIN_GAP_MS
        {
            return false;
        }
        self.last.insert(player, now_ms);
        true
    }
}

/// Frame-Zeiten der letzten 5 s.
#[derive(Debug, Default)]
pub struct FrameTimes {
    list: VecDeque<(u64, f64)>,
}

impl FrameTimes {
    /// Meldet ein Bild, das zur Zeit `now_ms` fertig war und `ms` seit dem vorigen dauerte.
    pub fn push(&mut self, now_ms: u64, ms: f64) {
        self.list.push_back((now_ms, ms));
        while self
            .list
            .front()
            .is_some_and(|&(t, _)| now_ms.saturating_sub(t) > WORST_WINDOW_MS)
        {
            self.list.pop_front();
        }
    }

    /// Schlechteste Zeit der letzten 5 s bis `now_ms` (0 ohne Bild).
    pub fn worst(&self, now_ms: u64) -> f64 {
        self.list
            .iter()
            .filter(|&&(t, _)| now_ms.saturating_sub(t) <= WORST_WINDOW_MS)
            .map(|&(_, ms)| ms)
            .fold(0.0, f64::max)
    }
}
