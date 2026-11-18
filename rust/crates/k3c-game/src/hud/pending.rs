//! Platzhalter für Teile des HUD, die spätere Sprints liefern (Leitfaden § 7, Plan RU20.4). RU23 (Skill-Menü, Seekarte,
//! Kampf-Anzeige, Geschichte, Schiffsfahrt, Welt-Texte) löscht seine Platzhalter; RU18 löscht die Shims für Sim-Werte
//! am Ende. RU22 prüft, dass keiner übrig ist.
//!
//! Abweichung von Leitfaden § 7 (steht im Ergebnis von RU20.4): Wo Go einen Wert braucht, damit `listen` ohne RU23 läuft
//! (`slot_views`, `new_bar_key`, `boss_bar`, `SeaView::refresh`, `Hud::story`, `Painter::outline.sweep`), liefern die
//! Platzhalter neutrale Werte statt `unimplemented!`; alles, was nur beim Zeichnen oder bei offenem Menü läuft, stürzt
//! mit `RU23: <name>` ab.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use k3c_sim::state::{ActiveEvent, Enemy, Merchant, Player, World};

use crate::view::frame::Frame;

/// VoyageView ist die Schiffsfahrt im Vollbild (Go: `voyageView`, voyage.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VoyageView {
    pub ok: bool,
}

/// BossView ist die Boss-Leiste (Go: `bossView`, kampf.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BossView {
    pub ok: bool,
    /// Anteil der Lebensleiste 0 … 1 (Go: `bossView.share`)
    pub share: f64,
}

/// EventsKey sind die laufenden Nacht-Events (Go: `eventsKey`, kampf.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventsKey {
    pub lang: String,
}

/// LabelSpot ist ein sichtbares Label: Index in `SeatState::labels` und Fenster-Punkt (Go: `labelSpot`, drawworld.go).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LabelSpot {
    pub i: usize,
    pub x: f64,
    pub y: f64,
}

/// Go: `bossBar` (kampf.go, RU23). Platzhalter: kein Boss.
pub fn boss_bar(_lang: &str, _enemies: &[Enemy]) -> BossView {
    BossView::default()
}

/// Go: `bossTitle` (kampf.go, RU23). Platzhalter: leer.
pub fn boss_title(_b: &BossView) -> String {
    String::new()
}

/// Go: `newEventsKey` (kampf.go, RU23).
pub fn new_events_key(lang: &str, _events: &[ActiveEvent], _speed: f64, _m: Option<&Merchant>) -> EventsKey {
    EventsKey { lang: lang.to_string() }
}

/// Go: `eventBanner` (kampf.go, RU23). Platzhalter: kein Event.
pub fn event_banner(_k: &EventsKey) -> String {
    String::new()
}

/// Go: `sim.AvailablePoints` (RU18): 0 freie Punkte, bis RU18 die Skill-Logik liefert; nichts ist lernbar.
pub fn available_points(_w: &World, _p: &Player) -> i64 {
    0
}

/// Go: `sim.CanLearn` (RU18): nichts ist lernbar.
pub fn can_learn(_w: &World, _p: &Player, _id: &str) -> bool {
    false
}

/// Go: `sim.CanRespec` (RU18): kein Respec.
pub fn can_respec(_w: &World, _p: &Player) -> bool {
    false
}

/// Go: `(*hud).drawWorldTexts` (drawworld.go, RU23). Platzhalter: zeichnet nichts (Labels, Hinweise, Radar fehlen bis RU23).
pub fn draw_world_texts() {}

/// Go: `(*hud).drawRadar` (RU23).
pub fn draw_radar() {
    unimplemented!("RU23: draw_radar")
}

/// Go: `(*hud).drawGuide` (RU23).
pub fn draw_guide() {
    unimplemented!("RU23: draw_guide")
}

/// WorldDef ist die Welt-Definition der Kampagne (Go: `sim.WorldDef`, RU19): Platzhalter ohne Inhalt; `Campaign::world`
/// gibt es in Rust erst mit RU19, `campaign_world` liefert bis dahin `None`.
#[derive(Clone, Debug, Default)]
pub struct WorldDef;

impl WorldDef {
    /// Go: `StoryStart` (RU19).
    pub fn story_start(&self) -> Vec<String> {
        unimplemented!("RU19: story_start")
    }
    /// Go: `len(w.Story.Start)` (RU19).
    pub fn story_start_len(&self) -> usize {
        unimplemented!("RU19: story_start_len")
    }
    /// Go: `StoryArrival` (RU19).
    pub fn story_arrival(&self, _island: &str) -> Vec<String> {
        unimplemented!("RU19: story_arrival")
    }
    /// Go: `StoryBoss` (RU19).
    pub fn story_boss(&self, _island: &str) -> Vec<String> {
        unimplemented!("RU19: story_boss")
    }
    /// Go: `StoryDefeated` (RU19).
    pub fn story_defeated(&self, _island: &str, _completed: &[String]) -> Vec<String> {
        unimplemented!("RU19: story_defeated")
    }
    /// Go: `StoryVictory` (RU19).
    pub fn story_victory(&self, _island: &str, _completed: &[String]) -> Vec<String> {
        unimplemented!("RU19: story_victory")
    }
}

/// Go: `f.Campaign.World` (RU19): `None`, solange `Campaign` die Welt nicht trägt.
pub fn campaign_world(_f: &Frame) -> Option<&WorldDef> {
    None
}

/// Go: `storyDoneLines` (story.go, RU19): Zahl der Zeilen der Abschlusszeile der Insel.
pub fn story_done_lines(_w: &WorldDef, _island: &str) -> usize {
    unimplemented!("RU19: story_done_lines")
}

// RU18: ersetzen. Die drei Shims rufen die Sim nicht (deren Gegenstücke sind bis RU18 `unimplemented!`, `Listen` und
// Draw stürzten sonst mit einer echten Welt ab) und liefern neutrale Werte wie eine Welt ohne gelernten Skill und ohne
// Zahlziel. RU18 löscht sie; `state.rs` und `hints.rs` rufen dann wieder die Sim-Funktionen.

/// Go: `sim.PurseMaxOf` (RU18): 0, damit `SeatState::card` auf `purse_max_gold()` zurückfällt (wie Go bei `<= 0`). Der
/// Dev-Bonus und die gelernte Größe fehlen bis RU18.
pub fn purse_max_of(_w: &World, _p: &Player) -> i64 {
    0
}

/// Go: `sim.TrainAt` (RU18): "" (kein Zahlziel).
pub fn train_at(_w: &World, _p: &Player) -> String {
    String::new()
}

/// Go: `sim.TradeAt` (RU18): `("", "")` (kein Zahlziel).
pub fn trade_at(_w: &World, _p: &Player) -> (String, String) {
    (String::new(), String::new())
}
