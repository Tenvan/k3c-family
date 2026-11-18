//! Der Tick einer Stufe (Go: `Step`, world.go). Die Reihenfolge der Teilschritte ist verbindlich (AC-01): Zeile für Zeile
//! wie in Go. Teilschritte späterer Sprints stehen als Platzhalter in `pending.rs`; ein Teilschritt, der noch nicht
//! geliefert ist, meldet den liefernden Sprint.

use crate::boss::abilities::step_hazards;
use crate::boss::endboss::step_endboss;
use crate::combat::squad::guarded_leader;
use crate::combat::step_combat;
use crate::crown::step_crowns;
use crate::enemies::guard::spawn_guards;
use crate::enemies::waves::{remove_dead_enemies, step_projectiles};
use crate::enemies::{step_enemies, step_spawns};
use crate::portal::step_portals;
use crate::state::{IslandRef, PlayerCommand, World};
use crate::unit::warrior::lose_layer;
use crate::unit::{step_camps, step_troops};

use super::cycle::step_cycle;
use super::lava::step_lava;
use super::pending::{
    cap_events, step_healing, step_passives, step_plantations, step_players, step_sites, step_spell_towers, step_storms,
};
use super::travel::step_travel;
use super::{castle_fallen, vals};

/// Go: `Step`: rechnet einen Tick. `commands[i]` gehört zu `players[i]`; `isl` ist der Bezug zur Insel (`None` = Go
/// `w.island == nil`).
pub fn step(w: &mut World, commands: &[PlayerCommand], mut isl: Option<&mut IslandRef>, dt: f64) {
    w.events.clear();
    w.time += dt;
    step_cycle(w, isl.as_deref_mut(), dt);
    let v = vals(w);
    spawn_guards(w, &v, isl.as_deref());
    step_spawns(w, &v, isl.as_deref());
    step_players(w, commands, isl.as_deref_mut(), dt);
    step_crowns(w, &v, dt);
    step_passives(w, isl.as_deref(), dt);
    step_camps(w, &v, dt);
    step_sites(w, isl.as_deref_mut());
    step_plantations(w, dt);
    step_combat(w, &v);
    step_troops(w, &v, isl.as_deref(), dt);
    step_portals(w, &v, isl.as_deref_mut(), dt);
    step_healing(w, dt);
    step_spell_towers(w, dt);
    step_endboss(w, &v, isl.as_deref_mut(), dt);
    step_enemies(w, isl.as_deref_mut(), dt);
    step_lava(w, dt);
    step_hazards(w, &v, dt);
    step_storms(w, dt);
    step_projectiles(w, dt);
    remove_dead_enemies(w, isl.as_deref_mut());
    for ti in 0..w.troops.len() {
        // Bürger sterben nicht: Verlust-Kaskade (Q67, unit_warrior.go)
        if w.troops[ti].hp <= 0.0 && guarded_leader(w, ti) {
            w.troops[ti].hp = w.troops[ti].max_hp; // der Führer fällt nicht, solange Soldaten seiner Gruppe stehen
        } else if w.troops[ti].hp <= 0.0 {
            lose_layer(w, &v, ti);
        }
    }
    if w.castle.hp <= 0.0 {
        castle_fallen(w, isl);
    }
    step_travel(w, dt);
    cap_events(w);
}
