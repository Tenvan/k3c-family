//! Heiler-Linie (Port von `engine/sim/skills_healer.go`; docs/rules/monarch.md § 3, kampf.md § 5): Heal und Group Heal heilen
//! Truppen (Monarchen haben keine HP), Divine Shield ist der Schild aus `tank.rs` (`cast_shield`), Resurrection bringt
//! gefallene Kronen zurück. Werte aus monarch.json › skills. Heilung geht nie über `max_hp`.

use k3c_data::monarch::SkillEffect;

use crate::crown::restore_crown;
use crate::state::World;
use crate::state::shared::CROWN_ENEMY;

/// Go: `castHeal`: Die Truppe mit dem kleinsten HP-Anteil in Range (Gleichstand: kleinere ID) bekommt `amount` HP.
pub fn cast_heal(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let px = w.players[pi].x;
    let mut best: Option<usize> = None;
    for (ti, t) in w.troops.iter().enumerate() {
        if t.hp <= 0.0 || t.hp >= t.max_hp || (t.x - px).abs() > e.range {
            continue;
        }
        let better = best.is_none_or(|bi| {
            let b = &w.troops[bi];
            t.hp / t.max_hp < b.hp / b.max_hp || (t.hp / t.max_hp == b.hp / b.max_hp && t.id < b.id)
        });
        if better {
            best = Some(ti);
        }
    }
    let Some(bi) = best else {
        return false;
    };
    let t = &mut w.troops[bi];
    t.hp = t.max_hp.min(t.hp + e.amount);
    true
}

/// Go: `castGroupHeal`: Alle Truppen im Radius um den Wirkenden bekommen `amount` HP.
pub fn cast_group_heal(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let px = w.players[pi].x;
    for t in &mut w.troops {
        if t.hp > 0.0 && (t.x - px).abs() <= e.radius {
            t.hp = t.max_hp.min(t.hp + e.amount);
        }
    }
    true
}

/// Go: `castReviveCrown` (Resurrection, Annahme aus kampf.md § 9): Jede gefallene Krone im Radius, die nicht ein Gegner trägt,
/// sitzt sofort wieder auf ihrem Monarchen. Ohne Krone im Radius keine Wirkung.
pub fn cast_revive_crown(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let px = w.players[pi].x;
    let mut hit = false;
    for qi in 0..w.players.len() {
        let reaches = w.players[qi]
            .crown
            .as_ref()
            .is_some_and(|c| c.state != CROWN_ENEMY && (c.x - px).abs() <= e.radius);
        if reaches {
            restore_crown(w, qi, pi);
            hit = true;
        }
    }
    hit
}
