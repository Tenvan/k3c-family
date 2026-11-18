//! Gegnerlager (Port von `engine/level/guards.go`, docs/rules/geplante-chunks.md § 9, B-517): feste Wachen eines
//! geplanten Chunks. Dieses Modul prüft Art und Lage und gibt die Wachen je Stufe weiter (`Layout::guards`); als
//! Gegner entstehen sie in `k3c-sim`. Werte aus `data/guards.json` (`Rules::guard`, vorher `guardRules`).

use k3c_det::golden::go_float;

use crate::biome::Biome;
use crate::level::Gen;
use crate::planned::{EDGE_GAP_UNITS, Geometry, go_list};
use crate::rules::Rules;
use crate::types::{Chunk, Guard, PlannedChunk};

/// Go: `checkGuards`, prüft das Gegnerlager eines Chunks: Anzahl, Art im Biom (Portal-Pool), Lage ≥ 2 Units von der
/// Chunk-Grenze und außerhalb der Verteidigungszone, Abstand der Wachen untereinander (§ 9). Fehler nennen die Wache.
pub(crate) fn check_guards(c: &PlannedChunk, b: &Biome, g: &Geometry, rules: &Rules) -> Result<(), String> {
    let gr = &rules.guard;
    if c.enemies.len() as i64 > gr.max_per_chunk {
        return Err(format!(
            "höchstens {} Wachen je Chunk, geplant sind {}",
            gr.max_per_chunk,
            c.enemies.len()
        ));
    }
    let outer = rules.site_placement.outside_units;
    let edge = EDGE_GAP_UNITS as f64;
    for (i, e) in c.enemies.iter().enumerate() {
        let at = go_float(e.at);
        if !b.enemies.portal.contains(&e.kind) {
            return Err(format!(
                "die Wache {:?} ist im Biom {} nicht erlaubt (erlaubt: {})",
                e.kind,
                b.id,
                go_list(&b.enemies.portal)
            ));
        }
        if e.at < edge || e.at > g.cw - edge {
            return Err(format!(
                "die Wache {} bei {at}: mindestens {EDGE_GAP_UNITS} Units Abstand zur Chunk-Grenze ({EDGE_GAP_UNITS}…{})",
                e.kind,
                go_float(g.cw - edge)
            ));
        }
        let dist = g.dist(c.slot, e.at);
        if dist <= outer {
            return Err(format!(
                "die Wache {} bei {at} liegt {} Units vom Hub, innerhalb der Verteidigungszone ({})",
                e.kind,
                go_float(dist),
                go_float(outer)
            ));
        }
        for o in &c.enemies[..i] {
            if (e.at - o.at).abs() < gr.min_gap_units {
                return Err(format!(
                    "die Wache {} bei {at}, weniger als {} Units von {} bei {}",
                    e.kind,
                    go_float(gr.min_gap_units),
                    o.kind,
                    go_float(o.at)
                ));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)] // RU6.5: generate ruft guards_of
impl Gen {
    /// Go: `guardsOf`, die Wachen der geplanten Chunks mit ihrer X-Position; ohne Würfe, die Ströme bleiben unberührt.
    pub(crate) fn guards_of(&self, chunks: &[Chunk]) -> Vec<Guard> {
        let mut out = Vec::new();
        for c in chunks {
            if let Some(pc) = self.planned.get(&c.index) {
                out.extend(pc.enemies.iter().map(|e| Guard {
                    kind: e.kind.clone(),
                    x: c.start_units + e.at,
                }));
            }
        }
        out
    }
}

#[cfg(test)]
#[path = "guards_tests.rs"]
mod guards_tests;
