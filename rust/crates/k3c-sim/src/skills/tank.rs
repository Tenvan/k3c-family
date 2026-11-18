//! Tank-Linie (Port von `engine/sim/skills_tank.go`; docs/rules/monarch.md § 3): Taunt, Shield Bash, Iron Wall, Last Stand.
//! Werte aus monarch.json › skills. Alle `cast_*` haben die Signatur von `cast_effect`; wo Go kein `w` hat (`_ *World`),
//! bleibt `w` für die einheitliche Signatur stehen.

use k3c_data::monarch::SkillEffect;

use crate::economy::nearest;
use crate::state::World;

/// Go: `castTaunt`: Lebende Gegner im Radius zielen `duration` Sekunden auf den Wirkenden, sofern sie ihn erreichen
/// (`chooseTarget`).
pub fn cast_taunt(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let (px, pid) = (w.players[pi].x, w.players[pi].id);
    for en in &mut w.enemies {
        if en.hp > 0.0 && (en.x - px).abs() <= e.radius {
            (en.taunt_id, en.taunt_for) = (pid, e.duration);
        }
    }
    true
}

/// Go: `castStun` (Shield Bash): Der nächste lebende Gegner in Range ist `duration` Sekunden betäubt. Ohne Gegner keine
/// Wirkung.
pub fn cast_stun(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let px = w.players[pi].x;
    let Some(ei) = nearest(&w.enemies, |en| en.x, px, e.range, |en| en.hp > 0.0) else {
        return false;
    };
    w.enemies[ei].stun = e.duration;
    true
}

/// Go: `castShield` (Iron Wall, Divine Shield): Barriere aus `hp` Schild-Punkten (fängt Treffer ohne Goldverlust ab) auf dem
/// Wirkenden für `duration` Sekunden. Ein laufender Schild wird nur durch einen größeren ersetzt (B-220); die Abklingzeit
/// startet in jedem Fall.
pub fn cast_shield(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    let p = &mut w.players[pi];
    if e.hp > p.shield {
        (p.shield, p.shield_for) = (e.hp, e.duration);
    }
    true
}

/// Go: `castLastStand`: `duration` Sekunden lang hält die letzte Münze einen Treffer länger (`damagePlayer`).
pub fn cast_last_stand(w: &mut World, pi: usize, e: &SkillEffect) -> bool {
    w.players[pi].last_stand_for = e.duration;
    true
}
