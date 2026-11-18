//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use k3c_data::Data;
use k3c_data::buildings::Plantation;
use k3c_data::hub::HubData;
use k3c_data::monarch::{SkillData, SkillEffect};

use super::shared::WorldValues;
use super::{Player, Site, World};
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

/// Go: `merchantID` (merchant.go), die ID des Händlers als Ziel.
pub const MERCHANT_ID: i64 = -1;

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

pub use crate::world::values_room::values; // Von RU14.4 portiert (world_values_room.go)

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

pub use crate::world::vein::gather_of; // Von RU14.4 portiert (world_vein.go)

/// Go: `WorldValues.skillRef` (monarch_data.go): der Skill `id` im Katalog des Wertesatzes.
pub fn skill_ref<'a>(_v: &'a WorldValues, _id: &str) -> Option<&'a SkillData> {
    unimplemented!("RU18: skill_ref")
}

/// Go: `siteHubLevel` (hub_level.go): Hub-Stufe, ab der ein Platz bezahlbar ist, nach data/hub.json.
pub fn site_hub_level(_w: &World, _s: &Site) -> i64 {
    unimplemented!("RU18: site_hub_level")
}

/// Go: `hubSteps` (hub_level.go): Zahl der Ausbauschritte der aktuellen Hub-Stufe.
pub fn hub_steps(_w: &World) -> i64 {
    unimplemented!("RU18: hub_steps")
}

/// Go: `plotX` (building_plantation.go): Position von Platz `i` im Streifen um die Baumschule.
pub fn plot_x(_s: &Site, _p: &Plantation, _i: usize) -> f64 {
    unimplemented!("RU18: plot_x")
}

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

/// Go: `ValidateHubDataIn` (hub_validate.go): Befunde der Prüfung einer Kandidaten-hub.json (leer = gültig).
pub fn validate_hub_data_in(_h: &HubData, _world: &WorldDef) -> Vec<String> {
    unimplemented!("RU18: validate_hub_data_in (hub_validate, RU18.3)")
}

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

/// Go: `refundPending` (economy.go): wirft gezahltes, nicht vollendetes Gold des Spielers `pi` zurück auf den Boden.
pub fn refund_pending(_w: &mut World, _pi: usize) {
    unimplemented!("RU18: refund_pending")
}

/// Go: `toHub` (economy.go): stellt den Monarchen `pi` neben die Burg (Burgfall, Stufenwechsel).
pub fn to_hub(_w: &mut World, _pi: usize) {
    unimplemented!("RU18: to_hub")
}

/// Go: `returnBanner` (combat_squad.go): stellt das Banner des Monarchen `pi` am Tor wieder auf; sein Trupp kehrt zurück.
pub fn return_banner(_w: &mut World, _pi: usize) {
    unimplemented!("RU15: return_banner")
}
