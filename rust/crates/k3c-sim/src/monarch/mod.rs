//! Monarch: Schlag, Fund-Pool, Verteilen, Respec und Presets (Port von `engine/sim/monarch.go`; docs/rules/monarch.md
//! §§ 1–3, B-118). Ein Punkt kauft einen Skill oder ein Passiv, jeden höchstens einmal. Verfügbar sind Pool der Insel minus
//! gelernte Skills; wer spät beitritt, hat damit alle bisherigen Punkte.
//!
//! Zustandsmodell: Go-Zeiger `p *Player` sind `pi: usize`, Gos `w.island` ist `Option<&mut IslandRef>`. Der Pool einer Insel
//! steht in `IslandRef::skill_pool` und wird in jeder Stufe nach `World::skill_points` gespiegelt (`add_pool_points`,
//! `Island::put_stage_ref`); darum liest `pool_of` immer `w.skill_points`, in einer Insel wie ohne sie. Fehler sind
//! `Result<(), String>` mit dem Text von Go. Reittier und Passive stehen in `mount.rs` und `passives.rs` (RU18.5).

use serde_json::json;

use crate::economy::nearest;
use crate::pending::{emit, unit_x};
use crate::state::common::{apply_damage, event};
use crate::state::island_ref::IslandRef;
use crate::state::{PlayerCommand, World, WorldValues};
use crate::world::vals;

pub mod data;
pub mod mount;
pub mod passives;

#[cfg(test)]
mod can_tests;
#[cfg(test)]
mod mod_tests;
#[cfg(test)]
mod mount_tests;
#[cfg(test)]
mod passives_own_tests;
#[cfg(test)]
mod passives_tests;
#[cfg(test)]
mod presets_tests;
#[cfg(test)]
mod testutil;

pub use data::{check_presets, new_skill_table, skill_by_id, skill_ref};
pub use mount::mount_of;
pub use passives::{
    cooldown_mult_of, damage_mult_of, each_passive, frost_armor_hit, speed_of, step_passives, troop_damage,
};

/// Go: `stepAttack`: Schlag (Taste X) auf den nächsten lebenden Gegner in Reichweite, beide Seiten. Ohne Gegner gilt dieselbe
/// Abklingzeit, `strike` meldet es mit `hit` false (B-321). Nur für lebende Monarchen aufgerufen.
pub fn step_attack(w: &mut World, v: &WorldValues, pi: usize, cmd: &PlayerCommand, dt: f64) {
    let p = &mut w.players[pi];
    p.attack_cooldown = (p.attack_cooldown - dt).max(0.0);
    if !cmd.attack || p.attack_cooldown > 0.0 {
        return;
    }
    let a = &v.monarch.attack;
    let (px, pid) = (p.x, p.id);
    let target = nearest(&w.enemies, |e| e.x, px, a.range, |e| e.hp > 0.0).map(|ei| w.enemies[ei].id);
    let p = &mut w.players[pi];
    p.attack_cooldown = a.cooldown;
    p.stealth_for = 0.0; // ein eigener Schlag beendet die Unsichtbarkeit (Smoke Bomb, B-577)
    let ev = json!({"from": pid, "x": unit_x(px), "hit": target.is_some()});
    emit(w, "strike", event(ev));
    if let Some(id) = target {
        let damage = a.damage * damage_mult_of(w, v, pi, false);
        apply_damage(w, id, damage);
    }
}

/// Go: `poolOf`: der Fund-Pool der Insel; alle Stufen spiegeln ihn nach `skill_points`, ohne Insel ist es der Zähler der Welt.
pub fn pool_of(w: &World) -> i64 {
    w.skill_points
}

/// Go: `addPoolPoints`: der einzige Weg, Pool-Punkte zu vergeben (Verstecke, Truhen; später Bosse und Meilensteine). Alle
/// Stufen der Insel spiegeln den Pool nach `skill_points` (der Spielstand speichert den Pool, S1.4): diese Stufe sofort, die
/// übrigen in `Island::put_stage_ref`.
pub fn add_pool_points(w: &mut World, isl: Option<&mut IslandRef>, n: i64) {
    match isl {
        None => w.skill_points += n,
        Some(isl) => {
            isl.skill_pool += n;
            w.skill_points = isl.skill_pool;
        }
    }
}

/// Go: `chestSkillPoint`: zählt eine geöffnete Truhe der Insel; jede n-te (`skillPointSources.chestEvery`) gibt einen Punkt.
pub fn chest_skill_point(w: &mut World, v: &WorldValues, isl: Option<&mut IslandRef>, pi: usize) {
    let every = v.monarch.skill_point_sources.chest_every;
    let Some(isl) = isl.filter(|_| every > 0) else {
        return;
    };
    isl.chests_opened += 1;
    if isl.chests_opened % every == 0 {
        add_pool_points(w, Some(isl), 1);
        let ev = json!({"player": w.players[pi].index, "total": w.skill_points});
        emit(w, "skillPoint", event(ev));
    }
}

/// Go: `AvailablePoints`: die Punkte, die der Spieler noch verteilen kann. Mitgebrachte Skills einer früheren Insel
/// (`skill_base`) zählen nicht.
pub fn available_points(w: &World, pi: usize) -> i64 {
    let p = &w.players[pi];
    pool_of(w) - (p.skills.len() as i64 - p.skill_base)
}

/// Go: `lineCount`: zählt die gelernten Skills einer Linie.
pub fn line_count(v: &WorldValues, skills: &[String], line: &str) -> i64 {
    skills
        .iter()
        .filter(|id| skill_ref(v, id).is_some_and(|s| s.line == line))
        .count() as i64
}

/// Go: `tierNeed`: gelernte Skills der Linie, die Tier `tier` verlangt (`points` = monarch.json › tierPoints).
pub fn tier_need(points: &[i64], tier: i64) -> i64 {
    if tier < 1 || tier > points.len() as i64 {
        return i64::MAX;
    }
    points[(tier - 1) as usize]
}

/// Go: `CanLearn`: prüft, ob der Spieler den Skill jetzt lernen darf, ohne etwas zu ändern (B-270). `learn_skill` nutzt sie selbst.
pub fn can_learn(w: &World, v: &WorldValues, pi: usize, id: &str) -> Result<(), String> {
    let p = &w.players[pi];
    let Some(s) = skill_ref(v, id) else {
        return Err(format!("skill {id:?} unbekannt"));
    };
    if p.skills.iter().any(|k| k == id) {
        return Err(format!("skill {id:?} schon gelernt"));
    }
    if available_points(w, pi) <= 0 {
        return Err(format!("skill {id:?}: keine Punkte frei"));
    }
    if !s.requires.is_empty() && !p.skills.contains(&s.requires) {
        return Err(format!("skill {id:?} verlangt zuerst {:?}", s.requires));
    }
    let need = tier_need(&v.monarch.tier_points, s.tier);
    if line_count(v, &p.skills, &s.line) < need {
        return Err(format!(
            "skill {id:?}: Tier {} verlangt {need} gelernte Skills der Linie {}",
            s.tier, s.line
        ));
    }
    Ok(())
}

/// Go: `LearnableSkills`: die Skills, die der Spieler jetzt lernen darf (`can_learn` ohne Fehler), nach ID sortiert (B-285).
pub fn learnable_skills(w: &World, v: &WorldValues, pi: usize) -> Vec<String> {
    let ids = v.skills.list.iter().map(|s| s.id.as_str());
    ids.filter(|id| can_learn(w, v, pi, id).is_ok())
        .map(str::to_string)
        .collect()
}

/// Go: `LearnSkill`: lernt einen Skill für einen Punkt. Aktive Skills belegen den ersten freien Slot 1 bis 4; ohne freien
/// Slot bleibt der Skill gelernt ohne Slot.
pub fn learn_skill(w: &mut World, v: &WorldValues, pi: usize, id: &str) -> Result<(), String> {
    can_learn(w, v, pi, id)?;
    let active = skill_ref(v, id).is_some_and(|s| s.kind == "active");
    let p = &mut w.players[pi];
    p.skills.push(id.to_string());
    if !active {
        return Ok(());
    }
    if let Some(slot) = p.slots.iter_mut().find(|s| s.is_empty()) {
        *slot = id.to_string();
    } else if p.slots.len() < 4 {
        p.slots.push(id.to_string());
    }
    Ok(())
}

/// Go: `CanRespec`: prüft Tag und Burg-Nähe, ohne etwas zu ändern (B-270). Ohne gelernte Skills ist Respec erlaubt, ohne Wirkung.
pub fn can_respec(w: &World, v: &WorldValues, pi: usize) -> Result<(), String> {
    if w.cycle.phase != "day" {
        return Err(format!("respec nur am Tag (jetzt {})", w.cycle.phase));
    }
    if (w.players[pi].x - w.castle.x).abs() > v.hub.castle_radius_units {
        return Err("respec nur an der Burg".to_string());
    }
    Ok(())
}

/// Go: `Respec`: setzt alle Skills zurück, kostenlos, nur am Tag und an der Burg.
pub fn respec(w: &mut World, v: &WorldValues, pi: usize) -> Result<(), String> {
    can_respec(w, v, pi)?;
    let p = &mut w.players[pi];
    p.skills.clear();
    p.slots.clear();
    p.skill_base = 0;
    Ok(())
}

/// Go: `ApplyPreset`: lernt die `activeSkills` eines Presets in ihrer Reihenfolge, soweit Pool-Punkte frei sind. Skills, die
/// nicht lernbar sind (keine Punkte, Tier-Gating), werden übersprungen; unbekannte Skills lehnt schon das Laden ab (B-504).
pub fn apply_preset(w: &mut World, pi: usize, id: &str) -> Result<(), String> {
    let v = vals(w);
    let Some(preset) = v.monarch.presets.get(id) else {
        return Err(format!("preset {id:?} unbekannt"));
    };
    for s in &preset.active_skills {
        if available_points(w, pi) <= 0 {
            break;
        }
        let _ = learn_skill(w, &v, pi, s);
    }
    Ok(())
}

/// Go: `presetFor`: das Standard-Preset beim Beitritt (monarch.md § 2): Tank für Spieler 1, Zauberer für Spieler 2, sonst Heiler.
pub fn preset_for(index: i64) -> &'static str {
    match index {
        0 => "tank",
        1 => "mage",
        _ => "healer",
    }
}
