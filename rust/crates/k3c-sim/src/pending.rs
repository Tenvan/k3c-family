//! Platzhalter der Gruppen `enemies` und `unit` (Port-Leitfaden § 7): Funktionen späterer Sprints (RU10, RU15, RU18, RU19),
//! die RU11 ruft. Der liefernde Sprint löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Platzhalter
//! aus RU8 stehen weiter in `state/pending.rs`. Go-Zeiger auf Gegner und Truppen sind hier Indizes (`ei`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU19)

use crate::enemies::waves::SpawnOrder;
use crate::state::island_ref::IslandRef;
use crate::state::{Event, Job, Projectile, Site, World};

// Von RU15 portiert (Bosse): Die Import-Stelle bleibt `pending`, damit die Gegner-Dateien unverändert bleiben.
pub use crate::boss::abilities::hit_splash;
pub use crate::boss::{boss_ability, boss_defeated, trigger_boss};

// Von RU14.2 portiert (world_cycle.go): die Import-Stelle bleibt `pending`.
pub use crate::world::cycle::add_pool;

// Von RU15 portiert (Krone, Portale): die Import-Stelle bleibt `pending`.
pub use crate::crown::{crown_stolen, drop_crown_of, take_crown_or_loot};
pub use crate::portal::drop_loot;

/// Go: `merchantX` (merchant.go).
pub fn merchant_x(_w: &World, _what: &str) -> f64 {
    unimplemented!("RU18: merchant_x")
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

/// Go: `World.pure` (dev.go): reiner Stressmodus der Insel.
pub fn pure(_w: &World) -> bool {
    unimplemented!("RU19: pure")
}

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

/// Go: `upgradeJob` (hub_level.go): Ist der Auftrag ein Ausbau?
pub fn upgrade_job(_w: &World, _j: &Job) -> bool {
    unimplemented!("RU18: upgrade_job")
}

/// Go: `nextLevel` (hub_level.go): die nächste Ausbaustufe des Platzes.
pub fn next_level(_w: &World, _s: &Site) -> Option<k3c_data::buildings::LevelData> {
    unimplemented!("RU18: next_level")
}

/// Go: `finishUpgrade` (hub_level.go): schließt den Ausbau des Platzes `site_id` ab.
pub fn finish_upgrade(_w: &mut World, _site_id: i64) {
    unimplemented!("RU18: finish_upgrade")
}

/// Go: `eachSite` (hub_level.go): alle Plätze der Stufe samt Zahlplatz des Hubs.
pub fn each_site(_w: &World, _f: &mut dyn FnMut(&Site)) {
    unimplemented!("RU18: each_site")
}

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

/// Go: `addPoolPoints` (monarch.go): Punkte in den Fund-Pool der Insel (ohne Insel: Zähler der Welt).
pub fn add_pool_points(_w: &mut World, _isl: Option<&mut IslandRef>, _n: i64) {
    unimplemented!("RU18: add_pool_points")
}
