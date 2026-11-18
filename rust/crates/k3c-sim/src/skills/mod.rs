//! Aktive Skills (Port von `engine/sim/skills.go`; docs/rules/monarch.md § 3, B-119): Der Befehl `skill` löst den Skill in
//! Slot 1 bis 4 aus. Die Wirkung steht in monarch.json › skills › effect; `effect.type` wählt die Funktion. Abklingzeiten
//! laufen je Spieler und Slot.
//!
//! Zustandsmodell: Gos `skillEffects`-Map ist die `match`-Funktion `cast_effect`, `p *Player` ist `pi: usize`.

use serde_json::json;

use k3c_data::monarch::SkillEffect;

use crate::links::{emit, unit_x};
use crate::monarch::passives::{cooldown_mult_of, damage_mult_of, spell_echoes};
use crate::monarch::skill_ref;
use crate::state::common::event;
use crate::state::{PlayerCommand, World, WorldValues};

pub mod caster;
pub mod healer;
pub mod status;
pub mod tank;
pub mod thief;

#[cfg(test)]
mod cast_event_tests;
#[cfg(test)]
mod caster_tests;
#[cfg(test)]
mod feedback_tests;
#[cfg(test)]
mod healer_tests;
#[cfg(test)]
mod linien_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(test)]
mod tank_tests;
#[cfg(test)]
pub(crate) mod testutil;
#[cfg(test)]
mod thief_tests;

pub use caster::{enemies_around, step_storms};
pub use status::{Aura, auras_of};

/// Go: `skillEffects[e.Type](w, p, e)`: Wirkung je `effect.type`. `None` = unbekannter Effekt (wie ein fehlender
/// Map-Eintrag), `Some(false)` = ohne Wirkung, dann startet keine Abklingzeit.
pub fn cast_effect(w: &mut World, pi: usize, e: &SkillEffect) -> Option<bool> {
    Some(match e.r#type.as_str() {
        "taunt" => tank::cast_taunt(w, pi, e),
        "stun" => tank::cast_stun(w, pi, e),
        "shield" => tank::cast_shield(w, pi, e),
        "lastStand" => tank::cast_last_stand(w, pi, e),
        // Zauberer (caster.rs) und Heiler (healer.rs); Divine Shield nutzt "shield".
        "areaDamage" => caster::cast_area_damage(w, pi, e),
        "slow" => caster::cast_slow(w, pi, e),
        "damageOverTime" => caster::cast_damage_over_time(w, pi, e),
        "heal" => healer::cast_heal(w, pi, e),
        "groupHeal" => healer::cast_group_heal(w, pi, e),
        "reviveCrown" => healer::cast_revive_crown(w, pi, e),
        // Dieb (thief.rs, B-577)
        "backstab" => thief::cast_backstab(w, pi, e),
        "stealth" => thief::cast_stealth(w, pi, e),
        _ => return None,
    })
}

/// Go: `stepSkills` senkt Abklingzeiten und Skill-Effekte des Spielers `pi` und löst den Slot aus `cmd.skill` aus. Nur für
/// lebende Monarchen aufgerufen.
pub fn step_skills(w: &mut World, v: &WorldValues, pi: usize, cmd: &PlayerCommand, dt: f64) {
    let p = &mut w.players[pi];
    for cd in &mut p.cooldowns {
        *cd = (*cd - dt).max(0.0);
    }
    p.shield_for = (p.shield_for - dt).max(0.0);
    if p.shield_for == 0.0 {
        p.shield = 0.0;
    }
    p.last_stand_for = (p.last_stand_for - dt).max(0.0);
    p.stealth_for = (p.stealth_for - dt).max(0.0);
    if (1..=4).contains(&cmd.skill) {
        // Der Versuch läuft jeden Tick, nur das Ereignis castFailed kommt je Tastendruck (B-378).
        let announce = p.skill_held != cmd.skill;
        cast_skill(w, v, pi, (cmd.skill - 1) as usize, announce);
    }
    w.players[pi].skill_held = cmd.skill;
}

/// Go: `castSkill` wirkt den Skill in Slot `slot` (0 bis 3): Slot belegt, Skill gelernt und Abklingzeit 0, sonst nichts.
/// `announce`: Bei fehlendem Ziel castFailed melden (nur beim ersten Tick des Tastendrucks).
fn cast_skill(w: &mut World, v: &WorldValues, pi: usize, slot: usize, announce: bool) {
    let p = &w.players[pi];
    if slot >= p.slots.len() || !p.skills.contains(&p.slots[slot]) {
        return;
    }
    if slot < p.cooldowns.len() && p.cooldowns[slot] > 0.0 {
        return;
    }
    let Some(s) = skill_ref(v, &p.slots[slot]) else {
        return;
    };
    let mult = damage_mult_of(w, v, pi, true); // Passive (passives.rs): Zauberschaden, Abklingzeit, Spell Echo
    let mut e = s.effect.clone();
    (e.damage, e.per_second, e.back_damage) = (e.damage * mult, e.per_second * mult, e.back_damage * mult);
    let (at, radius) = caster::cast_place(w, pi, &e);
    match cast_effect(w, pi, &e) {
        None => return,
        Some(false) => {
            if announce {
                let (id, x) = (w.players[pi].id, w.players[pi].x);
                // kein Ziel: keine Abklingzeit (B-321)
                emit(
                    w,
                    "castFailed",
                    event(json!({"from": id, "slot": slot, "x": unit_x(x)})),
                );
            }
            return;
        }
        Some(true) => {}
    }
    let id = w.players[pi].id;
    emit(
        w,
        "skillCast",
        event(json!({"from": id, "skill": s.id, "x": unit_x(at), "radius": radius})),
    ); // Wirkort (B-674)
    if e.r#type != "stealth" {
        w.players[pi].stealth_for = 0.0; // jeder andere Skill beendet die Unsichtbarkeit (B-577, Annahme)
    }
    let mut cd = s.cooldown;
    if s.line == "mage" {
        cd *= cooldown_mult_of(w, v, pi);
        if spell_echoes(w, pi) {
            let _ = cast_effect(w, pi, &e); // zweite Wirkung: Ergebnis zählt nicht
        }
    }
    if w.players[pi].cooldowns.is_empty() {
        w.players[pi].cooldowns = vec![0.0; 4];
    }
    w.players[pi].cooldowns[slot] = cd;
}
