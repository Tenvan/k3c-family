//! Port von `engine/enginetools/level.go` (RU33.1): erzeugt ein Level und beschreibt es als Text.

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_data::{Source, load_all};
use k3c_level::biome::Biome;
use k3c_level::level::generate;
use k3c_level::rules::Rules;
use k3c_level::types::Layout;
use k3c_level::validate::validate;

use super::RunError;
use super::gotext::{g, quote};

/// Die Biome, die `sim run` und `sim level` kennen (data/biomes/).
pub const BIOMES: [&str; 3] = ["forest", "cave", "mine"];

fn failed(msg: String) -> RunError {
    RunError::Failed(msg)
}

/// Go: `loadBiome`: prüft den Namen (leer = forest) und lädt das Biom aus der Quelle.
pub fn load_biome(src: &Source, id: &str) -> Result<Biome, RunError> {
    let id = if id.is_empty() { "forest" } else { id };
    if !BIOMES.contains(&id) {
        return Err(failed(format!(
            "unbekanntes Biom {} (bekannt: {})",
            quote(id),
            BIOMES.join(", ")
        )));
    }
    let data = load_all(src).map_err(|e| failed(e.to_string()))?;
    data.biomes
        .into_iter()
        .find(|b| b.id == id)
        .ok_or_else(|| failed(format!("biom {id}: nicht gefunden")))
}

/// Go: `Level`: Kopfzeile, eine Zeile je Abschnitt, Objekte nach Art, Warnungen der Prüfung.
pub fn level(src: &Source, biome_id: &str, seed: &str) -> Result<String, RunError> {
    let b = load_biome(src, biome_id)?;
    let data = load_all(src).map_err(|e| failed(e.to_string()))?;
    let hub = src.read("hub.json").map_err(|e| failed(e.to_string()))?;
    let rules = Arc::new(Rules::from_data(&data, &hub).map_err(|e| failed(e.to_string()))?);
    let l = generate(&b, seed, &rules).map_err(|e| failed(format!("level nicht erzeugbar: {e}")))?;
    let mut lines = vec![format!(
        "Level {} · Seed {} · Breite {} Units · Hub-Mitte {} · {} Abschnitte à {} Units",
        l.biome_id,
        quote(&l.seed),
        g(l.width_units),
        g(l.hub_center_units),
        l.chunks.len(),
        g(l.chunk_width_units)
    )];
    for c in &l.chunks {
        lines.push(format!("{:>3} {} @{}", c.index, c.kind, g(c.start_units)));
    }
    lines.extend(entity_lines(&l));
    let warn = validate(&l, &b, &rules);
    if !warn.is_empty() {
        lines.push(format!("Warnungen: {}", warn.join("; ")));
    }
    Ok(lines.join("\n"))
}

/// Go: `entityLines`: zählt die Objekte je Art (alphabetisch) und nennt die ersten Positionen.
fn entity_lines(l: &Layout) -> Vec<String> {
    let mut xs: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
    for e in &l.entities {
        xs.entry(e.kind.as_str()).or_default().push(e.x);
    }
    let mut lines = vec![format!("Objekte: {}", l.entities.len())];
    for (kind, pos) in &xs {
        let first: Vec<String> = pos.iter().take(3).map(|x| g(*x)).collect();
        lines.push(format!("  {kind} ×{} · erste x: {}", pos.len(), first.join(", ")));
    }
    lines
}
