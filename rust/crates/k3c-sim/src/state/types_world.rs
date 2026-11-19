//! Zustand der Simulation, Teil 4: die Welt (Port von `engine/sim/types.go`, `World`).
//!
//! Die Welt besitzt alle Dinge als `Vec` in Entstehungs-Reihenfolge; Verweise sind IDs (Vorentscheidungen 1 und 2).
//! Rückverweise gibt es nicht: `island` (Go) fehlt, Funktionen bekommen die Insel als Parameter (Vorentscheidung 4).

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::bosses::BossData;
use k3c_data::hub::{HubData, Stock};
use k3c_level::types::{Biome, Layout, PlannedChunk};
use k3c_rng::Rng;
use serde::{Deserialize, Serialize};

use super::omit::is_zero_i64;
use super::shared::{ActiveEvent, CombatRuntime, CycleInfo, ItemDrop, Merchant, SwitchState, WorldValues};
use super::types::{Coin, Player, Troop};
use super::types_enemy::{Camp, Enemy, Event, Hazard, Pickup, Projectile, QueuedSpawn, Storm, Travel};
use super::types_things::{Castle, ResourceNode, Site};

/// Der vollständige Zustand einer Stufe.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct World {
    /// Seed der Stufe.
    #[serde(rename = "seed")]
    pub seed: String,
    /// Biom der Stufe; ohne JSON.
    #[serde(skip)]
    pub biome: Biome,
    /// Level der Stufe; ohne JSON.
    #[serde(skip)]
    pub level: Layout,
    /// Würfel der Stufe (Vorentscheidung 11); ohne JSON.
    #[serde(skip)]
    pub rng: Rng,
    /// Simulierte Sekunden seit Start.
    #[serde(rename = "time")]
    pub time: f64,
    /// Faktor für den Tag/Nacht-Zyklus.
    #[serde(rename = "cycleSpeed")]
    pub cycle_speed: f64,
    /// Nächste ID; `new_id` zählt wie Go.
    #[serde(rename = "nextId")]
    pub next_id: i64,
    #[serde(rename = "widthUnits")]
    pub width_units: f64,
    #[serde(rename = "hubX")]
    pub hub_x: f64,
    /// Hub-Stufe (Start 1); ohne JSON.
    #[serde(skip)]
    pub hub_level: i64,
    /// Laufender Ausbauschritt auf die nächste Hub-Stufe; ohne JSON.
    #[serde(skip)]
    pub hub_step: i64,
    /// Geplante Chunks, mit denen die Stufe gebaut wurde.
    #[serde(skip)]
    pub planned: Vec<PlannedChunk>,
    /// Chunk-Anzahl der Erzeugung.
    #[serde(skip)]
    pub chunk_count: i64,
    /// Rüstungsstufe der Kämpfer dieses Hubs (0 = keine).
    #[serde(rename = "armorLevel", skip_serializing_if = "is_zero_i64")]
    pub armor_level: i64,
    /// Zahlplatz für den Hub-Ausbau an der Burg (nicht in `sites`).
    #[serde(skip)]
    pub hub_site: Option<Site>,
    /// Kandidaten-hub.json, aus der die Stufe gebaut ist; `None` = eingebettete.
    #[serde(skip)]
    pub hub_data: Option<Arc<HubData>>,
    /// Wertesatz der Welt dieser Stufe (Vorentscheidung 6), nie je Tick neu.
    #[serde(skip)]
    pub vals: Option<Arc<WorldValues>>,
    #[serde(rename = "cycle")]
    pub cycle: CycleInfo,
    /// Nur unter Tage (0..100), sonst `None` (null).
    #[serde(rename = "aggression")]
    pub aggression: Option<f64>,
    #[serde(rename = "wave")]
    pub wave: i64,
    #[serde(rename = "players")]
    pub players: Vec<Player>,
    #[serde(rename = "coins")]
    pub coins: Vec<Coin>,
    #[serde(rename = "troops")]
    pub troops: Vec<Troop>,
    #[serde(rename = "nodes")]
    pub nodes: Vec<ResourceNode>,
    #[serde(rename = "sites")]
    pub sites: Vec<Site>,
    #[serde(rename = "castle")]
    pub castle: Castle,
    #[serde(rename = "enemies")]
    pub enemies: Vec<Enemy>,
    #[serde(rename = "projectiles")]
    pub projectiles: Vec<Projectile>,
    #[serde(rename = "pickups")]
    pub pickups: Vec<Pickup>,
    #[serde(rename = "camps")]
    pub camps: Vec<Camp>,
    #[serde(rename = "portals")]
    pub portals: Vec<f64>,
    /// Lebenspunkte je stehendem Portal (gleicher Index wie `portals`).
    #[serde(rename = "portalHp")]
    pub portal_hp: Vec<f64>,
    /// Startwert der Portal-Lebenspunkte.
    #[serde(rename = "portalMaxHp")]
    pub portal_max_hp: f64,
    /// X der zerstörten Portale; sind alle weg, ist die Stufe sicher.
    #[serde(rename = "portalsDown")]
    pub portals_down: Vec<f64>,
    /// Sekunden bis zur nächsten Verteidiger-Welle je Portal; ohne JSON.
    #[serde(skip)]
    pub portal_defense: Vec<f64>,
    /// Bezahlte Schilde, Auslöser, Banner; ohne JSON.
    #[serde(skip)]
    pub combat: CombatRuntime,
    #[serde(rename = "spawnQueue")]
    pub spawn_queue: Vec<QueuedSpawn>,
    /// Laufende Lightning Storms in Wirk-Reihenfolge.
    #[serde(rename = "storms", skip_serializing_if = "Vec::is_empty")]
    pub storms: Vec<Storm>,
    /// Bodenflächen in Entstehungs-Reihenfolge.
    #[serde(rename = "hazards", skip_serializing_if = "Vec::is_empty")]
    pub hazards: Vec<Hazard>,
    /// Baumaterial gehört allen (in einer Insel: allen Stufen, Vorentscheidung 5), Gold hat jeder Spieler selbst.
    #[serde(rename = "stock")]
    pub stock: Stock,
    #[serde(rename = "skillPoints")]
    pub skill_points: i64,
    #[serde(rename = "travel")]
    pub travel: Option<Travel>,
    /// Händler, nur in Tiefe 0.
    #[serde(rename = "merchant", skip_serializing_if = "Option::is_none")]
    pub merchant: Option<Merchant>,
    /// Ausrüstung am Boden.
    #[serde(rename = "drops", skip_serializing_if = "Vec::is_empty")]
    pub drops: Vec<ItemDrop>,
    /// Wird bei jedem Step geleert (Vorentscheidung 12).
    #[serde(rename = "events")]
    pub events: Vec<Event>,
    /// Spiegel für das Protokoll; fehlt, wenn es sie nicht gibt.
    #[serde(rename = "nightEvents", skip_serializing_if = "Vec::is_empty")]
    pub night_events: Vec<ActiveEvent>,
    #[serde(rename = "islandSwitch", skip_serializing_if = "Option::is_none")]
    pub island_switch: Option<SwitchState>,
    /// Fracht des Schiffs (RU62.1, B-967); fehlt auf Inseln ohne Fracht.
    #[serde(rename = "cargo", skip_serializing_if = "Option::is_none")]
    pub cargo: Option<super::cargo_view::CargoState>,
    /// Nutzbare Ladeplätze dieser Stufe (RU62.1); ohne JSON.
    #[serde(skip)]
    pub cargo_bays: Vec<super::cargo_view::CargoBay>,
    /// Der Wertesatz der Stufe trägt den Abschnitt Laderaum als 4. Schiffsstufe (RU62.1); ohne JSON.
    #[serde(skip)]
    pub cargo_hold: bool,
    /// Wirksame Doppel-Truhen der Stufe (RU62.3, B-968); fehlt ohne Koop-Aktion.
    #[serde(rename = "coop", skip_serializing_if = "super::coop_view::CoopState::is_empty")]
    pub coop: super::coop_view::CoopState,
    /// Die Koop-Regeln der Welt (RU62.3); `None` = Welt ohne Koop-Aktionen; ohne JSON.
    #[serde(skip)]
    pub coop_cfg: Option<std::sync::Arc<k3c_data::coop::CoopFile>>,
    /// Mindestens zwei gesteuerte Monarchen auf der Insel (RU62.3, die Insel setzt es je Takt); ohne JSON.
    #[serde(skip)]
    pub coop_active: bool,
    /// Art und Phase des Wetters (RU62.5, B-969); fehlt, solange keine Wetternacht ansteht oder die Welt kein Wetter hat.
    #[serde(rename = "weather", skip_serializing_if = "Option::is_none")]
    pub weather: Option<super::weather_view::WeatherState>,
    /// Der Wetter-Plan der Insel (RU62.5); `None` = Welt ohne Wetter; ohne JSON.
    #[serde(skip)]
    pub weather_cfg: Option<std::sync::Arc<crate::weather::WeatherPlan>>,
    /// Tempo-Faktor des Wetters dieser Stufe (1.0 = keine Wirkung, Taifun an der Oberfläche < 1); je Takt gesetzt, ohne JSON.
    #[serde(skip)]
    pub weather_speed: f64,
    /// Reichweiten-Faktor der Fernkämpfer (1.0 = keine Wirkung); je Takt gesetzt, ohne JSON.
    #[serde(skip)]
    pub weather_range: f64,
    /// Gesperrte Bereiche der aktiven Flut (RU62.6), berechnet je Takt aus dem Wetter-Plan, nie im Spielstand; leer sonst.
    #[serde(rename = "floodZones", default, skip_serializing_if = "Vec::is_empty")]
    pub flood_zones: Vec<super::weather_view::FloodZone>,
    /// Verworfene Ereignisse dieses Steps (Obergrenze je Tick).
    #[serde(rename = "eventsDropped", skip_serializing_if = "is_zero_i64")]
    pub events_dropped: i64,
    /// Die Stufe gehört zu einer Insel, deren Spieler einzeln wechseln; der gemeinsame Stufenwechsel ist dort aus. Ohne JSON.
    #[serde(skip)]
    pub no_travel: bool,
    /// Sekunden bis zum nächsten Lava-Schaden; ohne JSON.
    #[serde(skip)]
    pub lava_in: f64,
    /// Der Endboss im Bau dieser Stufe; `None` = die Stufe trägt keinen Bau. Ohne JSON.
    #[serde(skip)]
    pub lair_boss: Option<BossData>,
    /// Spielzeit der letzten Welle; ohne JSON.
    #[serde(skip)]
    pub last_wave_at: f64,
    /// Letzte Nacht, in der die Burg fiel; nicht im Spielstand.
    #[serde(skip)]
    pub castle_fell_night: i64,
    /// Die Wachen der Planung sind aufgestellt (erster Tick); ohne JSON.
    #[serde(skip)]
    pub guards_spawned: bool,
    /// Geräumte Lager `kind@x` (Go: `map[string]bool`); ohne JSON.
    #[serde(skip)]
    pub guards_cleared: BTreeMap<String, bool>,
    /// Spiegel von `island.Dev.Pure` (Go: `w.pure()` liest `w.island`): reiner Stressmodus, gesetzt von `dev_set_pure`; ohne JSON.
    #[serde(skip)]
    pub pure: bool,
    /// Spiegel der Insel für die Ereignisse (Go: `w.island` in `eventInNight`, `eventStage`); `None` = Go `w.island == nil`
    /// (Kampagne, einzelne Welt). `Island::sync_stock` setzt ihn; ohne JSON.
    #[serde(skip)]
    pub island_events: Option<super::island_ref::IslandEvents>,
    /// Lesende Sicht der Insel für HUD und Anzeige (Go: `w.island` in `purse_max_of`, `economy_of`, `at_lair`; B-1125);
    /// `None` = Go `w.island == nil`. `Island::sync_stock` setzt sie, `island_ref_view` macht daraus einen `IslandRef`;
    /// ohne JSON.
    #[serde(skip)]
    pub island_view: Option<super::island_ref::IslandView>,
}

impl Default for World {
    #[allow(clippy::too_many_lines)] // eine Zeile je Feld der Stufe
    fn default() -> Self {
        World {
            seed: Default::default(),
            biome: Default::default(),
            level: Default::default(),
            rng: Rng::new(""),
            time: Default::default(),
            cycle_speed: Default::default(),
            next_id: Default::default(),
            width_units: Default::default(),
            hub_x: Default::default(),
            hub_level: Default::default(),
            hub_step: Default::default(),
            planned: Default::default(),
            chunk_count: Default::default(),
            armor_level: Default::default(),
            hub_site: Default::default(),
            hub_data: Default::default(),
            vals: Default::default(),
            cycle: Default::default(),
            aggression: Default::default(),
            wave: Default::default(),
            players: Default::default(),
            coins: Default::default(),
            troops: Default::default(),
            nodes: Default::default(),
            sites: Default::default(),
            castle: Default::default(),
            enemies: Default::default(),
            projectiles: Default::default(),
            pickups: Default::default(),
            camps: Default::default(),
            portals: Default::default(),
            portal_hp: Default::default(),
            portal_max_hp: Default::default(),
            portals_down: Default::default(),
            portal_defense: Default::default(),
            combat: Default::default(),
            spawn_queue: Default::default(),
            storms: Default::default(),
            hazards: Default::default(),
            stock: Default::default(),
            skill_points: Default::default(),
            travel: Default::default(),
            merchant: Default::default(),
            drops: Default::default(),
            events: Default::default(),
            night_events: Default::default(),
            island_switch: Default::default(),
            cargo: None,
            cargo_bays: Vec::new(),
            cargo_hold: false,
            coop: Default::default(),
            coop_cfg: None,
            coop_active: false,
            weather: None,
            weather_cfg: None,
            weather_speed: 1.0,
            weather_range: 1.0,
            flood_zones: Vec::new(),
            events_dropped: Default::default(),
            no_travel: Default::default(),
            lava_in: Default::default(),
            lair_boss: Default::default(),
            last_wave_at: Default::default(),
            castle_fell_night: Default::default(),
            guards_spawned: Default::default(),
            guards_cleared: Default::default(),
            pure: false,
            island_events: None,
            island_view: None,
        }
    }
}

impl World {
    /// Go: `newID`, vergibt die nächste ID.
    pub fn new_id(&mut self) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
