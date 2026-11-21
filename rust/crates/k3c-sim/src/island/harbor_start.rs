//! HB1.5 (B-1188): Neue Spiele starten am Hafen; der Hafen liegt (vom Seed bestimmt) 1 bis 2 Chunks vom Lager (der Burg)
//! entfernt, und der ganze Hafen-Chunk ist von Bäumen bewachsen, die gerodet werden wollen. Nur mit den nativen Regeln
//! (`WorldValues::native_rules`); Zahlen stehen hier als Werte der Regel und sind je Welt einstellbar über die Konstanten.

use k3c_level::level::HARBOR_KIND;
use k3c_level::types::PlannedChunk;

use crate::state::{ResourceNode, World, WorldValues};

/// HARBOR_NEAR sind die kleinste und größte Zahl Chunks zwischen Lager und Hafen (Slot-Abstand von der Hub-Mitte).
pub const HARBOR_NEAR: (i64, i64) = (1, 2);
/// TREES_PER_UNIT ist die Baumdichte im Hafen-Chunk (Bäume je Unit; `0` = kahl).
pub const TREES_PER_UNIT: f64 = 0.8;
/// START_GAP ist der freie Streifen um den Steg in Units, in dem die Monarchen starten.
pub const START_GAP: f64 = 5.0;
/// SITE_GAP ist der freie Streifen um jeden Bauplatz im Hafen-Chunk.
const SITE_GAP: f64 = 1.0;

/// seed_pick liefert aus dem Seed eine Zahl in `min..=max` (FNV-1a, ohne Zufallsstrom).
fn seed_pick(seed: &str, min: i64, max: i64) -> i64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in seed.bytes() {
        h = (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
    }
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    min + (h % (max - min + 1) as u64) as i64
}

/// near_harbor rückt den Hafen auf `±HARBOR_NEAR` Chunks vom Lager (Seite wie in den Weltdaten).
pub fn near_harbor(mut planned: Vec<PlannedChunk>, seed: &str, v: &WorldValues) -> Vec<PlannedChunk> {
    if !v.native_rules {
        return planned;
    }
    for c in planned.iter_mut().filter(|c| c.kind == HARBOR_KIND) {
        let n = seed_pick(seed, HARBOR_NEAR.0, HARBOR_NEAR.1);
        c.slot = if c.slot < 0 { -n } else { n };
    }
    planned
}

/// harbor_x ist die Mitte des Hafens der Stufe.
pub fn harbor_x(w: &World) -> Option<f64> {
    w.level.entities.iter().find(|e| e.kind == HARBOR_KIND).map(|e| e.x)
}

/// plant_harbor_trees bewächst den Hafen-Chunk (bis auf den Startstreifen am Steg) mit Bäumen.
pub fn plant_harbor_trees(w: &mut World, v: &WorldValues) {
    let Some(hx) = harbor_x(w) else {
        return;
    };
    if !v.native_rules || TREES_PER_UNIT <= 0.0 {
        return;
    }
    let half = w.biome.chunk_width_units / 2.0;
    let step = 1.0 / TREES_PER_UNIT;
    let mut dx = -half + step / 2.0;
    while dx < half {
        let x = hx + dx;
        let on_site = w.sites.iter().any(|s| (s.x - x).abs() < SITE_GAP);
        if dx.abs() > START_GAP.max(crate::world::forest::CLEARING_RADIUS) && !on_site {
            let variant = crate::unit::camp_guard::nearest_variant(w, hx + dx);
            let id = w.new_id();
            w.nodes.push(ResourceNode {
                id,
                kind: "tree".into(),
                variant,
                x: hx + dx,
                ..ResourceNode::default()
            });
        }
        dx += step;
    }
}

/// start_x ist der Startplatz eines neuen Monarchen am Steg (je Spieler-Index zu einer Seite), `None` ohne Hafen.
pub fn start_x(w: &World, v: &WorldValues, index: i64) -> Option<f64> {
    if !v.native_rules {
        return None;
    }
    harbor_x(w).map(|x| x + if index % 2 != 0 { 3.0 } else { -3.0 })
}
