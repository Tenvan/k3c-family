//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::Data;
use k3c_data::biome::Biome;
use k3c_data::buildings::Plantation;
use k3c_data::economy::Gatherable;
use k3c_data::hub::HubData;
use k3c_data::monarch::{SkillData, SkillEffect};

use k3c_level::types::{NatureValues, PlannedChunk, WallSite};

use super::island_ref::IslandRef;
use super::save::HubSave;
use super::shared::WorldValues;
use super::{Player, PlayerCommand, Site, Troop, World};
use crate::island::{Island, WorldDef};

// Von RU11 portiert (Gegner, Truppen): Die Import-Stelle bleibt `pending`, damit Zustandsdateien unverändert bleiben.
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

/// Go: `gatherOf` (economy.go): die sammelbare Ressource zu einer Knoten-Art.
pub fn gather_of<'a>(_v: &'a WorldValues, _kind: &str) -> Option<&'a Gatherable> {
    unimplemented!("RU18: gather_of")
}

/// Go: `WorldValues.skillRef` (monarch_data.go): der Skill `id` im Katalog des Wertesatzes.
pub fn skill_ref<'a>(_v: &'a WorldValues, _id: &str) -> Option<&'a SkillData> {
    unimplemented!("RU18: skill_ref")
}

/// Go: `siteHubLevel` (hub_level.go): Hub-Stufe, ab der ein Platz bezahlbar ist, nach data/hub.json.
pub fn site_hub_level(_w: &World, _s: &Site) -> i64 {
    unimplemented!("RU18: site_hub_level")
}

/// Go: `levelSite` (world_lines.go): der verteilte Platz zu `s`; `None` für Plätze, die nicht aus `Layout.sites` stammen.
pub fn level_site(_w: &World, _s: &Site) -> Option<WallSite> {
    unimplemented!("RU18: level_site")
}

/// Go: `isSoldier` (combat.go): Ist die Truppe ein Nahkämpfer, der einer Gruppe vorstehen kann?
pub fn is_soldier(_t: &Troop) -> bool {
    unimplemented!("RU15: is_soldier")
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

// RU10.1: Platzhalter der Gruppe `island` für Funktionen und Typen späterer Sprints.

/// Go: `endbossFight` (boss_endboss.go): der laufende Kampf gegen den Endboss der Insel. Rumpf bis RU15.2.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EndbossFight;

/// Go: `DevFlags` (dev.go): Schalter der Dev-Aktionen einer Insel, nur im Speicher, nie im Spielstand. Rumpf bis RU19.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DevFlags {
    /// Alle Bau-, Ausbau- und Angebotskosten höchstens 1 je Posten.
    pub cheap_build: bool,
    /// Reiner Stressmodus (`DevSetPure`): keine Wellen, Lager, Wachen, Händler, Schänke.
    pub pure: bool,
    /// Zusätzliches Lager-Maximum je Rohstoff und Gold-Maximum je Spieler (`devMaxStock`).
    pub stock_bonus: i64,
    /// Event-Id → Nacht, in der das Event zusätzlich zum Rhythmus läuft (`DevStartEvent`).
    pub nights: BTreeMap<String, i64>,
}

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

/// Go: `Options` (world.go) für `CreateWorld`; die kleingeschriebenen Felder (`lair`, `hub`, `vals`) sind hier öffentlich.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldOptions {
    pub cycle_speed: f64,
    pub time: f64,
    pub planned: Vec<PlannedChunk>,
    pub chunk_count: i64,
    pub nature: NatureValues,
    /// Trägt die Stufe den Bau (`level.Options.Lair`); `None` = Feld `lair` des Bioms.
    pub lair: Option<bool>,
    /// Kandidaten-hub.json; `None` = die hub.json des Wertesatzes.
    pub hub: Option<Arc<HubData>>,
    /// Wertesatz der Welt; `None` = Basis-Wertesatz.
    pub vals: Option<Arc<WorldValues>>,
}

/// Go: `CreateWorld` (world.go): baut den Startzustand einer Stufe.
pub fn create_world_with(_b: &Biome, _seed: &str, _opts: &WorldOptions) -> Result<World, String> {
    unimplemented!("RU14: create_world_with")
}

/// Go: `Step` (world.go): rechnet einen Tick; `isl` ist der Bezug zur Insel (`None` = Go `w.island == nil`).
pub fn step(_w: &mut World, _commands: &[PlayerCommand], _isl: Option<&mut IslandRef>, _dt: f64) {
    unimplemented!("RU14: step")
}

/// Go: `addPlayerAt` (world.go): stellt den Monarchen mit dem Spielerindex `index` an die Burg; liefert seinen Platz in
/// `w.players`.
pub fn add_player_at(_w: &mut World, _index: i64) -> usize {
    unimplemented!("RU14: add_player_at")
}

/// Go: `castleLosses` (world.go): Verluste eines Burgfalls im Modus „Stufenverlust“.
pub fn castle_losses(_w: &mut World, _isl: Option<&IslandRef>) {
    unimplemented!("RU14: castle_losses")
}

/// Go: `hubOf` (world.go): die hub.json, aus der die Stufe gebaut ist (Kandidat oder die der Welt).
pub fn hub_of(_w: &World) -> Arc<HubData> {
    unimplemented!("RU14: hub_of")
}

/// Go: `emptySite` (world.go): ein unbezahlter Platz der Art `kind` bei `x`.
pub fn empty_site(_w: &mut World, _kind: &str, _x: f64) -> Site {
    unimplemented!("RU14: empty_site")
}

/// Go: `valuesOf` (world_values_room.go): der aufgelöste Wertesatz der Welt.
pub fn values_of(_world: &WorldDef) -> Result<Arc<WorldValues>, String> {
    unimplemented!("RU14: values_of")
}

/// Go: `ValidateHubDataIn` (hub_validate.go): Befunde der Prüfung einer Kandidaten-hub.json (leer = gültig).
pub fn validate_hub_data_in(_h: &HubData, _world: &WorldDef) -> Vec<String> {
    unimplemented!("RU14: validate_hub_data_in")
}

/// Go: `mirrorState` (world_state_mirror.go): spiegelt den Stand der Stufe für das Protokoll.
pub fn mirror_state(_w: &mut World) {
    unimplemented!("RU14: mirror_state")
}

/// Go: `decodeIslandStory` (story.go): liest `story` der Insel streng (arrival, boss, done); nur gerufen, wenn das Feld da ist.
pub fn decode_island_story(_raw: &serde_json::Value) -> Result<IslandStory, String> {
    unimplemented!("RU19: decode_island_story")
}

/// Go: `decodeWorldStory` (story.go): liest `story` der Welt streng; nur gerufen, wenn das Feld da ist.
pub fn decode_world_story(_raw: &serde_json::Value) -> Result<WorldStory, String> {
    unimplemented!("RU19: decode_world_story")
}

/// Go: `decodeWorldSkins` (world_skins.go): liest `skins` der Welt (Basis-Asset → Ersatz-Asset); nur gerufen, wenn das Feld
/// da ist.
pub fn decode_world_skins(_raw: &serde_json::Value) -> Result<BTreeMap<String, String>, String> {
    unimplemented!("RU14: decode_world_skins")
}

/// Go: `checkStory` (story_check.go): prüft die Geschichte der Welt und ihrer Inseln; nur gerufen, wenn die Welt oder eine
/// Insel `story` hat.
pub fn check_story(_w: &WorldDef, _data: &Data) -> Result<(), String> {
    unimplemented!("RU19: check_story")
}

/// Go: `mirrorSwitch` (world_state_mirror.go): spiegelt den Wechselpunkt der Insel in jede Stufe.
pub fn mirror_switch(_isl: &mut Island) {
    unimplemented!("RU14: mirror_switch")
}

/// Go: `travelPoint` (world_travel.go): ein Reisepunkt einer Stufe (`via`: exit, stairsUp, stairsDown).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TravelPoint {
    pub x: f64,
    pub to_depth: i64,
    pub via: String,
}

/// Go: `travelPoints` (world_travel.go): die Reisepunkte der Stufe (Tiefen-Eingang nach unten, gebaute Treppen).
pub fn travel_points(_w: &World) -> Vec<TravelPoint> {
    unimplemented!("RU14: travel_points")
}

/// Go: `Island.harbor` (ship.go): Stufe mit dem Hafen und der Platz des Schiffs darin (Indizes); `None` ohne Hafen.
pub fn harbor(_isl: &Island) -> Option<(usize, usize)> {
    unimplemented!("RU14: harbor")
}

/// Go: `shipComplete` (ship.go): Alle Abschnitte des Schiffs sind gebaut.
pub fn ship_complete(_w: &World, _s: &Site) -> bool {
    unimplemented!("RU14: ship_complete")
}

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
