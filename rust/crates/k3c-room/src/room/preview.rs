//! Vorschau-Welt für den Welt-Betrachter (WZ7.1, B-460). Port von `preview.go`: eine Welt in dem Format, das auch ein Raum
//! hat, aber ohne Raum, Spielstand und globalen Zustand. Gos Paket-Variablen (Biome, hub.json, Inselgrößen) kommen als
//! `&Data`. Die Hilfen der Vorschau mit Kandidaten-hub.json stehen in `preview_candidate.rs`, der Chunk-Inhalt in
//! `preview_chunk.rs`.
use std::collections::BTreeMap;

use k3c_data::Data;
use k3c_data::biome::Biome;
use k3c_level::types::PlannedChunk;
use k3c_sim::island::sizes::check_chunk_count;
use k3c_sim::state::World;
use k3c_sim::world::preview::{SiteLook, preview_cycle, preview_hub_level, preview_set_site};
use k3c_sim::world::{Options as WorldOptions, create_world};

use super::preview_candidate::candidate_world;
use crate::fehler::Fehler;

/// Länge des Seeds wie bei `MAX_SEED_RUNES`.
const MAX_PREVIEW_SEED_RUNES: usize = 64;

/// Die Quelle des Levels. In WZ7 ist es das Biom; Insel und Stufe kommen mit WZ8.5 als weitere Felder.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewSource {
    pub biome: String,
}

/// Beschreibt eine Vorschau-Welt. Der Nullwert jedes Feldes außer `seed` und `source` ist das Standardbild: höchste
/// Hub-Stufe, Tag mit Fortschritt 0, alle Gebäude.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreviewOptions {
    pub seed: String,
    pub source: PreviewSource,
    /// 1 bis Anzahl hub.json › levels; 0 = die höchste.
    pub hub_level: i64,
    /// Tageszeit als Sim-Phase (day, dusk, night, dawn; leer = day) und Fortschritt in [0, 1).
    pub phase: String,
    pub progress: f64,
    /// `false` schaltet einen Platz aus (Schlüssel `preview_site_key`); fehlende und unbekannte Schlüssel bleiben an
    /// bzw. werden ignoriert.
    pub buildings: BTreeMap<String, bool>,
    /// Baustufe, Trefferpunkte oder „nicht gebaut“ einzelner Plätze (Schlüssel `preview_site_key`), nach dem Ausbau der
    /// Hub-Stufe; unbekannte Schlüssel werden ignoriert, eine ungültige Stufe ist ein Fehler.
    pub sites: BTreeMap<String, SiteLook>,
    /// Geplante Chunks dieses Levels (WZ8.4), auch ein ungespeicherter Entwurf; leer = reiner Zufall.
    pub planned: Vec<PlannedChunk>,
    /// Kandidaten-hub.json (Gebäude-Planer, WZ9.1); leer = eingebettete hub.json. Go: `[]byte`, hier der JSON-Text.
    pub hub_candidate: String,
    /// Chunk-Anzahl der Stufe (Inselgröße, WZ17.3); 0 = Länge des Bioms.
    pub chunk_count: i64,
}

/// Der Schlüssel eines Bauplatzes in `PreviewOptions::buildings`: Art und X-Position in Units (Gos
/// `strconv.FormatFloat(x, 'f', -1, 64)`; Rusts `{}` ist der kürzeste Text ohne Exponent, nur Inf weicht ab).
pub fn preview_site_key(kind: &str, x: f64) -> String {
    if x.is_infinite() {
        let inf = if x > 0.0 { "+Inf" } else { "-Inf" };
        return format!("{kind}@{inf}");
    }
    format!("{kind}@{x}")
}

/// Ist `biome` ein Biom-Name der Form `^[a-z0-9-]{1,32}$` (Gos `previewBiome`, ohne `regex`)?
pub fn valid_preview_biome(biome: &str) -> bool {
    (1..=32).contains(&biome.len())
        && biome
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}

fn vorschau(text: String) -> Fehler {
    Fehler::Preview(text)
}

/// Erzeugt die Welt zu den Optionen: Level aus Seed und Biom, ohne Bürger und Gegner, auf der Hub-Stufe ausgebaut
/// (`preview_hub_level`), Tageszeit gesetzt (`preview_cycle`), abgewählte Plätze entfernt. Gleiche Optionen ergeben
/// dieselbe Welt; die Funktion schreibt nichts.
pub fn build_preview(o: &PreviewOptions, data: &Data) -> Result<World, Fehler> {
    let n = o.seed.chars().count();
    if n == 0 || n > MAX_PREVIEW_SEED_RUNES {
        return Err(vorschau(format!(
            "vorschau: Seed braucht 1 bis {MAX_PREVIEW_SEED_RUNES} Zeichen"
        )));
    }
    if o.source.biome.is_empty() {
        return Err(vorschau("vorschau: Quelle fehlt (Biom)".to_string()));
    }
    if !valid_preview_biome(&o.source.biome) {
        return Err(vorschau(format!("vorschau: ungültiges Biom {:?}", o.source.biome)));
    }
    let id = &o.source.biome;
    let Some(biome) = data.biomes.iter().find(|b| &b.id == id) else {
        return Err(vorschau(format!(
            "vorschau: Biom {id:?} nicht ladbar: open biomes/{id}.json: file does not exist"
        )));
    };
    let (mut w, top) = preview_world(biome, o, data)?;
    let hub_level = if o.hub_level == 0 { top } else { o.hub_level };
    let phase = if o.phase.is_empty() { "day" } else { o.phase.as_str() };
    w.troops = Vec::new();
    w.enemies = Vec::new();
    w.spawn_queue = Vec::new();
    preview_hub_level(&mut w, hub_level).map_err(vorschau)?;
    preview_cycle(&mut w, phase, o.progress).map_err(vorschau)?;
    apply_sites(&mut w, o)?;
    Ok(w)
}

/// Setzt den Zustand einzelner Plätze (`sites`) und entfernt abgewählte (`buildings`). Erst nach dem Ausbau der Hub-Stufe,
/// damit die Stufen der übrigen bleiben.
fn apply_sites(w: &mut World, o: &PreviewOptions) -> Result<(), Fehler> {
    for si in 0..w.sites.len() {
        let key = preview_site_key(&w.sites[si].kind, w.sites[si].x);
        if let Some(look) = o.sites.get(&key) {
            preview_set_site(w, si, look).map_err(vorschau)?;
        }
    }
    w.sites.retain(|s| {
        o.buildings
            .get(&preview_site_key(&s.kind, s.x))
            .copied()
            .unwrap_or(true)
    });
    Ok(())
}

/// Erzeugt die frische Welt und nennt die höchste Hub-Stufe: ohne Kandidat aus der eingebetteten hub.json (wie vor WZ9.1),
/// mit Kandidat aus ihm (`candidate_world`).
fn preview_world(biome: &Biome, o: &PreviewOptions, data: &Data) -> Result<(World, i64), Fehler> {
    check_preview_count(o, data)?;
    if !o.hub_candidate.is_empty() {
        return candidate_world(biome, &o.seed, &o.hub_candidate, &o.planned, o.chunk_count);
    }
    let top = (data.hub.levels.len() as i64).max(1);
    let opts = WorldOptions {
        cycle_speed: 1.0,
        planned: o.planned.clone(),
        chunk_count: o.chunk_count,
        ..WorldOptions::default()
    };
    let w =
        create_world(biome, &o.seed, &opts).map_err(|e| vorschau(format!("vorschau: Level nicht erzeugbar: {e}")))?;
    Ok((w, top))
}

/// Lehnt eine Chunk-Anzahl ab, die keine Inselgröße ist (`check_chunk_count`, die eine Prüfung, B-490); 0 = Länge des
/// Bioms. Mit Kandidaten-hub.json gilt dieselbe Prüfung.
fn check_preview_count(o: &PreviewOptions, data: &Data) -> Result<(), Fehler> {
    if o.chunk_count == 0 {
        return Ok(());
    }
    check_chunk_count(&data.island_sizes, o.chunk_count).map_err(|e| vorschau(format!("vorschau: {e}")))
}
