//! Level-Erzeugung (Port von `engine/level/level.go`). Der Rumpf wächst mit den Sessions von RU6: hier stehen bisher
//! die kleinen Hilfen, die die Plan-Prüfung braucht.

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_rng::Rng;

use crate::biome::{Biome, Ordered};
use crate::lines::HubSiteRef;
use crate::rules::Rules;
use crate::types::{Chunk, PlannedChunk};

pub use crate::level_gen::{generate, generate_with};

/// Go: `HarborKind`, die Chunk-Art des Hafens (BAS, B-637, geplante-chunks.md § 10): ein geplanter Chunk mit genau
/// einem festen Objekt in der Mitte, kein Zufalls-Ereignis (nicht in `EVENT_KINDS`) und nicht in den Biom-Summen.
pub const HARBOR_KIND: &str = "harbor";

/// Go: `lairCampGapUnits`: So nah am Bau des Endbosses liegt kein Camp: lair.noticeUnits (40) plus 40 Units Rand
/// (docs/rules/bosse.md § 1, B-645).
pub const LAIR_CAMP_GAP_UNITS: i64 = 80;

/// Go: `eventKinds` in fester Reihenfolge wie `EVENT_KINDS` in TS.
pub const EVENT_KINDS: [&str; 2] = ["chest", "recruitCamp"];

/// Go: `isEvent`.
pub fn is_event(kind: &str) -> bool {
    kind == "chest" || kind == "recruitCamp" || kind == HARBOR_KIND
}

/// Go: `hubChunksOf`, die gerade Zahl der Hub-Chunks eines Bioms. `round` entspricht `math.Round`.
pub fn hub_chunks_of(b: &Biome) -> i64 {
    i64::max(2, (b.hub_width_units / b.chunk_width_units / 2.0).round() as i64 * 2)
}

/// Go: `checkCount`, prüft eine eingestellte Chunk-Anzahl (0 = Länge aus lengthUnits): gerade, damit der Hub mittig
/// liegt, und Platz für Hub, Ausgang, Rand und je Seite mindestens einen Chunk. Die erlaubten Größen prüft `k3c-sim`.
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

/// Go: `gen`, der Zustand während der Erzeugung; "" in `kinds` heißt: Chunk noch frei. Die Methoden stehen in den
/// Modulen, die Go in derselben Datei hat (`planned_gen`, `guards`, `ruins`, `sites`, `level`).
#[derive(Debug, Clone)]
pub(crate) struct Gen {
    pub b: Biome,
    pub r: Rng,
    pub seed: String,
    pub kinds: Vec<String>,
    pub cw: f64,
    pub hub: f64,
    /// Eingestellte Chunk-Anzahl (`Options::chunk_count`); 0 = gewürfelte Länge aus lengthUnits.
    pub count: i64,
    /// Hub- und Insel-Plätze der Welt (`Options::hub_sites`).
    pub sites: Vec<HubSiteRef>,
    /// Geplante Chunks (`planned_gen`): `hub_start` und `hub_chunks` legen die Plätze fest, `planned` ordnet
    /// Chunk-Index und Plan zu, `fixed` zählt feste Portale, Truhen, Camps und Adern, `side_fixed` feste Portale je
    /// Seite.
    pub hub_start: i64,
    pub hub_chunks: i64,
    pub planned: BTreeMap<i64, PlannedChunk>,
    pub fixed: BTreeMap<String, i64>,
    pub side_fixed: [i64; 2],
    /// Ressourcen je Chunk-Art ohne Natur-Faktoren: geplante Chunks würfeln ihren Inhalt daraus, damit ein Chunk-Seed in
    /// jedem Level dieselben Objekte ergibt (B-891).
    pub planned_res: Ordered<Ordered<[i64; 2]>>,
    pub rules: Arc<Rules>,
}

impl Gen {
    /// Go: `gen.center`, die Mitte von Chunk `i`.
    pub(crate) fn center(&self, i: i64) -> f64 {
        i as f64 * self.cw + self.cw / 2.0
    }

    /// Go: `gen.free`, die Indizes der noch freien Chunks.
    pub(crate) fn free(&self) -> Vec<i64> {
        let mut out = Vec::new();
        for (i, k) in self.kinds.iter().enumerate() {
            if k.is_empty() {
                out.push(i as i64);
            }
        }
        out
    }
}

/// Go: `gen.inside`, eine zufällige Position aus dem Strom `r` mit 2 Units Abstand zu den Chunk-Grenzen. Freie
/// Funktion mit der Chunk-Breite `cw`, damit `Gen::r` als Strom übergeben werden kann.
pub(crate) fn inside(cw: f64, r: &mut Rng, c: &Chunk) -> f64 {
    c.start_units + 2.0 + r.next() * (cw - 4.0)
}
