//! Platzhalter für Teile des HUD, die spätere Sprints liefern (Leitfaden § 7, Plan RU20.4). RU23 (Skill-Menü, Seekarte,
//! Kampf-Anzeige, Geschichte, Schiffsfahrt, Welt-Texte) löscht seine Platzhalter; RU18 löscht die Shims für Sim-Werte
//! am Ende. RU22 prüft, dass keiner übrig ist.
//!
//! Abweichung von Leitfaden § 7 (steht im Ergebnis von RU20.4): Wo Go einen Wert braucht, damit `listen` ohne RU23 läuft
//! (`slot_views`, `new_bar_key`, `boss_bar`, `SeaView::refresh`, `Hud::story`, `Painter::outline.sweep`), liefern die
//! Platzhalter neutrale Werte statt `unimplemented!`; alles, was nur beim Zeichnen oder bei offenem Menü läuft, stürzt
//! mit `RU23: <name>` ab.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use k3c_sim::state::{ActiveEvent, Enemy, Merchant, Player, SwitchState, World};

use super::Hud;
use crate::view::frame::{Frame, RouteIsland};

/// MenuState ist der Zustand des Skill-Menüs eines Spielers (Go: `menuState`, skillmenu.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuState {
    pub open: bool,
    pub cursor: i64,
}

/// MenuEntry ist ein Eintrag des Skill-Menüs (Go: `menuEntry`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuEntry {
    pub id: String,
}

/// MenuKey ist alles, wovon der Text des Menüs abhängt (Go: `menuKey`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuKey {
    pub lang: String,
    pub profile: String,
}

/// MenuView ist der Text des Menüs (Go: `menuView`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MenuView {
    pub title: String,
    pub footer: String,
}

/// SlotView ist ein Skill-Slot: Skill ("" = frei) und Abklingzeit in Sekunden (Go: `slotView`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SlotView {
    pub skill: String,
    pub cooldown: f64,
}

/// SeaView ist die Seekarte am Hafen eines Spielers (Go: `seaView`, seekarte.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeaView {
    pub open: bool,
}

impl SeaView {
    /// refresh liest die Karte aus Sprache, Profil, Inselwechsel und Route (Platzhalter: tut nichts).
    pub fn refresh(&mut self, _lang: &str, _profile: &str, _sw: Option<&SwitchState>, _route: &[RouteIsland]) {}
}

/// VoyageView ist die Schiffsfahrt im Vollbild (Go: `voyageView`, voyage.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VoyageView {
    pub ok: bool,
}

/// TaleState sind die Geschichtszeilen an den Wendepunkten (Go: `taleState`, story.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaleState {
    pub run: u64,
}

/// StoryQueue ist die Geschichtszeile eines Spielers (Go: `storyQueue`, story.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StoryQueue {
    pub age: f64,
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

/// BarKey ist die Skill-Leiste (Go: `barKey`, skilltext.go).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarKey {
    pub lang: String,
    pub skills: [String; 4],
    pub cd: [i64; 4],
}

/// LabelSpot ist ein sichtbares Label: Index in `SeatState::labels` und Fenster-Punkt (Go: `labelSpot`, drawworld.go).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LabelSpot {
    pub i: usize,
    pub x: f64,
    pub y: f64,
}

/// Go: `slotViews` (skillmenu.go): liest die Slots 1 bis 4 (fehlende Felder: frei bzw. bereit).
pub fn slot_views(slots: &[String], cooldowns: &[f64]) -> [SlotView; 4] {
    std::array::from_fn(|i| SlotView {
        skill: slots.get(i).cloned().unwrap_or_default(),
        cooldown: cooldowns.get(i).map_or(0.0, |c| if *c > 0.0 { *c } else { 0.0 }),
    })
}

/// Go: `newBarKey` (skilltext.go): Skill und Abklingzeit (ganze Sekunden, aufgerundet) je Slot.
pub fn new_bar_key(lang: &str, slots: &[SlotView; 4]) -> BarKey {
    BarKey {
        lang: lang.to_string(),
        skills: std::array::from_fn(|i| slots[i].skill.clone()),
        cd: std::array::from_fn(|i| slots[i].cooldown.ceil() as i64),
    }
}

/// Go: `slotBar` (skilltext.go, RU23). Platzhalter: leere Leiste.
pub fn slot_bar(_k: &BarKey) -> String {
    String::new()
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

/// Go: `menuEntries` (skillmenu.go, RU23): läuft nur bei offenem Menü, das ohne Overlay (RU23) nie öffnet.
pub fn menu_entries(
    _out: Vec<MenuEntry>,
    _learned: &[String],
    _points: i64,
    _can_learn: &dyn Fn(&str) -> bool,
    _respec: bool,
) -> Vec<MenuEntry> {
    unimplemented!("RU23: menu_entries")
}

/// Go: `sim.AvailablePoints` (Skill-Logik, RU23): läuft nur bei offenem Menü.
pub fn available_points(_w: &World, _p: &Player) -> i64 {
    unimplemented!("RU23: available_points")
}

/// Go: `sim.CanLearn` (RU23): läuft nur bei offenem Menü.
pub fn can_learn(_w: &World, _p: &Player, _id: &str) -> bool {
    unimplemented!("RU23: can_learn")
}

/// Go: `sim.CanRespec` (RU23): läuft nur bei offenem Menü.
pub fn can_respec(_w: &World, _p: &Player) -> bool {
    unimplemented!("RU23: can_respec")
}

/// Go: `newMenuKey` (skilltext.go, RU23): läuft nur bei offenem Menü.
pub fn new_menu_key(
    _lang: &str,
    _profile: &str,
    _cursor: i64,
    _points: i64,
    _entries: &[MenuEntry],
    _slots: &[SlotView; 4],
) -> MenuKey {
    unimplemented!("RU23: new_menu_key")
}

/// Go: `buildMenu` (skilltext.go, RU23): läuft nur bei offenem Menü.
pub fn build_menu(_k: &MenuKey, _entries: &[MenuEntry]) -> MenuView {
    unimplemented!("RU23: build_menu")
}

impl Hud {
    /// Go: `(*hud).story` (story.go, RU23): schreibt die Geschichtszeilen fort. Platzhalter: tut nichts.
    pub fn story(&mut self, _f: &Frame) {}
}

/// Go: `(*hud).drawWorldTexts` (drawworld.go, RU23). Platzhalter: zeichnet nichts (Labels, Hinweise, Radar fehlen bis RU23).
pub fn draw_world_texts() {}

/// Go: `(*hud).drawSea` (drawsea.go, RU23).
pub fn draw_sea() {
    unimplemented!("RU23: draw_sea")
}

/// Go: `(*hud).drawMenu` (drawmenu.go, RU23).
pub fn draw_menu() {
    unimplemented!("RU23: draw_menu")
}

/// Go: `(*hud).drawRadar` (RU23).
pub fn draw_radar() {
    unimplemented!("RU23: draw_radar")
}

/// Go: `(*hud).drawGuide` (RU23).
pub fn draw_guide() {
    unimplemented!("RU23: draw_guide")
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
