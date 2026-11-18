//! Prüfung des Baus des Endbosses (Port von `engine/level/lair.go`).

use crate::biome::Biome;
use crate::level::{LAIR_CAMP_GAP_UNITS, hub_chunks_of};

/// Go: `CheckLairFit`, prüft, ob das Biom `b` als tiefste Stufe einer Insel den Bau des Endbosses samt Camp-Abstand
/// trägt (docs/rules/bosse.md § 1, B-981): Mindestens so viele Chunks wie Camps (`eventChunks.recruitCamp.min`) liegen
/// außerhalb der Hub-Chunks und der Randchunks nicht näher als `LAIR_CAMP_GAP_UNITS` am Bau. `chunk_count` ist die
/// Inselgröße der Stufe (0 = kürzeste Länge des Bioms). Unabhängig vom Seed; nichts wird repariert, die Meldung nennt
/// den Grund.
pub fn check_lair_fit(b: &Biome, chunk_count: i64) -> Result<(), String> {
    let cw = b.chunk_width_units;
    let mut count = chunk_count;
    if count == 0 {
        count = (b.length_units.min as f64 / cw).round() as i64;
        count += count % 2; // gerade Anzahl wie frame()
    }
    if count < 4 {
        return Err(format!(
            "biom {}: {count} Chunks sind zu wenig für den Bau des Endbosses",
            b.id
        ));
    }
    let hub_chunks = hub_chunks_of(b);
    let hub_start = count / 2 - hub_chunks / 2;
    let lair = if b.exit_side == "right" {
        (count - 1) as f64 * cw
    } else {
        cw
    };
    let mut safe = 0;
    for i in 1..count - 1 {
        if i >= hub_start && i < hub_start + hub_chunks {
            continue;
        }
        if (i as f64 * cw + cw / 2.0 - lair).abs() >= LAIR_CAMP_GAP_UNITS as f64 {
            safe += 1;
        }
    }
    let want = b.event_chunks.get("recruitCamp").map_or(0, |r| r.min);
    if safe < want {
        return Err(format!(
            "biom {}: nur {safe} Chunks mit {LAIR_CAMP_GAP_UNITS} Units Abstand zum Bau, {want} Camp(s) brauchen Platz (Inselgröße {count} Chunks)",
            b.id
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "lair_tests.rs"]
mod lair_tests;
