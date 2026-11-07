//! Geplante Chunks (Port von `engine/level/planned.go`, docs/rules/geplante-chunks.md, B-461): Typen und Prüfung der
//! Planung unabhängig vom Seed gegen die kürzeste Länge des Bioms bzw. die eingestellte Chunk-Anzahl.

use super::biome::{Biome, Rules, nullable};
use super::{check_count, gv, hub_chunks_of, is_event};
use serde::Deserialize;

/// Abstand von Ressourcen, Truhen und Adern zur Chunk-Grenze (wie `inside` im Generator).
pub const EDGE_GAP_UNITS: f64 = 2.0;
/// Mindestabstand einer festen Ader zum Baugrund der Zone 1 (§ 4, Q57).
const VEIN_GAP_UNITS: f64 = 4.0;

/// Festes Objekt eines geplanten Chunks; `at` zählt in Units ab Chunk-Anfang.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PlannedObject {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub at: f64,
}

/// Ruine (`buildings`) und Wache (`enemies`) haben dieselbe Form wie ein Objekt.
pub type PlannedBuilding = PlannedObject;
pub type PlannedEnemy = PlannedObject;

/// Geplanter Chunk; `slot` zählt relativ zu den Hub-Chunks (−n links, +n rechts, nie 0). Streng gelesen wie in Go.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct PlannedChunk {
    pub slot: i64,
    pub kind: String,
    #[serde(deserialize_with = "nullable")]
    pub seed: String,
    #[serde(deserialize_with = "nullable")]
    pub objects: Vec<PlannedObject>,
    #[serde(deserialize_with = "nullable")]
    pub buildings: Vec<PlannedBuilding>,
    #[serde(deserialize_with = "nullable")]
    pub enemies: Vec<PlannedEnemy>,
}

/// Länge eines Bioms in Chunks, gegen die eine Planung geprüft wird.
#[derive(Clone, Copy, Debug)]
pub struct Geometry {
    pub cw: f64,
    pub hub_half: f64,
    pub usable: i64,
    pub count: i64,
}

/// Rechnet wie `frame`: mit der eingestellten Anzahl `count`, ohne sie (0) mit der kleinsten Länge.
pub fn geometry_of(b: &Biome, count: i64) -> Geometry {
    let cw = b.chunk_width_units;
    let hub_chunks = hub_chunks_of(b);
    let mut n = count;
    if n == 0 {
        n = (b.length_units.min as f64 / cw).round() as i64;
        if n % 2 != 0 {
            n += 1;
        }
    }
    Geometry {
        cw,
        hub_half: hub_chunks as f64 * cw / 2.0,
        usable: (n - hub_chunks) / 2 - 1,
        count,
    }
}

impl Geometry {
    fn beyond(&self) -> String {
        if self.count > 0 {
            return format!(
                "jenseits der Chunk-Anzahl {} (höchstens ±{})",
                self.count, self.usable
            );
        }
        format!("jenseits der kürzesten Länge (höchstens ±{})", self.usable)
    }

    /// Abstand einer Stelle `at` Units hinter dem Chunk-Anfang von der Hub-Mitte.
    pub fn dist(&self, slot: i64, at: f64) -> f64 {
        if slot > 0 {
            return self.hub_half + (slot - 1) as f64 * self.cw + at;
        }
        self.hub_half + (-slot) as f64 * self.cw - at
    }
}

/// Festes oder implizites Objekt mit seinem Abstand zum Hub.
#[derive(Clone, Debug)]
pub struct Placed {
    pub kind: String,
    pub slot: i64,
    pub at: f64,
    pub dist: f64,
}

impl Placed {
    /// Lage als Abstand zur Hub-Mitte, links negativ (Teil von `kind@x`).
    pub fn key_x(&self) -> f64 {
        if self.slot < 0 { -self.dist } else { self.dist }
    }
}

fn has_chunk_kind(b: &Biome, kind: &str) -> bool {
    kind == "portal" || is_event(kind) || b.chunk_weights.get(kind).is_some()
}

fn has_object_kind(b: &Biome, kind: &str) -> bool {
    if kind == "portal" || is_event(kind) || (b.veins.count > 0 && kind == b.veins.kind) {
        return true;
    }
    b.resources_per_chunk
        .0
        .iter()
        .any(|(_, res)| res.get(kind).is_some())
}

/// Prüft die geplanten Chunks gegen die Chunk-Anzahl `count` (0 = kürzeste Länge); Fehler nennen Platz und Grund.
pub fn validate_plan_for(
    b: &Biome,
    rules: &Rules,
    plan: &[PlannedChunk],
    count: i64,
) -> Result<(), String> {
    check_count(b, count)?;
    if plan.is_empty() {
        return Ok(());
    }
    let g = geometry_of(b, count);
    let mut seen: Vec<i64> = Vec::with_capacity(plan.len());
    let mut spots: Vec<(String, f64)> = Vec::with_capacity(plan.len());
    let (mut all, mut all_ruins) = (Vec::new(), Vec::new());
    for c in plan {
        let at = |e: String| format!("auf Platz {:+}: {e}", c.slot);
        check_chunk(c, b, &g, &seen).map_err(at)?;
        seen.push(c.slot);
        let items = check_objects(c, b, rules, &g).map_err(at)?;
        let ruins = super::ruins::check_buildings(c, rules, &g, &items).map_err(at)?;
        super::ruins::check_guards(c, b, rules, &g).map_err(at)?;
        mark_spots(&mut spots, &[&items, &ruins]).map_err(at)?;
        all.extend(items);
        all_ruins.extend(ruins);
    }
    super::ruins::check_ruins_across(&all_ruins, &all)?;
    super::totals::check_totals(b, &g, plan, &all)
}

/// Trägt Objekte und Ruinen in `spots` ein; `kind@x` muss je Stufe eindeutig sein.
fn mark_spots(spots: &mut Vec<(String, f64)>, lists: &[&[Placed]]) -> Result<(), String> {
    for list in lists {
        for it in list.iter() {
            let x = it.key_x();
            if spots.iter().any(|(k, sx)| *k == it.kind && *sx == x) {
                return Err(format!(
                    "{}@{} doppelt (kind@x muss je Stufe eindeutig sein)",
                    it.kind,
                    gv(x)
                ));
            }
            spots.push((it.kind.clone(), x));
        }
    }
    Ok(())
}

fn check_chunk(c: &PlannedChunk, b: &Biome, g: &Geometry, seen: &[i64]) -> Result<(), String> {
    if c.slot == 0 {
        return Err("es gibt keinen Platz 0 (Hub)".into());
    }
    if c.kind == "hub" || c.kind == "exit" || c.kind == "edge" {
        return Err(format!("die Art {:?} ist nicht planbar", c.kind));
    }
    if c.slot > g.usable || -c.slot > g.usable {
        return Err(g.beyond());
    }
    if seen.contains(&c.slot) {
        return Err("doppelt geplant".into());
    }
    if !has_chunk_kind(b, &c.kind) {
        return Err(format!("unbekannte Chunk-Art {:?}", c.kind));
    }
    Ok(())
}

/// Prüft Art, Lage und Eindeutigkeit der Objekte eines Chunks; ein Ereignis-Chunk ohne Objektliste trägt sein Objekt
/// in der Chunk-Mitte.
fn check_objects(
    c: &PlannedChunk,
    b: &Biome,
    rules: &Rules,
    g: &Geometry,
) -> Result<Vec<Placed>, String> {
    if c.objects.is_empty() {
        if is_event(&c.kind) || c.kind == "portal" {
            let p = Placed {
                kind: c.kind.clone(),
                slot: c.slot,
                at: g.cw / 2.0,
                dist: g.dist(c.slot, g.cw / 2.0),
            };
            check_placed(&p, b, rules, g)?;
            return Ok(vec![p]);
        }
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(c.objects.len());
    for (i, o) in c.objects.iter().enumerate() {
        if !has_object_kind(b, &o.kind) {
            return Err(format!(
                "unbekannte Objekt-Art {:?} im Biom {}",
                o.kind, b.id
            ));
        }
        if c.objects[..i].contains(o) {
            return Err(format!(
                "{}@{} doppelt (kind@x muss eindeutig sein)",
                o.kind,
                gv(o.at)
            ));
        }
        let p = Placed {
            kind: o.kind.clone(),
            slot: c.slot,
            at: o.at,
            dist: g.dist(c.slot, o.at),
        };
        check_placed(&p, b, rules, g)?;
        out.push(p);
    }
    Ok(out)
}

/// Lage-Regeln eines Objekts (geplante-chunks.md § 4).
fn check_placed(p: &Placed, b: &Biome, rules: &Rules, g: &Geometry) -> Result<(), String> {
    if p.kind == "portal" || p.kind == "recruitCamp" {
        if p.at < 0.0 || p.at > g.cw {
            return Err(format!(
                "{} bei {} liegt außerhalb des Chunks (0…{})",
                p.kind,
                gv(p.at),
                gv(g.cw)
            ));
        }
        if p.kind == "portal" {
            return check_portal(p, b, rules);
        }
        return Ok(());
    }
    if p.at < EDGE_GAP_UNITS || p.at > g.cw - EDGE_GAP_UNITS {
        return Err(format!(
            "{} bei {}: mindestens 2 Units Abstand zur Chunk-Grenze (2…{})",
            p.kind,
            gv(p.at),
            gv(g.cw - EDGE_GAP_UNITS)
        ));
    }
    if b.veins.count > 0 && p.kind == b.veins.kind {
        return check_vein(p, rules);
    }
    Ok(())
}

fn check_portal(p: &Placed, b: &Biome, rules: &Rules) -> Result<(), String> {
    let min = b.portals.min_distance_from_hub_units;
    if p.dist < min {
        return Err(format!(
            "das Portal {} Units vom Hub, mindestens {}",
            gv(p.dist),
            gv(min)
        ));
    }
    let outer = rules.site_placement.outside_units;
    if p.dist <= outer {
        return Err(format!(
            "das Portal {} Units vom Hub, innerhalb der Verteidigungszone ({})",
            gv(p.dist),
            gv(outer)
        ));
    }
    Ok(())
}

fn check_vein(p: &Placed, rules: &Rules) -> Result<(), String> {
    let sp = &rules.site_placement;
    for site in [sp.zone1_wall_units, sp.zone1_tower_units] {
        if (p.dist - site).abs() < VEIN_GAP_UNITS {
            return Err(format!(
                "die Ader {} Units vom Hub, weniger als 4 Units von einem Bauplatz ({})",
                gv(p.dist),
                gv(site)
            ));
        }
    }
    Ok(())
}
