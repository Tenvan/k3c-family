//! Platzhalter für Teile des HUD, die spätere Sprints liefern (Leitfaden § 7, Plan RU20.4). RU23 (Skill-Menü, Seekarte,
//! Kampf-Anzeige, Geschichte, Schiffsfahrt, Welt-Texte) löscht seine Platzhalter; RU20.5 ersetzt `Painter` und
//! `LayoutCache` durch `paint.rs` und `draw.rs`. RU22 prüft, dass keiner übrig ist.
//!
//! Abweichung von Leitfaden § 7 (steht im Ergebnis von RU20.4): Wo Go einen Wert braucht, damit `listen` ohne RU23 läuft
//! (`slot_views`, `new_bar_key`, `boss_bar`, `SeaView::refresh`, `Hud::story`, `Painter::outline.sweep`), liefern die
//! Platzhalter neutrale Werte statt `unimplemented!`; alles, was nur beim Zeichnen oder bei offenem Menü läuft, stürzt
//! mit `RU23: <name>` ab.

use k3c_sim::state::{ActiveEvent, Enemy, Merchant, Player, SwitchState, World};

use super::Hud;
use crate::view::frame::{Frame, RouteIsland};

/// PURSE_ASSET ist das Geldsack-Icon (Go: `purseAsset`, purse.go; RU20.5 verschiebt die Konstante nach `purse.rs`).
pub const PURSE_ASSET: &str = "icons/geldsack";

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

/// Painter ist der Zeichen-Zustand des HUD (Go: `painter`, paint.go; RU20.5 ersetzt ihn durch `paint.rs`).
#[derive(Debug, Default)]
pub struct Painter {
    pub outline: Outline,
}

/// Outline ist der Umriss-Cache des Painters; `sweep` ist bis RU20.5 ein leerer Aufruf.
#[derive(Debug, Default)]
pub struct Outline;

impl Outline {
    pub fn sweep(&mut self) {}
}

/// LayoutCache ist die gemerkte Anordnung der Anzeigen einer Zelle (Go: `layoutCache`, draw.go; RU20.5).
#[derive(Clone, Debug, Default)]
pub struct LayoutCache;

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

/// Go: `(*hud).drawWorldTexts` (drawworld.go, RU23).
pub fn draw_world_texts() {
    unimplemented!("RU23: draw_world_texts")
}

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

/// Go: `sim.PurseMaxOf` (RU18): `k3c_sim::state::purse::purse_max_of` ruft `purse_max` der Insel-Wirtschaft (RU18) und
/// stürzt bis dahin ab. Ohne gelernten Geldsack-Skill (immer, solange der Spieler keinen hat) liefert Go den Grundwert
/// samt Dev-Bonus; hier 0, damit `SeatState::card` auf `purse_max_gold()` zurückfällt (wie Go bei `<= 0`). Der Dev-Bonus
/// fehlt bis RU18; RU18 löscht diese Funktion und `state.rs` ruft wieder `k3c_sim::state::purse::purse_max_of`.
pub fn purse_max_of(w: &World, p: &Player) -> i64 {
    if p.skills.is_empty() {
        return 0;
    }
    k3c_sim::state::purse::purse_max_of(w, p)
}

/// nothing_to_pay: Die Welt hat nichts, was `find_pay_target` (Go: economy.go) als Zahlziel finden könnte (Plätze, Hub,
/// Truppen, Knoten, Händler, Kampfziele, Aufheb-Dinge); dann ist das Ziel in Go `nil`.
fn nothing_to_pay(w: &World) -> bool {
    w.sites.is_empty()
        && w.hub_site.is_none()
        && w.troops.is_empty()
        && w.nodes.is_empty()
        && w.merchant.is_none()
        && w.camps.is_empty()
        && w.portals.is_empty()
        && w.enemies.is_empty()
        && w.pickups.is_empty()
        && w.drops.is_empty()
}

/// Go: `sim.TrainAt` (RU18): `k3c_sim::state::action_view::train_at` ruft `find_pay_target` (RU18) und stürzt bis dahin
/// ab. In einer Welt ohne Zahlziel ist die Antwort "" (wie Go); sonst gilt die Sim-Funktion. RU18 löscht diese Funktion.
pub fn train_at(w: &World, p: &Player) -> String {
    if nothing_to_pay(w) {
        return String::new();
    }
    k3c_sim::state::action_view::train_at(w, p)
}

/// Go: `sim.TradeAt` (RU18): wie `train_at`; ohne Zahlziel `("", "")`. RU18 löscht diese Funktion.
pub fn trade_at(w: &World, p: &Player) -> (String, String) {
    if nothing_to_pay(w) {
        return (String::new(), String::new());
    }
    k3c_sim::state::action_view::trade_at(w, p)
}
