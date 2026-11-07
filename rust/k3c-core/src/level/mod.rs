//! Prozeduraler Level-Generator, Port von `engine/level` (Go). Gleicher Biom, Seed und gleiche Optionen ergeben
//! dasselbe Layout wie `level.GenerateWith`, geprüft gegen `testdata/golden/level-*.json` (Rust-Tests in
//! `tests.rs`, Go-Vergleich in `engine/level/rust_test.go`). Die Reihenfolge der RNG-Aufrufe und der
//! Gleitkomma-Operationen folgt dem Go-Code Schritt für Schritt; Summen werden nicht umgestellt.
//!
//! Aufbau: `generator.rs` (Rahmen, Portale, Ereignisse), `entities.rs` (Objekte, Adern, geplante Inhalte), `sites.rs`
//! (Mauer-, Turm- und Baumschul-Plätze), `plan.rs`/`totals.rs`/`ruins.rs` (Prüfung der Planung), `backdrop.rs`,
//! `output.rs` (JSON wie `json.Marshal`).

pub mod backdrop;
pub mod biome;
mod entities;
mod generator;
mod output;
pub mod plan;
pub mod ruins;
mod sites;
mod totals;

#[cfg(test)]
mod tests;

use biome::{Biome, Input, nullable};
use plan::PlannedChunk;
use ruins::Spot;
use serde::Deserialize;

pub use generator::Gen;
pub use sites::WallSite;

/// Ereignis-Arten in fester Reihenfolge (`eventKinds` in Go).
pub const EVENT_KINDS: [&str; 2] = ["chest", "recruitCamp"];
/// Chunk-Art des Hafens (geplanter Chunk mit einem festen Objekt, kein Zufalls-Ereignis).
pub const HARBOR_KIND: &str = "harbor";

/// Truhe, Camp oder Hafen.
pub fn is_event(kind: &str) -> bool {
    kind == "chest" || kind == "recruitCamp" || kind == HARBOR_KIND
}

/// Optionen wie `level.Options`; liest `planned`/`chunkCount` und die Go-Namen ohne Tags (`Planned`, `ChunkCount`).
/// `withSim` hängt die Felder der Sim an die Ausgabe (nur für den Vergleich mit Go).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Options {
    #[serde(alias = "Planned", deserialize_with = "nullable")]
    pub planned: Vec<PlannedChunk>,
    #[serde(rename = "chunkCount", alias = "ChunkCount")]
    pub chunk_count: i64,
    #[serde(rename = "withSim")]
    pub with_sim: bool,
}

/// Abschnitt fester Breite.
#[derive(Clone, Debug, PartialEq)]
pub struct Chunk<'a> {
    pub index: usize,
    pub kind: &'a str,
    pub start: f64,
}

/// Objekt im Level (Burg, Portal, Baum, Truhe, Skill-Punkt …): Art und X in Units.
pub type Entity<'a> = Spot<'a>;

/// Fertiges Level (Go: `level.Layout`).
#[derive(Clone, Debug)]
pub struct Layout<'a> {
    pub seed: &'a str,
    pub biome_id: &'a str,
    pub width: f64,
    pub cw: f64,
    pub hub: f64,
    pub chunks: Vec<Chunk<'a>>,
    pub entities: Vec<Entity<'a>>,
    pub sites: Vec<WallSite<'a>>,
    pub free_ground: Vec<f64>,
    pub ruins: Vec<Spot<'a>>,
    pub guards: Vec<Spot<'a>>,
    pub backdrops: Vec<i64>,
}

/// Gerade Zahl der Hub-Chunks eines Bioms (`f64::round` rundet wie `math.Round` von der Null weg).
pub fn hub_chunks_of(b: &Biome) -> i64 {
    2.max((b.hub_width_units / b.chunk_width_units / 2.0).round() as i64 * 2)
}

/// Prüft eine eingestellte Chunk-Anzahl (0 = Länge aus lengthUnits): gerade und Platz für Hub, Ausgang, Rand und je
/// Seite mindestens einen Chunk.
pub fn check_count(b: &Biome, n: i64) -> Result<(), String> {
    let least = hub_chunks_of(b) + 4;
    if n != 0 && (n % 2 != 0 || n < least) {
        return Err(format!(
            "eingestellte Chunk-Anzahl {n} in {}: gerade und mindestens {least}",
            b.id
        ));
    }
    Ok(())
}

/// Zahl wie `%v` in Go (kürzeste Darstellung, ganze Werte ohne Nachkomma).
pub fn gv(v: f64) -> String {
    format!("{v}")
}

/// Liste wie `%v` in Go: `[a b c]`.
pub fn list(items: &[String]) -> String {
    format!("[{}]", items.join(" "))
}

/// Baut das Level wie `level.GenerateWith`; eine ungültige Planung wird abgelehnt.
pub fn generate<'a>(inp: &'a Input, seed: &'a str, o: &'a Options) -> Result<Layout<'a>, String> {
    let b = &inp.biome;
    check_count(b, o.chunk_count)?;
    plan::validate_plan_for(b, &inp.rules, &o.planned, o.chunk_count)
        .map_err(|e| format!("geplante Chunks: {e}"))?;
    if !(b.chunk_width_units > 0.0
        && b.chunk_width_units.is_finite()
        && b.hub_width_units.is_finite())
    {
        return Err(format!(
            "Biom {}: chunkWidthUnits und hubWidthUnits müssen endlich und positiv sein",
            b.id
        ));
    }
    let mut g = Gen::new(inp, seed, o.chunk_count);
    g.frame()?;
    g.reserve(&o.planned)?;
    g.place_portals()?;
    g.place_events()?;
    g.fill_free()?;
    let chunks: Vec<Chunk<'a>> = g
        .kinds
        .iter()
        .enumerate()
        .map(|(i, k)| Chunk {
            index: i,
            kind: k,
            start: i as f64 * g.cw,
        })
        .collect();
    let mut entities = g.entities(&chunks)?;
    if b.lair {
        entities.push(Entity {
            kind: "lair",
            x: g.lair_x(),
        });
    }
    entities.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
    let sites = g.place_sites(&entities, &chunks);
    let width = chunks.len() as f64 * g.cw;
    Ok(Layout {
        seed,
        biome_id: &b.id,
        width,
        cw: g.cw,
        hub: g.hub,
        free_ground: sites::free_ground(&sites),
        ruins: g.planned_spots(&chunks, true),
        guards: g.planned_spots(&chunks, false),
        backdrops: backdrop::backdrops(&b.id, seed, b.backdrop_variants, width),
        chunks,
        entities,
        sites,
    })
}

/// Fehler des Generators: ungültige Eingabe (JSON) oder abgelehntes Level.
#[derive(Debug, PartialEq)]
pub enum Error {
    Input(String),
    Generate(String),
}

/// JSON hinein, JSON heraus: Eingabe `{"biome", "rules"}`, Seed, Optionen (leer = Nullwert).
pub fn generate_json(input: &[u8], seed: &str, opts: &[u8]) -> Result<String, Error> {
    let inp: Input =
        serde_json::from_slice(input).map_err(|e| Error::Input(format!("Biom/Regeln: {e}")))?;
    let o: Options = if opts.iter().all(u8::is_ascii_whitespace) {
        Options::default()
    } else {
        serde_json::from_slice(opts).map_err(|e| Error::Input(format!("Optionen: {e}")))?
    };
    let l = generate(&inp, seed, &o).map_err(Error::Generate)?;
    Ok(l.to_json(o.with_sim))
}
