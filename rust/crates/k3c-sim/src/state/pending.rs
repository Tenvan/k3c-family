//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use k3c_data::Data;
use k3c_data::monarch::SkillEffect;

use super::{Player, World};
use crate::island::WorldDef;

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

/// Go: `hitEvent` (events.go).
pub fn hit_event(_w: &mut World, _target: &str, _id: i64, _x: f64, _damage: f64) {
    unimplemented!("RU19: hit_event")
}

/// Go: `troopDamage` (monarch_passives.go).
pub fn troop_damage(_w: &World, _ti: usize, _damage: f64) -> f64 {
    unimplemented!("RU18: troop_damage")
}

/// Go: `eachPassive` (monarch_passives.go); `f` bekommt den Besitzer und den Effekt.
pub fn each_passive(_w: &World, _f: &mut dyn FnMut(&Player, &SkillEffect)) {
    unimplemented!("RU18: each_passive")
}

/// Go: `protectedNight` (World-Methode, Nacht-Schutz der Gebäude).
pub fn protected_night(_w: &World) -> bool {
    unimplemented!("RU18: protected_night")
}

pub use crate::building::plantation::plot_x; // Von RU18.1 portiert (building_plantation.go)
pub use crate::building::ruin::is_ruin; // Von RU18.1 portiert (building_ruin.go)

pub use crate::world::values_room::values; // Von RU14.4 portiert (world_values_room.go)

pub use crate::world::vein::gather_of; // Von RU14.4 portiert (world_vein.go)

pub use crate::monarch::skill_ref; // Von RU18.4 portiert (monarch_data.go)

// RU10.1: Platzhalter der Gruppe `island` für Funktionen und Typen späterer Sprints.

// Von RU19.1 portiert (dev.go): die Import-Stelle bleibt `pending`.
pub use crate::dev::DevFlags;

/// Go: `IslandStory` (story.go): Ankunft, vor dem Endboss und Abschlusszeile nach dem Sieg über ihn. Rumpf bis RU19.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IslandStory {
    pub arrival: Vec<String>,
    pub boss: Vec<String>,
    pub done: Vec<String>,
}

/// Go: `WorldStory` (story.go): die Geschichte einer Welt. Rumpf bis RU19.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldStory;

pub use crate::world::values_room::values_of; // Von RU14.4 portiert (world_values_room.go)

/// Go: `decodeIslandStory` (story.go): liest `story` der Insel streng (arrival, boss, done); nur gerufen, wenn das Feld da ist.
pub fn decode_island_story(_raw: &serde_json::Value) -> Result<IslandStory, String> {
    unimplemented!("RU19: decode_island_story")
}

/// Go: `decodeWorldStory` (story.go): liest `story` der Welt streng; nur gerufen, wenn das Feld da ist.
pub fn decode_world_story(_raw: &serde_json::Value) -> Result<WorldStory, String> {
    unimplemented!("RU19: decode_world_story")
}

/// Go: `checkStory` (story_check.go): prüft die Geschichte der Welt und ihrer Inseln; nur gerufen, wenn die Welt oder eine
/// Insel `story` hat.
pub fn check_story(_w: &WorldDef, _data: &Data) -> Result<(), String> {
    unimplemented!("RU19: check_story")
}

pub use crate::world::travel::{TravelPoint, travel_points}; // Von RU14.3 portiert (world_travel.go)

pub use crate::economy::to_hub; // Von RU18.2 portiert (economy.go)

/// Go: `returnBanner` (combat_squad.go): stellt das Banner des Monarchen `pi` am Tor wieder auf; sein Trupp kehrt zurück.
pub fn return_banner(_w: &mut World, _pi: usize) {
    unimplemented!("RU15: return_banner")
}

pub use crate::hub::{hub_steps, site_hub_level, validate_hub_data_in}; // Von RU18.3 portiert (hub_level.go, hub_validate.go)
