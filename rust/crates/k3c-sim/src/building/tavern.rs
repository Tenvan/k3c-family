//! Taverne (Port von `engine/sim/building_tavern.go`, B-116, Q30): Bei jedem Morgengrauen entstehen an jeder gebauten Taverne
//! Landstreicher, solange dort weniger als das Maximum stehen; sie sind an die Taverne gebunden (`anchor_x`) und wandern in
//! ihrem Radius (data/buildings.json › tavern.vagrants).

use crate::pending::pure;
use crate::state::World;
use crate::unit::hammer::spawn_vagrant_with;
use crate::world::values_room::values;

/// Go: `tavernDawn`: der Wirt bringt zum Morgengrauen Landstreicher. Der Wurf je Landstreicher steht vor `spawn_vagrant`.
pub fn tavern_dawn(w: &mut World) {
    if pure(w) {
        return;
    }
    let v = values(w);
    let Some(tavern) = v.buildings.get("tavern") else {
        return;
    };
    let vg = &tavern.vagrants;
    for si in 0..w.sites.len() {
        if w.sites[si].kind != "tavern" || w.sites[si].state != "built" {
            continue;
        }
        let sx = w.sites[si].x;
        for _ in 0..vg.per_dawn {
            if vagrants_at(w, sx) >= vg.max {
                break;
            }
            let x = sx + (w.rng.next() - 0.5) * vg.wander_units;
            spawn_vagrant_with(w, &v, sx, x);
        }
    }
}

/// Go: `vagrantsAt` zählt die Landstreicher, die an `x` gebunden sind (Camp oder Taverne).
pub fn vagrants_at(w: &World, x: f64) -> i64 {
    w.troops
        .iter()
        .filter(|t| t.kind == "vagrant" && t.anchor_x == x)
        .count() as i64
}

/// Go: `wanderRadius`: Landstreicher einer Taverne wandern im Radius der Taverne, alle anderen um ihr Camp.
pub fn wander_radius(w: &World, ti: usize) -> f64 {
    let v = values(w);
    let ax = w.troops[ti].anchor_x;
    if w.sites.iter().any(|s| s.kind == "tavern" && s.x == ax) {
        v.buildings.get("tavern").map_or(0.0, |b| b.vagrants.wander_units)
    } else {
        v.economy.recruit_camp.wander_units
    }
}
