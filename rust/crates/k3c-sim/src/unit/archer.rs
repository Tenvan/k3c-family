//! Bogenschützen (Port von `engine/sim/unit_archer.go`, `stepArcher` und `isOnTower` aus `units.ts`).
//!
//! Zeiger auf Truppen sind Indizes (`ti`); ein Turm ist seine Platz-ID (Zustandsmodell, Vorentscheidung 2).

use k3c_data::troops::TroopData;

use super::escort::escort_post;
use super::{ARRIVE, ARROW_SPEED, walk_to};
use crate::combat::squad_post;
use crate::state::common::{outer_wall, sign};
use crate::state::island_ref::IslandRef;
use crate::state::pending::{arrow_event, is_spell_tower};
use crate::state::{Projectile, Troop, World, WorldValues};

/// Go: `isOnTower`: Steht der Schütze auf seinem Turm?
pub fn is_on_tower(w: &World, t: &Troop) -> bool {
    let Some(id) = t.tower_id else {
        return false;
    };
    w.site(id).is_some_and(|tower| (tower.x - t.x).abs() < ARRIVE)
}

/// Go: `freeTower`: der nächste gebaute Turm auf der Seite des Schützen mit freiem Platz; ein Zaubertum hat keine.
/// Liefert die Platz-ID.
pub(crate) fn free_tower(w: &World, v: &WorldValues, ti: usize) -> Option<i64> {
    let t = &w.troops[ti];
    let slots = v.buildings.get("tower").map_or(0, |b| b.archer_slots);
    let mut best: Option<(i64, f64)> = None;
    for s in &w.sites {
        if s.kind != "tower"
            || s.state != "built"
            || is_spell_tower(w, s)
            || sign(s.x - w.hub_x) != sign(t.anchor_x - w.hub_x)
        {
            continue;
        }
        let used = w.troops.iter().filter(|o| o.tower_id == Some(s.id)).count() as i64;
        if used < slots && best.is_none_or(|(_, bx)| (s.x - t.x).abs() < (bx - t.x).abs()) {
            best = Some((s.id, s.x));
        }
    }
    best.map(|(id, _)| id)
}

/// Go: `archerTower`: der Turm des Schützen (einen freien nimmt er sich); `None` ohne Turm. Auf einem fertigen Zaubertum
/// steigt er ab und nimmt einen freien Posten (Q31).
pub fn archer_tower(w: &mut World, v: &WorldValues, ti: usize) -> Option<i64> {
    if w.troops[ti].group != 0 {
        // im Trupp hinter dem Banner: kein Turm (combat.go)
        w.troops[ti].tower_id = None;
        return None;
    }
    if w.troops[ti].tower_id.is_none() {
        w.troops[ti].tower_id = free_tower(w, v, ti);
    }
    let id = w.troops[ti].tower_id?;
    let tower = w.site(id)?;
    if is_spell_tower(w, tower) {
        w.troops[ti].tower_id = None;
        return None;
    }
    Some(id)
}

/// Go: `stepArcher`: Posten ist ein freier Platz auf einem Turm, sonst hinter der äußersten Mauer seiner Seite.
pub fn step_archer(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, ti: usize, dt: f64) {
    let tower = archer_tower(w, v, ti);
    let side = if w.troops[ti].anchor_x < w.hub_x { -1.0 } else { 1.0 };
    let mut post = w.hub_x + side * 10.0;
    if let Some(x) = tower.and_then(|id| w.site(id)).map(|s| s.x) {
        post = x;
    } else if let Some(wall) = outer_wall(w, side) {
        post = wall.x - side * 2.0;
    }
    // Begleitschutz: der Schütze verlässt Turm und Mauer (unit_escort.go); Trupp hinter dem Banner (combat.go)
    if let Some(x) = escort_post(w, v, isl, &w.troops[ti]).or_else(|| squad_post(w, v, &w.troops[ti])) {
        post = x;
    }
    walk_to(w, v, ti, post, dt);

    let fallback = TroopData::default(); // unbekannte Art: Nullwerte wie Gos Map-Zugriff
    let a = v.troops.get(w.troops[ti].kind.as_str()).unwrap_or(&fallback);
    let t = &mut w.troops[ti];
    t.cooldown = (t.cooldown - dt).max(0.0);
    let mut reach = a.range;
    if is_on_tower(w, &w.troops[ti]) {
        reach += v.buildings.get("tower").map_or(0, |b| b.range_bonus) as f64;
    }
    let t = &w.troops[ti];
    if t.cooldown > 0.0 {
        return;
    }
    let mut target: Option<(i64, f64)> = None;
    for e in &w.enemies {
        if (e.x - t.x).abs() > reach {
            continue;
        }
        if target.is_none_or(|(_, tx)| (e.x - t.x).abs() < (tx - t.x).abs()) {
            target = Some((e.id, e.x));
        }
    }
    let Some((target_id, _)) = target else {
        return;
    };
    let (x, tid) = (t.x, t.id);
    let p = Projectile {
        id: w.new_id(),
        x,
        target_id,
        team: "player".into(),
        damage: a.damage,
        speed: ARROW_SPEED,
        ..Default::default()
    };
    arrow_event(w, &p, tid);
    w.projectiles.push(p);
    w.troops[ti].cooldown = 1.0 / a.attacks_per_second;
}
