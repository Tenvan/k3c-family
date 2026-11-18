//! Test-Hilfen (Port der Hilfen aus `engine/level/level_test.go`).

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::OnceLock;

use k3c_data::{Data, Source, load_all};
use k3c_det::golden;

use crate::biome::Biome;
use crate::rules::Rules;
use crate::types::Layout;

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

/// Go: `Generate(b, seed)` für die Tests; läuft ab RU6.5.
#[allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7, RU6.5 ersetzt ihn
pub(crate) fn generate(_b: &Biome, _seed: &str) -> Layout {
    unimplemented!("RU6.5")
}
