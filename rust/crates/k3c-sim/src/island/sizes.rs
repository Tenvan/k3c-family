//! Inselgrößen (Port von `engine/sim/island_sizes.go`, docs/rules/geplante-chunks.md § 7, B-485): Die Chunk-Anzahl einer
//! Stufe steht je Insel und Tiefe in der Welt-Datei (`chunkCounts`); erlaubt sind nur die Anzahlen der Tabelle
//! data/island-sizes.json (für alle Biome). Laden und Prüfen der Tabelle liegt seit RU3 in `k3c_data::island_sizes`; Gos
//! Paket-Variable `islandSizes` ist hier der Parameter `&IslandSizesFile`. `count_at` steht in `data.rs`.

use k3c_data::island_sizes::{IslandSize, IslandSizesFile};

use super::IslandDef;
use super::data::depth_key;

/// Go: `IslandSizes` liefert eine Kopie der Inselgrößen in der Reihenfolge der Daten (für Editor und Vorschau).
pub fn island_sizes(sizes: &IslandSizesFile) -> Vec<IslandSize> {
    sizes.sizes.clone()
}

/// Go: `CheckChunkCount` lehnt eine Chunk-Anzahl ab, die keine Inselgröße ist; der Fehler nennt die gültigen Werte. 0
/// (Länge des Bioms) ist keine Inselgröße, die Aufrufer lassen 0 vorher durch.
pub fn check_chunk_count(sizes: &IslandSizesFile, n: i64) -> Result<(), String> {
    if sizes.sizes.iter().any(|s| s.chunk_count == n) {
        return Ok(());
    }
    let valid: Vec<String> = sizes.sizes.iter().map(|s| s.chunk_count.to_string()).collect();
    Err(format!(
        "die Chunk-Anzahl {n} ist keine Inselgröße (gültig: {})",
        valid.join(", ")
    ))
}

/// Go: `checkChunkCounts` prüft `chunkCounts` einer Insel: Schlüssel ist eine Tiefe der Insel, Wert eine Inselgröße. Fehler
/// nennen Tiefe und die gültigen Werte (der Aufrufer ergänzt die Insel). Die Schlüssel laufen sortiert (`BTreeMap`).
pub fn check_chunk_counts(d: &IslandDef, sizes: &IslandSizesFile) -> Result<(), String> {
    for (key, n) in &d.chunk_counts {
        let Some(depth) = depth_key(key, &d.depths) else {
            return Err(format!("chunkCounts: Tiefe {key:?} ist keine Tiefe der Insel"));
        };
        check_chunk_count(sizes, *n).map_err(|e| format!("chunkCounts Tiefe {depth}: {e}"))?;
    }
    Ok(())
}
