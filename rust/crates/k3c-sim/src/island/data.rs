//! Inseln als Daten (Port von `engine/sim/island_data.go`; Welt-Datei data/worlds/<id>.json, B-103, B-488): Liste in fester
//! Reihenfolge, je Insel ihre Tiefen und ihre Tabelle der Gegner-Skalierung. Welten ohne Insel skalieren nach
//! waves.json › depthScaling. Laden und strenges Lesen: `worlds.rs` (RU10.5).
//!
//! Gos Paket-Tabellen (`biomes`, `bosses`) gibt es in Rust nicht: Sie kommen als `&Data` (`k3c_data::load_all`).

use std::collections::BTreeMap;

use k3c_data::Data;
use k3c_data::biome::Biome;
use k3c_data::waves::WavesDataDepthScaling;
use k3c_level::lines::HubSite;
use k3c_level::planned::validate_plan_with;
use k3c_level::rules::Rules;
use k3c_level::types::{NatureValues, PlannedChunk};

use crate::state::data::has_depth;
use crate::state::pending::{IslandStory, check_chunk_counts, check_end_boss};

use super::nature::check_nature;

/// Go: `IslandDef`, eine Insel aus den Daten.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IslandDef {
    pub id: String,
    pub name: String,
    /// Etappe (>= 1): Inseln einer Etappe sind frei anfahrbar, die nächste öffnet nach allen (B-961); 0 = Listenplatz + 1.
    pub etappe: i64,
    pub depths: Vec<i64>,
    pub depth_scaling: WavesDataDepthScaling,
    /// Geplante Chunks je Tiefe (Schlüssel = Tiefe, docs/rules/geplante-chunks.md); leer = reiner Zufall.
    pub planned_chunks: BTreeMap<String, Vec<PlannedChunk>>,
    /// Chunk-Anzahlen je Tiefe (Schlüssel = Tiefe, Wert = Inselgröße, geplante-chunks.md § 7); ohne Eintrag gilt die
    /// Länge des Bioms.
    pub chunk_counts: BTreeMap<String, i64>,
    /// Vorgabe der Insel für die Natur-Werte (nature.rs, B-891).
    pub nature: NatureValues,
    /// Natur-Werte je Tiefe (Schlüssel = Tiefe); sie gewinnen gegen die Vorgabe.
    pub nature_by_depth: BTreeMap<String, NatureValues>,
    /// ID des Endbosses der Insel (data/bosses.json › endBosses, B-981); leer = der Endboss der tiefsten Stufe.
    pub end_boss: String,
    /// Geschichte der Insel (story.go, B-965); leer = keine Zeilen.
    pub story: IslandStory,
}

/// Go: `biomeForDepth` (campaign.go): das Biom der Tiefe. Go bricht bei einer unbekannten Tiefe mit `panic` ab; die
/// Aufrufer hier prüfen vorher `has_depth` oder melden einen Fehler.
pub fn biome_for_depth(biomes: &[Biome], depth: i64) -> Option<&Biome> {
    biomes.iter().find(|b| b.depth == depth)
}

/// Go: `miniBoss != nil` (boss_data.go): Gibt es einen Miniboss in der Tiefe?
fn has_mini_boss(data: &Data, depth: i64) -> bool {
    data.bosses.bosses.iter().any(|b| b.kind == "mini" && b.depth == depth)
}

/// Go: `strconv.Atoi(key)` mit Gegenprobe `Itoa(depth) == key`: der Schlüssel ist eine Tiefe der Insel.
pub(crate) fn depth_key(key: &str, depths: &[i64]) -> Option<i64> {
    let depth = key.parse::<i64>().ok()?;
    (depth.to_string() == key && depths.contains(&depth)).then_some(depth)
}

impl IslandDef {
    /// Go: `plannedAt`, die geplanten Chunks der Insel für eine Tiefe (leer = reiner Zufall).
    pub fn planned_at(&self, depth: i64) -> Vec<PlannedChunk> {
        self.planned_chunks.get(&depth.to_string()).cloned().unwrap_or_default()
    }

    /// Go: `countAt`, die Chunk-Anzahl der Insel für eine Tiefe (0 = Länge des Bioms).
    pub fn count_at(&self, depth: i64) -> i64 {
        self.chunk_counts.get(&depth.to_string()).copied().unwrap_or(0)
    }
}

/// Go: `checkIsland` prüft eine Insel in der Reihenfolge aus docs/rules/welten.md § 3: Name, Tiefen (vorhanden, bekannt,
/// einmalig), je Tiefe ein Miniboss, Faktoren > 0, chunkCounts, Endboss und Bau der tiefsten Stufe (island_endboss.go),
/// plannedChunks. ID und Eindeutigkeit prüft `checkWorld`.
pub fn check_island(d: &IslandDef, data: &Data, rules: &Rules) -> Result<(), String> {
    if d.name.is_empty() {
        return Err("name ist leer".to_string());
    }
    if d.depths.is_empty() {
        return Err("ohne Tiefen".to_string());
    }
    for (i, &depth) in d.depths.iter().enumerate() {
        if !has_depth(&data.biomes, depth) {
            return Err(format!("unbekannte Tiefe {depth}"));
        }
        if d.depths[..i].contains(&depth) {
            return Err(format!("doppelte Tiefe {depth}"));
        }
    }
    for &depth in &d.depths {
        if !has_mini_boss(data, depth) {
            return Err(format!("kein Miniboss in Tiefe {depth}"));
        }
    }
    let s = &d.depth_scaling;
    if s.hp <= 0.0 || s.damage <= 0.0 || s.speed <= 0.0 {
        return Err("depthScaling braucht Faktoren > 0".to_string());
    }
    check_chunk_counts(d, &data.island_sizes)?;
    check_end_boss(d, data)?;
    // Platz-unabhängig (leere Plätze): Die Plätze kennt erst der Wertesatz der Welt, ResolveWorld prüft die Planung danach
    // gegen sie (checkWorldPlans, B-983).
    check_planned_chunks_with(d, Some(&[]), &data.biomes, rules)?;
    check_nature(d, &data.biomes)
}

/// Go: `checkPlannedChunksWith` prüft die geplanten Chunks gegen die Hub- und Insel-Plätze `sites` einer Welt (`None` =
/// Basis, B-983).
pub fn check_planned_chunks_with(
    d: &IslandDef,
    sites: Option<&[HubSite]>,
    biomes: &[Biome],
    rules: &Rules,
) -> Result<(), String> {
    for (key, plan) in &d.planned_chunks {
        let Some(depth) = depth_key(key, &d.depths) else {
            return Err(format!("plannedChunks: Tiefe {key:?} ist keine Tiefe der Insel"));
        };
        let Some(b) = biome_for_depth(biomes, depth) else {
            return Err(format!("unbekannte Tiefe {depth}"));
        };
        validate_plan_with(b, plan, d.count_at(depth), sites, rules)
            .map_err(|e| format!("plannedChunks Tiefe {depth}: {e}"))?;
    }
    Ok(())
}
