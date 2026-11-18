//! Bosse (Port von `engine/sim/boss.go`, docs/rules/bosse.md § 1): Ein Miniboss kommt mit seiner Welle über das erste
//! Portal der Stufe, nutzt seine Fähigkeit, flieht nie und gibt beim Tod seine Belohnung. Ein besiegter Boss steht in
//! `IslandRef::defeated_bosses` und kommt nie wieder (auch nicht nach einem Burgfall). Bosse verbrauchen beim Erscheinen
//! kein rng; nur die Münzen der Belohnung werden wie jeder Drop gestreut. Bosse gibt es nur auf Inseln (der Merker gehört
//! der Insel). Der Endboss wartet in seinem Bau (`endboss.rs`).
//!
//! Funktionen, die Go über `w.values()` den Wertesatz holen lassen, gibt es zweimal: die Fassung ohne Wertesatz (Namen wie
//! in Go, ruft `values(w)`) und `*_with`, der der Wertesatz mitgegeben wird (Tests brauchen `values` nicht).

pub mod abilities;
pub mod data;
pub mod endboss;

use serde_json::json;

use k3c_data::bosses::{BossAbility, BossData};

use crate::enemies::spawn_with;
use crate::enemies::target::TargetKind;
use crate::enemies::traits::aoe_targets;
use crate::state::common::{apply_damage_by, event};
use crate::state::island_ref::IslandRef;
use crate::state::pending::{
    add_pool_points, add_stock_capped, emit, frost_armor_hit, moon_damage, scatter_coins, unit_x, values,
};
use crate::state::{Enemy, World, WorldValues};

use abilities::{arm_boss, flame_trail, shard_throw};
use data::{boss_by_id, mini_boss};
use endboss::boss_ability_of;

/// Go: `triggerBoss`: Am Wellenstart erscheint der Miniboss der Stufe ab seiner Welle, solange er nicht besiegt ist und
/// kein Boss der Stufe lebt.
pub fn trigger_boss(w: &mut World, isl: Option<&mut IslandRef>) {
    let v = values(w);
    trigger_boss_with(w, &v, isl.as_deref());
}

/// `trigger_boss` mit bereits geholtem Wertesatz.
pub fn trigger_boss_with(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>) {
    let Some(isl) = isl else {
        return;
    };
    let Some(b) = mini_boss(&v.bosses, w.biome.depth) else {
        return;
    };
    if w.wave < b.wave || w.portals.is_empty() || isl.defeated_bosses.contains(&b.id) {
        return;
    }
    if w.enemies.iter().any(|e| e.boss) {
        return;
    }
    let x = w.portals[0];
    spawn_boss(w, v, b, x, Some(isl));
}

/// Go: `spawnBoss`: stellt den Boss bei `x` auf und gibt seinen Index zurück: Werte des Bezugs-Gegners × Boss-Faktoren,
/// mit Tiefen-Skalierung, Grad (`enemyHp`, `enemyDamage`) und Spielerfaktor der Insel auf die HP; die Wellengröße des
/// Grads gilt nicht. Der Spielerfaktor wirkt auf die gerundeten HP, damit 3 Spieler genau doppelt so viele HP ergeben
/// wie einer.
pub fn spawn_boss(w: &mut World, v: &WorldValues, b: &BossData, x: f64, isl: Option<&IslandRef>) -> usize {
    let ei = place_boss(w, v, b, x, isl);
    emit(w, "bossSpawned", event(json!({"boss": b.id, "x": unit_x(x)})));
    ei
}

/// `spawn_boss` ohne das Ereignis `bossSpawned`: der Boss steht in der Stufe, kein rng, kein Ereignis.
pub(crate) fn place_boss(w: &mut World, v: &WorldValues, b: &BossData, x: f64, isl: Option<&IslandRef>) -> usize {
    let (hp, damage, players) = isl.map_or((1.0, 1.0, 1), |i| {
        (i.grade.enemy_hp, i.grade.enemy_damage, i.players.max(1))
    });
    let ei = spawn_with(w, v, &b.base, x, b.hp_factor * hp, b.damage_factor * damage, isl);
    let extra = v.waves.boss_hp_per_extra_player.unwrap_or(0.0);
    let speed = b.speed * v.waves.depth_scaling.speed.powf(w.biome.depth as f64);
    let e = &mut w.enemies[ei];
    e.hp = (e.hp * (1.0 + extra * (players - 1) as f64)).round();
    e.max_hp = e.hp;
    e.kind.clone_from(&b.id);
    e.boss = true;
    e.traits.clear();
    e.range = b.range;
    e.speed = speed;
    arm_boss(e, &b.ability);
    ei
}

/// Go: `bossAbility` (nur Bosse): Ist die Fähigkeit (beim Endboss die seiner Phase, `endboss.rs`) fällig
/// (`Enemy::aoe_in` zählt bei Bossen bis zur nächsten), ruft `summon` Gegner an den Ort des Bosses; `aoe` schlägt, sobald
/// ein Ziel im Radius steht, und trifft alle Ziele dort je einmal; `flame_trail` und `shard_throw` stehen in
/// `abilities.rs`.
pub fn boss_ability(w: &mut World, ei: usize, isl: Option<&mut IslandRef>) {
    let v = values(w);
    boss_ability_with(w, &v, ei, isl.as_deref());
}

/// `boss_ability` mit bereits geholtem Wertesatz.
pub fn boss_ability_with(w: &mut World, v: &WorldValues, ei: usize, isl: Option<&IslandRef>) {
    let e = &w.enemies[ei];
    if !e.boss || e.aoe_in > 0.0 {
        return;
    }
    let Some(b) = boss_by_id(&v.bosses, &e.kind) else {
        return;
    };
    let a = boss_ability_of(b, isl);
    match a.r#type.as_str() {
        "summon" => {
            w.enemies[ei].aoe_in = a.interval_seconds;
            summon(w, v, ei, a, isl);
        }
        "flameTrail" => flame_trail(w, ei, a),
        "shardThrow" => shard_throw(w, v, ei, a),
        "aoe" => boss_aoe(w, v, ei, a),
        _ => {}
    }
}

/// Der Flächenschlag des Bosses (`aoe`): trifft, sobald ein Ziel im Radius steht, alle Ziele dort je einmal.
fn boss_aoe(w: &mut World, v: &WorldValues, ei: usize, a: &BossAbility) {
    let (x, id, damage) = (w.enemies[ei].x, w.enemies[ei].id, w.enemies[ei].damage);
    let targets = aoe_targets(w, v, x, a.radius);
    if targets.is_empty() {
        return;
    }
    w.enemies[ei].aoe_in = a.interval_seconds;
    emit(w, "strike", event(json!({"from": id, "x": unit_x(x)})));
    let kind = w.enemies[ei].kind.clone(); // ein Schlag je Intervall
    for c in &targets {
        let dmg = moon_damage(w, damage); // Blutmond (events_moon.go)
        apply_damage_by(w, c.id, dmg, &kind);
        if c.kind == TargetKind::Player
            && let Some(pi) = c.player
        {
            frost_armor_hit(w, pi, ei);
        }
    }
}

/// Go: `summon`: `count` Gegner (ein Schwarm-Gegner als ganzer Schwarm) mit den Wellenfaktoren der Insel am Ort des
/// Bosses; sie fliehen zu seinem Portal.
fn summon(w: &mut World, v: &WorldValues, ei: usize, a: &BossAbility, isl: Option<&IslandRef>) {
    let (hp, damage) = isl.map_or((1.0, 1.0), |i| {
        let (_, hp, damage) = i.wave_factors();
        (hp, damage)
    });
    let mut n = a.count;
    if let Some(d) = v.enemies.get(a.enemy.as_str())
        && d.traits.iter().any(|t| t == "swarm")
    {
        n *= d.swarm_size.max(1);
    }
    let (x, home_x) = (w.enemies[ei].x, w.enemies[ei].home_x);
    for _ in 0..n {
        let si = spawn_with(w, v, &a.enemy, x, hp, damage, isl);
        w.enemies[si].home_x = home_x;
    }
}

/// Go: `bossDefeated`: Belohnung (Gold als Münzen am Ort, Material in den Insel-Vorrat bis zum Lager-Maximum,
/// Skill-Punkte in den Fund-Pool), Merker „besiegt“ und Ereignis.
pub fn boss_defeated(w: &mut World, e: &Enemy, isl: Option<&mut IslandRef>) {
    let v = values(w);
    boss_defeated_with(w, &v, e, isl);
}

/// `boss_defeated` mit bereits geholtem Wertesatz.
pub fn boss_defeated_with(w: &mut World, v: &WorldValues, e: &Enemy, mut isl: Option<&mut IslandRef>) {
    let Some(b) = boss_by_id(&v.bosses, &e.kind) else {
        return; // Go: nil-Zugriff; ein Gegner mit `boss` hat immer einen Eintrag
    };
    scatter_coins(w, e.x, b.reward.gold + e.carried_gold);
    let res = w.biome.primary_resource.clone(); // einmal je Boss
    if let Some(taken) = add_stock_capped(w, isl.as_deref(), &res, b.reward.material)
        && taken > 0
    {
        emit(w, "gathered", event(json!({"resource": res, "amount": taken})));
    }
    let sources = &v.monarch.skill_point_sources;
    let points = if b.kind == "end" {
        sources.endboss
    } else {
        sources.miniboss
    };
    let given = isl.as_deref().is_some_and(|i| i.defeated_bosses.contains(&b.id)); // schon vergeben (secureStage): kein Doppel
    if !given {
        add_pool_points(w, isl.as_deref_mut(), points);
    }
    if let Some(i) = isl.as_deref_mut()
        && !i.defeated_bosses.contains(&b.id)
    {
        i.defeated_bosses.push(b.id.clone());
    }
    endboss::endboss_defeated(b, isl);
    emit(w, "bossDefeated", event(json!({"boss": b.id, "x": unit_x(e.x)})));
}

#[cfg(test)]
mod abilities_tests;
#[cfg(test)]
mod data_tests;
#[cfg(test)]
mod endboss_tests;
#[cfg(test)]
mod real_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) mod testutil;
