//! Geldsack (Port von `engine/sim/purse.go`): Der Beutel eines Spielers hat drei Größen. Größe 1 ist der Grundwert
//! (`economy.purse.maxGold`), Größe 2 und 3 lernt der Spieler als Passive purseMedium und purseLarge. Jede Größe ist die
//! Münz-Obergrenze des Spielers; die Größe wird berechnet, nicht gespeichert.

use super::pending::{purse_max, skill_ref, values};
use super::{Player, World};

/// Go: `PurseSize`: die Größe 1 bis 3 des Geldsacks von `p`, für das HUD (die höchste gelernte Stufe).
pub fn purse_size(w: &World, p: &Player) -> i64 {
    purse_skill(w, p).0
}

/// Go: `PurseMaxOf`: die Münz-Obergrenze von `p`: Grundwert oder gelernte Größe, dazu der Dev-Bonus der Insel (`purse_max`).
pub fn purse_max_of(w: &World, p: &Player) -> i64 {
    let (_, gold) = purse_skill(w, p);
    if gold == 0 {
        return purse_max(w);
    }
    gold + purse_max(w) - values(w).economy.purse.max_gold
}

/// Go: `purseSkill`: Größe und Obergrenze des besten gelernten Geldsack-Skills von `p` (Größe 1 und Obergrenze 0 ohne Skill).
pub fn purse_skill(w: &World, p: &Player) -> (i64, i64) {
    let (mut size, mut max_gold) = (1, 0);
    for id in &p.skills {
        let v = values(w);
        if let Some(s) = skill_ref(&v, id)
            && s.kind == "passive"
            && s.effect.r#type == "purse"
            && s.effect.size > size
        {
            (size, max_gold) = (s.effect.size, s.effect.max_gold);
        }
    }
    (size, max_gold)
}
