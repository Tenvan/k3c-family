//! Platzhalter der Gruppe `world` für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion
//! liefert, löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Reihenfolge der Aufrufe in `step` steht in
//! `step.rs` (AC-01); jeder Teilschritt anderer Gruppen ist hier ein benannter Platzhalter mit dem liefernden Sprint.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU14, RU18, RU19)

use std::sync::Arc;

use k3c_data::buildings::Cost;
use k3c_data::events::NightEvent;
use k3c_data::hub::HubData;

use crate::state::{IslandRef, PlayerCommand, Site, World, WorldValues};

// RU18 (Wirtschaft) und RU19 (Ereignisse), gerufen von `step_cycle` (world_cycle.go) und `mirror_state`.

/// Go: `payDawnIncome` (economy.go): Morgengold je lebendem Spieler.
pub fn pay_dawn_income(_w: &mut World) {
    unimplemented!("RU18: pay_dawn_income")
}

/// Go: `tavernDawn` (building_tavern.go): der Wirt bringt zum Morgengrauen Landstreicher.
pub fn tavern_dawn(_w: &mut World) {
    unimplemented!("RU18: tavern_dawn")
}

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

// RU14.3 (world_travel): Stufenwechsel.

/// Go: `stepTravel` (world_travel.go): startet und rechnet den gemeinsamen Stufenwechsel.
pub fn step_travel(_w: &mut World, _dt: f64) {
    unimplemented!("RU14: step_travel")
}

// RU14.4 (world_values): Basis-Wertesatz.

/// Go: `defaultValues` (world_values.go): der Basis-Wertesatz aus den eingebetteten Daten.
pub fn default_values() -> Arc<WorldValues> {
    unimplemented!("RU14: default_values")
}

// RU18 (Wirtschaft, Bau, Monarch).

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

/// Go: `stepPlantations` (building_plantation.go): Wachstum der Plantagen.
pub fn step_plantations(_w: &mut World, _dt: f64) {
    unimplemented!("RU18: step_plantations")
}

/// Go: `stepHealing` (building_healer.go): Heiler heilen Truppen und Monarchen.
pub fn step_healing(_w: &mut World, _dt: f64) {
    unimplemented!("RU18: step_healing")
}

/// Go: `stepSpellTowers` (building_spell_tower.go): Zauberturm-Schüsse.
pub fn step_spell_towers(_w: &mut World, _dt: f64) {
    unimplemented!("RU18: step_spell_towers")
}

/// Go: `stepStorms` (skills_caster.go): laufende Lightning Storms.
pub fn step_storms(_w: &mut World, _dt: f64) {
    unimplemented!("RU18: step_storms")
}

/// Go: `newHubSite` (hub_level.go): der Zahlplatz für den Hub-Ausbau an der Burg.
pub fn new_hub_site(_w: &World) -> Site {
    unimplemented!("RU18: new_hub_site")
}

/// Go: `ruinSite` (building_ruin.go): der Ausgangszustand eines Ruinen-Platzes (zieht wie `empty_site` eine ID).
pub fn ruin_site(_w: &mut World, _kind: &str, _x: f64) -> Site {
    unimplemented!("RU18: ruin_site")
}

/// Go: `addRuinSites` (building_ruin.go): legt die Ruinen der Stufe als Plätze an.
pub fn add_ruin_sites(_w: &mut World) {
    unimplemented!("RU18: add_ruin_sites")
}

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
