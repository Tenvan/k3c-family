//! Skills lernen (Port von `engine/balance/bot_learn.go`, BS3.4, B-647 Weg B): Ein Mensch lernt Skills im Menü (Aktion
//! `learn`, `learn_skill`), das ist keine Eingabe des Monarchen (`PlayerCommand`). Der Tester gibt dem Bot dieselbe
//! Aktion: Hat ein Monarch einen freien Skill-Punkt (Pool aus Bossen und Truppen-Funden, `monarch.md` § 3), lernt er den
//! nächsten Skill der Reihenfolge `LEARN_ORDER`, den die Regeln erlauben. Die Reihenfolge ist die der Tank-Linie: erst die
//! beiden Aktiven (öffnen Tier 2), dann Regeneration, mehr HP und Verteidigung; der Bot überlebt damit längere Kämpfe, ohne
//! dass die Spielregeln geändert werden.

use k3c_sim::island::Island;
use k3c_sim::monarch::{available_points, can_learn, learn_skill};
use k3c_sim::state::World;
use k3c_sim::world::vals;

/// Die Rangfolge der Skills, die ein Bot lernt.
pub const LEARN_ORDER: [&str; 9] = [
    "taunt",
    "shieldBash",
    "regeneration",
    "thickSkin",
    "fortified",
    "armorAura",
    "guardian",
    "ironWall",
    "lastStand",
];

/// Lässt jeden Monarchen der Insel mit freien Punkten Skills lernen; deterministisch, ohne Zufall.
pub fn bot_learn(isl: &mut Island) {
    let indexes: Vec<i64> = isl.players().iter().map(|p| p.index).collect();
    for index in indexes {
        let Some(si) = isl.stage_of(index) else {
            continue;
        };
        let w = &mut isl.stages[si];
        let Some(pi) = w.players.iter().position(|p| p.index == index) else {
            continue;
        };
        while available_points(w, pi) > 0 {
            if !learn_next(w, pi) {
                break;
            }
        }
    }
}

/// Lernt den ersten erlaubten Skill aus `LEARN_ORDER`; `false`, wenn keiner mehr geht.
fn learn_next(w: &mut World, pi: usize) -> bool {
    let v = vals(w);
    for id in LEARN_ORDER {
        if can_learn(w, &v, pi, id).is_ok() {
            return learn_skill(w, &v, pi, id).is_ok();
        }
    }
    false
}
