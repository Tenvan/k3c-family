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
use crate::rules::Rules;
use crate::types::{Layout, NatureValues};

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
    RULES.get_or_init(|| Rules::from_data(data()).unwrap_or_else(|e| panic!("Rules: {e}")))
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

/// Go: `Options` der Tests: fest gesetzte Natur-Werte und der Schalter `Lair` (RU6.3 liefert `Options`).
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // RU6.5: generate_with liest die Felder
pub(crate) struct TestOptions {
    pub nature: NatureValues,
    pub lair: Option<bool>,
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
pub(crate) fn generate_with(_b: &Biome, _seed: &str, _o: &TestOptions) -> Result<Layout, String> {
    unimplemented!("RU6.5")
}

/// Go: `Validate(l, b)`; läuft ab RU6.5 (RU6.4 liefert `validate`).
pub(crate) fn validate(_l: &Layout, _b: &Biome) -> Vec<String> {
    unimplemented!("RU6.5")
}

/// Go: `ValidateWith(l, b, o)`; läuft ab RU6.5.
pub(crate) fn validate_with(_l: &Layout, _b: &Biome, _o: &TestOptions) -> Vec<String> {
    unimplemented!("RU6.5")
}

/// Go: `countEntities` (RU6.4 liefert die Funktion in `validate.rs`).
pub(crate) fn count_entities(l: &Layout, kind: &str) -> usize {
    l.entities.iter().filter(|e| e.kind == kind).count()
}

/// Go: `checkCamp` aus `camps_test.go`; läuft ab RU6.5.
pub(crate) fn check_camp(_l: &Layout, _x: f64, _id: &str, _seed: i64) {
    unimplemented!("RU6.5")
}

/// Go: `campBehindPortal` aus `camps_test.go`; läuft ab RU6.5.
pub(crate) fn camp_behind_portal(_l: &Layout, _id: &str, _seed: i64) -> bool {
    unimplemented!("RU6.5")
}
