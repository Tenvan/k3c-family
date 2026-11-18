//! Heilplatz (Port von `engine/sim/building_healer.go`, B-116, Q32, Q63): Ein gebauter Heilplatz heilt Bürger und Truppen im
//! Radius (Monarchen haben keine HP, B-930), immer, auch im Kampf, bis `max_hp` (data/buildings.json › healer.heal). Sonst
//! gibt es keine Regeneration (buerger.md § 3).

use crate::state::World;
use crate::world::values_room::values;

/// Go: `stepHealing`.
pub fn step_healing(w: &mut World, dt: f64) {
    let v = values(w);
    let Some(b) = v.buildings.get("healer") else {
        return;
    };
    let amount = b.heal.hp_per_second * dt;
    let radius = b.heal.radius_units;
    for s in &w.sites {
        if s.kind != "healer" || s.state != "built" {
            continue;
        }
        for t in &mut w.troops {
            if (t.x - s.x).abs() <= radius {
                t.hp = t.max_hp.min(t.hp + amount);
            }
        }
    }
}
