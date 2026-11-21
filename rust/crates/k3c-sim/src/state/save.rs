//! Spielstand (Port von `engine/sim/save.go`). Gespeichert wird nur, was sich nicht aus dem Seed ergibt: Hubs, Truppen,
//! Vorräte, Gold und was aus der Welt schon entfernt wurde. Das Format ist ein Vertrag mit Version.
//!
//! Nicht Teil dieser Datei: `Campaign.ToSave` und `FromSave` (sie hängen an `Campaign`, RU14). Die Anwendung eines Hubs
//! auf eine Welt steht in `save_apply.rs`.

use std::collections::{BTreeMap, BTreeSet};

use k3c_data::biome::Biome;
use k3c_data::hub::Stock;
use k3c_level::types::{NatureValues, PlannedChunk};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::data::has_depth;
use super::fehler::Fehler;
use super::links::{count_fetching, guards_cleared_keys};
use super::node_mark::marked_by_of;
use super::omit::{is_false, is_zero_i64};
use super::save_economy::{CraftSave, craft_save, troops_save, validate_economy};
use super::types::CITIZEN;
use super::{Pickup, ResourceNode, Site, World, WorldValues};

/// Go: `SaveVersion`, die einzige Version, die Go liest und schreibt.
pub const SAVE_VERSION: i64 = 1;

/// Go: `SaveGame`, ein Spielstand (dieselbe JSON-Form wie in TS).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveGame {
    #[serde(rename = "version")]
    pub version: i64,
    /// Gleiche ID = gleiches Spiel.
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    #[serde(rename = "savedAt")]
    pub saved_at: String,
    #[serde(rename = "seed")]
    pub seed: String,
    #[serde(rename = "depth")]
    pub depth: i64,
    #[serde(rename = "unlockedDepth")]
    pub unlocked_depth: i64,
    #[serde(rename = "time")]
    pub time: f64,
    #[serde(rename = "skillPoints")]
    pub skill_points: i64,
    #[serde(rename = "players")]
    pub players: Vec<PlayerSave>,
    #[serde(rename = "hubs")]
    pub hubs: Vec<HubSave>,
}

/// Go: `HubSave`, der Hub einer Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubSave {
    #[serde(rename = "depth")]
    pub depth: i64,
    #[serde(rename = "castleHp")]
    pub castle_hp: f64,
    #[serde(rename = "stock")]
    pub stock: Stock,
    #[serde(rename = "wave")]
    pub wave: i64,
    #[serde(rename = "aggression")]
    pub aggression: Option<f64>,
    #[serde(rename = "sites")]
    pub sites: Vec<SiteSave>,
    /// Hub-Stufe (0 = 1) und ein laufender Hub-Ausbau an der Burg, beide optional.
    #[serde(rename = "hubLevel", skip_serializing_if = "is_zero_i64")]
    pub hub_level: i64,
    #[serde(rename = "hubUpgrade", skip_serializing_if = "Option::is_none")]
    pub hub_upgrade: Option<SiteSave>,
    /// Der laufende Ausbauschritt; fehlt bei Schritt 1.
    #[serde(rename = "hubStep", skip_serializing_if = "is_zero_i64")]
    pub hub_step: i64,
    /// Rüstungsstufe der Kämpfer des Hubs (Version 7), 0 = keine.
    #[serde(rename = "armorLevel", skip_serializing_if = "is_zero_i64")]
    pub armor_level: i64,
    /// Nur Bauern und Bogenschützen. Landstreicher kommen von allein aus den Camps.
    #[serde(rename = "troops")]
    pub troops: Vec<TroopSave>,
    /// Schlüssel `kind@x` der abgebauten Bäume/Felsen.
    #[serde(rename = "nodesGone")]
    pub nodes_gone: Vec<String>,
    #[serde(rename = "nodesMarked")]
    pub nodes_marked: Vec<String>,
    /// Spieler-Index je markiertem Knoten (`kind@x`), nur bekannte; fehlt in älteren Ständen.
    #[serde(rename = "nodesMarkedBy", skip_serializing_if = "BTreeMap::is_empty")]
    pub nodes_marked_by: BTreeMap<String, i64>,
    #[serde(rename = "pickupsTaken")]
    pub pickups_taken: Vec<String>,
    /// Erzeugung der Stufe: geplante Chunks und Chunk-Anzahl, mit denen sie gebaut wurde.
    #[serde(rename = "planned", skip_serializing_if = "Vec::is_empty")]
    pub planned: Vec<PlannedChunk>,
    #[serde(rename = "chunkCount", skip_serializing_if = "is_zero_i64")]
    pub chunk_count: i64,
    /// Natur-Werte, mit denen die Stufe gebaut wurde. Go: `omitzero` (nil-Maps); leere Maps fehlen hier ebenso.
    #[serde(rename = "nature", skip_serializing_if = "NatureValues::is_empty")]
    pub nature: NatureValues,
    /// Geräumte Gegnerlager (Version 9): Schlüssel `kind@x` der besiegten Wachen, sortiert.
    #[serde(rename = "guardsCleared", skip_serializing_if = "Vec::is_empty")]
    pub guards_cleared: Vec<String>,
    /// Zerstörte Portale: X der Portale; sind alle weg, ist die Stufe dauerhaft sicher.
    #[serde(rename = "portalsDown", skip_serializing_if = "Vec::is_empty")]
    pub portals_down: Vec<f64>,
    /// Bezahlte, noch nicht abgeholte Schilde der Stufe (Variante groups).
    #[serde(rename = "shields", skip_serializing_if = "is_zero_i64")]
    pub shields: i64,
}

/// Go: `PlayerSave`, das Gold eines Spielerplatzes.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerSave {
    #[serde(rename = "gold")]
    pub gold: i64,
}

/// Go: `TroopSave`, ein gespeicherter Bauer oder Kämpfer (Bogenschütze, Krieger, Elite). Beruf und Arbeitsplatz
/// (`kind@x` des Gebäudes) stehen ab Version 7 im Stand.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TroopSave {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "anchorX")]
    pub anchor_x: f64,
    #[serde(rename = "profession", skip_serializing_if = "String::is_empty")]
    pub profession: String,
    #[serde(rename = "workSite", skip_serializing_if = "String::is_empty")]
    pub work_site: String,
    /// Handwerker.
    #[serde(rename = "hammer", skip_serializing_if = "is_false")]
    pub hammer: bool,
    /// Gruppenführer mit Schild; die Gruppe füllt sich nach dem Laden neu.
    #[serde(rename = "leader", skip_serializing_if = "is_false")]
    pub leader: bool,
}

/// Go: `SiteSave`, ein gespeicherter Bauplatz.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteSave {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "paidGold")]
    pub paid_gold: i64,
    #[serde(rename = "buildProgress")]
    pub build_progress: f64,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "bows")]
    pub bows: i64,
    #[serde(rename = "bowPaidGold")]
    pub bow_paid_gold: i64,
    /// Ausbau-Stufe (0 = 1) und laufender Ausbau, optional.
    #[serde(rename = "level", skip_serializing_if = "is_zero_i64")]
    pub level: i64,
    #[serde(rename = "upgrade", skip_serializing_if = "String::is_empty")]
    pub upgrade: String,
    #[serde(rename = "upgradePaid", skip_serializing_if = "is_zero_i64")]
    pub upgrade_paid: i64,
    /// Wirtschaft (Version 7): Herstellung, Angebote, Schwerter, Plantage.
    #[serde(rename = "craft", skip_serializing_if = "Option::is_none")]
    pub craft: Option<CraftSave>,
}

/// Go: `key` ist `kind@x` mit x wie `x.toFixed(2)` in JS: Gleichstände (nur bei x = n/8) runden weg von 0, Go `FormatFloat`
/// würde zur geraden Ziffer runden.
pub fn key(kind: &str, x: f64) -> String {
    let mut x = x;
    let t = x * 8.0;
    if t == t.trunc() && t % 2.0 != 0.0 {
        x += 0.001_f64.copysign(x);
    }
    format!("{kind}@{x:.2}")
}

/// Go: `keys`: die Schlüssel aller Einträge, für die `k` einen liefert (in Reihenfolge).
pub fn keys<T>(items: &[T], k: impl Fn(&T) -> Option<String>) -> Vec<String> {
    items.iter().filter_map(k).collect()
}

/// Go: `without`: die Einträge von `all`, die nicht in `now` stehen (Reihenfolge von `all`).
pub fn without(all: &[String], now: &[String]) -> Vec<String> {
    let have: BTreeSet<&String> = now.iter().collect();
    all.iter().filter(|k| !have.contains(k)).cloned().collect()
}

/// Go: `nodeKey`.
pub fn node_key(n: &ResourceNode) -> Option<String> {
    Some(key(&n.kind, n.x))
}

/// Go: `pickupKey`.
pub fn pickup_key(p: &Pickup) -> Option<String> {
    Some(key(&p.kind, p.x))
}

/// Go: `hubSave`: der Hub der Welt `w`; `base` ist die Vergleichswelt frisch aus dem Seed (was dort ist, hier aber fehlt,
/// wurde entfernt).
pub fn hub_save(w: &World, base: &World) -> HubSave {
    let mut h = HubSave {
        depth: w.biome.depth,
        castle_hp: w.castle.hp,
        stock: w.stock.clone(),
        wave: w.wave,
        sites: Vec::new(),
        planned: w.planned.clone(),
        chunk_count: w.chunk_count,
        nature: w.level.nature.clone(),
        aggression: w.aggression,
        ..Default::default()
    };
    h.sites = w.sites.iter().map(|s| site_save(w, s)).collect();
    if w.hub_level > 1 {
        h.hub_level = w.hub_level;
    }
    if w.hub_step > 1 {
        h.hub_step = w.hub_step;
    }
    if let Some(u) = w
        .hub_site
        .as_ref()
        .filter(|u| !u.upgrade.is_empty() || u.upgrade_paid > 0)
    {
        h.hub_upgrade = Some(site_save(w, u));
    }
    h.armor_level = w.armor_level;
    h.troops = troops_save(w);
    h.nodes_gone = without(&keys(&base.nodes, node_key), &keys(&w.nodes, node_key));
    h.nodes_marked = keys(&w.nodes, |n| n.marked.then(|| key(&n.kind, n.x)));
    h.nodes_marked_by = marked_by_of(w);
    h.pickups_taken = without(&keys(&base.pickups, pickup_key), &keys(&w.pickups, pickup_key));
    h.guards_cleared = guards_cleared_keys(w);
    h.portals_down = w.portals_down.clone();
    h.shields = w.combat.shields + count_fetching(w);
    h
}

/// Go: `siteSave`.
pub fn site_save(w: &World, s: &Site) -> SiteSave {
    SiteSave {
        kind: s.kind.clone(),
        x: s.x,
        state: s.state.clone(),
        paid_gold: s.paid_gold,
        build_progress: s.build_progress,
        hp: s.hp,
        bows: s.bows,
        bow_paid_gold: s.bow_paid_gold,
        level: s.level,
        upgrade: s.upgrade.clone(),
        upgrade_paid: s.upgrade_paid,
        craft: craft_save(w, s),
    }
}

/// Go: `ParseSave`: liest einen Spielstand. Andere Versionen und fehlende Pflichtfelder sind ein Fehler (wie `isSaveGame`).
/// `biomes` ersetzt Gos Paket-Variable (`hasDepth`), `defaults` ist `defaultValues()` (die Standard-Welt der Kampagne).
pub fn parse_save(raw: &str, biomes: &[Biome], defaults: &WorldValues) -> Result<SaveGame, Fehler> {
    let fail = |s: String| Fehler::Spielstand(format!("spielstand: {s}"));
    let check: Value = serde_json::from_str(raw).map_err(|e| fail(e.to_string()))?;
    let obj = check
        .as_object()
        .ok_or_else(|| fail("json: cannot unmarshal into Go value of type map".to_string()))?;
    for f in ["version", "campaignId", "seed", "depth", "time", "hubs", "players"] {
        if obj.get(f).is_none_or(Value::is_null) {
            return Err(fail(format!("Feld {f} fehlt")));
        }
    }
    let s: SaveGame = serde_json::from_value(check).map_err(|e| fail(e.to_string()))?;
    if s.version != SAVE_VERSION {
        return Err(fail(format!("Version {}, erwartet {SAVE_VERSION}", s.version)));
    }
    if !has_depth(biomes, s.depth) {
        return Err(Fehler::Spielstand("spielstand: unbekannte Tiefe".to_string()));
    }
    for h in &s.hubs {
        validate_economy(defaults, h)?; // Kampagne: nur die Standard-Welt
    }
    Ok(s)
}

impl TroopSave {
    /// Bürger im Dienst: im Stand `peasant` (Go-Format) oder `citizen`.
    pub fn serving(&self) -> bool {
        self.kind == "peasant" || self.kind == CITIZEN
    }
}
