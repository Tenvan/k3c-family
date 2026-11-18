//! Level-Erzeugung (Port von `engine/level/level.go`). Der Rumpf wächst mit den Sessions von RU6: hier stehen bisher
//! die kleinen Hilfen, die die Plan-Prüfung braucht.

use crate::biome::Biome;

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
