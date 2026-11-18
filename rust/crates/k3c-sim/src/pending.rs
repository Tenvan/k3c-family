//! Platzhalter der Gruppen `enemies` und `unit` (Port-Leitfaden § 7): Funktionen späterer Sprints (RU10, RU14, RU15, RU18,
//! RU19), die RU11 ruft. Der liefernde Sprint löscht den Platzhalter; RU22 prüft, dass keiner übrig ist. Die Platzhalter
//! aus RU8 stehen weiter in `state/pending.rs`. Go-Zeiger auf Gegner und Truppen sind hier Indizes (`ei`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU14, RU15, RU18, RU19)

use crate::enemies::waves::SpawnOrder;
use crate::state::island_ref::IslandRef;
use crate::state::shared::WorldValues;
use crate::state::{Enemy, Event, Job, Projectile, Site, Troop, World};
use k3c_data::economy::Gatherable;

// Von RU15 portiert (Bosse): Die Import-Stelle bleibt `pending`, damit die Gegner-Dateien unverändert bleiben.
pub use crate::boss::abilities::hit_splash;
pub use crate::boss::{boss_ability, boss_defeated, trigger_boss};

/// Go: `takeCrownOrLoot` (crown.go).
pub fn take_crown_or_loot(_w: &mut World, _ei: usize) {
    unimplemented!("RU15: take_crown_or_loot")
}

/// Go: `crownStolen` (crown.go): `thief` ist schon aus `w.enemies` genommen.
pub fn crown_stolen(_w: &mut World, _thief: &Enemy) {
    unimplemented!("RU15: crown_stolen")
}

/// Go: `dropLoot` (portal.go): die Beute des Gegners `ei` fällt als Münzen zu Boden.
pub fn drop_loot(_w: &mut World, _ei: usize) {
    unimplemented!("RU15: drop_loot")
}

/// Go: `dropCrownOf` (crown.go).
pub fn drop_crown_of(_e: &mut Enemy) {
    unimplemented!("RU15: drop_crown_of")
}

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

/// Go: `addPool` (world_cycle.go): Aggressions-Pool um `amount` Prozent.
pub fn add_pool(_w: &mut World, _amount: f64) {
    unimplemented!("RU14: add_pool")
}

/// Go: `addStock` (economy.go): `true`, wenn das Material in den Vorrat kam.
pub fn add_stock(_w: &mut World, _resource: &str, _amount: i64) -> bool {
    unimplemented!("RU18: add_stock")
}

/// Go: `scaledGold` (economy_fountain.go).
pub fn scaled_gold(_w: &World, _gold: i64) -> i64 {
    unimplemented!("RU18: scaled_gold")
}

/// Go: `scatterCoins` (economy.go): `count` Münzen um `x` auf den Boden.
pub fn scatter_coins(_w: &mut World, _x: f64, _count: i64) {
    unimplemented!("RU18: scatter_coins")
}

/// Go: `isSpellTower` (building_spell_tower.go).
pub fn is_spell_tower(_w: &World, _s: &Site) -> bool {
    unimplemented!("RU18: is_spell_tower")
}

/// Go: `squadPost` (combat.go): Posten hinter dem Banner.
pub fn squad_post(_w: &World, _t: &Troop) -> Option<f64> {
    unimplemented!("RU15: squad_post")
}

/// Go: `escortPost` (unit_escort.go): liest den Endboss-Kampf der Insel; den Ort liefert `unit::escort::escort_post_at`.
pub fn escort_post(_w: &World, _t: &Troop) -> Option<f64> {
    unimplemented!("RU15: escort_post")
}

/// Go: `repair` (building_repair.go): der Bauer `ti` repariert.
pub fn repair(_w: &mut World, _v: &WorldValues, _ti: usize, _dt: f64) {
    unimplemented!("RU18: repair")
}

/// Go: `repairJob` (building_repair.go).
pub fn repair_job(_w: &World, _v: &WorldValues, _ti: usize) -> Option<Job> {
    unimplemented!("RU18: repair_job")
}

/// Go: `upgradeJob` (hub_level.go): Ist der Auftrag ein Ausbau?
pub fn upgrade_job(_w: &World, _j: &Job) -> bool {
    unimplemented!("RU18: upgrade_job")
}

/// Go: `outpostJob` (building_outpost.go).
pub fn outpost_job(_w: &World, _j: &Job) -> bool {
    unimplemented!("RU18: outpost_job")
}

/// Go: `outpost` (building_outpost.go): Liegt `x` an einem Vorposten?
pub fn outpost(_w: &World, _x: f64) -> bool {
    unimplemented!("RU18: outpost")
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

/// Go: `nodeFree` (world_vein.go).
pub fn node_free(_w: &World, _n: &crate::state::ResourceNode) -> bool {
    unimplemented!("RU14: node_free")
}

/// Go: `isVein` (world_vein.go).
pub fn is_vein(_v: &WorldValues, _n: &crate::state::ResourceNode) -> bool {
    unimplemented!("RU14: is_vein")
}

/// Go: `gatherVein` (world_vein.go).
pub fn gather_vein(_t: &mut Troop, _g: &Gatherable, _dt: f64) {
    unimplemented!("RU14: gather_vein")
}

/// Go: `vagrantsAt` (building_tavern.go): Landstreicher um `x`.
pub fn vagrants_at(_w: &World, _x: f64) -> i64 {
    unimplemented!("RU18: vagrants_at")
}

/// Go: `wanderRadius` (building_tavern.go): Wanderradius des Landstreichers `ti`.
pub fn wander_radius(_w: &World, _ti: usize) -> f64 {
    unimplemented!("RU18: wander_radius")
}

/// Go: `carryToStock` (island_storage.go): Tragen in den Vorrat der Insel.
pub fn carry_to_stock(_w: &mut World, _v: &WorldValues, _ti: usize, _dt: f64) {
    unimplemented!("RU10: carry_to_stock")
}

/// Go: `armoredHP` (building_upgrades.go): HP der Figur `kind` mit der Rüstung des Hubs.
pub fn armored_hp(_w: &World, _kind: &str) -> f64 {
    unimplemented!("RU18: armored_hp")
}

/// Go: `fighters` (building_barracks.go): Zahl der Kämpfer, die zum Truppenlimit zählen.
pub fn fighters(_w: &World) -> i64 {
    unimplemented!("RU18: fighters")
}

/// Go: `troopLimit` (building_barracks.go): das Truppenlimit der Kaserne.
pub fn troop_limit(_w: &World) -> i64 {
    unimplemented!("RU18: troop_limit")
}

/// Go: `combatSpotNear` (combat.go): Liegt `x` an einem Kampf-Zahlziel?
pub fn combat_spot_near(_w: &World, _x: f64) -> bool {
    unimplemented!("RU15: combat_spot_near")
}

/// Go: `leaveSquad` (combat.go): die Truppe verlässt ihren Trupp.
pub fn leave_squad(_w: &mut World, _ti: usize) {
    unimplemented!("RU15: leave_squad")
}

/// Go: `weaponsInFlight` (building_barracks.go): Waffen, die andere Bürger gerade holen.
pub fn weapons_in_flight(_w: &World, _ti: usize) -> i64 {
    unimplemented!("RU18: weapons_in_flight")
}

/// Go: `addStockCapped` (island_storage.go): legt höchstens bis zur Kapazität der Insel ab und liefert die aufgenommene
/// Menge; `None`: kein Baumaterial (Go: `ok` false).
pub fn add_stock_capped(_w: &mut World, _isl: Option<&IslandRef>, _resource: &str, _amount: i64) -> Option<i64> {
    unimplemented!("RU10: add_stock_capped")
}

/// Go: `addPoolPoints` (monarch.go): Punkte in den Fund-Pool der Insel (ohne Insel: Zähler der Welt).
pub fn add_pool_points(_w: &mut World, _isl: Option<&mut IslandRef>, _n: i64) {
    unimplemented!("RU18: add_pool_points")
}
