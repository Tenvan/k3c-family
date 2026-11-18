//! Dieb-Linie (Port von `engine/sim/skills_thief.go`; docs/rules/monarch.md § 3, B-577): Backstab und Smoke Bomb. Werte aus
//! monarch.json › skills.

use k3c_data::monarch::SkillEffect;

use crate::economy::nearest;
use crate::state::common::{apply_damage, sign};
use crate::state::{Enemy, World};

/// Go: `castBackstab`: `damage` am nächsten lebenden Gegner in Range, `back_damage`, wenn der Wirkende in dessen Rücken
/// steht (`behind_enemy`). Ohne Gegner keine Wirkung.
pub fn cast_backstab(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let px = w.players[pi].x;
    let Some(ei) = nearest(&w.enemies, |en| en.x, px, e.range, |en| en.hp > 0.0) else {
        return false;
    };
    let damage = if behind_enemy(w, &w.enemies[ei], px) {
        e.back_damage
    } else {
        e.damage
    };
    let id = w.enemies[ei].id;
    apply_damage(w, id, damage);
    true
}

/// Go: `enemyFacing` ist die Blickrichtung eines Gegners (1 rechts, -1 links): Fliehende blicken zu ihrem Portal, alle
/// übrigen (auch Wachen und Betäubte) zur Burg, in Richtung ihres Marschs.
pub fn enemy_facing(w: &World, en: &Enemy) -> f64 {
    let to = if en.fleeing { en.home_x } else { w.hub_x };
    let d = sign(to - en.x);
    if d != 0.0 { d } else { 1.0 }
}

/// Go: `behindEnemy`: `x` liegt hinter dem Gegner, entgegen seiner Blickrichtung; dieselbe Stelle zählt als vorn.
pub fn behind_enemy(w: &World, en: &Enemy, x: f64) -> bool {
    (x - en.x) * enemy_facing(w, en) < 0.0
}

/// Go: `castStealth` (Smoke Bomb): `duration` Sekunden unsichtbar (`stealth_for`, shielded): Gegner wählen den Wirkenden nicht
/// als Ziel und verlieren ihn, kein Schaden. Ein eigener Schlag oder ein anderer Skill beendet es vorzeitig (`step_attack`,
/// `cast_skill`).
pub fn cast_stealth(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let p = &mut w.players[pi];
    p.stealth_for = p.stealth_for.max(e.duration);
    true
}
