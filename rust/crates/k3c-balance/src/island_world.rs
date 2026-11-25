//! Insel und Welt eines Szenarios (Port von `engine/balance/island_world.go`, BS11, B-642): ohne Welt eine Stufe der Tiefe,
//! mit Welt alle Tiefen der ersten Insel. Dazu die geteilten Daten, die Go als Paket-Variablen hält.

use std::path::Path;
use std::sync::{Arc, OnceLock};

use k3c_data::source::EMBEDDED;
use k3c_data::{Data, Source, load_all};
use k3c_level::rules::Rules;
use k3c_sim::island::{
    CreateOptions, Island, WorldDef, create_island as sim_create_island, create_island_in,
    load_world as sim_load_world, standard_world,
};
use k3c_sim::world::values_room::{resolve_world, values_of};

use crate::fehler::Fehler;
use crate::run::Scenario;
use crate::vary::current_data;

/// Die Daten, die Go als Paket-Variablen hält (`data.Files`, Regeln, Standard-Welt), einmal gelesen.
pub struct Shared {
    pub source: Source,
    pub data: Data,
    pub rules: Rules,
    pub standard: WorldDef,
}

static SHARED: OnceLock<Result<Shared, Fehler>> = OnceLock::new();

fn load_shared() -> Result<Shared, Fehler> {
    load_shared_from(Source::from_env())
}

/// Liest die geteilten Daten aus `source` (Go: `sim.UseData(fsys)`); `vary::with_data` ruft es mit Daten im Speicher.
pub fn load_shared_from(source: Source) -> Result<Shared, Fehler> {
    let wrap = |e: String| Fehler::Daten(format!("balance: {e}"));
    let data = load_all(&source).map_err(|e| wrap(e.to_string()))?;
    let hub = source.read("hub.json").map_err(|e| wrap(e.to_string()))?;
    let rules = Rules::from_data(&data, &hub).map_err(|e| wrap(e.to_string()))?;
    let standard = standard_world(&data, &rules).map_err(wrap)?;
    Ok(Shared {
        source,
        data,
        rules,
        standard,
    })
}

/// Die geteilten Daten; der Fehler beim Lesen bleibt für alle Aufrufer derselbe. Innerhalb von `vary::with_data` (auf diesem
/// Thread und in den Läufen, die er startet) sind es die Daten der Variante.
pub fn shared() -> Result<&'static Shared, Fehler> {
    match current_data() {
        Some(ctx) => Ok(ctx.shared),
        None => base_shared(),
    }
}

/// Die geteilten Daten von `data/` ohne Varianten (Go: `data.Files`, die `with_data` nie ersetzt).
pub fn base_shared() -> Result<&'static Shared, Fehler> {
    SHARED.get_or_init(load_shared).as_ref().map_err(Clone::clone)
}

/// Lädt eine Welt (Go: `loadWorld`); `standard` ist die eingebettete Standard-Welt.
pub fn load_world(id: &str) -> Result<WorldDef, Fehler> {
    let sh = shared()?;
    if id == "standard" {
        return Ok(sh.standard.clone());
    }
    sim_load_world(&sh.source, id, &sh.data, &sh.rules).map_err(Fehler::Sim)
}

/// Baut die Insel des Szenarios: ohne Welt eine Stufe der Tiefe, mit Welt alle Tiefen der ersten Insel (Go: `createIsland`).
pub fn create_island(sc: &Scenario) -> Result<Island, Fehler> {
    let sh = shared()?;
    if sc.world.is_empty() {
        let world = legacy_if(&standard_of(sh)?, sc.legacy)?;
        return sim_create_island(&sh.data, &world, &sc.seed, &[sc.depth], 1.0, CreateOptions::default())
            .map_err(Fehler::Sim);
    }
    let world = load_world(&sc.world)?;
    if world.islands.is_empty() {
        return Err(Fehler::Sim(format!("welt {:?} hat keine Insel", sc.world)));
    }
    create_in_with(&world, &sc.seed, sc.legacy)
}

/// Die Standard-Welt der geteilten Daten. Innerhalb von `vary::with_data` bekommt sie den Wertesatz aus den Daten der
/// Variante; sonst löst `k3c-sim` ihn beim Anlegen aus den eingebetteten Daten auf (Go: `valuesOf` liest die Paket-Variablen,
/// die `UseData` ersetzt).
fn standard_of(sh: &Shared) -> Result<WorldDef, Fehler> {
    if current_data().is_none() {
        return Ok(sh.standard.clone());
    }
    let lookup = |id: &str| load_world(id).map_err(|e| e.to_string());
    resolve_world(&sh.standard, &sh.source, &sh.data, &sh.rules, &lookup).map_err(Fehler::Sim)
}

/// Löst die Welt samt `extends`-Kette und Overrides auf (Eltern aus data/worlds) und baut ihre erste Insel; ein Fehler der
/// Kette nennt die Welt (welten.md § 1.2; Go: `createIn`).
pub fn create_in(world: &WorldDef, seed: &str) -> Result<Island, Fehler> {
    create_in_with(world, seed, false)
}

/// Wie `create_in`; `legacy` rechnet mit den Regeln des Go-Orakels (siehe `legacy_if`).
fn create_in_with(world: &WorldDef, seed: &str, legacy: bool) -> Result<Island, Fehler> {
    let sh = shared()?;
    let lookup = |id: &str| load_world(id).map_err(|e| e.to_string());
    let world = resolve_world(world, &sh.source, &sh.data, &sh.rules, &lookup).map_err(Fehler::Sim)?;
    let first = world
        .islands
        .first()
        .ok_or_else(|| Fehler::Sim(format!("welt {:?} hat keine Insel", world.id)))?;
    let opts = first.create_options(&sh.data.biomes);
    let world = legacy_if(&world, legacy)?;
    create_island_in(&sh.data, &world, seed, &first.depths, 1.0, opts).map_err(Fehler::Sim)
}

/// Mit `legacy` rechnet die Welt ohne die nativen Regeln des Rust-Spiels (`WorldValues::native_rules`: Hafenstart,
/// Wächter-Bäume, Besorgung, Rüstung, …); nur so lässt sich ein Lauf mit Go vergleichen (`rustref_tests.rs`).
fn legacy_if(world: &WorldDef, legacy: bool) -> Result<WorldDef, Fehler> {
    let mut world = world.clone();
    if legacy {
        let values = match &world.values {
            Some(v) => Arc::clone(v),
            None => values_of(&world).map_err(Fehler::Sim)?,
        };
        let mut v = (*values).clone();
        v.native_rules = false;
        world.values = Some(Arc::new(v));
    }
    Ok(world)
}

/// Die IDs der Welt-Dateien in data/worlds (sortiert), die ein Lauf als Welt nehmen darf (Go: `Worlds`).
pub fn worlds() -> Result<Vec<String>, Fehler> {
    let mut ids: Vec<String> = match &shared()?.source {
        Source::Embedded => EMBEDDED
            .iter()
            .filter_map(|(n, _)| n.strip_prefix("worlds/"))
            .filter(|n| !n.contains('/'))
            .filter_map(|n| n.strip_suffix(".json").map(str::to_string))
            .collect(),
        Source::Dir(dir) => dir_worlds(&dir.join("worlds"))?,
        Source::Mem(files) => files
            .keys()
            .filter_map(|n| n.strip_prefix("worlds/"))
            .filter(|n| !n.contains('/'))
            .filter_map(|n| n.strip_suffix(".json").map(str::to_string))
            .collect(),
    };
    ids.sort();
    Ok(ids)
}

fn dir_worlds(dir: &Path) -> Result<Vec<String>, Fehler> {
    let entries = std::fs::read_dir(dir).map_err(|e| Fehler::Daten(format!("balance: {e}")))?;
    let mut ids = Vec::new();
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let is_file = e.file_type().is_ok_and(|t| !t.is_dir());
        if let Some(id) = name.strip_suffix(".json").filter(|_| is_file) {
            ids.push(id.to_string());
        }
    }
    Ok(ids)
}
