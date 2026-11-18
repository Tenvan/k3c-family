//! Platzhalter der Gruppen `enemies` und `unit` (Port-Leitfaden § 7): Funktionen späterer Sprints (RU10, RU15, RU18, RU19),
//! die RU11 ruft. Der liefernde Sprint löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Platzhalter
//! aus RU8 stehen weiter in `state/pending.rs`. Go-Zeiger auf Gegner und Truppen sind hier Indizes (`ei`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU19)

use crate::enemies::waves::SpawnOrder;
use crate::state::island_ref::IslandRef;
use crate::state::{Event, Projectile, World, WorldValues};

// Von RU15 portiert (Bosse): Die Import-Stelle bleibt `pending`, damit die Gegner-Dateien unverändert bleiben.
pub use crate::boss::abilities::hit_splash;
pub use crate::boss::{boss_ability, boss_defeated, trigger_boss};

// Von RU14.2 portiert (world_cycle.go): die Import-Stelle bleibt `pending`.
pub use crate::world::cycle::add_pool;

// Von RU15 portiert (Krone, Portale): die Import-Stelle bleibt `pending`.
pub use crate::crown::{crown_stolen, drop_crown_of, take_crown_or_loot};
pub use crate::portal::drop_loot;

pub use crate::merchant::merchant_x; // Von RU18.4 portiert (merchant.go)

/// Go: `merchantRaidStart` (events_merchant.go): nach der Ankunft; jeder n-te Besuch wird überfallen.
pub fn merchant_raid_start(_w: &mut World, _isl: Option<&IslandRef>) {
    unimplemented!("RU19: merchant_raid_start")
}

/// Go: `merchantRaidEnd` (events_merchant.go): beendet den Überfall des anwesenden Händlers; `protected`: er reist ab.
pub fn merchant_raid_end(_w: &mut World, _protected: bool) {
    unimplemented!("RU19: merchant_raid_end")
}

/// Go: `damageMultOf` (monarch_passives.go): Faktor auf Schlag- und Zauberschaden des Spielers `pi` (Blessings).
pub fn damage_mult_of(_w: &World, _v: &WorldValues, _pi: usize, _spell: bool) -> f64 {
    unimplemented!("RU18: damage_mult_of (RU18.5)")
}

/// Go: `moonDamage` (events_moon.go).
pub fn moon_damage(_w: &World, _damage: f64) -> f64 {
    unimplemented!("RU19: moon_damage")
}

/// Go: `emit` (events.go): hängt das Ereignis mit `type` an `w.events`.
pub fn emit(_w: &mut World, _typ: &str, _fields: Event) {
    unimplemented!("RU19: emit")
}

/// Go: `unitX` (events.go): X auf eine Nachkommastelle gerundet.
pub fn unit_x(_x: f64) -> f64 {
    unimplemented!("RU19: unit_x")
}

/// Go: `arrowEvent` (events.go): Ereignis zum Pfeil `p` (noch nicht in `w.projectiles`) vom Schützen `from`.
pub fn arrow_event(_w: &mut World, _p: &Projectile, _from: i64) {
    unimplemented!("RU19: arrow_event")
}

/// Go: `frostArmorHit` (monarch_passives.go): Frost Armor des Spielers `pi` gegen den Gegner `ei`.
pub fn frost_armor_hit(_w: &mut World, _pi: usize, _ei: usize) {
    unimplemented!("RU18: frost_armor_hit")
}

// Von RU19.1 portiert (dev.go): die Import-Stelle bleibt `pending`.
pub use crate::dev::pure;

/// Go: `fullMoonExtra` (events_moon.go): zusätzliche Plätze bei Vollmond.
pub fn full_moon_extra(_w: &World, _plan: &[SpawnOrder]) -> Vec<SpawnOrder> {
    unimplemented!("RU19: full_moon_extra")
}

/// Go: `moonDrop` (events_moon.go): Blutmond-Faktor auf den Gold-Drop.
pub fn moon_drop(_w: &World, _gold: i64) -> i64 {
    unimplemented!("RU19: moon_drop")
}

pub use crate::economy::{scaled_gold, scatter_coins}; // Von RU18.2 portiert (economy_fountain.go, economy.go)

// Von RU18.1 portiert (building_*.go): die Import-Stelle bleibt `pending`, damit die Gruppen `unit` und `enemies`
// unverändert bleiben.
pub use crate::building::barracks::{fighters, troop_limit, weapons_in_flight};
pub use crate::building::outpost::{outpost, outpost_job};
pub use crate::building::repair::{repair, repair_job};
pub use crate::building::spell_tower::is_spell_tower;
pub use crate::building::tavern::{vagrants_at, wander_radius};
pub use crate::building::upgrades::armored_hp;

/// Go: `buildProgressEvent` (events.go): Fortschritts-Ereignis des Platzes `site_id`.
pub fn build_progress_event(_w: &mut World, _site_id: i64, _before: f64) {
    unimplemented!("RU19: build_progress_event")
}

// Von RU14.4 portiert (world_vein.go): die Import-Stelle bleibt `pending`.
pub use crate::world::vein::{gather_vein, is_vein, node_free};

/// Go: `addStockCapped` (island_storage.go): legt höchstens bis zur Kapazität der Insel ab und liefert die aufgenommene
/// Menge; `None`: kein Baumaterial (Go: `ok` false).
pub fn add_stock_capped(_w: &mut World, _isl: Option<&IslandRef>, _resource: &str, _amount: i64) -> Option<i64> {
    unimplemented!("RU10: add_stock_capped")
}

pub use crate::monarch::add_pool_points; // Von RU18.4 portiert (monarch.go)

pub use crate::hub::{each_site, finish_upgrade, next_level, upgrade_job}; // Von RU18.3 portiert (hub_level.go)
