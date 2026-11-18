//! Objekte eines Levels (Port von `engine/level/level.go` ab `entities`): Burg, Portale, Ausgang, Ereignisse,
//! Ressourcen, Skill-Punkte und Adern.

use crate::level::{Gen, inside, is_event};
use crate::types::{Chunk, Entity};

impl Gen {
    /// Go: `entities`, setzt Burg, Portale, Ausgang, Ereignisse, Ressourcen und Skill-Punkte.
    pub(crate) fn entities(&mut self, chunks: &[Chunk]) -> Result<Vec<Entity>, String> {
        let mut out = vec![Entity {
            kind: "castle".into(),
            x: self.hub,
            ..Entity::default()
        }];
        for c in chunks {
            if let Some(pc) = self.planned.get(&c.index) {
                out.extend(self.planned_entities(c, pc));
                continue;
            }
            if c.kind == "hub" || c.kind == "edge" {
                continue;
            }
            if c.kind == "exit" || c.kind == "portal" || is_event(&c.kind) {
                out.push(Entity {
                    kind: c.kind.clone(),
                    x: c.start_units + self.cw / 2.0,
                    ..Entity::default()
                });
                continue;
            }
            if let Some(resources) = self.b.resources_per_chunk.get(&c.kind) {
                for res in &resources.0 {
                    for _ in 0..self.r.int(res.value[0], res.value[1]) {
                        let x = inside(self.cw, &mut self.r, c);
                        out.push(Entity {
                            kind: res.key.clone(),
                            x,
                            ..Entity::default()
                        });
                    }
                }
            }
        }
        // Skill-Punkte: versteckt in beliebigen Chunks außerhalb von Hub und Rand.
        let hideouts: Vec<&Chunk> = chunks
            .iter()
            .filter(|c| c.kind != "hub" && c.kind != "edge" && !self.planned.contains_key(&c.index))
            .collect();
        for _ in 0..self.r.int(self.b.skill_points.min, self.b.skill_points.max) {
            // STAB(B-1079): Go bricht bei leerer Liste mit panic ab; hier ein Fehler (unerreichbar bei echten Biomen).
            if hideouts.is_empty() {
                return Err("keine Chunks für Skill-Punkte".into());
            }
            let c = hideouts[self.r.int(0, hideouts.len() as i64 - 1) as usize];
            let x = inside(self.cw, &mut self.r, c);
            out.push(Entity {
                kind: "skillPoint".into(),
                x,
                ..Entity::default()
            });
        }
        out.extend(self.veins(chunks));
        Ok(out)
    }

    /// Go: `veins`, setzt die Adern (B-114) nach allen übrigen Würfen, damit bestehende Seeds gleich bleiben: je Ader ein
    /// eigener Ressourcen-Chunk (nicht Hub, Rand, Ausgang, Portal oder Ereignis).
    pub(crate) fn veins(&mut self, chunks: &[Chunk]) -> Vec<Entity> {
        let mut spots: Vec<&Chunk> = chunks
            .iter()
            .filter(|c| {
                let k = c.kind.as_str();
                k != "hub"
                    && k != "edge"
                    && k != "exit"
                    && k != "portal"
                    && !is_event(k)
                    && !self.planned.contains_key(&c.index)
            })
            .collect();
        let fixed = self.fixed.get(&self.b.veins.kind).copied().unwrap_or(0);
        // feste Adern zählen mit
        let n = i64::min(i64::max(0, self.b.veins.count - fixed), spots.len() as i64);
        let mut out = Vec::new();
        for _ in 0..n {
            let i = self.r.int(0, spots.len() as i64 - 1) as usize;
            let x = inside(self.cw, &mut self.r, spots[i]);
            out.push(Entity {
                kind: self.b.veins.kind.clone(),
                x,
                ..Entity::default()
            });
            spots.remove(i);
        }
        out
    }
}
