//! Plätze an den Mauerlinien, Baumschulen und Baugrund (Port von `engine/level/sites.go`, B-652, B-636).
//!
//! Die Plätze würfeln aus eigenen Strömen `<biom>:<seed>:wallsites` und `<biom>:<seed>:nursery`; der Hauptstrom bleibt
//! unberührt. `taken` (Go: `*[]float64`) ist eine Liste, die mehrere Aufrufe füllen: `&mut Vec<f64>` als Parameter.

use k3c_det::fmin;
use k3c_rng::Rng;
use serde::Deserialize;

use crate::level::{Gen, HARBOR_KIND};
use crate::rules::Rules;
use crate::types::{Chunk, Entity, Layout, WallSite};

/// Go: `SitePlacement`, die Felder aus `data/hub.json` › `sitePlacement` (B-652).
#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
#[serde(default)]
pub struct SitePlacement {
    /// Abstand der ersten Zone zur Hub-Mitte.
    #[serde(rename = "zoneStartUnits")]
    pub zone_start_units: f64,
    /// Fester Baugrund-Platz der Zone 1 je Seite: Mauer (wie die frühere Linie 1) …
    #[serde(rename = "zone1WallUnits")]
    pub zone1_wall_units: f64,
    /// … und Turm.
    #[serde(rename = "zone1TowerUnits")]
    pub zone1_tower_units: f64,
    /// Zonen je Seite (Zone n ab Hub-Stufe n sichtbar).
    #[serde(rename = "zones")]
    pub zones: i64,
    /// Mauern und Türme je Zone, mindestens …
    #[serde(rename = "perZoneMin")]
    pub per_zone_min: i64,
    /// … und höchstens.
    #[serde(rename = "perZoneMax")]
    pub per_zone_max: i64,
    /// Anteil der Türme neben einer Mauer.
    #[serde(rename = "towerNearShare")]
    pub tower_near_share: f64,
    /// Abstand eines nahen Turms zur Mauer, von …
    #[serde(rename = "towerNearMinUnits")]
    pub tower_near_min_units: f64,
    /// … bis.
    #[serde(rename = "towerNearMaxUnits")]
    pub tower_near_max_units: f64,
    /// Freiraum von Bäumen und Felsen (Sim, BS12.2).
    #[serde(rename = "clearRadiusUnits")]
    pub clear_radius_units: f64,
    /// Abstand zum Level-Rand und zum nächsten Portal.
    #[serde(rename = "edgeMarginUnits")]
    pub edge_margin_units: f64,
    /// Portale, Gegnerlager und Vorposten geplanter Chunks liegen weiter vom Hub entfernt (frühere äußerste Linie).
    #[serde(rename = "outsideUnits")]
    pub outside_units: f64,
    /// Baumschul-Plätze liegen mindestens so weit vom Hub (nur Wald, hinter Zone 2).
    #[serde(rename = "nurseryMinUnits")]
    pub nursery_min_units: f64,
}

/// Go: `zonePlacer`, legt die Plätze einer Zone [lo, hi) (Abstand zur Hub-Mitte) einer Seite an. Hält nur Werte; die
/// belegten Orte (`taken`), den Strom und die Chunks bekommt jeder Aufruf.
pub(crate) struct ZonePlacer {
    p: SitePlacement,
    pay_gap: f64,
    hub: f64,
    cw: f64,
    side: i64,
    zone: i64,
    lo: f64,
    hi: f64,
    walls: Vec<f64>,
    out: Vec<WallSite>,
}

/// Go: `lavaAt`: x liegt in einem Lava-Chunk.
fn lava_at(cw: f64, chunks: &[Chunk], x: f64) -> bool {
    let i = (x / cw) as i64;
    i >= 0 && (i as usize) < chunks.len() && chunks[i as usize].kind == "lava"
}

impl ZonePlacer {
    /// Go: `free`: d (Abstand zur Hub-Mitte) liegt in der Zone, nicht in Lava und hält `pay_gap` zu allen belegten Orten.
    fn free(&self, d: f64, taken: &[f64], chunks: &[Chunk]) -> bool {
        if d < self.lo || d >= self.hi {
            return false;
        }
        let x = self.hub + self.side as f64 * d;
        if lava_at(self.cw, chunks, x) {
            return false;
        }
        !taken.iter().any(|t| (t - x).abs() < self.pay_gap)
    }

    /// Go: `pick`, würfelt einen freien Abstand in [from, to]; findet das nichts, nimmt es lückenlos den ersten freien
    /// der Zone.
    fn pick(&self, r: &mut Rng, taken: &[f64], chunks: &[Chunk], from: f64, to: f64) -> Option<f64> {
        for _ in 0..40 {
            let d = (from + r.next() * (to - from)).round();
            if self.free(d, taken, chunks) {
                return Some(d);
            }
        }
        let mut d = self.lo.ceil();
        while d < self.hi {
            if self.free(d, taken, chunks) {
                return Some(d);
            }
            d += 1.0;
        }
        None
    }

    /// Go: `nearWall`, würfelt, ob ein Turm neben einer Mauer der Zone steht (`towerNearShare`), und liefert dessen
    /// Abstand.
    fn near_wall(&self, r: &mut Rng, taken: &[f64], chunks: &[Chunk]) -> Option<f64> {
        if self.walls.is_empty() || r.next() >= self.p.tower_near_share {
            return None;
        }
        let w = self.walls[r.int(0, self.walls.len() as i64 - 1) as usize];
        let sign = if r.next() < 0.5 { -1.0 } else { 1.0 };
        let (min, max) = (self.p.tower_near_min_units, self.p.tower_near_max_units);
        let off = (min + r.next() * (max - min)).round();
        [w + sign * off, w - sign * off]
            .into_iter()
            .find(|&d| self.free(d, taken, chunks))
    }

    fn add(&mut self, taken: &mut Vec<f64>, kind: &str, d: f64) {
        let x = self.hub + self.side as f64 * d;
        taken.push(x);
        self.out.push(WallSite {
            side: self.side,
            kind: kind.to_string(),
            x,
            zone: self.zone,
        });
    }
}

#[allow(dead_code)] // RU6.5: generate ruft die Methoden
impl Gen {
    /// Go: `placeSites`, verteilt je Seite und Zone 1 bis `perZoneMax` Mauern und Türme aus dem eigenen Strom
    /// `<biom>:<seed>:wallsites` (die übrigen Ströme bleiben unberührt). Mindestabstand zweier Zahlziele ist
    /// `pay_gap_units`; nahe Türme stehen `towerNearMin…Max` neben einer Mauer der Zone. Reihenfolge der Würfe: links,
    /// dann rechts, je Zone von innen.
    pub(crate) fn place_sites(&self, entities: &[Entity], chunks: &[Chunk]) -> Vec<WallSite> {
        let p = self.rules.site_placement;
        let mut r = Rng::new(&format!("{}:{}:wallsites", self.b.id, self.seed));
        let mut taken = self.blocked_xs(entities);
        let mut out = Vec::new();
        for side in [-1_i64, 1] {
            let limit = self.side_limit(side, entities, p.edge_margin_units);
            for zone in 1..=p.zones {
                let lo = p.zone_start_units + (zone - 1) as f64 * self.cw;
                let hi = fmin(lo + self.cw, limit);
                if hi - lo < 2.0 * self.rules.pay_gap_units {
                    continue;
                }
                out.extend(self.place_zone(&mut r, (side, zone, lo, hi), &mut taken, chunks));
            }
        }
        out.extend(self.place_nurseries(chunks, &mut taken));
        out
    }

    /// Go: `placeZone`, die Plätze einer Zone `(side, zone, lo, hi)`.
    fn place_zone(
        &self,
        r: &mut Rng,
        zone: (i64, i64, f64, f64),
        taken: &mut Vec<f64>,
        chunks: &[Chunk],
    ) -> Vec<WallSite> {
        let (side, nr, lo, hi) = zone;
        let p = self.rules.site_placement;
        let mut z = ZonePlacer {
            p,
            pay_gap: self.rules.pay_gap_units,
            hub: self.hub,
            cw: self.cw,
            side,
            zone: nr,
            lo,
            hi,
            walls: Vec::new(),
            out: Vec::new(),
        };
        let mut extra_walls = r.int(p.per_zone_min, p.per_zone_max);
        let mut extra_towers = r.int(p.per_zone_min, p.per_zone_max);
        if nr == 1 {
            // fester Baugrund wie die frühere Linie 1 (B-652, E); die Würfe darüber hinaus bleiben Zusatzplätze
            z.add(taken, "wall", p.zone1_wall_units);
            z.walls.push(p.zone1_wall_units);
            z.add(taken, "tower", p.zone1_tower_units);
            extra_walls -= 1;
            extra_towers -= 1;
        }
        for _ in 0..extra_walls {
            if let Some(d) = z.pick(r, taken, chunks, lo, hi) {
                z.add(taken, "wall", d);
                z.walls.push(d);
            }
        }
        for _ in 0..extra_towers {
            let d = z
                .near_wall(r, taken, chunks)
                .or_else(|| z.pick(r, taken, chunks, lo, hi));
            if let Some(d) = d {
                z.add(taken, "tower", d);
            }
        }
        z.out
    }

    /// Go: `placeNurseries`, legt je Seite einen Baumschul-Platz in einen Wald-Chunk (Art `forest`) des Wald-Bioms
    /// (Tiefe 0), mindestens `nurseryMinUnits` vom Hub, aus dem eigenen Strom `<biom>:<seed>:nursery` (B-636). Gibt es
    /// keinen Wald-Chunk in Reichweite, nimmt er den äußersten der Seite, sonst entfällt der Platz.
    fn place_nurseries(&self, chunks: &[Chunk], taken: &mut Vec<f64>) -> Vec<WallSite> {
        let mut out = Vec::new();
        if self.b.depth != 0 {
            return out;
        }
        let mut r = Rng::new(&format!("{}:{}:nursery", self.b.id, self.seed));
        let min = self.rules.site_placement.nursery_min_units;
        let (mut far, mut near) = (Vec::new(), Vec::new());
        for side in [-1_i64, 1] {
            far.clear();
            near.clear();
            for (i, c) in chunks.iter().enumerate() {
                let d = (self.center(i as i64) - self.hub) * side as f64;
                if c.kind != "forest" || d <= 0.0 {
                    continue;
                }
                if d >= min { far.push(i) } else { near.push(i) }
            }
            let pool: &[usize] = if far.is_empty() && !near.is_empty() {
                &near[near.len() - 1..]
            } else {
                &far
            };
            if pool.is_empty() {
                continue;
            }
            let i = pool[r.int(0, pool.len() as i64 - 1) as usize];
            if let Some(x) = self.free_near(self.center(i as i64), taken) {
                taken.push(x);
                out.push(WallSite {
                    side,
                    kind: "nursery".into(),
                    x,
                    zone: 1,
                });
            }
        }
        out
    }

    /// Go: `freeNear`, sucht von x aus nach außen (±1, ±2 … Units) den nächsten Ort, der `pay_gap_units` zu allen
    /// belegten Orten hält.
    fn free_near(&self, x: f64, taken: &[f64]) -> Option<f64> {
        let mut d = 0.0;
        while d <= self.cw / 2.0 - 2.0 {
            for c in [x + d, x - d] {
                if !taken.iter().any(|t| (t - c).abs() < self.rules.pay_gap_units) {
                    return Some(c);
                }
            }
            d += 1.0;
        }
        None
    }

    /// Go: `blockedXs`, die Orte, von denen jedes Zahlziel `pay_gap_units` Abstand halten muss: Hub-Plätze, Burg und
    /// alle Ereignisse, Portale, Ausgang und Bau.
    fn blocked_xs(&self, entities: &[Entity]) -> Vec<f64> {
        let mut out: Vec<f64> = self.sites.iter().map(|h| self.hub + h.offset).collect();
        for e in entities {
            let blocks = match e.kind.as_str() {
                "castle" | "portal" | "exit" | "chest" | "recruitCamp" | "lair" => true,
                k if k == HARBOR_KIND => true,
                // feste und gewürfelte Adern: die Plätze halten Abstand
                k => k.ends_with("Vein"),
            };
            if blocks {
                out.push(e.x);
            }
        }
        out
    }

    /// Go: `sideLimit`, die größte Entfernung zur Hub-Mitte auf einer Seite: kurz vor dem nächsten Portal bzw. Level-Rand.
    fn side_limit(&self, side: i64, entities: &[Entity], margin: f64) -> f64 {
        let mut limit = self.kinds.len() as f64 * self.cw / 2.0 - margin;
        for e in entities.iter().filter(|e| e.kind == "portal" || e.kind == "exit") {
            let d = (e.x - self.hub) * side as f64;
            if d > 0.0 && d - margin < limit {
                limit = d - margin;
            }
        }
        limit
    }
}

/// Go: `freeGround`, liefert den Baugrund (B-652, Variante A): Je Seite liegen die erste Mauer und der erste Turm der
/// Zone 1 auf freiem Grund; die Sim legt im Freiraum dieser Plätze keine Bäume und Felsen an (World, FreeGround). Die
/// Level-Entitäten bleiben unverändert (Vorschau, Umwandlung in feste Objekte).
pub fn free_ground(sites: &[WallSite]) -> Vec<f64> {
    let mut free = Vec::new();
    for side in [-1_i64, 1] {
        for kind in ["wall", "tower"] {
            if let Some(s) = sites.iter().find(|s| s.side == side && s.zone == 1 && s.kind == kind) {
                free.push(s.x);
            }
        }
    }
    free
}

impl Layout {
    /// Go: `OnFreeGround`: x liegt im Freiraum eines Baugrund-Platzes (`Layout::free_ground`).
    pub fn on_free_ground(&self, x: f64, rules: &Rules) -> bool {
        let radius = rules.site_placement.clear_radius_units;
        self.free_ground.iter().any(|g| (x - g).abs() < radius)
    }
}

#[cfg(test)]
#[path = "sites_tests.rs"]
mod sites_tests;
