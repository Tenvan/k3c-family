//! Hilfen der Tests dieser Gruppe (Go: `tankPlayer`, `stepIdle` aus `skills_test.go`, `casterPlayer`, `toughEnemy`, `lost` aus
//! `skills_caster_test.go`). `quiet_world`, `add_player`, `spawn`, `must_learn` und `add_pool_points` kommen aus den anderen
//! Gruppen.

use super::step_skills;
use crate::island::testutil::{add_pool_points, must_learn};
use crate::state::{PlayerCommand, World};
use crate::world::testutil::DT;
use crate::world::{add_player, vals};

/// Ein Befehl `skill` (Slot 1 bis 4).
pub fn skill_cmd(slot: i64) -> PlayerCommand {
    PlayerCommand {
        skill: slot,
        ..PlayerCommand::default()
    }
}

/// Go: `stepSkills(w, p, cmd, dt)`: ein Tick der Fertigkeiten des Spielers `pi`.
pub fn skills_tick(w: &mut World, pi: usize, cmd: &PlayerCommand) {
    let v = vals(w);
    step_skills(w, &v, pi, cmd, DT);
}

/// Go: `stepIdle`: `ticks` Ticks ohne Befehl.
pub fn step_idle(w: &mut World, pi: usize, ticks: usize) {
    for _ in 0..ticks {
        skills_tick(w, pi, &PlayerCommand::default());
    }
}

/// Go: `tankPlayer`: ein Spieler, der die vier aktiven Tank-Skills gelernt hat (Slots: taunt, shieldBash, ironWall,
/// lastStand). Füller für das Gating sind Passive ohne Einfluss auf Verteidigung.
pub fn tank_player(w: &mut World) -> usize {
    let pi = add_player(w);
    add_pool_points(w, 7);
    must_learn(
        w,
        pi,
        &[
            "taunt",
            "shieldBash",
            "guardian",
            "thickSkin",
            "ironWall",
            "regeneration",
            "lastStand",
        ],
    );
    pi
}

/// Go: `mageSkills`: die sieben Skills für alle vier aktiven Zauber (Slots: fireball, iceWall, lightningStorm, meteor). Füller
/// fürs Gating: arcanePower, elementalMastery und frostArmor; spellEcho bleibt draußen, damit kein Zauber zufällig doppelt
/// wirkt.
pub const MAGE_SKILLS: [&str; 7] = [
    "fireball",
    "iceWall",
    "arcanePower",
    "elementalMastery",
    "lightningStorm",
    "frostArmor",
    "meteor",
];

/// Go: `casterPlayer`: ein Zauberer abseits der Burg, der alle vier aktiven Zauber gelernt hat.
pub fn caster_player(w: &mut World) -> usize {
    let pi = add_player(w);
    add_pool_points(w, 7);
    must_learn(w, pi, &MAGE_SKILLS);
    w.players[pi].x = w.hub_x + 40.0;
    pi
}

/// Go: `toughEnemy` überlebt auch Meteor (700 HP).
pub fn tough_enemy(w: &mut World, x: f64) -> usize {
    crate::enemies::spawn_scaled(w, "skeleton", x, 10.0, 1.0, None)
}

/// Go: `troopAt`: eine Truppe (Bogenschütze, 10 Max-HP) mit `hp` an `x`.
pub fn troop_at(w: &mut World, x: f64, hp: f64) -> usize {
    let id = w.new_id();
    w.troops.push(crate::state::Troop {
        id,
        kind: "archer".into(),
        x,
        anchor_x: x,
        hp,
        max_hp: 10.0,
        ..crate::state::Troop::default()
    });
    w.troops.len() - 1
}

/// Go: `lost`: verlorene HP des Gegners `ei`.
pub fn lost(w: &World, ei: usize) -> f64 {
    w.enemies[ei].max_hp - w.enemies[ei].hp
}
