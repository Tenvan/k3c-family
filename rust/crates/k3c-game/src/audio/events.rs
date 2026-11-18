//! Port von `native/k3c-game/audio/events.go`: Ereignis → Klang und Lautstärke je Spielerhälfte.

use k3c_sim::state::{Player, World};

use crate::view::frame::{CueKind, Event};

/// Klang eines Ereignisses: Slot im Register und ob er überall gleich laut klingt (Warnung).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SoundCue {
    pub slot: &'static str,
    pub global: bool,
}

const fn cue(slot: &'static str, global: bool) -> SoundCue {
    SoundCue { slot, global }
}

/// Zuordnung Ereignis → Klang wie src/audio/events.ts › CUES; Warnungen (global) senken die Musik.
pub const EVENT_CUES: [(&str, SoundCue); 15] = [
    ("coinPickup", cue("sfx:coin", false)),
    ("hit", cue("sfx:hit", false)),
    ("kill", cue("sfx:kill", false)),
    ("arrow", cue("sfx:arrow", false)),
    ("strike", cue("sfx:strike", false)),
    ("built", cue("sfx:build", false)),
    ("crownFallen", cue("sfx:death", false)),
    ("skillPoint", cue("sfx:skill", false)),
    ("arrived", cue("sfx:portal", false)),
    ("coinGive", cue("sfx:coinGive", false)),
    ("buildProgress", cue("sfx:buildProgress", false)),
    ("bossSpawned", cue("sfx:boss", true)),
    ("dusk", cue("sfx:warn", true)),
    ("wave", cue("sfx:warn", true)),
    ("castleFallen", cue("sfx:warn", true)),
];

/// Der Klang des Ereignisses `name` (None = still).
pub fn event_cue(name: &str) -> Option<SoundCue> {
    EVENT_CUES.iter().find(|(n, _)| *n == name).map(|(_, c)| *c)
}

/// Ereignis eines gewirkten Skills; sein Slot kommt aus `Register::skill_slot` (skills.ts).
pub const SKILL_CAST_EVENT: &str = "skillCast";

/// Menü-Klänge wie src/audio/ui.ts › UI_SLOTS: Auswahl bewegt und Bestätigen; alle anderen Cues bleiben still.
pub const SLOT_UI_SELECT: &str = "sfx:uiSelect";
pub const SLOT_UI_CONFIRM: &str = "sfx:uiConfirm";
pub const SLOT_HUB: &str = "sfx:hub";
pub const SLOT_RIDE: &str = "sfx:ride";

/// Menü-Klang eines Cues ("" = still).
pub fn cue_slot(k: CueKind) -> &'static str {
    match k {
        CueKind::Move => SLOT_UI_SELECT,
        CueKind::Confirm => SLOT_UI_CONFIRM,
        _ => "",
    }
}

/// Pegel wie events.ts: Rand des sichtbaren Bereichs, Nacht leise, Hufschlag als Teppich.
pub const EDGE_GAIN: f64 = 0.5;
pub const NIGHT_GAIN_VAL: f64 = 0.6;
pub const RIDE_MIN_SPEED: f64 = 0.3;
pub const RIDE_GAIN: f64 = 0.5;

/// Pegelfaktor der Effekte zur Tageszeit (events.ts › nightGain).
pub fn night_gain(phase: &str) -> f64 {
    if phase == "night" { NIGHT_GAIN_VAL } else { 1.0 }
}

/// Sichtbarer Ausschnitt eines lokalen Spielers in Units (events.ts › Listener).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ear {
    pub center: f64,
    pub span: f64,
}

/// Lautstärke 0 … 1 einer Quelle bei x wie events.ts › attenuation: im Ausschnitt eines Ohrs 1, darüber hinaus ab
/// `EDGE_GAIN` fallend bis 0 nach einer Bildschirmbreite; das lauteste Ohr gilt.
pub fn attenuation(x: f64, has_x: bool, ears: &[Ear], global: bool) -> f64 {
    if global || ears.is_empty() || !has_x {
        return 1.0;
    }
    let best = ears
        .iter()
        .map(|e| ((x - e.center).abs() - e.span / 2.0).max(0.0) / e.span.max(1.0))
        .fold(f64::INFINITY, f64::min);
    if best == 0.0 {
        return 1.0;
    }
    (EDGE_GAIN * (1.0 - best)).max(0.0)
}

/// Ort eines Ereignisses in Units: Feld x, sonst der Ort des genannten Spielers (player) in `w`; ohne beides
/// `(0, false)` (keine Dämpfung). Vorbild events.ts › eventX mit dem Ort des Effekts als Rückfall.
pub fn event_x(e: &Event, w: Option<&World>) -> (f64, bool) {
    if let Some(x) = number(e.data.get("x")) {
        return (x, true);
    }
    let Some(p) = number(e.data.get("player")) else {
        return (0.0, false);
    };
    let Some(w) = w else {
        return (0.0, false);
    };
    match w.players.iter().find(|m| m.index as f64 == p) {
        Some(m) => (m.x, true),
        None => (0.0, false),
    }
}

/// Liest eine Zahl aus einem Ereignis-Feld (JSON-Zahl, ganz oder gebrochen).
pub fn number(v: Option<&serde_json::Value>) -> Option<f64> {
    v?.as_f64()
}

/// Meldet, ob man den Hufschlag eines Monarchen hört (events.ts › rideLoops): lebt und bewegt sich.
pub fn riding(p: Option<&Player>) -> bool {
    p.is_some_and(|p| p.crown.is_none() && p.vx.abs() >= RIDE_MIN_SPEED)
}
