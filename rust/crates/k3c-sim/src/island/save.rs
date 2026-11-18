//! Spielstand der Insel (Port von `engine/sim/island_save.go` bis `islandFromV1`; Version 9, B-100, S1.4, S2.2, K2.3b,
//! K3.2a, W7.2, WZ18.3, WZ27.1). Die Campaign schreibt weiter Version 1 (`state/save.rs`). Gespeichert wird wie dort nur,
//! was sich nicht aus dem Seed ergibt: Hubs, Truppen, Vorrat, Gold und Entferntes. Flüchtiges (Gegner, Münzen am Boden,
//! Geschosse, Zufallsstand) geht beim Laden verloren.
//!
//! Lesen und Prüfen: `save_parse.rs`; die Insel aus dem Stand bauen: `save_load.rs`.
//!
//! Gos Paket-Tabellen (`biomes`, `bosses`, `defaultValues`, `standardWorld`) kommen als `SaveTables`. Fehler sind
//! `Fehler::Spielstand` mit dem Text von Go (auch die mit Präfix `insel:`).

use k3c_data::Data;
use k3c_data::hub::Stock;
use k3c_level::nature::neutral_nature;
use serde::{Deserialize, Serialize};

use super::options::IslandOptions;
use super::save_v5::saved_merchant;
use super::{Island, WorldDef};
use crate::state::omit::{is_false, is_zero_i64};
use crate::state::pending::{WorldOptions, create_world_with, island_players, stage_of};
use crate::state::save::{HubSave, SaveGame, hub_save};
use crate::state::{Fehler, WorldValues};

/// Go: `IslandSaveVersion`, die Version des Insel-Spielstands.
pub const ISLAND_SAVE_VERSION: i64 = 9;

/// Die Tabellen, die Go als Paket-Variablen hält: die Daten (`biomes`, `bosses`, `difficulty`), der Basis-Wertesatz
/// (`defaultValues()`, Skill-Katalog für Stände ohne Welt) und die Standard-Welt (`standardWorld`).
#[derive(Debug, Clone, Copy)]
pub struct SaveTables<'a> {
    pub data: &'a Data,
    pub defaults: &'a WorldValues,
    pub standard_world: &'a WorldDef,
}

/// Go: `MerchantSave`, der anwesende Händler im Spielstand (Version 6, B-373).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MerchantSave {
    #[serde(rename = "resource")]
    pub resource: String,
    #[serde(rename = "leaves")]
    pub leaves: i64,
    #[serde(rename = "buyPaid", skip_serializing_if = "is_zero_i64")]
    pub buy_paid: i64,
    #[serde(rename = "raid", skip_serializing_if = "is_false")]
    pub raid: bool,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "maxHp")]
    pub max_hp: f64,
}

/// Go: `IslandSave`, ein Spielstand einer Insel. Die Felder stehen in der Reihenfolge von Go.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandSave {
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    /// Welt-ID (Version 8, welten.md § 5; ältere Stände: standard).
    #[serde(rename = "world")]
    pub world: String,
    /// Insel-ID der aktuellen Insel in der Welt (Version 8).
    #[serde(rename = "islandId")]
    pub island_id: String,
    #[serde(rename = "savedAt")]
    pub saved_at: String,
    /// Tag der Insel beim Speichern (Version 4, nur Anzeige).
    #[serde(rename = "day", skip_serializing_if = "is_zero_i64")]
    pub day: i64,
    /// day, dusk, night, dawn beim Speichern (Version 4, nur Anzeige).
    #[serde(rename = "phase", skip_serializing_if = "String::is_empty")]
    pub phase: String,
    #[serde(rename = "seed")]
    pub seed: String,
    #[serde(rename = "time")]
    pub time: f64,
    /// Vorrat der Insel (alle Stufen teilen ihn).
    #[serde(rename = "stock")]
    pub stock: Stock,
    /// Fehlt im Stand: Standard (Normal, Endboss, Stufenverlust).
    #[serde(rename = "options")]
    pub options: IslandOptions,
    /// Je Stufe der Hub, nach Tiefe aufsteigend.
    #[serde(rename = "stages")]
    pub stages: Vec<HubSave>,
    /// Fund-Pool der Insel (Version 2: Summe der Zähler je Stufe).
    #[serde(rename = "skillPool")]
    pub skill_pool: i64,
    #[serde(rename = "players")]
    pub players: Vec<IslandPlayerSave>,
    /// Version 5: besiegte Bosse, Münzzähler und ob der Sieg schon gemeldet ist (die aktuelle Insel stand bis Version 7
    /// als Index in `island`).
    #[serde(rename = "defeatedBosses", skip_serializing_if = "Vec::is_empty")]
    pub defeated_bosses: Vec<String>,
    #[serde(rename = "goldCollected", skip_serializing_if = "is_zero_i64")]
    pub gold_collected: i64,
    #[serde(rename = "won", skip_serializing_if = "is_false")]
    pub won: bool,
    /// Version 6 (B-373): Besuche des Händlers und der anwesende Händler (Material, Abreisetag, HP).
    #[serde(rename = "merchantVisits", skip_serializing_if = "is_zero_i64")]
    pub merchant_visits: i64,
    #[serde(rename = "merchant", skip_serializing_if = "Option::is_none")]
    pub merchant: Option<MerchantSave>,
    /// B-961 (ohne neue Version, ältere Stände: leer): abgeschlossene Inseln (IDs) und das gewählte Ziel am Hafen.
    #[serde(rename = "completed", skip_serializing_if = "Vec::is_empty")]
    pub completed: Vec<String>,
    #[serde(rename = "chosen", skip_serializing_if = "String::is_empty")]
    pub chosen: String,
}

/// Go: `IslandPlayerSave`, ein Spieler der Insel: inselweiter Index, Gold, die Tiefe seiner Stufe und seine Verteilung
/// (gelernte Skills in Lernreihenfolge, Slots 1 bis 4).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandPlayerSave {
    #[serde(rename = "index")]
    pub index: i64,
    #[serde(rename = "gold")]
    pub gold: i64,
    #[serde(rename = "depth")]
    pub depth: i64,
    #[serde(rename = "skills", skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(rename = "slots", skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<String>,
    /// Mitgebrachte Skills einer früheren Insel (Version 5), zählen nicht gegen den Pool.
    #[serde(rename = "skillBase", skip_serializing_if = "is_zero_i64")]
    pub skill_base: i64,
}

impl Island {
    /// Go: `Island.ToSave` schreibt den Spielstand. `saved_at` ist ein Zeitstempel (ISO 8601). Die Reihenfolge der Stufen
    /// und der Spieler (nach Index) ist die von Go. Go bricht bei einer Insel ohne Stufen mit `panic` ab.
    pub fn to_save(&self, saved_at: &str) -> Result<IslandSave, Fehler> {
        let fail = |s: &str| Fehler::Spielstand(s.to_string());
        let first = self.stages.first().ok_or_else(|| fail("insel: keine Stufen"))?;
        let mut s = IslandSave {
            version: ISLAND_SAVE_VERSION,
            campaign_id: self.id.clone(),
            world: self.world.id.clone(),
            island_id: self.island_id.clone(),
            saved_at: saved_at.to_string(),
            day: first.cycle.day,
            phase: first.cycle.phase.clone(),
            seed: self.seed.clone(),
            time: first.time,
            stock: self.stock.clone(),
            options: self.options.clone(),
            skill_pool: self.skill_pool,
            defeated_bosses: self.defeated_bosses.clone(),
            gold_collected: self.gold_collected,
            won: self.won,
            merchant_visits: self.merchant_visits,
            merchant: saved_merchant(self),
            completed: self.completed.clone(),
            chosen: self.chosen.clone(),
            ..IslandSave::default()
        };
        for w in &self.stages {
            // Go: `newWorld(depth, seed, Options{})`: die Vergleichswelt frisch aus dem Seed, mit neutraler Natur.
            let opts = WorldOptions {
                nature: neutral_nature(&w.biome),
                ..WorldOptions::default()
            };
            let base = create_world_with(&w.biome, &self.seed, &opts).map_err(Fehler::Spielstand)?;
            s.stages.push(hub_save(w, &base));
        }
        for p in island_players(self) {
            let si = stage_of(self, p.index).ok_or_else(|| fail("insel: Spieler ohne Stufe"))?;
            s.players.push(IslandPlayerSave {
                index: p.index,
                gold: p.gold,
                depth: self.stages[si].biome.depth,
                skills: p.skills.clone(),
                slots: p.slots.clone(),
                skill_base: p.skill_base,
            });
        }
        Ok(s)
    }
}

/// Go: `islandFromV1` überführt einen Stand der Campaign: die Hubs werden Stufen, alle Spieler stehen in der gespeicherten
/// Tiefe, die Vorräte der Hubs zählen zusammen als Vorrat der Insel (Annahme), die Skill-Punkte werden der Pool.
/// `options` ist `DefaultOptions()` (Go-Paket-Variable `difficulty`).
pub fn island_from_v1(old: &SaveGame, options: IslandOptions) -> IslandSave {
    let mut hubs = old.hubs.clone();
    hubs.sort_by_key(|h| h.depth); // stabil wie `sort.SliceStable`
    let mut s = IslandSave {
        version: ISLAND_SAVE_VERSION,
        campaign_id: old.campaign_id.clone(),
        saved_at: old.saved_at.clone(),
        seed: old.seed.clone(),
        time: old.time,
        options,
        skill_pool: old.skill_points,
        ..IslandSave::default()
    };
    let present = hubs.iter().any(|h| h.depth == old.depth);
    for h in &hubs {
        s.stock.wood += h.stock.wood;
        s.stock.stone += h.stock.stone;
        s.stock.copper += h.stock.copper;
    }
    // Stufen ohne Hub im Stand (die aktuelle und die noch unbesuchten Tiefen 0..2) sind unverändert und kommen leer dazu,
    // sonst säße ein alter Stand auf seinen Stufen fest (`moveToStage` findet kein Ziel).
    for d in 0..=2 {
        if d != old.depth && !has_hub(&hubs, d) {
            hubs.push(HubSave {
                depth: d,
                ..HubSave::default()
            });
        }
    }
    if !present {
        hubs.push(HubSave {
            depth: old.depth,
            ..HubSave::default()
        });
    }
    hubs.sort_by_key(|h| h.depth);
    s.stages = hubs;
    s.players = old
        .players
        .iter()
        .enumerate()
        .map(|(i, p)| IslandPlayerSave {
            index: i as i64,
            gold: p.gold,
            depth: old.depth,
            ..IslandPlayerSave::default()
        })
        .collect();
    s
}

/// Go: `hasHub`.
pub fn has_hub(hubs: &[HubSave], depth: i64) -> bool {
    hubs.iter().any(|h| h.depth == depth)
}
