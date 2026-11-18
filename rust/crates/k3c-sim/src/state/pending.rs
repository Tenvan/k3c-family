//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use std::sync::Arc;

use k3c_data::buildings::Plantation;
use k3c_data::economy::Gatherable;
use k3c_data::hub::Stock;
use k3c_data::monarch::{SkillData, SkillEffect};

use k3c_level::types::WallSite;

use super::save::HubSave;
use super::shared::WorldValues;
use super::{Player, Site, World};

// Von RU11 portiert (Gegner, Truppen): Die Import-Stelle bleibt `pending`, damit Zustandsdateien unverändert bleiben.
pub use crate::combat::is_soldier; // Von RU15 portiert (combat.go)
pub use crate::enemies::guard::{apply_guards, guards_cleared_keys, is_wave_enemy};
pub use crate::enemies::traits::{attack_rate, invulnerable, kite, strike};
pub use crate::pending::*; // Platzhalter der Gruppen enemies und unit
pub use crate::unit::archer::is_on_tower;
pub use crate::unit::jobs::make_fighter;
pub use crate::unit::professions::profession_of;
pub use crate::unit::spawn_vagrant;
pub use crate::unit::warrior::is_fighter;

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
