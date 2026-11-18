//! Geplante Chunks prüfen (Port von `engine/level/planned.go`, docs/rules/geplante-chunks.md, B-461): feste Gegenden je
//! Insel und Tiefe. Die Prüfung ist unabhängig vom Seed und rechnet gegen die kürzeste mögliche Länge des Bioms.

use std::collections::BTreeSet;

use k3c_det::golden::go_float;

use crate::biome::Biome;
use crate::error::Fehler;
use crate::level::{check_count, hub_chunks_of, is_event};
use crate::lines::{HubSite, HubSiteRef, sites_of};
use crate::pending;
use crate::rules::Rules;
use crate::types::PlannedChunk;

/// Abstand von Ressourcen, Truhen und Adern zur Chunk-Grenze (wie `inside` im Generator).
pub(crate) const EDGE_GAP_UNITS: i64 = 2;

/// Mindestabstand einer festen Ader zum Baugrund der Zone 1 (geplante-chunks.md § 4, Q57).
const VEIN_GAP_UNITS: i64 = 4;

/// Go: `geometry`, die Länge eines Bioms in Chunks, gegen die eine Planung geprüft wird.
#[derive(Debug, Clone)]
pub(crate) struct Geometry {
    /// Chunk-Breite.
    pub cw: f64,
    /// Halbe Breite der Hub-Chunks.
    pub hub_half: f64,
    /// Planbare Plätze je Seite (ohne Ausgang und Rand).
    pub usable: i64,
    /// Eingestellte Chunk-Anzahl; 0 = kürzeste Länge des Bioms.
    pub count: i64,
    /// Hub- und Insel-Plätze, gegen die Ruinen geprüft werden.
    #[allow(dead_code)] // RU6.3: ruins.rs liest die Plätze
    pub sites: Vec<HubSiteRef>,
}

/// Go: `geometryOf`, rechnet wie `frame` im Generator: mit der eingestellten Anzahl `count`, ohne sie (0) mit der
/// kleinsten Länge.
pub(crate) fn geometry_of(b: &Biome, count: i64, sites: Vec<HubSiteRef>) -> Geometry {
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
        sites,
    }
}

impl Geometry {
    /// Der Grund für einen Platz jenseits der geprüften Länge.
    pub fn beyond(&self) -> String {
        if self.count > 0 {
            return format!("jenseits der Chunk-Anzahl {} (höchstens ±{})", self.count, self.usable);
        }
        format!("jenseits der kürzesten Länge (höchstens ±{})", self.usable)
    }

    /// Der Abstand einer Stelle `at` Units hinter dem Chunk-Anfang von der Hub-Mitte.
    pub fn dist(&self, slot: i64, at: f64) -> f64 {
        if slot > 0 {
            return self.hub_half + (slot - 1) as f64 * self.cw + at;
        }
        self.hub_half + (-slot) as f64 * self.cw - at
    }
}

/// Go: `placed`, ein festes oder implizites Objekt mit seinem Abstand zum Hub.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Placed {
    pub kind: String,
    pub slot: i64,
    pub at: f64,
    pub dist: f64,
    /// Vom Nutzer gesetzt; false = Ereignis-Chunk ohne Objektliste, Objekt in der Chunk-Mitte.
    pub fixed: bool,
}

impl Placed {
    /// Go: `placed.key`, `kind@x` mit der Lage x als Abstand zur Hub-Mitte, links negativ.
    pub fn x(&self) -> f64 {
        if self.slot < 0 { -self.dist } else { self.dist }
    }

    /// Schlüssel für die Eindeutigkeit von `kind@x`; `+ 0.0` macht `-0` zu `0` (Gos Map-Schlüssel sind gleich).
    fn spot(&self) -> (String, u64) {
        (self.kind.clone(), (self.x() + 0.0).to_bits())
    }
}

/// Go: `hasChunkKind`: Chunk-Arten des Bioms (chunkWeights) und die Ereignis-Arten.
fn has_chunk_kind(b: &Biome, kind: &str) -> bool {
    kind == "portal" || is_event(kind) || b.chunk_weights.get(kind).is_some()
}

/// Go: `hasObjectKind`: Ressourcen laut resourcesPerChunk, die Ader des Bioms und die Ereignis-Arten.
fn has_object_kind(b: &Biome, kind: &str) -> bool {
    if kind == "portal" || is_event(kind) || (b.veins.count > 0 && kind == b.veins.kind) {
        return true;
    }
    b.resources_per_chunk.0.iter().any(|e| e.value.get(kind).is_some())
}

/// Go: `ValidatePlan`, prüft die geplanten Chunks einer Tiefe unabhängig vom Seed. Ist die Planung gültig, scheitert
/// kein Seed zur Laufzeit. Gerechnet wird mit der kürzesten Länge des Bioms.
pub fn validate_plan(b: &Biome, plan: &[PlannedChunk], rules: &Rules) -> Result<(), Fehler> {
    validate_plan_for(b, plan, 0, rules)
}

/// Go: `ValidatePlanFor`, wie `validate_plan`, aber gegen die Chunk-Anzahl `count` statt gegen die kürzeste Länge.
pub fn validate_plan_for(b: &Biome, plan: &[PlannedChunk], count: i64, rules: &Rules) -> Result<(), Fehler> {
    validate_plan_with(b, plan, count, None, rules)
}

/// Go: `ValidatePlanWith`, wie `validate_plan_for` gegen die Hub- und Insel-Plätze `sites` einer Welt (`None` = Basis).
pub fn validate_plan_with(
    b: &Biome,
    plan: &[PlannedChunk],
    count: i64,
    sites: Option<&[HubSite]>,
    rules: &Rules,
) -> Result<(), Fehler> {
    check_plan(b, plan, count, sites, rules).map_err(Fehler::Level)
}

/// Gesammelter Zustand der Prüfung über alle Chunks eines Plans.
#[derive(Default)]
struct PlanState {
    seen: BTreeSet<i64>,
    spots: BTreeSet<(String, u64)>,
    all: Vec<Placed>,
    all_ruins: Vec<Placed>,
}

fn check_plan(
    b: &Biome,
    plan: &[PlannedChunk],
    count: i64,
    sites: Option<&[HubSite]>,
    rules: &Rules,
) -> Result<(), String> {
    check_count(b, count)?;
    if plan.is_empty() {
        return Ok(());
    }
    let g = geometry_of(b, count, sites_of(sites, &rules.hub_sites));
    let mut st = PlanState::default();
    for c in plan {
        check_one(c, b, &g, rules, &mut st).map_err(|e| format!("auf Platz {:+}: {e}", c.slot))?;
    }
    pending::check_ruins_across(&st.all_ruins, &st.all)?;
    pending::check_totals(b, &g, plan, &st.all, rules)
}

/// Prüft einen Chunk des Plans (Art, Objekte, Ruine, Wachen, Eindeutigkeit) und nimmt ihn in `st` auf.
fn check_one(c: &PlannedChunk, b: &Biome, g: &Geometry, rules: &Rules, st: &mut PlanState) -> Result<(), String> {
    check_chunk(c, b, g, &st.seen)?;
    st.seen.insert(c.slot);
    let items = check_objects(c, b, g, rules)?;
    let ruins = pending::check_buildings(c, g, &items, rules)?;
    pending::check_guards(c, b, g, rules)?;
    mark_spots(&mut st.spots, &[&items, &ruins])?;
    st.all.extend(items);
    st.all_ruins.extend(ruins);
    Ok(())
}

/// Go: `markSpots`, trägt Objekte und Ruinen eines Chunks in `spots` ein; `kind@x` muss je Stufe eindeutig sein.
fn mark_spots(spots: &mut BTreeSet<(String, u64)>, lists: &[&[Placed]]) -> Result<(), String> {
    for list in lists {
        for it in *list {
            if !spots.insert(it.spot()) {
                return Err(format!(
                    "{}@{} doppelt (kind@x muss je Stufe eindeutig sein)",
                    it.kind,
                    go_float(it.x())
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn check_chunk(c: &PlannedChunk, b: &Biome, g: &Geometry, seen: &BTreeSet<i64>) -> Result<(), String> {
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

/// Go: `checkObjects`, prüft Art, Lage und Eindeutigkeit der Objekte eines Chunks. Ein Ereignis-Chunk ohne
/// Objektliste trägt sein Objekt in der Chunk-Mitte.
pub(crate) fn check_objects(c: &PlannedChunk, b: &Biome, g: &Geometry, rules: &Rules) -> Result<Vec<Placed>, String> {
    if c.objects.is_empty() {
        if is_event(&c.kind) || c.kind == "portal" {
            let at = g.cw / 2.0;
            let p = Placed {
                kind: c.kind.clone(),
                slot: c.slot,
                at,
                dist: g.dist(c.slot, at),
                fixed: false,
            };
            check_placed(&p, b, g, rules)?;
            return Ok(vec![p]);
        }
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity(c.objects.len());
    for (i, o) in c.objects.iter().enumerate() {
        if !has_object_kind(b, &o.kind) {
            return Err(format!("unbekannte Objekt-Art {:?} im Biom {}", o.kind, b.id));
        }
        if c.objects[..i].contains(o) {
            return Err(format!(
                "{}@{} doppelt (kind@x muss eindeutig sein)",
                o.kind,
                go_float(o.at)
            ));
        }
        let p = Placed {
            kind: o.kind.clone(),
            slot: c.slot,
            at: o.at,
            dist: g.dist(c.slot, o.at),
            fixed: true,
        };
        check_placed(&p, b, g, rules)?;
        out.push(p);
    }
    Ok(out)
}

/// Go: `checkPlaced`, prüft die Lage-Regeln (geplante-chunks.md § 4) eines Objekts.
pub(crate) fn check_placed(p: &Placed, b: &Biome, g: &Geometry, rules: &Rules) -> Result<(), String> {
    let (at, cw) = (go_float(p.at), go_float(g.cw));
    if p.kind == "portal" || p.kind == "recruitCamp" {
        if p.at < 0.0 || p.at > g.cw {
            return Err(format!("{} bei {at} liegt außerhalb des Chunks (0…{cw})", p.kind));
        }
        if p.kind == "portal" {
            return check_portal(p, b, rules);
        }
        return Ok(());
    }
    let edge = EDGE_GAP_UNITS as f64;
    if p.at < edge || p.at > g.cw - edge {
        return Err(format!(
            "{} bei {at}: mindestens {EDGE_GAP_UNITS} Units Abstand zur Chunk-Grenze ({EDGE_GAP_UNITS}…{})",
            p.kind,
            go_float(g.cw - edge)
        ));
    }
    if b.veins.count > 0 && p.kind == b.veins.kind {
        return check_vein(p, rules);
    }
    Ok(())
}

pub(crate) fn check_portal(p: &Placed, b: &Biome, rules: &Rules) -> Result<(), String> {
    if p.dist < b.portals.min_distance_from_hub_units {
        return Err(format!(
            "das Portal {} Units vom Hub, mindestens {}",
            go_float(p.dist),
            go_float(b.portals.min_distance_from_hub_units)
        ));
    }
    let outer = rules.site_placement.outside_units;
    if p.dist <= outer {
        return Err(format!(
            "das Portal {} Units vom Hub, innerhalb der Verteidigungszone ({})",
            go_float(p.dist),
            go_float(outer)
        ));
    }
    Ok(())
}

pub(crate) fn check_vein(p: &Placed, rules: &Rules) -> Result<(), String> {
    // Baugrund der Zone 1
    let targets = [
        rules.site_placement.zone1_wall_units,
        rules.site_placement.zone1_tower_units,
    ];
    for site in targets {
        if (p.dist - site).abs() < VEIN_GAP_UNITS as f64 {
            return Err(format!(
                "die Ader {} Units vom Hub, weniger als {VEIN_GAP_UNITS} Units von einem Bauplatz ({})",
                go_float(p.dist),
                go_float(site)
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "planned_tests.rs"]
mod planned_tests;
