//! Zustand des Generators, Rahmen, Portale und Ereignisse (Port von `engine/level/level.go` und `planned_gen.go`).
//! Die Reihenfolge der Würfe ist Teil des Vertrags.

/// Obergrenze der Chunk-Zahl: Schutz vor Riesen-Allokationen, die mit `panic = "abort"` den Server beenden würden.
pub const MAX_CHUNKS: i64 = 100_000;

use super::biome::{Biome, Input, Rules};
use super::plan::PlannedChunk;
use super::{EVENT_KINDS, hub_chunks_of, is_event};
use crate::rng::Rng;
use std::cmp::Ordering;

/// So nah am Bau des Endbosses liegt kein Camp (bosse.md § 1, B-645).
const LAIR_CAMP_GAP_UNITS: f64 = 80.0;

/// Zustand während der Erzeugung; `""` in `kinds` heißt: Chunk noch frei.
pub struct Gen<'a> {
    pub(super) b: &'a Biome,
    pub(super) rules: &'a Rules,
    pub(super) r: Rng,
    pub(super) seed: &'a str,
    pub(super) kinds: Vec<&'a str>,
    pub(super) cw: f64,
    pub(super) hub: f64,
    count: i64,
    hub_start: i64,
    hub_chunks: i64,
    /// Geplanter Chunk je Chunk-Index.
    planned: Vec<Option<&'a PlannedChunk>>,
    /// Feste Portale, Truhen, Camps und Adern je Art; feste Portale je Seite.
    fixed: Vec<(&'a str, i64)>,
    side_fixed: [i64; 2],
}

impl<'a> Gen<'a> {
    pub fn new(inp: &'a Input, seed: &'a str, count: i64) -> Self {
        let b = &inp.biome;
        Gen {
            b,
            rules: &inp.rules,
            r: Rng::new(&format!("{}:{seed}", b.id)),
            seed,
            kinds: Vec::new(),
            cw: b.chunk_width_units,
            hub: 0.0,
            count,
            hub_start: 0,
            hub_chunks: 0,
            planned: Vec::new(),
            fixed: Vec::new(),
            side_fixed: [0, 0],
        }
    }

    /// Mitte des Chunks `i`.
    pub(super) fn center(&self, i: usize) -> f64 {
        i as f64 * self.cw + self.cw / 2.0
    }

    pub(super) fn free(&self) -> Vec<usize> {
        (0..self.kinds.len())
            .filter(|&i| self.kinds[i].is_empty())
            .collect()
    }

    pub(super) fn planned_at(&self, i: usize) -> Option<&'a PlannedChunk> {
        self.planned.get(i).copied().flatten()
    }

    /// Feste Objekte der Art `kind` (Go: `g.fixed[kind]`).
    pub(super) fn fixed_of(&self, kind: &str) -> i64 {
        self.fixed
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, n)| *n)
    }

    /// Chunk-Anzahl, Hub in der Mitte, Ausgang und Rand. Mit eingestellter Anzahl entfällt der Wurf der Länge.
    pub fn frame(&mut self) -> Result<(), String> {
        let hub_chunks = hub_chunks_of(self.b);
        let mut count = self.count;
        if count == 0 {
            let len = self.r.int(self.b.length_units.min, self.b.length_units.max);
            count = (len as f64 / self.cw).round() as i64;
            if count % 2 != 0 {
                count += 1;
            }
        }
        if count < hub_chunks {
            return Err(format!(
                "Biom {}: {count} Chunks reichen nicht für {hub_chunks} Hub-Chunks",
                self.b.id
            ));
        }
        if count > MAX_CHUNKS {
            return Err(format!(
                "Biom {}: {count} Chunks sind zu viel (höchstens {MAX_CHUNKS})",
                self.b.id
            ));
        }
        let n = count as usize;
        self.kinds = vec![""; n];
        self.planned = vec![None; n];
        let hub_start = count / 2 - hub_chunks / 2;
        (self.hub_start, self.hub_chunks) = (hub_start, hub_chunks);
        for i in 0..hub_chunks {
            self.kinds[(hub_start + i) as usize] = "hub";
        }
        let (exit, edge) = if self.b.exit_side == "right" {
            (n - 1, 0)
        } else {
            (0, n - 1)
        };
        self.kinds[exit] = "exit";
        self.kinds[edge] = "edge";
        self.hub = (count / 2) as f64 * self.cw;
        Ok(())
    }

    /// Chunk-Index eines Platzes: +n liegt n Chunks rechts der Hub-Chunks, −n links davon.
    fn slot_index(&self, slot: i64) -> Result<usize, String> {
        let i = if slot > 0 {
            self.hub_start + self.hub_chunks + slot - 1
        } else {
            self.hub_start + slot
        };
        usize::try_from(i)
            .ok()
            .filter(|&i| i < self.kinds.len())
            .ok_or_else(|| format!("Platz {slot:+} liegt außerhalb des Levels"))
    }

    /// Belegt die geplanten Plätze und zählt die festen Objekte für die Biom-Summen.
    pub fn reserve(&mut self, plan: &'a [PlannedChunk]) -> Result<(), String> {
        for c in plan {
            let i = self.slot_index(c.slot)?;
            self.kinds[i] = &c.kind;
            self.planned[i] = Some(c);
            let side = usize::from(c.slot > 0);
            for (kind, _) in fixed_objects(c, self.cw) {
                match self.fixed.iter_mut().find(|(k, _)| *k == kind) {
                    Some((_, n)) => *n += 1,
                    None => self.fixed.push((kind, 1)),
                }
                if kind == "portal" {
                    self.side_fixed[side] += 1;
                }
            }
        }
        Ok(())
    }

    /// Portale abwechselnd links/rechts mit Mindestabstand zum Hub, Startseite zufällig.
    pub fn place_portals(&mut self) -> Result<(), String> {
        let mut left = self.r.next() < 0.5;
        if self.side_fixed[0] != self.side_fixed[1] {
            left = self.side_fixed[0] < self.side_fixed[1]; // beginnt auf der Seite mit weniger festen Portalen
        }
        let fixed = self.fixed_of("portal");
        for n in 0..(self.b.portals.count - fixed).max(0) {
            let mut options = self.eligible(left);
            if options.is_empty() {
                options = self.eligible(!left);
            }
            if options.is_empty() {
                let (want, id) = (self.b.portals.count, &self.b.id);
                return Err(format!(
                    "kein Platz für Portal {}/{want} in {id}",
                    n + 1 + fixed
                ));
            }
            let i = options[self.r.pick_index(options.len())];
            self.kinds[i] = "portal";
            left = !left;
        }
        Ok(())
    }

    fn eligible(&self, left: bool) -> Vec<usize> {
        let min = self.b.portals.min_distance_from_hub_units;
        (0..self.kinds.len())
            .filter(|&i| {
                let dx = self.center(i) - self.hub;
                let on_side = if left { dx < 0.0 } else { dx > 0.0 };
                self.kinds[i].is_empty() && on_side && dx.abs() >= min
            })
            .collect()
    }

    /// Bau des Endbosses an der inneren Kante des Ausgangs-Chunks (ohne Würfel).
    pub fn lair_x(&self) -> f64 {
        if self.b.exit_side == "right" {
            return (self.kinds.len() - 1) as f64 * self.cw;
        }
        self.cw
    }

    /// Truhen- und Camp-Chunks; Camps bevorzugt außerhalb der Verteidigungszone, sicher und nah am Hub (B-261).
    pub fn place_events(&mut self) -> Result<(), String> {
        for kind in EVENT_KINDS {
            let want = self.b.event(kind);
            let mut slots = self.free();
            self.r.shuffle(&mut slots);
            let count = (self.r.int(want.min, want.max) - self.fixed_of(kind)).max(0) as usize;
            if slots.len() < count {
                return Err(format!(
                    "level zu kurz für {count}x {kind} (Seed {})",
                    self.seed
                ));
            }
            if kind == "recruitCamp" {
                let safe = self.safe_camp_slots();
                let less = |a: usize, b: usize| self.camp_before(&safe, a, b);
                slots.sort_by(|&a, &b| {
                    if less(a, b) {
                        Ordering::Less
                    } else if less(b, a) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
            }
            for &s in &slots[..count] {
                self.kinds[s] = kind;
            }
        }
        Ok(())
    }

    /// Ordnung der Camp-Plätze: außerhalb der Verteidigungszone, dann sicher, dann nah am Hub.
    fn camp_before(&self, safe: &[bool], a: usize, b: usize) -> bool {
        let outside = self.rules.site_placement.outside_units;
        let (da, db) = (self.center(a) - self.hub, self.center(b) - self.hub);
        let (oa, ob) = (da.abs() > outside, db.abs() > outside);
        if oa != ob {
            return oa;
        }
        if safe[a] != safe[b] {
            return safe[a];
        }
        da.abs() < db.abs()
    }

    /// Chunks, in denen ein Camp sicher steht: bei Biomen mit Bau weit genug vom Bau und nie hinter einem Portal.
    fn safe_camp_slots(&self) -> Vec<bool> {
        let portals: Vec<f64> = (0..self.kinds.len())
            .filter(|&i| self.kinds[i] == "portal")
            .map(|i| self.center(i))
            .collect();
        let lair = self.lair_x();
        let hub = self.hub;
        (0..self.kinds.len())
            .map(|i| {
                let x = self.center(i);
                let behind = portals
                    .iter()
                    .any(|&p| (p - hub) * (x - hub) > 0.0 && (p - hub).abs() < (x - hub).abs());
                !self.b.lair || ((x - lair).abs() >= LAIR_CAMP_GAP_UNITS && !behind)
            })
            .collect()
    }

    /// Füllt die freien Chunks nach Gewicht.
    pub fn fill_free(&mut self) -> Result<(), String> {
        let free = self.free();
        let b: &'a Biome = self.b;
        let weights = &b.chunk_weights.0;
        if !free.is_empty() && weights.iter().map(|w| w.1).sum::<f64>() <= 0.0 {
            return Err(format!(
                "Biom {}: chunkWeights braucht mindestens ein positives Gewicht",
                self.b.id
            ));
        }
        for i in free {
            self.kinds[i] = self.r.weighted(weights);
        }
        Ok(())
    }
}

/// Objekte eines geplanten Chunks mit At ab Chunk-Anfang; leer bei einer Chunk-Art, die ihre Objekte würfelt.
pub(super) fn fixed_objects(c: &PlannedChunk, cw: f64) -> Vec<(&str, f64)> {
    if !c.objects.is_empty() {
        return c.objects.iter().map(|o| (o.kind.as_str(), o.at)).collect();
    }
    if is_event(&c.kind) || c.kind == "portal" {
        return vec![(c.kind.as_str(), cw / 2.0)];
    }
    Vec::new()
}
