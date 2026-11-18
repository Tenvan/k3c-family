//! Zauberer-Linie (Port von `engine/sim/skills_caster.go`; docs/rules/monarch.md § 3): Fireball, Ice Wall, Lightning Storm,
//! Meteor. Werte aus monarch.json › skills. Zielpunkt der Flächenzauber ist der nächste lebende Gegner in Range; ohne ihn
//! keine Wirkung (keine Abklingzeit). Schaden geht über `apply_damage`, damit Tod, Gold-Drop und Ereignisse wie beim Schlag
//! laufen.

use k3c_data::monarch::SkillEffect;

use crate::economy::nearest;
use crate::state::common::apply_damage;
use crate::state::{Storm, World};

/// Go: `spellCenter`: Lage des nächsten lebenden Gegners in Range um den Wirkenden `pi`; `None`, wenn keiner da ist.
fn spell_center(w: &World, pi: usize, e: &SkillEffect) -> Option<f64> {
    let px = w.players[pi].x;
    nearest(&w.enemies, |en| en.x, px, e.range, |en| en.hp > 0.0).map(|ei| w.enemies[ei].x)
}

/// Go: `areaEffects`: effect.type der Flächenzauber, die um `spell_center` wirken (Ort und Radius im Ereignis skillCast, B-674).
fn is_area_effect(t: &str) -> bool {
    matches!(t, "areaDamage" | "slow" | "damageOverTime")
}

/// Go: `castPlace`: Wirkort (Units) und Radius für das Ereignis skillCast, vor dem Wirken bestimmt (stirbt das Ziel im
/// selben Takt, bleibt der Ort). Flächenzauber am Ziel mit ihrem Radius, alle anderen Skills am Wirkenden mit Radius 0.
pub fn cast_place(w: &World, pi: usize, e: &SkillEffect) -> (f64, f64) {
    if is_area_effect(&e.r#type)
        && let Some(x) = spell_center(w, pi, e)
    {
        return (x, e.radius);
    }
    (w.players[pi].x, 0.0)
}

/// Go: `enemiesAround`: ruft `f(w, ei)` für jeden lebenden Gegner (Index `ei`) im Radius `r` um `x` auf (Reihenfolge von
/// `w.enemies`; die Bedingung wird beim Besuch geprüft, ein Treffer kann spätere Gegner nicht mehr erreichbar machen).
pub fn enemies_around(w: &mut World, x: f64, r: f64, f: &mut dyn FnMut(&mut World, usize)) {
    for ei in 0..w.enemies.len() {
        if w.enemies.get(ei).is_some_and(|en| en.hp > 0.0 && (en.x - x).abs() <= r) {
            f(w, ei);
        }
    }
}

/// Schaden `damage` an den Gegner `ei` über `apply_damage`.
fn hurt(w: &mut World, ei: usize, damage: f64) {
    let id = w.enemies[ei].id;
    apply_damage(w, id, damage);
}

/// Go: `castAreaDamage` (Fireball, Meteor): Damage an alle Gegner im Radius um das Ziel.
pub fn cast_area_damage(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let center = spell_center(w, pi, e);
    if let Some(x) = center {
        enemies_around(w, x, e.radius, &mut |w, ei| hurt(w, ei, e.damage));
    }
    center.is_some()
}

/// Go: `castSlow` (Ice Wall): Gegner im Radius um das Ziel laufen `duration` Sekunden mit `factor` der Geschwindigkeit.
pub fn cast_slow(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let center = spell_center(w, pi, e);
    if let Some(x) = center {
        enemies_around(w, x, e.radius, &mut |w, ei| {
            let en = &mut w.enemies[ei];
            (en.slow, en.slow_for) = (e.factor, e.duration);
        });
    }
    center.is_some()
}

/// Go: `castDamageOverTime` (Lightning Storm): ein Sturm um das Ziel, der `duration` Sekunden lang `per_second` Schaden macht.
pub fn cast_damage_over_time(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let center = spell_center(w, pi, e);
    if let Some(x) = center {
        let owner = w.players[pi].id;
        w.storms.push(Storm {
            x,
            radius: e.radius,
            per_second: e.per_second,
            left: e.duration,
            owner,
        });
    }
    center.is_some()
}

/// Go: `stepStorms`: Jeder Sturm trifft die Gegner in seinem Radius mit `per_second` × dt (im letzten Tick nur der Rest) und
/// endet nach seiner Dauer. Reihenfolge: Wirk-Reihenfolge.
pub fn step_storms(w: &mut World, dt: f64) {
    let mut storms = std::mem::take(&mut w.storms);
    for s in &mut storms {
        let damage = s.per_second * dt.min(s.left);
        enemies_around(w, s.x, s.radius, &mut |w, ei| hurt(w, ei, damage));
        s.left -= dt;
    }
    storms.retain(|s| s.left > 1e-9);
    w.storms = storms;
}
