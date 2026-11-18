//! Fähigkeiten der Minibosse von Eisenstollen und Kristallhöhle (Port von `engine/sim/boss_abilities.go`,
//! docs/rules/bosse.md § 1.1, K2.1b):
//! Flammenspur (`flame_trail`, Lava-Golem): Der Boss legt alle `interval_seconds` eine Bodenfläche an seinen Ort, solange
//! dort noch keine liegt (so entsteht die Spur beim Gehen, ein stehender Boss stapelt keine Flächen). Wer in einer
//! Fläche steht (Spieler, Truppen, Gebäude außer der Burg), nimmt einmal je Sekunde den Schaden einer Sekunde; die
//! Fläche läuft nach `duration_seconds` aus.
//! Splitter-Wurf (`shard_throw`, Splitter-Titan): Der Boss kämpft auf Reichweite (`range` aus den Daten, Verhalten wie
//! `ranged`); jeder Angriff ist ein Geschoss mit Splash, dessen Einschlag alle Ziele im Radius trifft.
//! Beides verbraucht kein rng.

use k3c_data::bosses::BossAbility;

use crate::enemies::target::{TargetKind, blocking_wall, choose_target};
use crate::enemies::traits::aoe_targets;
use crate::state::common::{apply_damage_by, sign};
use crate::state::pending::{arrow_event, moon_damage, values};
use crate::state::{Enemy, Hazard, Projectile, World, WorldValues};

/// Go: `shardSpeed`: Tempo des Splitters in Units/s (wie die Geschosse der Fernkämpfer, enemies.go).
const SHARD_SPEED: f64 = 20.0;

/// Go: `armBoss`: stellt die Fähigkeit beim Erscheinen ein: Wartezeit bis zum ersten Einsatz; der Splitter-Wurf ist sofort
/// bereit und macht den Boss zum Fernkämpfer.
pub fn arm_boss(e: &mut Enemy, a: &BossAbility) {
    e.aoe_in = a.interval_seconds;
    if a.r#type == "shardThrow" {
        e.aoe_in = 0.0;
        e.traits = vec!["ranged".to_string()];
    }
}

/// Go: `flameTrail`: legt eine Fläche an den Ort des Bosses `ei`, wenn dort noch keine liegt.
pub fn flame_trail(w: &mut World, ei: usize, a: &BossAbility) {
    let e = &mut w.enemies[ei];
    e.aoe_in = a.interval_seconds;
    let x = e.x;
    if w.hazards.iter().any(|h| (h.x - x).abs() < h.radius) {
        return;
    }
    w.hazards.push(Hazard {
        x,
        radius: a.radius,
        damage_per_second: a.damage_per_second,
        seconds_left: a.duration_seconds,
        cause: w.enemies[ei].kind.clone(), // je Fläche eine Art; Flächen entstehen höchstens einmal je Intervall
        ..Hazard::default()
    });
}

/// Go: `shardThrow`: wirft einen Splitter auf das Ziel, das der Boss auch sonst wählen würde. Die Abklingzeit des
/// Angriffs läuft mit dem Intervall, damit der Boss keinen Angriff ohne Splash macht.
pub fn shard_throw(w: &mut World, v: &WorldValues, ei: usize, a: &BossAbility) {
    let e = &w.enemies[ei];
    let dir = match sign(w.hub_x - e.x) {
        0.0 => 1.0,
        d => d,
    };
    let wall = blocking_wall(w, e, dir);
    let Some(t) = choose_target(w, v, ei, dir, wall) else {
        return;
    };
    let e = &mut w.enemies[ei];
    e.aoe_in = a.interval_seconds;
    e.cooldown = a.interval_seconds;
    let (x, damage, eid, cause) = (e.x, e.damage, e.id, e.kind.clone()); // ein Wurf je Intervall
    let p = Projectile {
        id: w.new_id(),
        x,
        target_id: t.id,
        team: "enemy".into(),
        damage: moon_damage(w, damage),
        speed: SHARD_SPEED,
        cause,
        splash: a.radius,
    };
    arrow_event(w, &p, eid);
    w.projectiles.push(p);
}

/// Go: `hitSplash`: Einschlag eines Geschosses mit Splash bei `x`: das Ziel und alle übrigen Ziele im Radius je einmal.
pub fn hit_splash(w: &mut World, p: &Projectile, x: f64) {
    let v = values(w);
    hit_splash_with(w, &v, p, x);
}

/// `hit_splash` mit bereits geholtem Wertesatz.
pub fn hit_splash_with(w: &mut World, v: &WorldValues, p: &Projectile, x: f64) {
    apply_damage_by(w, p.target_id, p.damage, &p.cause);
    for c in aoe_targets(w, v, x, p.splash) {
        if c.id != p.target_id {
            apply_damage_by(w, c.id, p.damage, &p.cause);
        }
    }
}

/// Go: `stepHazards`: lässt die Flächen auslaufen und zieht allen Zielen darin einmal je Sekunde den Schaden einer
/// Sekunde ab (wie Lava, world_lava.go); ausgelaufene Flächen fallen in Listen-Reihenfolge heraus.
pub fn step_hazards(w: &mut World, v: &WorldValues, dt: f64) {
    if w.hazards.is_empty() {
        return;
    }
    let mut hazards = std::mem::take(&mut w.hazards); // burn braucht die Welt, aber nicht die Flächenliste
    hazards.retain_mut(|h| {
        h.seconds_left -= dt;
        if h.seconds_left <= 0.0 {
            return false;
        }
        h.tick_in -= dt;
        if h.tick_in <= 0.0 {
            h.tick_in += 1.0;
            burn(w, v, h);
        }
        true
    });
    w.hazards = hazards;
}

/// Go: `burn`: Schaden an allen Zielen der Fläche außer der Burg und getrennten Monarchen.
fn burn(w: &mut World, v: &WorldValues, h: &Hazard) {
    for c in aoe_targets(w, v, h.x, h.radius) {
        if c.kind == TargetKind::Castle || c.player.is_some_and(|pi| w.players[pi].free) {
            continue;
        }
        apply_damage_by(w, c.id, h.damage_per_second, &h.cause);
    }
}
