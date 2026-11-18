//! Seed-Inhalt eines Chunks (WZ17.3, B-485/AC-06, geplante-chunks.md § 7). Port von `preview_chunk.go`: Die Umwandlung
//! eines Seed-Chunks in feste Objekte übernimmt den Inhalt aus dem Vorschau-Level, kein neuer Generator-Code.
use k3c_data::Data;
use k3c_level::types::{Chunk, Layout, PlannedBuilding, PlannedObject};

use super::preview::{PreviewOptions, build_preview};
use crate::fehler::Fehler;

fn no_hub_slot() -> Fehler {
    Fehler::Preview("vorschau: es gibt keinen Platz 0 (Hub)".to_string())
}

/// Liefert die Objekte des Chunks auf Platz `slot` im Vorschau-Level der Optionen (Biom, Seed, Planung, Chunk-Anzahl wie
/// `build_preview`): Art und Lage `at` in Units ab Chunk-Anfang, im Format von `plannedChunks.objects`. Bei einem
/// geplanten Chunk sind das sein Seed- bzw. Strom-Inhalt oder seine festen Objekte; bei einem ungeplanten alles, was dort
/// steht. Platz 0 (Hub), Ausgang, Rand und Plätze außerhalb des Levels sind ein Fehler.
pub fn preview_chunk_objects(o: &PreviewOptions, slot: i64, data: &Data) -> Result<Vec<PlannedObject>, Fehler> {
    if slot == 0 {
        return Err(no_hub_slot());
    }
    let w = build_preview(o, data)?;
    let c = chunk_at(&w.level, slot)?;
    let end = c.start_units + w.level.chunk_width_units;
    // Exakt: Anfang ≥ eine Chunk-Breite und e.x < 2·Anfang, also ist die Differenz ohne Rundung (Sterbenz); Anfang + at
    // ergibt wieder genau e.x, die Umwandlung ändert kind@x nicht.
    Ok(w.level
        .entities
        .iter()
        .filter(|e| e.x >= c.start_units && e.x < end)
        .map(|e| PlannedObject {
            kind: e.kind.clone(),
            at: e.x - c.start_units,
        })
        .collect())
}

/// Liefert die Ruinen des Chunks auf Platz `slot` im Vorschau-Level der Optionen (WZ14.5, B-468, geplante-chunks.md § 8):
/// Art und Lage `at` in Units ab Chunk-Anfang, nach Lage sortiert, im Format von `plannedChunks.buildings`. Ohne Planung
/// oder ohne Ruinen dort leer; Fehler wie `preview_chunk_objects`.
pub fn preview_chunk_ruins(o: &PreviewOptions, slot: i64, data: &Data) -> Result<Vec<PlannedBuilding>, Fehler> {
    if slot == 0 {
        return Err(no_hub_slot());
    }
    let w = build_preview(o, data)?;
    let c = chunk_at(&w.level, slot)?;
    let end = c.start_units + w.level.chunk_width_units;
    let mut out: Vec<PlannedBuilding> = w
        .level
        .ruins
        .iter()
        .filter(|r| r.x >= c.start_units && r.x < end)
        .map(|r| PlannedBuilding {
            kind: r.kind.clone(),
            at: r.x - c.start_units, // exakt wie bei den Objekten
        })
        .collect();
    out.sort_by(|a, b| a.at.total_cmp(&b.at)); // stabil wie `sort.SliceStable`
    Ok(out)
}

/// Sucht den Chunk zum Platz: +n ist der n-te Chunk rechts der Hub-Chunks, −n der n-te links davon.
fn chunk_at(lv: &Layout, slot: i64) -> Result<Chunk, Fehler> {
    let mut hub_start: Option<usize> = None;
    let mut hub_chunks = 0_i64;
    for (i, c) in lv.chunks.iter().enumerate() {
        if c.kind == "hub" {
            hub_start.get_or_insert(i);
            hub_chunks += 1;
        }
    }
    let Some(hub_start) = hub_start else {
        return Err(Fehler::Preview("vorschau: Level ohne Hub".to_string()));
    };
    let hub_start = hub_start as i64;
    let i = if slot > 0 {
        hub_start + hub_chunks + slot - 1
    } else {
        hub_start + slot
    };
    let Some(chunk) = usize::try_from(i).ok().and_then(|i| lv.chunks.get(i)) else {
        return Err(Fehler::Preview(format!(
            "vorschau: Platz {slot:+} liegt außerhalb des Levels ({} Chunks)",
            lv.chunks.len()
        )));
    };
    if chunk.kind == "exit" || chunk.kind == "edge" {
        return Err(Fehler::Preview(format!(
            "vorschau: Platz {slot:+} ist Ausgang oder Rand ({})",
            chunk.kind
        )));
    }
    Ok(chunk.clone())
}
