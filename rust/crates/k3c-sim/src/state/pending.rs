//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::buildings::Plantation;
use k3c_data::economy::Gatherable;
use k3c_data::hub::Stock;
use k3c_data::monarch::{SkillData, SkillEffect};
use k3c_data::troops::{ProfessionData, TroopData};
use k3c_level::types::WallSite;

use super::island_ref::IslandRef;
use super::save::HubSave;
use super::shared::WorldValues;
use super::{Enemy, Player, Site, Troop, World};
use super::{Event, Projectile};
use crate::enemies::waves::SpawnOrder;

/// Go: `merchantID` (merchant.go), die ID des Händlers als Ziel.
pub const MERCHANT_ID: i64 = -1;

/// Go: `crowned` (crown.go).
pub fn crowned(_p: &Player) -> bool {
    unimplemented!("RU15: crowned")
}

/// Go: `dropCrown` (crown.go).
pub fn drop_crown(_w: &mut World, _pi: usize, _cause: &str) {
    unimplemented!("RU15: drop_crown")
}

/// Go: `hitCoins` (crown.go).
pub fn hit_coins(_w: &mut World, _pi: usize, _n: i64) {
    unimplemented!("RU15: hit_coins")
}

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

/// Go: `damageMerchant` (merchant.go).
pub fn damage_merchant(_w: &mut World, _damage: f64) {
    unimplemented!("RU18: damage_merchant")
}

/// Go: `protectedNight` (World-Methode, Nacht-Schutz der Gebäude).
pub fn protected_night(_w: &World) -> bool {
    unimplemented!("RU18: protected_night")
}

/// Go: `isRuin` (building_ruin.go).
pub fn is_ruin(_w: &World, _kind: &str, _x: f64) -> bool {
    unimplemented!("RU18: is_ruin")
}

/// Go: `World.values` (world_values_room.go), der Wertesatz der Welt (Standard, wenn `vals` fehlt).
pub fn values(_w: &World) -> Arc<WorldValues> {
    unimplemented!("RU10: values")
}

/// Go: `payTarget` (economy_pay.go), das Zahlziel am Ort eines Spielers; hier nur die Felder, die RU8 liest.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PayTarget {
    pub offer: Option<PayOffer>,
    /// "buy", "sell" oder "" (kein Händler-Ziel).
    pub merchant: String,
}

/// Go: das Angebot eines Zahlziels (`offer`, unit_professions.go).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PayOffer;

impl PayOffer {
    /// Go: `offer.kind()`.
    pub fn kind(&self) -> &str {
        unimplemented!("RU18: PayOffer::kind")
    }
}

/// Go: `findPayTarget` (economy_pay.go).
pub fn find_pay_target(_w: &World, _p: &Player) -> Option<PayTarget> {
    unimplemented!("RU18: find_pay_target")
}

/// Go: `professionOf` (unit_professions.go): der Beruf `kind` aus den Truppen-Daten.
pub fn profession_of<'a>(_troops: &'a BTreeMap<String, TroopData>, _kind: &str) -> Option<&'a ProfessionData> {
    unimplemented!("RU18: profession_of")
}

/// Go: `stockField` (economy.go): der Vorrat an `resource`; `None` bei unbekanntem Material.
pub fn stock_field(_stock: &Stock, _resource: &str) -> Option<i64> {
    unimplemented!("RU18: stock_field")
}

/// Go: `resourceFull` (economy.go): Ist das Lager für `resource` voll?
pub fn resource_full(_w: &World, _resource: &str) -> bool {
    unimplemented!("RU18: resource_full")
}

/// Go: `gatherOf` (economy.go): die sammelbare Ressource zu einer Knoten-Art.
pub fn gather_of<'a>(_v: &'a WorldValues, _kind: &str) -> Option<&'a Gatherable> {
    unimplemented!("RU18: gather_of")
}

/// Go: `WorldValues.skillRef` (monarch_data.go): der Skill `id` im Katalog des Wertesatzes.
pub fn skill_ref<'a>(_v: &'a WorldValues, _id: &str) -> Option<&'a SkillData> {
    unimplemented!("RU18: skill_ref")
}

/// Go: `purseMax` (economy.go): Münz-Obergrenze der Insel samt Dev-Bonus.
pub fn purse_max(_w: &World) -> i64 {
    unimplemented!("RU18: purse_max")
}

/// Go: `siteHubLevel` (hub_level.go): Hub-Stufe, ab der ein Platz bezahlbar ist, nach data/hub.json.
pub fn site_hub_level(_w: &World, _s: &Site) -> i64 {
    unimplemented!("RU18: site_hub_level")
}

/// Go: `levelSite` (world_lines.go): der verteilte Platz zu `s`; `None` für Plätze, die nicht aus `Layout.sites` stammen.
pub fn level_site(_w: &World, _s: &Site) -> Option<WallSite> {
    unimplemented!("RU18: level_site")
}

/// Go: `spawnVagrant` (enemies/units): legt einen Landstreicher an und gibt seinen Index in `w.troops` zurück.
pub fn spawn_vagrant(_w: &mut World, _anchor_x: f64, _x: f64) -> usize {
    unimplemented!("RU11: spawn_vagrant")
}

/// Go: `makeFighter` (unit_warrior.go): macht die Truppe `ti` zum Kämpfer der Art `kind`.
pub fn make_fighter(_w: &mut World, _ti: usize, _kind: &str) {
    unimplemented!("RU11: make_fighter")
}

/// Go: `isSoldier` (combat.go): Ist die Truppe ein Nahkämpfer, der einer Gruppe vorstehen kann?
pub fn is_soldier(_t: &Troop) -> bool {
    unimplemented!("RU15: is_soldier")
}

/// Go: `isFighter` (unit_warrior.go): Ist `kind` eine Kämpfer-Figur aus den Truppen-Daten?
pub fn is_fighter(_troops: &BTreeMap<String, TroopData>, _kind: &str) -> bool {
    unimplemented!("RU11: is_fighter")
}

/// Go: `hubSteps` (hub_level.go): Zahl der Ausbauschritte der aktuellen Hub-Stufe.
pub fn hub_steps(_w: &World) -> i64 {
    unimplemented!("RU18: hub_steps")
}

/// Go: `countFetching` (combat.go): Zahl der Truppen, die gerade ihr Schild holen.
pub fn count_fetching(_w: &World) -> i64 {
    unimplemented!("RU15: count_fetching")
}

/// Go: `applyPortals` (portal.go): übernimmt die zerstörten Portale eines gespeicherten Hubs.
pub fn apply_portals(_w: &mut World, _h: &HubSave) {
    unimplemented!("RU15: apply_portals")
}

/// Go: `plotX` (building_plantation.go): Position von Platz `i` im Streifen um die Baumschule.
pub fn plot_x(_s: &Site, _p: &Plantation, _i: usize) -> f64 {
    unimplemented!("RU18: plot_x")
}

// Von RU11.1 portiert: Gegner-Wache. Die Import-Stelle bleibt `pending`, damit Zustandsdateien unverändert bleiben.
pub use crate::enemies::guard::{apply_guards, guards_cleared_keys, is_wave_enemy};

/// Go: `hammerKind` (unit_hammer.go); RU11.3 verschiebt die Konstante nach `unit/hammer.rs`.
pub const HAMMER_KIND: &str = "hammer";

/// Go: `bossAbility` (boss.go): Boss-Fähigkeit des Gegners `ei`.
pub fn boss_ability(_w: &mut World, _ei: usize, _isl: Option<&mut IslandRef>) {
    unimplemented!("RU15: boss_ability")
}

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

// Von RU11.2 portiert: Gegner-Traits, Wellen, Bogenschützen. Die Import-Stelle bleibt `pending`.
pub use crate::enemies::traits::{attack_rate, invulnerable, kite, strike};
pub use crate::unit::archer::is_on_tower;

/// Go: `arrive` (unit_common.go); RU11.3 verschiebt die Konstante nach `unit/common.rs`.
pub const ARRIVE: f64 = 0.3;

/// Go: `arrowSpeed` (unit_common.go); RU11.3 verschiebt die Konstante nach `unit/common.rs`.
pub const ARROW_SPEED: f64 = 25.0;

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

/// Go: `triggerBoss` (boss.go).
pub fn trigger_boss(_w: &mut World, _isl: Option<&mut IslandRef>) {
    unimplemented!("RU15: trigger_boss")
}

/// Go: `bossDefeated` (boss.go): `e` ist der besiegte Boss.
pub fn boss_defeated(_w: &mut World, _e: &Enemy, _isl: Option<&mut IslandRef>) {
    unimplemented!("RU15: boss_defeated")
}

/// Go: `hitSplash` (boss_abilities.go): Splitter-Wurf des Geschosses `p` bei `x`.
pub fn hit_splash(_w: &mut World, _p: &Projectile, _x: f64) {
    unimplemented!("RU15: hit_splash")
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

/// Go: `escortPost` (unit_escort.go); RU11.3 liefert sie.
pub fn escort_post(_w: &World, _t: &Troop) -> Option<f64> {
    unimplemented!("RU11: escort_post")
}

/// Go: `walkTo` (unit_common.go); RU11.3 liefert sie. `ti` ist der Index der Truppe.
pub fn walk_to(_w: &mut World, _ti: usize, _x: f64, _dt: f64) -> bool {
    unimplemented!("RU11: walk_to")
}
