//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU19, RU23)

use super::World;

// Von RU11 portiert (Gegner, Truppen): Die Import-Stelle bleibt `pending`, damit Zustandsdateien unverändert bleiben.
pub use crate::combat::is_soldier; // Von RU15 portiert (combat.go)
pub use crate::combat::squad::count_fetching; // Von RU15 portiert (combat_squad.go)
pub use crate::crown::{crowned, drop_crown, hit_coins}; // Von RU15 portiert (crown.go)
pub use crate::enemies::guard::{apply_guards, guards_cleared_keys, is_wave_enemy};
pub use crate::enemies::traits::{attack_rate, invulnerable, kite, strike};
pub use crate::pending::*; // Platzhalter der Gruppen enemies und unit
pub use crate::portal::apply_portals; // Von RU15 portiert (portal.go)
pub use crate::unit::archer::is_on_tower;
pub use crate::unit::jobs::make_fighter;
pub use crate::unit::professions::profession_of;
pub use crate::unit::spawn_vagrant;
pub use crate::unit::warrior::is_fighter;

pub use crate::merchant::{MERCHANT_ID, damage_merchant}; // Von RU18.4 portiert (merchant.go)

pub use crate::monarch::{each_passive, troop_damage}; // Von RU18.5 portiert (monarch_passives.go)

/// Go: `protectedNight` (World-Methode, `island_options.go`): Die Stufe gehört zu einer Insel, deren Grad die laufende Nacht
/// schützt. Die Insel steht in Rust nicht an der Stufe; `World::island_events` spiegelt den Grad (`step_island`), ohne
/// Insel nie (wie `island_ref::protected_night(w, None)`).
pub fn protected_night(w: &World) -> bool {
    w.island_events
        .as_ref()
        .is_some_and(|i| w.cycle.phase == "night" && w.cycle.day <= i.protected_nights)
}

pub use crate::building::plantation::plot_x; // Von RU18.1 portiert (building_plantation.go)
pub use crate::building::ruin::is_ruin; // Von RU18.1 portiert (building_ruin.go)

pub use crate::world::values_room::values; // Von RU14.4 portiert (world_values_room.go)

pub use crate::world::vein::gather_of; // Von RU14.4 portiert (world_vein.go)

pub use crate::monarch::skill_ref; // Von RU18.4 portiert (monarch_data.go)

// RU10.1: Platzhalter der Gruppe `island` für Funktionen und Typen späterer Sprints.

pub use crate::world::values_room::values_of; // Von RU14.4 portiert (world_values_room.go)

pub use crate::world::travel::{TravelPoint, travel_points}; // Von RU14.3 portiert (world_travel.go)

pub use crate::economy::to_hub; // Von RU18.2 portiert (economy.go)

pub use crate::combat::squad::return_banner; // Von RU15 portiert (combat_squad.go)

pub use crate::hub::{hub_steps, site_hub_level, validate_hub_data_in}; // Von RU18.3 portiert (hub_level.go, hub_validate.go)

// Von RU19.1 bis RU19.3 portiert (dev.go, events.go, story.go): die Import-Stelle bleibt `pending`.
pub use crate::dev::DevFlags;
pub use crate::events::hit_event;
pub use crate::story::{IslandStory, WorldStory, check_story, decode_island_story, decode_world_story};
