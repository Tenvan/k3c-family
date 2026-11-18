//! Lava (Port von `engine/sim/world_lava.go`, B-115, Beschluss Q28): Spieler, Truppen und Bauern auf einem Lava-Streifen
//! nehmen Schaden je Sekunde (data/biomes/*.json › lava). Der Streifen liegt mitten im Chunk der Art „lava“. Gegner nehmen
//! keinen Schaden (Startwert, bestätigt der Balancing-Workshop BR1).

use crate::crown::crowned;
use crate::state::World;
use crate::state::common::apply_damage;

/// Go: `onLava`: Liegt `x` auf einem Lava-Streifen der Stufe?
pub fn on_lava(w: &World, x: f64) -> bool {
    let half = w.biome.lava.width_units / 2.0;
    w.level
        .chunks
        .iter()
        .any(|c| c.kind == "lava" && (x - (c.start_units + w.level.chunk_width_units / 2.0)).abs() <= half)
}

/// Go: `stepLava`: zieht allen Figuren auf Lava einmal je Sekunde den Schaden einer Sekunde ab (über `apply_damage`: Tod,
/// Krone fällt, Treffer). Je Sekunde statt je Tick, weil ein Treffer am Spieler mindestens 1 Schaden macht.
pub fn step_lava(w: &mut World, dt: f64) {
    let dps = w.biome.lava.damage_per_second;
    if dps <= 0.0 {
        return;
    }
    w.lava_in -= dt;
    if w.lava_in > 0.0 {
        return;
    }
    w.lava_in += 1.0;
    let mut ids: Vec<i64> = Vec::new();
    for p in &w.players {
        if crowned(p) && !p.free && on_lava(w, p.x) {
            ids.push(p.id);
        }
    }
    for t in &w.troops {
        if t.hp > 0.0 && on_lava(w, t.x) {
            ids.push(t.id);
        }
    }
    for id in ids {
        apply_damage(w, id, dps);
    }
}
