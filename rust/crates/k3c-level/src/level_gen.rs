//! `generate` und `generate_with` (Port von `engine/level/level.go` bis `placeEvents`): die Reihenfolge der Würfe ist
//! Teil des Ergebnisses und bleibt wie in Go: `check_count` → `validate_plan_with` → `resolve_nature` → `Gen` bauen →
//! `frame` → `reserve` → `place_portals` → `place_events` → Gewichte je freiem Chunk → `entities` → Bau → stabil nach
//! `x` sortieren → `assign_variants` → `place_sites`.

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_rng::{Rng, Weight};

use crate::backdrop::backdrops;
use crate::biome::{Biome, Ordered};
use crate::error::Fehler;
use crate::level::{EVENT_KINDS, Gen, LAIR_CAMP_GAP_UNITS, check_count, hub_chunks_of};
use crate::lines::sites_of;
use crate::nature::{assign_variants, resolve_nature, with_nature};
use crate::planned::validate_plan_with;
use crate::planned_gen::Options;
use crate::rules::Rules;
use crate::sites::free_ground;
use crate::types::{Chunk, Entity, Layout, NatureValues};

/// Go: `Generate`, baut das Level für Biom und Seed ohne Planung und mit den Mauerlinien der Basis.
pub fn generate(b: &Biome, seed: &str, rules: &Arc<Rules>) -> Result<Layout, Fehler> {
    generate_with(b, seed, &Options::default(), rules)
}

/// Go: `GenerateWith`, baut das Level mit Planung und Plätzen aus `o`; der Nullwert von `Options` ergibt dasselbe Level
/// wie `generate`. Eine ungültige Planung wird abgelehnt (`validate_plan_with`). `rules` ist als `Arc` übergeben, weil
/// `Gen` es hält (keine Lebenszeit an Typen).
pub fn generate_with(b: &Biome, seed: &str, o: &Options, rules: &Arc<Rules>) -> Result<Layout, Fehler> {
    check_count(b, o.chunk_count).map_err(Fehler::Level)?;
    validate_plan_with(b, &o.planned, o.chunk_count, o.hub_sites.as_deref(), rules)
        .map_err(|e| Fehler::Level(format!("geplante Chunks: {e}")))?;
    let nature = resolve_nature(b, seed, &o.nature).map_err(|e| Fehler::Level(format!("natur: {e}")))?;
    let mut g = new_gen(b, seed, o, &nature, rules);
    g.frame();
    g.reserve(&o.planned);
    g.place_portals().map_err(Fehler::Level)?;
    g.place_events(seed).map_err(Fehler::Level)?;
    let w = weights(&g.b.chunk_weights);
    for i in g.free() {
        g.kinds[i as usize] = g.r.weighted(&w);
    }
    let chunks: Vec<Chunk> = g
        .kinds
        .iter()
        .enumerate()
        .map(|(i, k)| Chunk {
            index: i as i64,
            kind: k.clone(),
            start_units: i as f64 * g.cw,
        })
        .collect();
    let mut entities = g.entities(&chunks).map_err(Fehler::Level)?;
    if g.b.lair {
        entities.push(Entity {
            kind: "lair".into(),
            x: g.lair_x(),
            ..Entity::default()
        });
    }
    entities.sort_by(|a, b| a.x.total_cmp(&b.x)); // stabil, wie sort.SliceStable
    assign_variants(b, seed, &nature, &mut entities);
    let sites = g.place_sites(&entities, &chunks);
    let width = chunks.len() as f64 * g.cw;
    Ok(Layout {
        seed: seed.to_string(),
        biome_id: b.id.clone(),
        width_units: width,
        chunk_width_units: g.cw,
        hub_center_units: g.hub,
        free_ground: free_ground(&sites),
        ruins: g.ruins_of(&chunks),
        guards: g.guards_of(&chunks),
        backdrops: backdrops(&b.id, seed, b.backdrop_variants, width),
        chunks,
        entities,
        sites,
        nature,
    })
}

/// Der Anfangszustand von `generate_with`: Biom mit Natur, Haupt-Strom `<biom>:<seed>`, Bau nach `Options`.
fn new_gen(b: &Biome, seed: &str, o: &Options, nature: &NatureValues, rules: &Arc<Rules>) -> Gen {
    let mut g = Gen {
        b: with_nature(b, nature),
        r: Rng::new(&format!("{}:{seed}", b.id)),
        seed: seed.to_string(),
        kinds: Vec::new(),
        cw: b.chunk_width_units,
        hub: 0.0,
        count: o.chunk_count,
        sites: sites_of(o.hub_sites.as_deref(), &rules.hub_sites),
        hub_start: 0,
        hub_chunks: 0,
        planned: BTreeMap::new(),
        fixed: BTreeMap::new(),
        side_fixed: [0, 0],
        planned_res: b.resources_per_chunk.clone(),
        rules: Arc::clone(rules),
    };
    g.b.lair = o.has_lair(b);
    g
}

fn weights(o: &Ordered<f64>) -> Vec<Weight> {
    o.0.iter()
        .map(|e| Weight {
            key: e.key.clone(),
            w: e.value,
        })
        .collect()
}

impl Gen {
    /// Go: `frame`, legt Chunk-Anzahl, Hub in der Mitte, Ausgang und Rand fest. Mit eingestellter Anzahl (`count`)
    /// entfällt der Wurf der Länge; ohne bleibt die Reihenfolge der Würfe wie bisher.
    pub(crate) fn frame(&mut self) {
        // Gerade Anzahl Chunks, damit der Hub exakt in der Mitte liegt.
        let hub_chunks = hub_chunks_of(&self.b);
        let mut count = self.count;
        if count == 0 {
            let len = self.r.int(self.b.length_units.min, self.b.length_units.max);
            count = (len as f64 / self.cw).round() as i64;
            if count % 2 != 0 {
                count += 1;
            }
        }
        self.kinds = vec![String::new(); count as usize];
        let hub_start = count / 2 - hub_chunks / 2;
        self.hub_start = hub_start;
        self.hub_chunks = hub_chunks;
        for i in 0..hub_chunks {
            self.kinds[(hub_start + i) as usize] = "hub".into();
        }
        let (mut exit, mut edge) = (0, count - 1);
        if self.b.exit_side == "right" {
            (exit, edge) = (count - 1, 0);
        }
        self.kinds[exit as usize] = "exit".into();
        self.kinds[edge as usize] = "edge".into();
        self.hub = (count / 2) as f64 * self.cw;
    }

    /// Go: `placePortals`, verteilt die Portale abwechselnd links/rechts mit Mindestabstand zum Hub, Startseite zufällig.
    pub(crate) fn place_portals(&mut self) -> Result<(), String> {
        let mut left = self.r.next() < 0.5;
        if self.side_fixed[0] != self.side_fixed[1] {
            left = self.side_fixed[0] < self.side_fixed[1]; // beginnt auf der Seite mit weniger festen Portalen
        }
        let fixed = self.fixed.get("portal").copied().unwrap_or(0);
        let mut options = Vec::new();
        for n in 0..self.b.portals.count - fixed {
            self.eligible(left, &mut options);
            if options.is_empty() {
                self.eligible(!left, &mut options);
            }
            if options.is_empty() {
                return Err(format!(
                    "kein Platz für Portal {}/{} in {}",
                    n + 1 + fixed,
                    self.b.portals.count,
                    self.b.id
                ));
            }
            let i = k3c_rng::pick(&mut self.r, &options);
            self.kinds[i as usize] = "portal".into();
            left = !left;
        }
        Ok(())
    }

    /// Go: `eligible`, die freien Chunks einer Seite mit Mindestabstand zum Hub (Puffer `out` wird wiederverwendet).
    pub(crate) fn eligible(&self, left: bool, out: &mut Vec<i64>) {
        out.clear();
        let min = self.b.portals.min_distance_from_hub_units;
        for (i, k) in self.kinds.iter().enumerate() {
            let dx = self.center(i as i64) - self.hub;
            let on_side = if left { dx < 0.0 } else { dx > 0.0 };
            if k.is_empty() && on_side && dx.abs() >= min {
                out.push(i as i64);
            }
        }
    }

    /// Go: `lairX`: Der Bau des Endbosses (docs/rules/bosse.md § 1: „am Ende vor dem Eingang“, K2.1c) liegt an der
    /// inneren Kante des Ausgangs-Chunks. Er belegt kein eigenes Chunk und würfelt nicht: Portale, Ereignisse,
    /// Ressourcen und die Dichte bleiben, wie sie ohne Bau wären.
    pub(crate) fn lair_x(&self) -> f64 {
        if self.b.exit_side == "right" {
            return (self.kinds.len() as i64 - 1) as f64 * self.cw;
        }
        self.cw
    }

    /// Go: `placeEvents`, belegt Truhen- und Camp-Chunks; Camps bevorzugt nah am Hub, aber zuerst Chunks außerhalb der
    /// Verteidigungszone (B-261). Die Würfe bleiben dieselben, nur die Sortierung ändert sich.
    pub(crate) fn place_events(&mut self, seed: &str) -> Result<(), String> {
        for kind in EVENT_KINDS {
            let (min, max) = self.b.event_chunks.get(kind).map_or((0, 0), |r| (r.min, r.max));
            let free = self.free();
            let mut slots = k3c_rng::shuffle(&mut self.r, free);
            // feste Truhen und Camps zählen mit
            let fixed = self.fixed.get(kind).copied().unwrap_or(0);
            let count = i64::max(0, self.r.int(min, max) - fixed) as usize;
            if slots.len() < count {
                return Err(format!("level zu kurz für {count}x {kind} (Seed {seed})"));
            }
            if kind == "recruitCamp" {
                self.sort_camp_slots(&mut slots);
            }
            for &s in &slots[..count] {
                self.kinds[s as usize] = kind.to_string();
            }
        }
        Ok(())
    }

    /// Die Camp-Reihenfolge von `placeEvents`: stabil, gleich gute Slots behalten ihre Reihenfolge aus dem Mischen.
    fn sort_camp_slots(&self, slots: &mut [i64]) {
        let safe = self.safe_camp_slots();
        let outside = self.rules.site_placement.outside_units;
        slots.sort_by(|&i, &j| {
            // mit Bau zuerst Chunks weit genug vom Bau (B-981: in jedem Biom, auch wo wenig Platz bleibt)
            let (fi, fj) = (self.far_from_lair(i), self.far_from_lair(j));
            let (di, dj) = (self.center(i) - self.hub, self.center(j) - self.hub);
            // Camps zuerst außerhalb der Verteidigungszone (früher: Abstand zu den Linien, B-261)
            let (oi, oj) = (di.abs() > outside, dj.abs() > outside);
            fj.cmp(&fi)
                .then(oj.cmp(&oi))
                .then(safe[j as usize].cmp(&safe[i as usize]))
                .then(di.abs().total_cmp(&dj.abs()))
        });
    }

    /// Go: `farFromLair`: Der Chunk liegt mindestens `LAIR_CAMP_GAP_UNITS` vom Bau entfernt (ohne Bau immer).
    pub(crate) fn far_from_lair(&self, chunk: i64) -> bool {
        !self.b.lair || (self.center(chunk) - self.lair_x()).abs() >= LAIR_CAMP_GAP_UNITS as f64
    }

    /// Go: `safeCampSlots`, die Chunks, in denen ein Camp sicher steht (Ordnung nach dem Abstand zu den Linien, B-261,
    /// bleibt davor): Bei Biomen mit Bau nicht näher als `LAIR_CAMP_GAP_UNITS` am Bau und nie hinter einem Portal (vom
    /// Hub aus gesehen); dort laufen die Beschwörung des Endbosses und die Portal-Gegner, und der Weg zum Camp wäre
    /// tödlich (BS3.4, Kristallhöhle). Biome ohne Bau: alle sicher. Index = Chunk-Index.
    pub(crate) fn safe_camp_slots(&self) -> Vec<bool> {
        let portals: Vec<f64> = self
            .kinds
            .iter()
            .enumerate()
            .filter(|(_, k)| *k == "portal")
            .map(|(i, _)| self.center(i as i64))
            .collect();
        let lair = self.lair_x();
        (0..self.kinds.len() as i64)
            .map(|i| {
                let x = self.center(i);
                let behind = portals
                    .iter()
                    .any(|p| (p - self.hub) * (x - self.hub) > 0.0 && (p - self.hub).abs() < (x - self.hub).abs());
                !self.b.lair || ((x - lair).abs() >= LAIR_CAMP_GAP_UNITS as f64 && !behind)
            })
            .collect()
    }
}
