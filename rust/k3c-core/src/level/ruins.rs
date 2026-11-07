//! Ruinen und Wachen geplanter Chunks (Port von `engine/level/ruins.go` und `guards.go`, geplante-chunks.md § 8 und
//! § 9): Prüfung von Art und Lage und die Ausgabe je Stufe ohne Würfe.

use super::biome::{Biome, Rules};
use super::plan::{EDGE_GAP_UNITS, Geometry, Placed, PlannedChunk};
use super::{Chunk, Gen, gv, list};

/// Mindestabstand einer Ruine zu Adern, Portalen und festen Objekten.
pub const RUIN_GAP_UNITS: f64 = 4.0;

/// Ruine oder Wache im Level: Art und X-Position in Units (Chunk-Anfang + At).
#[derive(Clone, Debug, PartialEq)]
pub struct Spot<'a> {
    pub kind: &'a str,
    pub x: f64,
}

/// Prüft die Ruine eines Chunks (höchstens eine) gegen die Lage-Regeln; `near` sind die festen Objekte des Chunks.
pub fn check_buildings(
    c: &PlannedChunk,
    rules: &Rules,
    g: &Geometry,
    near: &[Placed],
) -> Result<Vec<Placed>, String> {
    if c.buildings.is_empty() {
        return Ok(Vec::new());
    }
    if c.buildings.len() > 1 {
        return Err(format!(
            "höchstens eine Ruine je Chunk, geplant sind {}",
            c.buildings.len()
        ));
    }
    let b = &c.buildings[0];
    if !rules.ruin_kinds.contains(&b.kind) {
        return Err(format!(
            "{:?} ist als Ruine nicht erlaubt (erlaubt: {})",
            b.kind,
            list(&rules.ruin_kinds)
        ));
    }
    let p = Placed {
        kind: b.kind.clone(),
        slot: c.slot,
        at: b.at,
        dist: g.dist(c.slot, b.at),
    };
    check_ruin_place(&p, rules, g, near)?;
    Ok(vec![p])
}

fn check_ruin_place(
    p: &Placed,
    rules: &Rules,
    g: &Geometry,
    near: &[Placed],
) -> Result<(), String> {
    let mut edge = EDGE_GAP_UNITS;
    if let Some(strip) = rules.ruin_edges.get(&p.kind) {
        edge = edge.max(*strip);
    }
    if p.at < edge || p.at > g.cw - edge {
        return Err(format!(
            "die Ruine {} bei {}: mindestens {} Units Abstand zur Chunk-Grenze ({}…{})",
            p.kind,
            gv(p.at),
            gv(edge),
            gv(edge),
            gv(g.cw - edge)
        ));
    }
    for h in &rules.hub_sites {
        if h.kind == p.kind && h.offset_units == p.key_x() {
            return Err(format!(
                "die Ruine {} liegt auf der Lage des Hub-Platzes gleicher Art ({} Units vom Hub)",
                p.kind,
                gv(h.offset_units)
            ));
        }
    }
    for o in near {
        if (p.at - o.at).abs() < RUIN_GAP_UNITS {
            return Err(format!(
                "die Ruine {} bei {}, weniger als 4 Units von {} bei {}",
                p.kind,
                gv(p.at),
                o.kind,
                gv(o.at)
            ));
        }
    }
    Ok(())
}

/// Abstand jeder Ruine zu festen Objekten anderer Chunks.
pub fn check_ruins_across(ruins: &[Placed], objects: &[Placed]) -> Result<(), String> {
    for r in ruins {
        for o in objects {
            if o.slot != r.slot && (r.key_x() - o.key_x()).abs() < RUIN_GAP_UNITS {
                return Err(format!(
                    "auf Platz {:+}: Ruine {} bei {}, weniger als 4 Units von {} auf Platz {:+}",
                    r.slot,
                    r.kind,
                    gv(r.at),
                    o.kind,
                    o.slot
                ));
            }
        }
    }
    Ok(())
}

/// Prüft das Gegnerlager eines Chunks: Anzahl, Art im Biom (Portal-Pool), Lage und Abstand der Wachen (§ 9).
pub fn check_guards(
    c: &PlannedChunk,
    b: &Biome,
    rules: &Rules,
    g: &Geometry,
) -> Result<(), String> {
    let gr = &rules.guards;
    if c.enemies.len() as i64 > gr.max_per_chunk {
        return Err(format!(
            "höchstens {} Wachen je Chunk, geplant sind {}",
            gr.max_per_chunk,
            c.enemies.len()
        ));
    }
    let outer = rules.site_placement.outside_units;
    for (i, e) in c.enemies.iter().enumerate() {
        if !b.enemies.portal.contains(&e.kind) {
            return Err(format!(
                "die Wache {:?} ist im Biom {} nicht erlaubt (erlaubt: {})",
                e.kind,
                b.id,
                list(&b.enemies.portal)
            ));
        }
        if e.at < EDGE_GAP_UNITS || e.at > g.cw - EDGE_GAP_UNITS {
            return Err(format!(
                "die Wache {} bei {}: mindestens 2 Units Abstand zur Chunk-Grenze (2…{})",
                e.kind,
                gv(e.at),
                gv(g.cw - EDGE_GAP_UNITS)
            ));
        }
        let d = g.dist(c.slot, e.at);
        if d <= outer {
            return Err(format!(
                "die Wache {} bei {} liegt {} Units vom Hub, innerhalb der Verteidigungszone ({})",
                e.kind,
                gv(e.at),
                gv(d),
                gv(outer)
            ));
        }
        for o in &c.enemies[..i] {
            if (e.at - o.at).abs() < gr.min_gap_units {
                return Err(format!(
                    "die Wache {} bei {}, weniger als {} Units von {} bei {}",
                    e.kind,
                    gv(e.at),
                    gv(gr.min_gap_units),
                    o.kind,
                    gv(o.at)
                ));
            }
        }
    }
    Ok(())
}

impl<'a> Gen<'a> {
    /// Ruinen (`ruins = true`) oder Wachen der geplanten Chunks mit ihrer X-Position; ohne Würfe.
    pub(super) fn planned_spots(&self, chunks: &[Chunk<'a>], ruins: bool) -> Vec<Spot<'a>> {
        let mut out = Vec::new();
        for c in chunks {
            if let Some(pc) = self.planned_at(c.index) {
                let items = if ruins { &pc.buildings } else { &pc.enemies };
                out.extend(items.iter().map(|o| Spot {
                    kind: o.kind.as_str(),
                    x: c.start + o.at,
                }));
            }
        }
        out
    }
}
