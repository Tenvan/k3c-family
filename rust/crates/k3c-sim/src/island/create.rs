//! Erzeugung einer Insel (Port von `engine/sim/island.go` ab `type CreateOptions`): `CreateOptions`, `create_island`,
//! `create_island_in`, `create_island_at` (Go: `createIsland`).
//!
//! Gos Paket-Tabellen (`biomes`, `islandSizes`, `difficulty`, `standardWorld`) kommen als Parameter: `&Data` und die
//! Standard-Welt als `&WorldDef`. Fehler sind `Result<_, String>` mit dem Meldungstext von Go.

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::Data;
use k3c_data::hub::HubData;
use k3c_level::types::{NatureValues, PlannedChunk};

use crate::state::links::{validate_hub_data_in, values_of};
use crate::world::{Options as WorldOptions, create_world, empty_site, hub_of};

use super::Island;
use super::data::{IslandDef, biome_for_depth};
use super::endboss::lair_options;
use super::options::default_options;
use super::sizes::check_chunk_count;
use super::worlds::WorldDef;

/// Go: `CreateOptions`, die Parameter der Erzeugung einer Insel (B-461, B-462). Sie gelten nur für die Erzeugung und werden
/// nicht als Ganzes gespeichert; getrennt von `IslandOptions` (Grad, Ziel, Niederlage). `None` bei einer Map ist Go `nil`:
/// die Daten der ersten Insel der Welt; eine leere Map schaltet ab.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreateOptions {
    /// Geplante Chunks je Tiefe (docs/rules/geplante-chunks.md). `None` = die Planung der ersten Insel der Welt.
    pub planned: Option<BTreeMap<i64, Vec<PlannedChunk>>>,
    /// Kandidaten-hub.json (Gebäude-Planer, B-462); `None` = eingebettete data/hub.json. `create_island_in` prüft ihn vorher
    /// mit `validate_hub_data_in` und lehnt ihn bei Befunden ab (fail closed).
    pub candidate: Option<HubData>,
    /// Chunk-Anzahlen je Tiefe (Inselgrößen, geplante-chunks.md § 7). `None` = die der ersten Insel der Welt; ohne Eintrag
    /// gilt die Länge des Bioms.
    pub chunk_counts: Option<BTreeMap<i64, i64>>,
    /// Fest gesetzte Natur-Werte je Tiefe (B-891). `None` = die der ersten Insel der Welt; ohne Eintrag würfelt der
    /// Generator alle Werte aus dem Seed.
    pub nature: Option<BTreeMap<i64, NatureValues>>,
}

impl CreateOptions {
    /// Go: `plannedFor` liefert die geplanten Chunks der Tiefe; ohne `planned` die der Insel `def`.
    pub fn planned_for(&self, depth: i64, def: &IslandDef) -> Vec<PlannedChunk> {
        match &self.planned {
            Some(m) => m.get(&depth).cloned().unwrap_or_default(),
            None => def.planned_at(depth),
        }
    }

    /// Go: `countFor` liefert die Chunk-Anzahl der Tiefe (0 = Länge des Bioms); ohne `chunk_counts` die der Insel `def`.
    pub fn count_for(&self, depth: i64, def: &IslandDef) -> i64 {
        match &self.chunk_counts {
            Some(m) => m.get(&depth).copied().unwrap_or(0),
            None => def.count_at(depth),
        }
    }
}

impl IslandDef {
    /// Go: `IslandDef.CreateOptions` liefert die Erzeugungs-Parameter dieser Insel aus den Daten (ihre eigene Planung und
    /// Chunk-Anzahl).
    pub fn create_options(&self, biomes: &[k3c_data::biome::Biome]) -> CreateOptions {
        let (mut planned, mut counts, mut nature) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
        for &depth in &self.depths {
            let v = self.nature_at(biomes, depth);
            if !v.is_empty() {
                nature.insert(depth, v);
            }
            let plan = self.planned_at(depth);
            if !plan.is_empty() {
                planned.insert(depth, plan);
            }
            let n = self.count_at(depth);
            if n > 0 {
                counts.insert(depth, n);
            }
        }
        CreateOptions {
            planned: Some(planned),
            chunk_counts: Some(counts),
            nature: Some(nature),
            candidate: None,
        }
    }
}

/// Die Tabellen und die Welt einer Erzeugung (Go: Paket-Variablen und die Parameter `world`, `first` von `createIsland`).
#[derive(Debug, Clone)]
pub struct IslandBuild<'a> {
    pub data: &'a Data,
    pub world: &'a WorldDef,
    /// Die Insel, die zuerst gebaut wird (Go: `world.Islands[0]`; Laden und Inselwechsel nehmen eine andere).
    pub first: &'a IslandDef,
    /// Die geprüfte Kopie der Kandidaten-hub.json; `None` = die hub.json des Wertesatzes.
    pub hub: Option<Arc<HubData>>,
}

/// Go: `CreateIsland` baut die erste Insel der Standard-Welt (`create_island_in`).
pub fn create_island(
    data: &Data,
    standard_world: &WorldDef,
    seed: &str,
    depths: &[i64],
    cycle_speed: f64,
    opts: CreateOptions,
) -> Result<Island, String> {
    create_island_in(data, standard_world, seed, depths, cycle_speed, opts)
}

/// Go: `CreateIslandIn` baut die erste Insel der Welt `world` mit den Stufen für die angegebenen Tiefen (Reihenfolge =
/// Stufenindex). `cycle_speed` 0 bedeutet 1. Eine neue Insel startet mit dem Vorrat aus hub.json › islandStartStock;
/// Spielstände setzen ihren eigenen (`from_island_save_in`).
pub fn create_island_in(
    data: &Data,
    world: &WorldDef,
    seed: &str,
    depths: &[i64],
    cycle_speed: f64,
    mut opts: CreateOptions,
) -> Result<Island, String> {
    let mut hub = None;
    if let Some(candidate) = opts.candidate.take() {
        if let Some(finding) = validate_hub_data_in(&candidate, world).findings.first() {
            return Err(format!("insel: Kandidaten-hub.json abgelehnt: {finding}"));
        }
        hub = Some(Arc::new(candidate)); // geprüft wird die Kopie, die die Welt behält: spätere Änderungen des Aufrufers wirken nicht
    }
    let Some(first) = world.islands.first() else {
        // Go: Index außerhalb (`world.Islands[0]`); hier dieselbe Meldung wie in `create_island_at`.
        return Err(format!("insel: Welt {:?} ohne Insel", world.id));
    };
    let build = IslandBuild {
        data,
        world,
        first,
        hub,
    };
    let mut isl = create_island_at(&build, seed, depths, cycle_speed, 0.0, &opts)?;
    isl.stock = hub_of(&isl.stages[0]).island_start_stock.clone(); // Kandidat oder hub.json der Welt
    isl.sync_stock();
    Ok(isl)
}

/// Go: `createIsland`, wie `create_island_in`, mit Startzeit (Laden eines Spielstands). Die Insel ist zunächst die erste
/// der Welt; Laden und Inselwechsel setzen danach `number`, `island_id` und `scaling`.
pub fn create_island_at(
    b: &IslandBuild,
    seed: &str,
    depths: &[i64],
    cycle_speed: f64,
    start_time: f64,
    opts: &CreateOptions,
) -> Result<Island, String> {
    if depths.is_empty() {
        return Err("insel: keine Stufen".to_string());
    }
    if b.world.islands.is_empty() {
        return Err(format!("insel: Welt {:?} ohne Insel", b.world.id));
    }
    let vals = values_of(b.world).map_err(|e| format!("insel: {e}"))?;
    let mut world = b.world.clone();
    world.values = Some(Arc::clone(&vals)); // Inselwechsel (NextIsland) übernimmt den Wertesatz
    let mut isl = Island {
        id: seed.to_string(),
        seed: seed.to_string(),
        cycle_speed,
        options: default_options(&b.data.difficulty),
        island_id: b.first.id.clone(),
        scaling: b.first.depth_scaling.clone(),
        world,
        ..Island::default()
    };
    for &d in depths {
        let w = create_stage(b, d, seed, cycle_speed, start_time, opts, &vals)?;
        isl.stages.push(w);
    }
    if crate::cargo::world_has_cargo(b.data, &b.world.id) {
        isl.enable_cargo(b.data); // RU62.1: Fracht nur in Welten aus data/cargo.json
    }
    Ok(isl)
}

/// Eine Stufe der Tiefe `d` (Schleifenkörper von `createIsland`).
fn create_stage(
    b: &IslandBuild,
    d: i64,
    seed: &str,
    cycle_speed: f64,
    start_time: f64,
    opts: &CreateOptions,
    vals: &Arc<crate::state::WorldValues>,
) -> Result<crate::state::World, String> {
    let data = b.data;
    let Some(biome) = biome_for_depth(&data.biomes, d) else {
        return Err(format!("insel: unbekannte Tiefe {d}")); // Go: `!hasDepth(d)`
    };
    let count = opts.count_for(d, b.first);
    if count != 0 {
        check_chunk_count(&data.island_sizes, count).map_err(|e| format!("insel: Stufe mit Tiefe {d}: {e}"))?;
    }
    let (lair, boss) = lair_options(b.first, d, data);
    let wo = WorldOptions {
        cycle_speed,
        time: start_time,
        planned: opts.planned_for(d, b.first),
        chunk_count: count,
        nature: opts.nature_for(d, b.first, &data.biomes),
        lair,
        hub: b.hub.clone(),
        vals: Some(Arc::clone(vals)),
        rules: None,
    };
    let mut w = create_world(biome, seed, &wo).map_err(|e| format!("insel: Stufe mit Tiefe {d}: {e}"))?;
    w.lair_boss = boss;
    let island_sites = hub_of(&w).island_sites.clone();
    for s in &island_sites {
        let x = w.hub_x + s.offset_units;
        let site = empty_site(&mut w, &s.kind, x);
        w.sites.push(site);
    }
    w.no_travel = true;
    // Go: `w.Stock = isl.Stock`: Der Vorrat liegt in `Island::stock`, die Stufe bekommt ihn nur im Schritt (Entscheidung 5).
    Ok(w)
}
