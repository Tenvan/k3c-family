//! HB1.4 (B-1187): Landstreicher-Lager stehen zwischen 2 bis 3 Bäumen. Sind diese Bäume gefällt, ist das Lager zerstört;
//! an der Stelle entsteht ein freier Bauplatz für eine Taverne (sie lockt je Morgengrauen einen Landstreicher, höchstens
//! `tavern.vagrants.max`). Nur Biome mit Holz als Hauptrohstoff (Wald); in Höhlen und Minen bleiben Lager unberührt.
//! Ohne Zufallsstrom: Zahl und Lage der Bäume folgen aus dem Ort des Lagers, die Sim-Würfel bleiben unberührt.

use serde_json::json;

use crate::state::common::event;
use crate::state::{ResourceNode, World, WorldValues};
use crate::world::empty_site;

/// GUARD_RADIUS ist der Umkreis in Units, in dem die Wächter-Bäume eines Lagers stehen.
pub const GUARD_RADIUS: f64 = 4.0;
/// Höchstzahl der Wächter-Bäume je Lager; mindestens 2 stehen immer (`plant_camp_trees`).
pub const GUARD_MAX: usize = 3;
const GUARD_MIN: usize = 2;

fn wooded(w: &World, v: &WorldValues) -> bool {
    v.native_rules && w.biome.primary_resource == "wood"
}

fn live_trees_near(w: &World, x: f64) -> Vec<(f64, i64)> {
    let mut near: Vec<(f64, i64)> = w
        .nodes
        .iter()
        .filter(|n| n.kind == "tree" && !n.plant && !n.gone && (n.x - x).abs() <= GUARD_RADIUS)
        .map(|n| ((n.x - x).abs(), n.id))
        .collect();
    near.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    near
}

/// nearest_variant ist die Baumart des Level-Baums, der dem Lager am nächsten steht (sonst leer).
pub(crate) fn nearest_variant(w: &World, x: f64) -> String {
    w.nodes
        .iter()
        .filter(|n| n.kind == "tree" && !n.plant)
        .min_by(|a, b| (a.x - x).abs().total_cmp(&(b.x - x).abs()))
        .map(|n| n.variant.clone())
        .unwrap_or_default()
}

/// plant_camp_trees stellt neben jedes Lager im Wald so viele Bäume, dass mindestens 2 im Umkreis stehen (zählt
/// Bäume des Levels mit); Lage und Zahl (2 oder 3) kommen aus dem Ort des Lagers.
pub fn plant_camp_trees(w: &mut World, v: &WorldValues) {
    if !wooded(w, v) {
        return;
    }
    let xs: Vec<f64> = w.camps.iter().map(|c| c.x).collect();
    for x in xs {
        let want = GUARD_MIN + (x.abs() as i64 % 2) as usize;
        let have = live_trees_near(w, x).len().min(GUARD_MAX);
        for i in have..want {
            let side = if i % 2 == 0 { -1.0 } else { 1.0 };
            let variant = nearest_variant(w, x);
            let id = w.new_id();
            w.nodes.push(ResourceNode {
                id,
                kind: "tree".into(),
                variant,
                x: x + side * (1.6 + 0.9 * i as f64),
                ..ResourceNode::default()
            });
        }
    }
}

/// step_native_rules wählt je Lager einmal seine Wächter-Bäume (die nächsten, höchstens `GUARD_MAX`) und zerstört das
/// Lager, sobald keiner mehr steht.
pub fn step_native_rules(w: &mut World, v: &WorldValues) {
    if !wooded(w, v) {
        return;
    }
    for ci in 0..w.camps.len() {
        if !w.camps[ci].armed {
            let x = w.camps[ci].x;
            let guards: Vec<i64> = live_trees_near(w, x).into_iter().take(GUARD_MAX).map(|t| t.1).collect();
            w.camps[ci].guards = guards;
            w.camps[ci].armed = true;
        }
    }
    let nodes = &w.nodes;
    let (kept, lost): (Vec<_>, Vec<_>) = std::mem::take(&mut w.camps)
        .into_iter()
        .partition(|c| c.guards.iter().any(|g| nodes.iter().any(|n| n.id == *g)));
    w.camps = kept;
    for c in lost {
        destroy_camp(w, v, c.x);
    }
}

/// destroy_camp meldet das Ende des Lagers und legt an seiner Stelle einen freien Taverne-Platz an.
fn destroy_camp(w: &mut World, v: &WorldValues, x: f64) {
    w.events.push(event(json!({"type": "campDestroyed", "x": x})));
    let has_tavern = v.buildings.contains_key("tavern");
    if has_tavern && !w.sites.iter().any(|s| s.kind == "tavern" && (s.x - x).abs() < 0.5) {
        let site = empty_site(w, "tavern", x);
        w.sites.push(site);
    }
}
