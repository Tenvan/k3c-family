//! data/island-sizes.json: Go `struct{ Sizes []sim.IslandSize }` (engine/sim/island_sizes.go, `loadIslandSizes`).

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `struct { Sizes []sim.IslandSize }`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandSizesFile {
    #[serde(rename = "Sizes", alias = "sizes")]
    pub sizes: Vec<IslandSize>,
}

/// Go: `sim.IslandSize`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct IslandSize {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "chunkCount")]
    pub chunk_count: i64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "island-sizes.json";

/// `load` liest island-sizes.json und prüft die Tabelle wie `loadIslandSizes`: nicht leer, Name gesetzt, Anzahl
/// gerade, größer 0 und eindeutig.
pub fn load(raw: &str) -> Result<IslandSizesFile, Fehler> {
    let v: IslandSizesFile = decode("data/island-sizes.json", raw)?;
    let fail = |s: String| Err(Fehler::Pruefung(format!("data/island-sizes.json: {s}")));
    if v.sizes.is_empty() {
        return fail("keine Inselgröße".into());
    }
    for (i, s) in v.sizes.iter().enumerate() {
        if s.name.is_empty() {
            return fail(format!("Größe {} ohne Namen", i + 1));
        }
        if s.chunk_count <= 0 || s.chunk_count % 2 != 0 {
            return fail(format!(
                "{}: Chunk-Anzahl {} ist nicht gerade und größer 0",
                s.name, s.chunk_count
            ));
        }
        if v.sizes[..i].iter().any(|o| o.chunk_count == s.chunk_count) {
            return fail(format!("Chunk-Anzahl {} doppelt", s.chunk_count));
        }
    }
    Ok(v)
}
