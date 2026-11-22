//! Marker-Zeile, Drosselung und Frame-Zeiten (B-1220). Reine Rechnung ohne Datei und Fenster.

use std::collections::{HashMap, VecDeque};

use serde_json::json;

/// Mindestabstand zweier Ruckler-Marker in ms.
pub const JANK_GAP_MS: u64 = 5000;
/// Standard-Schwelle für einen Ruckler in ms (`--playtest-jank-ms`).
pub const JANK_MS: f64 = 25.0;
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

/// Art des Markers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Kind {
    /// vom Spieler gesetzt (Pad LT + RT, Tastatur Einfg)
    #[default]
    Pad,
    /// das Spiel hat einen Ruckler gemessen (B-1221)
    Ruckler,
}

/// Ein Marker, wie er als Zeile in `markers.jsonl` steht.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Marker {
    /// Unix-ms der Systemuhr
    pub ts: u64,
    pub kind: Kind,
    /// 1 oder 2; 0 beim Ruckler (gilt für das ganze Spiel, Kontext des ersten Spielers)
    pub player: i64,
    pub ctx: Context,
    /// schlechteste Frame-Zeit der letzten 5 s in ms
    pub worst_frame_ms: f64,
    /// nur Ruckler: Dauer des Frames in ms
    pub frame_ms: Option<f64>,
    /// nur Ruckler: Frames über der Schwelle, die seit dem letzten Ruckler-Marker nicht markiert wurden
    pub skipped: Option<u32>,
    /// nur Pad: Spielstand relativ zum Session-Ordner, z. B. `saves/marker-3.json`
    pub save: Option<String>,
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

impl Marker {
    /// Eine Zeile JSON ohne Zeilenende; `frame_ms`, `skipped` und `save` stehen nur, wenn sie gesetzt sind.
    pub fn to_line(&self) -> String {
        let c = &self.ctx;
        let kind = match self.kind {
            Kind::Pad => "pad",
            Kind::Ruckler => "ruckler",
        };
        let mut v = json!({
            "ts": self.ts,
            "kind": kind,
            "player": self.player,
            "world": c.world,
            "island": c.island,
            "island_id": c.island_id,
            "stage": c.stage,
            "night": c.night,
            "phase": c.phase,
            "day": c.day,
            "worst_frame_ms": round2(self.worst_frame_ms),
        });
        if let Some(f) = self.frame_ms {
            v["frame_ms"] = json!(round2(f));
        }
        if let Some(n) = self.skipped {
            v["skipped"] = json!(n);
        }
        if let Some(s) = &self.save {
            v["save"] = json!(s);
        }
        v.to_string()
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

/// Entprellung der Ruckler-Marker: höchstens einer je 5 s, die übersprungenen Frames werden gezählt.
#[derive(Debug, Default)]
pub struct Jank {
    last: Option<u64>,
    skipped: u32,
}

impl Jank {
    /// Meldet ein Bild über der Schwelle zur Zeit `now_ms`: `Some(übersprungene)`, wenn ein Marker folgt, sonst `None`.
    pub fn hit(&mut self, now_ms: u64) -> Option<u32> {
        if self.last.is_some_and(|t| now_ms.saturating_sub(t) < JANK_GAP_MS) {
            self.skipped += 1;
            return None;
        }
        self.last = Some(now_ms);
        Some(std::mem::take(&mut self.skipped))
    }
}
