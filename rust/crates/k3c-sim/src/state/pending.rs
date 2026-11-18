//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU10, RU15, RU18, RU23)

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::Data;
use k3c_data::biome::Biome;
use k3c_data::bosses::BossData;
use k3c_data::buildings::Plantation;
use k3c_data::economy::Gatherable;
use k3c_data::hub::{HubData, Stock};
use k3c_data::island_sizes::IslandSizesFile;
use k3c_data::monarch::{SkillData, SkillEffect};

use k3c_level::types::{NatureValues, PlannedChunk, WallSite};

use super::fehler::Fehler;
use super::island_ref::IslandRef;
use super::save::HubSave;
use super::shared::{SwitchTarget, WorldValues};
use super::{Player, PlayerCommand, Site, Troop, World};
use crate::island::{Island, IslandDef, IslandSave, MerchantSave, SaveTables, WorldDef};

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

/// Go: `CheckChunkCount` (island_sizes.go): lehnt eine Chunk-Anzahl ab, die keine Inselgröße ist.
pub fn check_chunk_count(_sizes: &IslandSizesFile, _n: i64) -> Result<(), String> {
    unimplemented!("RU10.3: check_chunk_count")
}

/// Go: `checkChunkCounts` (island_sizes.go): prüft `chunkCounts` einer Insel.
pub fn check_chunk_counts(_d: &IslandDef, _sizes: &IslandSizesFile) -> Result<(), String> {
    unimplemented!("RU10.3: check_chunk_counts")
}

/// Go: `checkEndBoss` (island_endboss.go): prüft Endboss und Bau der Insel.
pub fn check_end_boss(_d: &IslandDef, _data: &Data) -> Result<(), String> {
    unimplemented!("RU10.5: check_end_boss")
}

/// Go: `lairOptions` (island_endboss.go): trägt die Stufe der Tiefe `depth` den Bau (nur die tiefste), und ihr Endboss.
pub fn lair_options(_def: &IslandDef, _depth: i64, _data: &Data) -> (Option<bool>, Option<BossData>) {
    unimplemented!("RU10.5: lair_options")
}

/// Go: `stepIslandTravel` (island_travel.go).
pub fn step_island_travel(_isl: &mut Island, _dt: f64) {
    unimplemented!("RU10.4: step_island_travel")
}

/// Go: `stepIslandSwitch` (island_switch.go).
pub fn step_island_switch(_isl: &mut Island, _dt: f64) {
    unimplemented!("RU10.4: step_island_switch")
}

/// Go: `mirrorSwitch` (world_state_mirror.go): spiegelt den Wechselpunkt der Insel in jede Stufe.
pub fn mirror_switch(_isl: &mut Island) {
    unimplemented!("RU10.4: mirror_switch")
}

/// Go: `checkVictory` (island_victory.go).
pub fn check_victory(_isl: &mut Island) {
    unimplemented!("RU10.4: check_victory")
}

// RU10.2: Platzhalter für Funktionen aus `island_save_v3/v5/v8.go` (RU10.3) und `island_travel/targets.go` (RU10.4), die
// `to_save` und `from_island_save_in` rufen.

/// Go: `parseIslandV2` (island_save_v3.go): liest einen Stand der Version 2 (Pool = Summe der Zähler je Stufe).
pub fn parse_island_v2(_raw: &str, _t: &SaveTables) -> Result<IslandSave, Fehler> {
    unimplemented!("RU10.3: parse_island_v2")
}

/// Go: `migrateWorld` (island_save_v8.go): setzt Welt und Insel eines Stands bis Version 7 (Standard-Welt, Insel am alten
/// Index `index`).
pub fn migrate_world(_s: IslandSave, _index: i64, _standard_world: &WorldDef) -> Result<IslandSave, Fehler> {
    unimplemented!("RU10.3: migrate_world")
}

/// Go: `islandIn` (island_save_v8.go): der Index der Insel des Stands in der Welt `w`.
pub fn island_in(_w: &WorldDef, _s: &IslandSave) -> Result<usize, Fehler> {
    unimplemented!("RU10.3: island_in")
}

/// Go: `restorePool` (island_save_v3.go): setzt den Pool der Insel, spiegelt ihn in alle Stufen und leitet den
/// Truhen-Zähler ab.
pub fn restore_pool(_isl: &mut Island, _s: &IslandSave) {
    unimplemented!("RU10.3: restore_pool")
}

/// Go: `restoreProgress` (island_save_v5.go): besiegte Bosse, Zähler und Händler; `data` ist Gos Paket-Variable `bosses`.
pub fn restore_progress(_isl: &mut Island, _s: &IslandSave, _data: &Data) {
    unimplemented!("RU10.3: restore_progress")
}

/// Go: `savedMerchant` (island_save_v5.go): der anwesende Händler der Insel oder `None`.
pub fn saved_merchant(_isl: &Island) -> Option<MerchantSave> {
    unimplemented!("RU10.3: saved_merchant")
}

/// Go: `Island.Players` (island_travel.go): alle Spieler der Insel nach Index sortiert.
pub fn island_players(_isl: &Island) -> Vec<&Player> {
    unimplemented!("RU10.4: island_players")
}

/// Go: `Island.StageOf` (island_travel.go): Index der Stufe des Spielers mit dem Index `index`; `None` statt Gos -1.
pub fn stage_of(_isl: &Island, _index: i64) -> Option<usize> {
    unimplemented!("RU10.4: stage_of")
}

/// Go: `stageByDepth` (island_travel.go): Index der Stufe mit dieser Tiefe; `None` statt Gos -1.
pub fn stage_by_depth(_isl: &Island, _depth: i64) -> Option<usize> {
    unimplemented!("RU10.4: stage_by_depth")
}

/// Go: `Island.Targets` (island_targets.go): die Ziele am Hafen.
pub fn targets(_isl: &Island) -> Vec<SwitchTarget> {
    unimplemented!("RU10.4: targets")
}
