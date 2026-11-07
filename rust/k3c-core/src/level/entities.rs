//! Objekte des Levels: Burg, Portale, Ausgang, Ereignisse, Ressourcen, Skill-Punkte, Adern und die Inhalte geplanter
//! Chunks (Port von `entities`, `veins`, `inside` aus `level.go` und `plannedEntities`, `clearOfRuin` aus
//! `planned_gen.go`).

use super::generator::fixed_objects;
use super::plan::{EDGE_GAP_UNITS, PlannedChunk};
use super::ruins::RUIN_GAP_UNITS;
use super::{Chunk, Entity, Gen, is_event};
use crate::rng::Rng;

/// Neuwürfe, bevor eine Ressource an den Rand der Ruinen-Sperrzone gesetzt wird.
const CLEAR_TRIES: usize = 32;

/// Zufällige Position aus dem Strom `r` mit 2 Units Abstand zu den Chunk-Grenzen; das Produkt wird vor der Addition
/// gerundet wie `float64(…)` in Go (B-071), Rust fusioniert ohnehin nicht.
fn inside(r: &mut Rng, cw: f64, start: f64) -> f64 {
    start + 2.0 + r.next() * (cw - 4.0)
}

impl<'a> Gen<'a> {
    /// Burg, Portale, Ausgang, Ereignisse, Ressourcen, Skill-Punkte und Adern in dieser Reihenfolge der Würfe.
    pub fn entities(&mut self, chunks: &[Chunk<'a>]) -> Result<Vec<Entity<'a>>, String> {
        let b = self.b;
        let mut out = vec![Entity {
            kind: "castle",
            x: self.hub,
        }];
        for c in chunks {
            if let Some(pc) = self.planned_at(c.index) {
                self.planned_entities(c, pc, &mut out);
                continue;
            }
            match c.kind {
                "hub" | "edge" => continue,
                k if k == "exit" || k == "portal" || is_event(k) => {
                    out.push(Entity {
                        kind: c.kind,
                        x: c.start + self.cw / 2.0,
                    });
                    continue;
                }
                _ => {}
            }
            for (key, [lo, hi]) in b
                .resources_per_chunk
                .get(c.kind)
                .map_or(&[][..], |r| &r.0[..])
            {
                for _ in 0..self.r.int(*lo, *hi) {
                    let x = inside(&mut self.r, self.cw, c.start);
                    out.push(Entity {
                        kind: key.as_str(),
                        x,
                    });
                }
            }
        }
        // Skill-Punkte: versteckt in beliebigen Chunks außerhalb von Hub, Rand und geplanten Chunks.
        let hideouts: Vec<&Chunk> = chunks
            .iter()
            .filter(|c| c.kind != "hub" && c.kind != "edge" && self.planned_at(c.index).is_none())
            .collect();
        for _ in 0..self.r.int(b.skill_points.min, b.skill_points.max) {
            if hideouts.is_empty() {
                return Err(format!("Biom {}: kein Chunk für Skill-Punkte", b.id));
            }
            let c = hideouts[self.r.pick_index(hideouts.len())];
            out.push(Entity {
                kind: "skillPoint",
                x: inside(&mut self.r, self.cw, c.start),
            });
        }
        self.veins(chunks, &mut out);
        Ok(out)
    }

    /// Adern nach allen übrigen Würfen: je Ader ein eigener Ressourcen-Chunk (B-114).
    fn veins(&mut self, chunks: &[Chunk<'a>], out: &mut Vec<Entity<'a>>) {
        let mut spots: Vec<&Chunk> = chunks
            .iter()
            .filter(|c| {
                let k = c.kind;
                k != "hub" && k != "edge" && k != "exit" && k != "portal" && !is_event(k)
            })
            .filter(|c| self.planned_at(c.index).is_none())
            .collect();
        let b = self.b;
        let n = (b.veins.count - self.fixed_of(&b.veins.kind))
            .max(0)
            .min(spots.len() as i64);
        for _ in 0..n {
            let i = self.r.int(0, spots.len() as i64 - 1) as usize;
            out.push(Entity {
                kind: &b.veins.kind,
                x: inside(&mut self.r, self.cw, spots[i].start),
            });
            spots.remove(i);
        }
    }

    /// Eigener Strom eines geplanten Chunks: mit Chunk-Seed `<biom>:chunk:<seed>`, sonst `<biom>:<seed>:chunk:<platz>`.
    fn chunk_stream(&self, c: &PlannedChunk) -> Rng {
        if !c.seed.is_empty() {
            return Rng::new(&format!("{}:chunk:{}", self.b.id, c.seed));
        }
        Rng::new(&format!("{}:{}:chunk:{}", self.b.id, self.seed, c.slot))
    }

    /// Objekte eines geplanten Chunks: feste Objekte oder die Ressourcen der Chunk-Art aus dem eigenen Strom.
    fn planned_entities(&self, c: &Chunk<'a>, pc: &'a PlannedChunk, out: &mut Vec<Entity<'a>>) {
        let objects = fixed_objects(pc, self.cw);
        if !objects.is_empty() {
            out.extend(objects.into_iter().map(|(kind, at)| Entity {
                kind,
                x: c.start + at,
            }));
            return;
        }
        let mut r = self.chunk_stream(pc);
        let b = self.b;
        for (key, [lo, hi]) in b
            .resources_per_chunk
            .get(c.kind)
            .map_or(&[][..], |r| &r.0[..])
        {
            for _ in 0..r.int(*lo, *hi) {
                let x = self.clear_of_ruin(&mut r, c, pc);
                out.push(Entity {
                    kind: key.as_str(),
                    x,
                });
            }
        }
    }

    /// Position wie `inside`, mindestens RUIN_GAP_UNITS von der Ruine des Chunks entfernt (B-522).
    fn clear_of_ruin(&self, r: &mut Rng, c: &Chunk, pc: &PlannedChunk) -> f64 {
        let mut x = inside(r, self.cw, c.start);
        let Some(ruin) = pc.buildings.first() else {
            return x;
        };
        let ruin = c.start + ruin.at;
        for _ in 0..CLEAR_TRIES {
            if (x - ruin).abs() >= RUIN_GAP_UNITS {
                return x;
            }
            x = inside(r, self.cw, c.start);
        }
        if (x - ruin).abs() >= RUIN_GAP_UNITS {
            return x;
        }
        let lo = ruin - RUIN_GAP_UNITS;
        if lo >= c.start + EDGE_GAP_UNITS && x < ruin {
            return lo;
        }
        let hi = ruin + RUIN_GAP_UNITS;
        if hi <= c.start + self.cw - EDGE_GAP_UNITS {
            return hi;
        }
        ruin - RUIN_GAP_UNITS
    }
}
