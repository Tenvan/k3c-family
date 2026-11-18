//! Platzhalter der Gruppe `world` für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion
//! liefert, löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Reihenfolge der Aufrufe in `step` steht in
//! `step.rs` (AC-01); jeder Teilschritt anderer Gruppen ist hier ein benannter Platzhalter mit dem liefernden Sprint.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18, RU19)

use std::collections::BTreeMap;

use k3c_data::buildings::{BuildingData, Cost};
use k3c_data::economy::EconomyData;
use k3c_data::events::NightEvent;
use k3c_data::hub::HubData;
use k3c_data::monarch::MonarchData;

use crate::state::{IslandRef, PlayerCommand, Site, World};

// RU18 (Wirtschaft) und RU19 (Ereignisse), gerufen von `step_cycle` (world_cycle.go) und `mirror_state`.

/// Go: `payDawnIncome` (economy.go): Morgengold je lebendem Spieler.
pub fn pay_dawn_income(_w: &mut World) {
    unimplemented!("RU18: pay_dawn_income")
}

pub use crate::building::tavern::tavern_dawn; // Von RU18.1 portiert (building_tavern.go)

/// Go: `merchantDawn` (merchant.go): der Händler kommt oder geht zum Morgengrauen.
pub fn merchant_dawn(_w: &mut World) {
    unimplemented!("RU18: merchant_dawn")
}

/// Go: `nightEventsStart` (events_moon.go): Vollmond und Blutmond beginnen mit der Nacht.
pub fn night_events_start(_w: &mut World) {
    unimplemented!("RU19: night_events_start")
}

/// Go: `nightEventsEnd` (events_moon.go): Belohnung und Ende der Nacht-Events beim Morgengrauen.
pub fn night_events_end(_w: &mut World) {
    unimplemented!("RU19: night_events_end")
}

/// Go: `eventActive` (events_moon.go): Läuft das Nacht-Event `ev` gerade in dieser Stufe?
pub fn event_active(_w: &World, _ev: &NightEvent) -> bool {
    unimplemented!("RU19: event_active")
}

// RU18 (Wirtschaft, Bau, Monarch); `check_presets` und `check_world_hub_sites` rufen `values_from`.

/// Go: `checkPresets` (monarch_data.go): jedes Preset nennt nur bekannte Skills (RU18.4).
pub fn check_presets(_m: &MonarchData) -> Result<(), String> {
    unimplemented!("RU18: check_presets")
}

/// Go: `checkWorldHubSites` (hub_validate.go): prüft die Hub-Plätze des Wertesatzes gegen Wirtschaft und Gebäude.
pub fn check_world_hub_sites(
    _h: &mut HubData,
    _economy: &EconomyData,
    _buildings: &BTreeMap<String, BuildingData>,
) -> Result<(), String> {
    unimplemented!("RU18: check_world_hub_sites")
}

/// Go: `stepPlayers` (economy.go): Bewegung, Zahlen, Einsammeln der Spieler.
pub fn step_players(_w: &mut World, _commands: &[PlayerCommand], _isl: Option<&mut IslandRef>, _dt: f64) {
    unimplemented!("RU18: step_players")
}

/// Go: `stepPassives` (monarch_passives.go): Passiv-Skills der Monarchen.
pub fn step_passives(_w: &mut World, _dt: f64) {
    unimplemented!("RU18: step_passives")
}

/// Go: `stepSites` (economy.go): Bau, Ausbau und Handwerk der Plätze.
pub fn step_sites(_w: &mut World, _isl: Option<&mut IslandRef>) {
    unimplemented!("RU18: step_sites")
}

// Von RU18.1 portiert (building_plantation.go, building_healer.go, building_spell_tower.go).
pub use crate::building::healer::step_healing;
pub use crate::building::plantation::step_plantations;
pub use crate::building::spell_tower::step_spell_towers;

/// Go: `stepStorms` (skills_caster.go): laufende Lightning Storms.
pub fn step_storms(_w: &mut World, _dt: f64) {
    unimplemented!("RU18: step_storms")
}

/// Go: `newHubSite` (hub_level.go): der Zahlplatz für den Hub-Ausbau an der Burg.
pub fn new_hub_site(_w: &World) -> Site {
    unimplemented!("RU18: new_hub_site")
}

pub use crate::building::ruin::{add_ruin_sites, ruin_site}; // Von RU18.1 portiert (building_ruin.go)

/// Go: `ApplyPreset` (monarch.go): lernt die Skills eines Presets für den Spieler `pi`, soweit Pool-Punkte frei sind.
pub fn apply_preset(_w: &mut World, _pi: usize, _id: &str) -> Result<(), String> {
    unimplemented!("RU18: apply_preset")
}

/// Go: `presetFor` (monarch.go): das Standard-Preset beim Beitritt (Tank, Zauberer, sonst Heiler).
pub fn preset_for(_index: i64) -> &'static str {
    unimplemented!("RU18: preset_for")
}

/// Go: `levelOf` (hub_level.go): die Ausbau-Stufe des Platzes (mindestens 1).
pub fn level_of(_w: &World, _s: &Site) -> i64 {
    unimplemented!("RU18: level_of")
}

/// Go: `siteCost` (economy.go): der Preis des unbezahlten Platzes.
pub fn site_cost(_w: &World, _s: &Site) -> Cost {
    unimplemented!("RU18: site_cost")
}

/// Go: `upgradePayable` (hub_level.go): Ist der nächste Ausbau des Platzes bezahlbar?
pub fn upgrade_payable(_w: &World, _s: &Site) -> bool {
    unimplemented!("RU18: upgrade_payable")
}

/// Go: `ValidateHubData` (hub_validate.go): Befunde der Prüfung einer Kandidaten-hub.json (leer = gültig).
pub fn validate_hub_data(_h: &HubData) -> Vec<String> {
    unimplemented!("RU18: validate_hub_data")
}

// RU19 (Ereignisse, Dev).

/// Go: `capEvents` (events.go): begrenzt die Ereignisse eines Ticks auf die Obergrenze (Priorität zuerst).
pub fn cap_events(_w: &mut World) {
    unimplemented!("RU19: cap_events")
}

/// Go: `devBuildSite` (dev.go): baut den Platz `si` ohne Zahlung bis zur Stufe `level` fertig.
pub fn dev_build_site(_w: &mut World, _si: usize, _level: i64) {
    unimplemented!("RU19: dev_build_site")
}
