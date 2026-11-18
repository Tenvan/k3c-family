//! Test-Hilfen (Port der Hilfen aus `engine/level/*_test.go`).
//!
//! Die Platzhalter unten (`unimplemented!`, Leitfaden § 7) ersetzt RU6.5 durch die echten Aufrufe.
#![allow(clippy::unimplemented)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::OnceLock;

use k3c_data::{Data, Source, load_all};
use k3c_det::golden;

use crate::biome::Biome;
use crate::planned::validate_plan;
use crate::planned_gen::Options;
use crate::rules::Rules;
use crate::types::{Layout, NatureValues, PlannedBuilding, PlannedChunk, PlannedObject};

/// Der Ordner `data/` des Repos.
pub(crate) fn data_dir() -> PathBuf {
    golden::testdata().join("../data")
}

/// Alle Spiel-Daten aus `data/`, einmal geladen.
pub(crate) fn data() -> &'static Data {
    static DATA: OnceLock<Data> = OnceLock::new();
    DATA.get_or_init(|| load_all(&Source::Dir(data_dir())).unwrap_or_else(|e| panic!("data/: {e}")))
}

/// Der Regelsatz aus `data/`, einmal gebaut (Go: die Paket-Variablen).
pub(crate) fn rules() -> &'static Rules {
    static RULES: OnceLock<Rules> = OnceLock::new();
    RULES.get_or_init(|| {
        let hub = std::fs::read_to_string(data_dir().join("hub.json")).unwrap_or_else(|e| panic!("hub.json: {e}"));
        Rules::from_data(data(), &hub).unwrap_or_else(|e| panic!("Rules: {e}"))
    })
}

/// Go: `biomeIDs`, alle Biome aus `data/biomes/` (nach Namen sortiert).
pub(crate) fn biome_ids() -> Vec<String> {
    let ids: BTreeSet<String> = data().biomes.iter().map(|b| b.id.clone()).collect();
    assert!(!ids.is_empty(), "keine Biome in data/biomes/");
    ids.into_iter().collect()
}

/// Go: `biome(t, id)`, lädt ein Biom (eine Kopie, der Test darf sie ändern).
pub(crate) fn biome(id: &str) -> Biome {
    match data().biomes.iter().find(|b| b.id == id) {
        Some(b) => b.clone(),
        None => panic!("Biom {id} fehlt in data/biomes/"),
    }
}

/// Go: `Generate(b, seed)`; läuft ab RU6.5.
pub(crate) fn generate(_b: &Biome, _seed: &str) -> Layout {
    unimplemented!("RU6.5")
}

/// Go: der Test-Helfer `generate(t, b, seed, fixed)` aus `nature_test.go`; läuft ab RU6.5.
pub(crate) fn generate_nature(_b: &Biome, _seed: &str, _fixed: &NatureValues) -> Layout {
    unimplemented!("RU6.5")
}

/// Go: `GenerateWith(b, seed, o)`; läuft ab RU6.5.
pub(crate) fn generate_with(_b: &Biome, _seed: &str, _o: &Options) -> Result<Layout, String> {
    unimplemented!("RU6.5")
}

/// Go: `Validate(l, b)` mit dem Regelsatz aus `data/`.
pub(crate) fn validate(l: &Layout, b: &Biome) -> Vec<String> {
    crate::validate::validate(l, b, rules())
}

/// Go: `ValidateWith(l, b, o)` mit dem Regelsatz aus `data/`.
pub(crate) fn validate_with(l: &Layout, b: &Biome, o: &Options) -> Vec<String> {
    crate::validate::validate_with(l, b, o, rules())
}

pub(crate) use crate::validate::count_entities;

/// Go: `checkCamp` aus `camps_test.go`; läuft ab RU6.5.
pub(crate) fn check_camp(_l: &Layout, _x: f64, _id: &str, _seed: i64) {
    unimplemented!("RU6.5")
}

/// Go: `campBehindPortal` aus `camps_test.go`; läuft ab RU6.5.
pub(crate) fn camp_behind_portal(_l: &Layout, _id: &str, _seed: i64) -> bool {
    unimplemented!("RU6.5")
}

/// Go: der Test-Helfer `gen2` aus `planned_gen_test.go`, ein Level mit Planung; läuft ab RU6.5.
pub(crate) fn gen2(b: &Biome, seed: &str, plan: &[PlannedChunk]) -> Layout {
    let o = Options {
        planned: plan.to_vec(),
        ..Options::default()
    };
    generate_with(b, seed, &o).unwrap_or_else(|e| panic!("Biom {}, Seed {seed}: {e}", b.id))
}

/// Go: `plannedCase`, die Test-Planung unter `testdata/planned/` (keine echte Planung).
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(default)]
pub(crate) struct PlannedCase {
    pub biome: String,
    pub seeds: Vec<String>,
    pub planned: Vec<PlannedChunk>,
}

/// Go: `p` aus `planned_test.go`, ein geplanter Chunk mit Objekten.
pub(crate) fn p(slot: i64, kind: &str, objects: &[PlannedObject]) -> PlannedChunk {
    PlannedChunk {
        slot,
        kind: kind.into(),
        objects: objects.to_vec(),
        ..PlannedChunk::default()
    }
}

/// Go: `obj`, ein festes Objekt.
pub(crate) fn obj(kind: &str, at: f64) -> PlannedObject {
    PlannedObject { kind: kind.into(), at }
}

/// Go: `plain`, leere Chunks einer Art auf den Plätzen `slots`.
pub(crate) fn plain(kind: &str, slots: &[i64]) -> Vec<PlannedChunk> {
    slots.iter().map(|&s| p(s, kind, &[])).collect()
}

/// Go: `ValidatePlan(b, plan)` mit dem Regelsatz aus `data/`; der Fehler als Text.
pub(crate) fn plan_ok(b: &Biome, plan: &[PlannedChunk]) -> Result<(), String> {
    validate_plan(b, plan, rules()).map_err(|e| e.to_string())
}

/// Go: `withRuin`, ein geplanter Chunk mit einer Ruine.
pub(crate) fn with_ruin(mut c: PlannedChunk, kind: &str, at: f64) -> PlannedChunk {
    c.buildings = vec![PlannedBuilding { kind: kind.into(), at }];
    c
}
