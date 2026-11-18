//! Platzhalter der Gruppe `world` für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion
//! liefert, löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Reihenfolge der Aufrufe in `step` steht in
//! `step.rs` (AC-01); jeder Teilschritt anderer Gruppen ist hier ein benannter Platzhalter mit dem liefernden Sprint.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18, RU19)

use k3c_data::events::NightEvent;

use crate::state::World;

// RU18 (Wirtschaft) und RU19 (Ereignisse), gerufen von `step_cycle` (world_cycle.go) und `mirror_state`.

pub use crate::building::tavern::tavern_dawn; // Von RU18.1 portiert (building_tavern.go)

pub use crate::merchant::merchant_dawn; // Von RU18.4 portiert (merchant.go)

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

pub use crate::monarch::check_presets; // Von RU18.4 portiert (monarch_data.go)

pub use crate::economy::{step_players, step_sites}; // Von RU18.2 portiert (economy.go)

pub use crate::monarch::step_passives; // Von RU18.5 portiert (monarch_passives.go)

// Von RU18.1 portiert (building_plantation.go, building_healer.go, building_spell_tower.go).
pub use crate::building::healer::step_healing;
pub use crate::building::plantation::step_plantations;
pub use crate::building::spell_tower::step_spell_towers;

pub use crate::skills::step_storms; // Von RU18.5 portiert (skills_caster.go)

pub use crate::building::ruin::{add_ruin_sites, ruin_site}; // Von RU18.1 portiert (building_ruin.go)

pub use crate::monarch::{apply_preset, preset_for}; // Von RU18.4 portiert (monarch.go)

// RU19 (Ereignisse, Dev).

/// Go: `capEvents` (events.go): begrenzt die Ereignisse eines Ticks auf die Obergrenze (Priorität zuerst).
pub fn cap_events(_w: &mut World) {
    unimplemented!("RU19: cap_events")
}

/// Go: `devBuildSite` (dev.go): baut den Platz `si` ohne Zahlung bis zur Stufe `level` fertig.
pub fn dev_build_site(_w: &mut World, _si: usize, _level: i64) {
    unimplemented!("RU19: dev_build_site")
}

pub use crate::hub::sites::check_world_hub_sites; // Von RU18.3 portiert (world_hub_sites.go)
pub use crate::hub::{level_of, new_hub_site, site_cost, upgrade_payable, validate_hub_data}; // Von RU18.3 portiert (hub_level.go, hub_validate.go)
