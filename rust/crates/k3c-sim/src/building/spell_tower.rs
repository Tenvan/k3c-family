//! Zaubertum (Port von `engine/sim/building_spell_tower.go`, B-116, Q31): Ein Turm ab Stufe `spell.from_level` schießt selbst,
//! im Takt `interval_seconds` auf den nächsten Gegner in `range_units`, und trifft jeden Gegner im Radius `radius_units` um
//! das Ziel mit `damage` (data/buildings.json › tower.spell). Er trägt keine Schützen (`free_tower`, `step_archer`).

use crate::state::common::apply_damage;
use crate::state::{Site, World};
use crate::world::values_room::values;

use crate::hub::level_of;

/// Go: `isSpellTower`: Ist der Platz ein gebauter Turm ab der Zaubertum-Stufe?
pub fn is_spell_tower(w: &World, s: &Site) -> bool {
    let from_level = values(w).buildings.get("tower").map_or(0, |b| b.spell.from_level);
    s.kind == "tower" && s.state == "built" && level_of(w, s) >= from_level
}

/// Go: `stepSpellTowers`.
pub fn step_spell_towers(w: &mut World, dt: f64) {
    let v = values(w);
    let Some(tower) = v.buildings.get("tower") else {
        return;
    };
    let sp = &tower.spell;
    for si in 0..w.sites.len() {
        if !is_spell_tower(w, &w.sites[si]) {
            continue;
        }
        w.sites[si].spell_in = (w.sites[si].spell_in - dt).max(0.0);
        if w.sites[si].spell_in > 0.0 {
            continue;
        }
        let sx = w.sites[si].x;
        let mut target: Option<f64> = None;
        for e in &w.enemies {
            if (e.x - sx).abs() <= sp.range_units && target.is_none_or(|tx| (e.x - sx).abs() < (tx - sx).abs()) {
                target = Some(e.x);
            }
        }
        let Some(tx) = target else {
            continue;
        };
        let hit: Vec<i64> = w
            .enemies
            .iter()
            .filter(|e| (e.x - tx).abs() <= sp.radius_units)
            .map(|e| e.id)
            .collect();
        for id in hit {
            apply_damage(w, id, sp.damage);
        }
        w.sites[si].spell_in = sp.interval_seconds;
    }
}
