//! WD1.1 (B-1204): Voller Baumwuchs auf der Oberfläche. Mit nativen Regeln hat auf Tiefe 0 jeder Chunk außer dem Hub-Chunk
//! die volle Dichte des Chunk-Typs `forest` (Bereich `tree` in `resourcesPerChunk`, sonst 32 bis 42), auch Ereignis-,
//! Portal- und Ausgang-Chunks. Baumfrei bleiben der Hub-Streifen (bis `HUB_MARGIN` Units vor dem ersten Mauer-Bauplatz je
//! Seite) und je Truhe, Lager, Portal, Ausgang und Hafen eine Lichtung vom Radius `CLEARING_RADIUS`. Die Würfel kommen aus
//! einem eigenen Strom `<seed>:wald`, die Sim-Würfel bleiben unberührt. Der Hafen-Chunk gehört `plant_harbor_trees`.

use k3c_level::level::HARBOR_KIND;
use k3c_rng::Rng;

use crate::state::{ResourceNode, World, WorldValues};

/// Radius einer Lichtung in Units (Startwert, B-1204).
pub const CLEARING_RADIUS: f64 = 6.0;
/// Abstand des Waldrands im Hub in Units vor dem ersten Mauer-Bauplatz.
pub const HUB_MARGIN: f64 = 4.0;
/// Ereignisse, um die eine Lichtung bleibt.
const CLEARING_KINDS: [&str; 5] = ["chest", "recruitCamp", "portal", "exit", HARBOR_KIND];

/// Ist der Wald dieser Regel aktiv? Nur native Regeln auf der Oberfläche (Tiefe 0).
pub fn forest_active(w: &World, v: &WorldValues) -> bool {
    v.native_rules && w.biome.depth == 0
}

/// Die Mitten der Lichtungen der Stufe.
pub fn clearing_centres(w: &World) -> Vec<f64> {
    w.level
        .entities
        .iter()
        .filter(|e| CLEARING_KINDS.contains(&e.kind.as_str()))
        .map(|e| e.x)
        .collect()
}

/// Halbe Breite des baumfreien Hub-Streifens: bis `HUB_MARGIN` vor den nächsten Mauer-Bauplatz, sonst die halbe Hub-Breite.
pub fn hub_strip_half(w: &World) -> f64 {
    let nearest_wall = w
        .sites
        .iter()
        .filter(|s| s.kind == "wall")
        .map(|s| (s.x - w.hub_x).abs())
        .fold(f64::INFINITY, f64::min);
    if nearest_wall.is_finite() {
        (nearest_wall - HUB_MARGIN).max(0.0)
    } else {
        w.biome.hub_width_units / 2.0
    }
}

/// Darf an `x` ein Baum stehen (nicht im Hub-Streifen und nicht in einer Lichtung)?
pub fn tree_allowed(w: &World, clearings: &[f64], strip: f64, x: f64) -> bool {
    (x - w.hub_x).abs() >= strip && clearings.iter().all(|c| (x - c).abs() >= CLEARING_RADIUS)
}

/// Zielzahl Bäume je Chunk: Bereich `tree` des Chunk-Typs `forest`, sonst 32 bis 42.
fn density_range(w: &World) -> (i64, i64) {
    let r = w.biome.resources_per_chunk.get("forest").and_then(|m| m.get("tree"));
    r.map_or((32, 42), |r| (r[0], r[1]))
}

/// Legt den vollen Wald an (siehe Modul).
pub fn grow_forest(w: &mut World, v: &WorldValues) {
    if !forest_active(w, v) {
        return;
    }
    let clearings = clearing_centres(w);
    let strip = hub_strip_half(w);
    // Bäume im Hub-Streifen und in Lichtungen weg (der Level-Generator setzt sie nach Chunk-Typ)
    let allowed = |w: &World, x: f64| tree_allowed(w, &clearings, strip, x);
    let keep: Vec<bool> = w.nodes.iter().map(|n| n.kind != "tree" || allowed(w, n.x)).collect();
    let mut it = keep.iter();
    w.nodes.retain(|_| *it.next().unwrap_or(&true));
    let harbor = crate::island::harbor_start::harbor_x(w);
    let mut rng = Rng::new(&format!("{}:wald", w.level.seed));
    let (lo, hi) = density_range(w);
    let cw = w.biome.chunk_width_units;
    let chunks: Vec<(String, f64)> = w.level.chunks.iter().map(|c| (c.kind.clone(), c.start_units)).collect();
    for (kind, start) in chunks {
        if kind == "hub" || harbor.is_some_and(|h| h >= start && h < start + cw) {
            continue;
        }
        let have = w
            .nodes
            .iter()
            .filter(|n| n.kind == "tree" && !n.gone && n.x >= start && n.x < start + cw)
            .count();
        // gleiche Dichte je Unit: die Zielzahl schrumpft mit dem Anteil, der Hub-Streifen oder Lichtung ist
        let free = (0..50)
            .filter(|k| tree_allowed(w, &clearings, strip, start + (f64::from(*k) + 0.5) * cw / 50.0))
            .count() as f64
            / 50.0;
        let want = (rng.int(lo, hi) as f64 * free).round() as usize;
        for _ in have..want {
            let mut x = start + rng.next() * cw;
            for _ in 0..30 {
                if tree_allowed(w, &clearings, strip, x) {
                    break;
                }
                x = start + rng.next() * cw;
            }
            if !tree_allowed(w, &clearings, strip, x) {
                continue;
            }
            let variant = crate::unit::camp_guard::nearest_variant(w, x);
            let id = w.new_id();
            w.nodes.push(ResourceNode {
                id,
                kind: "tree".into(),
                variant,
                x,
                ..ResourceNode::default()
            });
        }
    }
}

#[cfg(test)]
#[path = "forest_tests.rs"]
mod forest_tests;
