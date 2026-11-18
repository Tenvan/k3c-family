//! Port von `native/k3c-game/game/assets.go`: die Grafik-Ordner, die das Spiel laden kann (`--assets`, Paket für den
//! Mini-PC, NG9.2): je Level der Biome aus data/ die Assets aller Asset-Quellen (ohne Welt: die Obermenge, die ein
//! Level brauchen kann), sortiert und ohne Doppelte.

use k3c_data::source::{EMBEDDED, Source};
use k3c_sim::state::World;

use crate::view::roles::AssetSource;

/// Die Level-IDs aller Biome (`data/biomes/<id>.json`) der Datenquelle des Programms, sortiert (Go: `Levels`).
pub fn levels() -> Result<Vec<String>, String> {
    levels_in(&Source::from_env())
}

/// Wie `levels`, aus einer gegebenen Quelle (Ordner oder eingebettete Dateien).
pub fn levels_in(source: &Source) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = match source {
        Source::Embedded => EMBEDDED
            .iter()
            .filter_map(|(n, _)| n.strip_prefix("biomes/").and_then(biome_id))
            .collect(),
        Source::Dir(dir) => {
            let rd = std::fs::read_dir(dir.join("biomes")).map_err(|e| format!("{}: {e}", dir.display()))?;
            rd.filter_map(|e| biome_id(&e.ok()?.file_name().to_string_lossy()))
                .collect()
        }
    };
    out.sort_unstable();
    Ok(out)
}

/// `cave` aus `cave.json` (Datei in `data/biomes/`); sonst `None`.
fn biome_id(file: &str) -> Option<String> {
    if file.contains('/') {
        return None;
    }
    file.strip_suffix(".json").map(str::to_string)
}

/// Die Assets `gruppe/id` (Ordner unter `public/grafik`) aller Level für die Asset-Quellen `sources` (Go: `AssetDirs`).
pub fn asset_dirs(sources: &[Box<dyn AssetSource>]) -> Result<Vec<String>, String> {
    asset_dirs_in(&levels()?, sources)
}

/// Wie `asset_dirs` für die gegebenen Level.
pub fn asset_dirs_in(levels: &[String], sources: &[Box<dyn AssetSource>]) -> Result<Vec<String>, String> {
    let none = World::default();
    let mut out: Vec<String> = Vec::new();
    for lv in levels {
        for src in sources {
            out.extend(src.assets(lv, &none));
        }
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

#[cfg(test)]
#[path = "assets_tests.rs"]
mod assets_tests;
