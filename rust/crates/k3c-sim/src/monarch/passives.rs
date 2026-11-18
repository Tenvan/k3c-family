//! Passive Skills (Port von `engine/sim/monarch_passives.go`; docs/rules/monarch.md § 3, B-119, S1.2c): Gelernte Passive
//! wirken immer (kein Slot, keine Abklingzeit). Werte aus monarch.json › skills › effect, alle vorläufig. Gleiche Effekte
//! mehrerer Besitzer addieren sich je Besitzer. Eigenschaften werden berechnet, nicht im Zustand gehalten; ohne gelernte
//! Passive liefern die Funktionen genau die Basiswerte. Nur lebende Besitzer wirken.
//!
//! Zustandsmodell: Go-Zeiger `p *Player` sind `pi: usize`; der Wertesatz kommt als `v` (Arc der Stufe, nicht Teil von `w`).

use k3c_data::monarch::SkillEffect;

use super::data::skill_ref;
use super::mount::mount_of;
use crate::crown::crowned;
use crate::state::purse::purse_max_of;
use crate::state::{IslandRef, Player, World, WorldValues};
use crate::world::vals;

/// Go: `eachPassive`: ruft `f` für jedes gelernte Passiv jedes lebenden Spielers: Spieler nach Index, Skills in
/// Lernreihenfolge.
pub fn each_passive(w: &World, f: &mut dyn FnMut(&Player, &SkillEffect)) {
    each_passive_v(w, &vals(w), f);
}

/// Wie `each_passive` mit dem Wertesatz `v` der Stufe (spart das Nachschlagen).
pub fn each_passive_v(w: &World, v: &WorldValues, f: &mut dyn FnMut(&Player, &SkillEffect)) {
    for o in &w.players {
        if !crowned(o) {
            continue;
        }
        for id in &o.skills {
            if let Some(s) = skill_ref(v, id)
                && s.kind == "passive"
            {
                f(o, &s.effect);
            }
        }
    }
}

/// Go: `statSum`: addiert `field` über alle stat-Passive, die den Monarchen `pi` erreichen (Aura um den Besitzer, ohne Aura
/// nur der Besitzer selbst).
fn stat_sum(w: &World, v: &WorldValues, pi: usize, field: impl Fn(&SkillEffect) -> f64) -> f64 {
    let (pid, px) = (w.players[pi].id, w.players[pi].x);
    let mut sum = 0.0;
    each_passive_v(w, v, &mut |o, e| {
        if e.r#type == "stat" && (o.id == pid || e.aura > 0.0 && (o.x - px).abs() <= e.aura) {
            sum += field(e);
        }
    });
    sum
}

/// Go: `speedOf`: Grundtempo des Monarchen (Basis × Reittier-Faktor × (1 + Blessings)).
pub fn speed_of(w: &World, v: &WorldValues, pi: usize) -> f64 {
    let ridden = v.monarch.base.speed * mount_of(v, pi).speed_factor;
    ridden * (1.0 + stat_sum(w, v, pi, |e| e.speed_bonus))
}

/// Go: `damageMultOf`: Faktor auf Schlag- und Zauberschaden (Blessings); `spell` = Zauber, dann zusätzlich Arcane Power.
pub fn damage_mult_of(w: &World, v: &WorldValues, pi: usize, spell: bool) -> f64 {
    1.0 + stat_sum(w, v, pi, |e| {
        if spell {
            e.damage_bonus + e.spell_bonus
        } else {
            e.damage_bonus
        }
    })
}

/// Go: `cooldownMultOf`: Faktor auf die Abklingzeit der Zauberer-Skills (Elemental Mastery).
pub fn cooldown_mult_of(w: &World, v: &WorldValues, pi: usize) -> f64 {
    1.0 + stat_sum(w, v, pi, |e| e.cooldown_bonus)
}

/// Go: `troopDamage`: Schaden an einer Truppe nach Verteidigung aus Auren (Armor Aura, mindestens 1) und Guardian (je
/// Besitzer im Radius `reduction` weniger). `ti` ist der Index der Truppe.
pub fn troop_damage(w: &World, ti: usize, damage: f64) -> f64 {
    let tx = w.troops[ti].x;
    let (mut def, mut cut) = (0.0, 0.0);
    each_passive(w, &mut |o, e| {
        let d = (o.x - tx).abs();
        if e.r#type == "stat" && e.aura > 0.0 && d <= e.aura {
            def += e.defense;
        } else if e.r#type == "guardian" && d <= e.radius {
            cut += e.reduction;
        }
    });
    let damage = if def > 0.0 { (damage - def).max(1.0) } else { damage };
    damage * (1.0 - cut).max(0.0)
}

/// Go: `stepPassives` (je Tick nach `stepPlayers`): Regeneration und Healing Aura. Ohne `aura` und `castle` ist `regen` ein
/// Gold-Rinnsal tagsüber für den Besitzer (kampf.md § 5, B-930: `perSecond` Münzen je Sekunde, bis der Geldsack voll ist); mit
/// `aura` oder `castle` heilt es Truppen (Monarchen haben keine HP mehr). Reihenfolge: Besitzer nach Index, dann Truppen.
pub fn step_passives(w: &mut World, isl: Option<&IslandRef>, dt: f64) {
    let v = vals(w);
    for oi in 0..w.players.len() {
        if !crowned(&w.players[oi]) {
            continue;
        }
        for k in 0..w.players[oi].skills.len() {
            let Some(s) = skill_ref(&v, &w.players[oi].skills[k]) else {
                continue;
            };
            let e = &s.effect;
            if s.kind != "passive" || e.r#type != "regen" {
                continue;
            }
            if e.aura == 0.0 && !e.castle {
                regen_gold(w, isl, oi, e.per_second * dt);
                continue;
            }
            let heal = e.per_second * dt;
            let ox = w.players[oi].x;
            for ti in 0..w.troops.len() {
                let t = &w.troops[ti];
                if t.hp > 0.0 && regen_reaches(w, &v, ox, e, t.x, false) {
                    let t = &mut w.troops[ti];
                    t.hp = t.max_hp.min(t.hp + heal);
                }
            }
        }
    }
}

/// Go: `regenGold`: sammelt Bruchteile und legt volle Münzen in den Geldsack des Besitzers; nur tagsüber, nie über das
/// Maximum.
fn regen_gold(w: &mut World, isl: Option<&IslandRef>, oi: usize, amount: f64) {
    let max = purse_max_of(w, &w.players[oi], isl);
    if w.cycle.phase != "day" || w.players[oi].gold >= max {
        return;
    }
    let o = &mut w.players[oi];
    o.regen_gold += amount;
    if o.regen_gold >= 1.0 {
        let n = o.regen_gold as i64;
        o.regen_gold -= n as f64;
        o.gold = max.min(o.gold + n);
    }
}

/// Go: `regenReaches`: `castle` = alle im Burgradius (Holy Ground, Auslegung), `aura` = im Radius um den Besitzer (`ox`),
/// sonst nur der Besitzer selbst (`own`).
fn regen_reaches(w: &World, v: &WorldValues, ox: f64, e: &SkillEffect, x: f64, own: bool) -> bool {
    if e.castle {
        return (x - w.castle.x).abs() <= v.hub.castle_radius_units;
    }
    if e.aura > 0.0 {
        return (x - ox).abs() <= e.aura;
    }
    own
}

/// Go: `frostArmorHit`: Trifft Gegner `ei` den Monarchen `pi` im Nahkampf, verlangsamt Frost Armor ihn (Feld wie Ice Wall).
/// Eine stärkere laufende Verlangsamung bleibt.
pub fn frost_armor_hit(w: &mut World, pi: usize, ei: usize) {
    let v = vals(w);
    for k in 0..w.players[pi].skills.len() {
        let Some(s) = skill_ref(&v, &w.players[pi].skills[k]) else {
            continue;
        };
        if s.kind != "passive" || s.effect.r#type != "frostArmor" {
            continue;
        }
        let en = &mut w.enemies[ei];
        if en.slow_for == 0.0 || s.effect.factor <= en.slow {
            (en.slow, en.slow_for) = (s.effect.factor, s.effect.duration);
        }
    }
}

/// Go: `spellEchoes`: Wirkt ein Zauber ein zweites Mal? Würfelt nur, wenn der Wirkende Spell Echo gelernt hat; sonst wird kein
/// Wert aus `w.rng` verbraucht.
pub fn spell_echoes(w: &mut World, pi: usize) -> bool {
    let v = vals(w);
    let mut chance = 0.0;
    for id in &w.players[pi].skills {
        if let Some(s) = skill_ref(&v, id)
            && s.kind == "passive"
            && s.effect.r#type == "spellEcho"
        {
            chance += s.effect.chance;
        }
    }
    chance > 0.0 && w.rng.next() < chance
}
