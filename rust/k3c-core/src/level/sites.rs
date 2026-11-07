//! Mauer-, Turm- und Baumschul-Plätze je Seite und Zone (Port von `engine/level/sites.go`, B-652, B-636) aus den
//! eigenen Strömen `<biom>:<seed>:wallsites` und `<biom>:<seed>:nursery`; der Level-Strom bleibt unberührt.

use super::biome::SitePlacement;
use super::{Chunk, Entity, Gen, HARBOR_KIND};
use crate::rng::Rng;

/// Verteilter Platz einer Seite: absolute Lage in Units, Zone n von innen (ab 1).
#[derive(Clone, Debug, PartialEq)]
pub struct WallSite<'a> {
    pub side: i64,
    pub kind: &'a str,
    pub x: f64,
    pub zone: i64,
}

/// Legt die Plätze einer Zone [lo, hi) (Abstand zur Hub-Mitte) einer Seite an und trägt sie in `taken` ein.
struct ZonePlacer<'g, 'a> {
    g: &'g Gen<'a>,
    p: &'g SitePlacement,
    gap: f64,
    side: i64,
    zone: i64,
    lo: f64,
    hi: f64,
    chunks: &'g [Chunk<'a>],
    walls: Vec<f64>,
    out: Vec<WallSite<'a>>,
}

impl<'a> Gen<'a> {
    /// Je Seite und Zone Mauern und Türme; Reihenfolge der Würfe: links, dann rechts, je Zone von innen.
    pub fn place_sites(&self, entities: &[Entity<'a>], chunks: &[Chunk<'a>]) -> Vec<WallSite<'a>> {
        let p = &self.rules.site_placement;
        let gap = self.rules.pay_gap();
        let mut r = Rng::new(&format!("{}:{}:wallsites", self.b.id, self.seed));
        let mut taken = self.blocked_xs(entities);
        let mut out = Vec::new();
        for side in [-1, 1] {
            let limit = self.side_limit(side, entities, p.edge_margin_units);
            for zone in 1..=p.zones {
                let lo = p.zone_start_units + (zone - 1) as f64 * self.cw;
                let hi = (lo + self.cw).min(limit);
                if hi - lo < 2.0 * gap {
                    continue;
                }
                let mut z = ZonePlacer {
                    g: self,
                    p,
                    gap,
                    side,
                    zone,
                    lo,
                    hi,
                    chunks,
                    walls: Vec::new(),
                    out: Vec::new(),
                };
                z.place(&mut r, &mut taken);
                out.append(&mut z.out);
            }
        }
        out.extend(self.place_nurseries(chunks, &mut taken));
        out
    }

    /// Je Seite ein Baumschul-Platz in einem Wald-Chunk des Wald-Bioms (Tiefe 0), mindestens `nurseryMinUnits` vom Hub.
    fn place_nurseries(&self, chunks: &[Chunk<'a>], taken: &mut Vec<f64>) -> Vec<WallSite<'a>> {
        if self.b.depth != 0 {
            return Vec::new();
        }
        let min = self.rules.site_placement.nursery_min_units;
        let mut r = Rng::new(&format!("{}:{}:nursery", self.b.id, self.seed));
        let mut out = Vec::new();
        for side in [-1i64, 1] {
            let (mut far, mut near) = (Vec::new(), Vec::new());
            for (i, c) in chunks.iter().enumerate() {
                let d = (self.center(i) - self.hub) * side as f64;
                if c.kind != "forest" || d <= 0.0 {
                    continue;
                }
                if d >= min { far.push(i) } else { near.push(i) }
            }
            let mut pool = far;
            if pool.is_empty() && !near.is_empty() {
                pool = near[near.len() - 1..].to_vec(); // der äußerste Wald-Chunk der Seite
            }
            if pool.is_empty() {
                continue;
            }
            let i = pool[r.int(0, pool.len() as i64 - 1) as usize];
            if let Some(x) = self.free_near(self.center(i), taken) {
                taken.push(x);
                out.push(WallSite {
                    side,
                    kind: "nursery",
                    x,
                    zone: 1,
                });
            }
        }
        out
    }

    /// Von `x` aus nach außen (±1, ±2 … Units) der nächste Ort mit payGapUnits Abstand zu allen belegten Orten.
    fn free_near(&self, x: f64, taken: &[f64]) -> Option<f64> {
        let gap = self.rules.pay_gap();
        let mut d = 0.0;
        while d <= self.cw / 2.0 - 2.0 {
            for c in [x + d, x - d] {
                if !taken.iter().any(|t| (t - c).abs() < gap) {
                    return Some(c);
                }
            }
            d += 1.0;
        }
        None
    }

    /// Orte, von denen jedes Zahlziel Abstand hält: Hub-Plätze, Burg, Ereignisse, Portale, Ausgang, Bau und Adern.
    fn blocked_xs(&self, entities: &[Entity]) -> Vec<f64> {
        let mut out: Vec<f64> = self
            .rules
            .hub_sites
            .iter()
            .map(|h| self.hub + h.offset_units)
            .collect();
        for e in entities {
            let blocks = matches!(
                e.kind,
                "castle" | "portal" | "exit" | "chest" | "recruitCamp" | "lair"
            ) || e.kind == HARBOR_KIND
                || e.kind.ends_with("Vein");
            if blocks {
                out.push(e.x);
            }
        }
        out
    }

    /// Größte Entfernung zur Hub-Mitte auf einer Seite: kurz vor dem nächsten Portal bzw. Level-Rand.
    fn side_limit(&self, side: i64, entities: &[Entity], margin: f64) -> f64 {
        let mut limit = self.kinds.len() as f64 * self.cw / 2.0 - margin;
        for e in entities {
            if e.kind != "portal" && e.kind != "exit" {
                continue;
            }
            let d = (e.x - self.hub) * side as f64;
            if d > 0.0 && d - margin < limit {
                limit = d - margin;
            }
        }
        limit
    }

    /// `x` liegt in einem Lava-Chunk (Abschneiden zur Null wie `int(…)` in Go).
    fn lava_at(&self, chunks: &[Chunk], x: f64) -> bool {
        let i = (x / self.cw) as i64;
        i >= 0 && (i as usize) < chunks.len() && chunks[i as usize].kind == "lava"
    }
}

impl<'a> ZonePlacer<'_, 'a> {
    fn place(&mut self, r: &mut Rng, taken: &mut Vec<f64>) {
        let p = self.p;
        let mut extra_walls = r.int(p.per_zone_min, p.per_zone_max);
        let mut extra_towers = r.int(p.per_zone_min, p.per_zone_max);
        if self.zone == 1 {
            // fester Baugrund wie die frühere Linie 1 (B-652, E); die Würfe darüber hinaus bleiben Zusatzplätze
            self.add("wall", p.zone1_wall_units, taken);
            self.walls.push(p.zone1_wall_units);
            self.add("tower", p.zone1_tower_units, taken);
            extra_walls -= 1;
            extra_towers -= 1;
        }
        for _ in 0..extra_walls {
            if let Some(d) = self.pick(r, taken, self.lo, self.hi) {
                self.add("wall", d, taken);
                self.walls.push(d);
            }
        }
        for _ in 0..extra_towers {
            let d = match self.near_wall(r, taken) {
                Some(d) => Some(d),
                None => self.pick(r, taken, self.lo, self.hi),
            };
            if let Some(d) = d {
                self.add("tower", d, taken);
            }
        }
    }

    /// `d` liegt in der Zone, nicht in Lava und hält payGapUnits zu allen belegten Orten.
    fn free(&self, d: f64, taken: &[f64]) -> bool {
        if d < self.lo || d >= self.hi {
            return false;
        }
        let x = self.g.hub + self.side as f64 * d;
        if self.g.lava_at(self.chunks, x) {
            return false;
        }
        !taken.iter().any(|t| (t - x).abs() < self.gap)
    }

    /// Würfelt einen freien Abstand in [from, to]; sonst lückenlos den ersten freien der Zone.
    fn pick(&self, r: &mut Rng, taken: &[f64], from: f64, to: f64) -> Option<f64> {
        for _ in 0..40 {
            let d = (from + r.next() * (to - from)).round();
            if self.free(d, taken) {
                return Some(d);
            }
        }
        let mut d = self.lo.ceil();
        while d < self.hi {
            if self.free(d, taken) {
                return Some(d);
            }
            d += 1.0;
        }
        None
    }

    /// Würfelt, ob ein Turm neben einer Mauer der Zone steht (towerNearShare), und liefert dessen Abstand.
    fn near_wall(&self, r: &mut Rng, taken: &[f64]) -> Option<f64> {
        if self.walls.is_empty() || r.next() >= self.p.tower_near_share {
            return None;
        }
        let w = self.walls[r.int(0, self.walls.len() as i64 - 1) as usize];
        let sign = if r.next() < 0.5 { -1.0 } else { 1.0 };
        let (lo, hi) = (self.p.tower_near_min_units, self.p.tower_near_max_units);
        let off = (lo + r.next() * (hi - lo)).round();
        [w + sign * off, w - sign * off]
            .into_iter()
            .find(|&d| self.free(d, taken))
    }

    fn add(&mut self, kind: &'static str, d: f64, taken: &mut Vec<f64>) {
        let x = self.g.hub + self.side as f64 * d;
        taken.push(x);
        self.out.push(WallSite {
            side: self.side,
            kind,
            x,
            zone: self.zone,
        });
    }
}

/// Baugrund: je Seite die erste Mauer und der erste Turm der Zone 1 (B-652, Variante A).
pub fn free_ground(sites: &[WallSite]) -> Vec<f64> {
    let mut free = Vec::new();
    for side in [-1, 1] {
        for kind in ["wall", "tower"] {
            if let Some(s) = sites
                .iter()
                .find(|s| s.side == side && s.zone == 1 && s.kind == kind)
            {
                free.push(s.x);
            }
        }
    }
    free
}
