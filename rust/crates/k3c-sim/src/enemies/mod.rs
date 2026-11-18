//! Gegner (Port von `engine/sim/enemies.go`). Sie laufen vom Portal geradeaus auf die Burg zu. Eine intakte Mauer hält sie
//! auf (außer `ignoresWalls`), dann greifen sie an, was in Reichweite ist. `stealsGold`: klaut Gold vom Monarchen und
//! flieht damit zum Portal. `fleesAtHalfHp`: flieht bei halber HP.
//!
//! Zeiger auf Gegner sind Indizes (`ei`) in `World::enemies`; der Bezug zur Insel (`w.island`) kommt als
//! `isl: Option<&IslandRef>` (Vorentscheidung 1 von RU11.1).

pub mod equipment_taken;
pub mod guard;
pub mod target;

#[cfg(test)]
mod behaviour_tests;
#[cfg(test)]
mod equipment_taken_tests;
#[cfg(test)]
mod guard_tests;
#[cfg(test)]
mod protect_tests;
#[cfg(test)]
mod targets_tests;
#[cfg(test)]
mod testutil;

use crate::state::common::sign;
use crate::state::island_ref::IslandRef;
use crate::state::pending::{boss_ability, crown_stolen, kite, strike, take_crown_or_loot, values};
use crate::state::{Enemy, QueuedSpawn, Site, World, WorldValues};

pub use guard::is_wave_enemy;
pub use target::{Target, TargetKind, Wall};

use equipment_taken::take_equipment;
use guard::step_guard;
use target::{blocking_wall, choose_target};

/// Go: `body`, halbe Körperbreite der Gegner in Units.
pub const BODY: f64 = 0.6;

impl Enemy {
    /// Go: `Enemy.has`.
    pub fn has(&self, trait_name: &str) -> bool {
        self.traits.iter().any(|t| t == trait_name)
    }

    /// Go: `Enemy.speed`: Laufgeschwindigkeit, solange Ice Wall wirkt mit Slow verlangsamt (skills_caster.go).
    pub fn speed(&self) -> f64 {
        if self.slow_for > 0.0 {
            self.speed * self.slow
        } else {
            self.speed
        }
    }
}

/// Go: `spawnEnemy`; gibt den Index des neuen Gegners zurück.
pub fn spawn_enemy(w: &mut World, kind: &str, x: f64, isl: Option<&IslandRef>) -> usize {
    spawn_scaled(w, kind, x, 1.0, 1.0, isl)
}

/// Go: `spawnScaled`: wie `spawn_enemy`, mit den Grad-Faktoren der Insel für HP und Schaden.
pub fn spawn_scaled(
    w: &mut World,
    kind: &str,
    x: f64,
    hp_factor: f64,
    damage_factor: f64,
    isl: Option<&IslandRef>,
) -> usize {
    let v = values(w);
    spawn_with(w, &v, kind, x, hp_factor, damage_factor, isl)
}

/// `spawn_scaled` mit bereits geholtem Wertesatz.
pub fn spawn_with(
    w: &mut World,
    v: &WorldValues,
    kind: &str,
    x: f64,
    hp_factor: f64,
    damage_factor: f64,
    isl: Option<&IslandRef>,
) -> usize {
    let fallback = Default::default(); // unbekannte Art: Nullwerte wie Gos Map-Zugriff
    let d = v.enemies.get(kind).unwrap_or(&fallback);
    let depth = w.biome.depth as f64;
    let s = isl.map_or(&v.waves.depth_scaling, |i| &i.scaling); // Tabelle der Insel (island_data.go)
    let hp = (d.hp * s.hp.powf(depth) * hp_factor).round();
    let e = Enemy {
        id: w.new_id(),
        kind: kind.to_string(),
        x,
        hp,
        max_hp: hp,
        damage: (d.damage * s.damage.powf(depth) * damage_factor).round(),
        speed: d.speed * s.speed.powf(depth),
        range: d.range,
        traits: d.traits.clone(),
        home_x: x,
        spawned: if d.traits.iter().any(|t| t == "phases") {
            w.time
        } else {
            0.0
        },
        ..Default::default()
    };
    w.enemies.push(e);
    w.enemies.len() - 1
}

/// Go: `stepSpawns`: Gegner aus der Warteschlange, deren Zeit gekommen ist.
pub fn step_spawns(w: &mut World, isl: Option<&IslandRef>) {
    if !w.spawn_queue.iter().any(|s| s.at <= w.time) {
        return; // meistens nichts fällig: keine neuen Listen je Tick
    }
    let time = w.time;
    let (due, rest): (Vec<QueuedSpawn>, Vec<QueuedSpawn>) = std::mem::take(&mut w.spawn_queue)
        .into_iter()
        .partition(|s| s.at <= time);
    w.spawn_queue = rest;
    for s in due {
        spawn_scaled(w, &s.kind, s.x, s.hp_factor, s.damage_factor, isl);
    }
}

/// Go: `sendEnemiesHome`: Bei Tagesanbruch fliehen alle Gegner zurück zu ihren Portalen (K2C), Bosse bleiben.
pub fn send_enemies_home(w: &mut World) {
    let has_portal = !w.portals.is_empty(); // ohne Portal gibt es kein Ziel der Flucht
    for e in &mut w.enemies {
        e.fleeing = !e.boss && is_wave_enemy(e) && has_portal;
    }
    w.spawn_queue.clear();
}

/// Go: `stepEnemies`.
pub fn step_enemies(w: &mut World, mut isl: Option<&mut IslandRef>, dt: f64) {
    let v = values(w);
    for ei in 0..w.enemies.len() {
        let e = &mut w.enemies[ei];
        if e.hp <= 0.0 {
            continue; // besiegt: flieht nicht, bewegt sich nicht, removeDeadEnemies rechnet ab (B-189)
        }
        e.cooldown = (e.cooldown - dt).max(0.0);
        e.aoe_in = (e.aoe_in - dt).max(0.0);
        if stunned(e, dt) {
            continue;
        }
        if e.guard {
            step_guard(w, &v, ei, isl.as_deref(), dt);
            continue;
        }
        boss_ability(w, ei, isl.as_deref_mut());
        take_equipment(w, &v, ei);
        take_crown_or_loot(w, ei);
        step_one(w, &v, ei, isl.as_deref(), dt);
    }
    leave_through_portal(w);
}

/// Der Rest von `stepEnemies` für einen Gegner: Flucht, Ziel wählen, zuschlagen, vorrücken.
fn step_one(w: &mut World, v: &WorldValues, ei: usize, isl: Option<&IslandRef>, dt: f64) {
    let e = &mut w.enemies[ei];
    if e.has("fleesAtHalfHp") && e.hp < e.max_hp / 2.0 && !w.portals.is_empty() {
        e.fleeing = true;
    }
    let speed = e.speed();
    if e.fleeing {
        let d = e.home_x - e.x;
        e.x += sign(d) * d.abs().min(speed * 1.2 * dt);
        return;
    }
    let mut dir = sign(w.hub_x - e.x);
    if dir == 0.0 {
        dir = 1.0;
    }
    let wall = if e.has("ignoresWalls") {
        None
    } else {
        blocking_wall(w, &w.enemies[ei], dir)
    };
    let t = choose_target(w, v, ei, dir, wall);
    if let Some(t) = &t
        && w.enemies[ei].cooldown <= 0.0
    {
        strike(w, v, ei, t, isl);
    }
    if !kite(w, v, ei, dt) && t.is_none() {
        advance(w, v, ei, dir, wall, speed * dt);
    }
}

/// Go: `leaveThroughPortal`: Geflohene lebende Gegner verschwinden im Portal (mitsamt geklautem Gold); besiegte nie
/// (B-189). Trägt einer eine Krone, ist das Kronenraub und das Spiel verloren (crown.go).
pub fn leave_through_portal(w: &mut World) {
    let mut thief: Option<Enemy> = None;
    w.enemies.retain(|e| {
        let stays = e.hp <= 0.0 || e.guard || !(e.fleeing && (e.x - e.home_x).abs() < 0.3);
        if !stays && e.crown.is_some() && thief.is_none() {
            thief = Some(e.clone()); // seltener Fall: nur bei Kronenraub
        }
        stays
    });
    if let Some(t) = thief {
        crown_stolen(w, &t);
    }
}

/// Go: `advance`: geradeaus Richtung Burg, höchstens bis zur Mauer oder zum Burgrand.
fn advance(w: &mut World, v: &WorldValues, ei: usize, dir: f64, wall: Option<Wall>, step: f64) {
    let e = &mut w.enemies[ei];
    let x = e.x + dir * step;
    let stop = match wall {
        Some(wl) => wl.x - dir * BODY,
        None => w.hub_x - dir * v.hub.castle_radius_units,
    };
    e.x = if dir > 0.0 { x.min(stop) } else { x.max(stop) };
}

/// Go: `stunned` senkt Betäubung, Verspottung und Verlangsamung (Skills, skills_tank.go, skills_caster.go); `true`: der
/// Gegner ist betäubt und tut nichts.
pub fn stunned(e: &mut Enemy, dt: f64) -> bool {
    e.taunt_for = (e.taunt_for - dt).max(0.0);
    if e.taunt_for == 0.0 {
        e.taunt_id = 0;
    }
    e.slow_for = (e.slow_for - dt).max(0.0);
    if e.slow_for == 0.0 {
        e.slow = 0.0;
    }
    if e.stun <= 0.0 {
        return false;
    }
    e.stun = (e.stun - dt).max(0.0);
    true
}

/// Go: `isBarrier`: Eine gebaute Mauer oder ein gebautes Tor hält Gegner auf und ist ihr Angriffsziel (Q27). Spieler und
/// Truppen kennen keine Hindernisse, sie passieren beides; `outer_wall` (Posten der Schützen) bleibt bei der Mauer.
pub fn is_barrier(s: &Site) -> bool {
    s.kind == "wall" && s.state == "built"
}

/// Go: `shipKind` (ship.go).
const SHIP_KIND: &str = "ship";

/// Go: `isBuilding`: übrige gebaute Gebäude (Heilplatz, Taverne, Lager, Schmiede, Rüstkammer, Werkstatt, Baumschule,
/// Kaserne) sind Ziele der Gegner (gegner.md § 4). Mauer und Tor halten auf, Türme sind Ziele der Fernkämpfer, Treppen
/// Reisepunkte; das Schiff ist kein Ziel (B-637).
pub fn is_building(s: &Site) -> bool {
    s.state == "built" && !is_barrier(s) && !matches!(s.kind.as_str(), "tower" | "stairsUp" | "stairsDown" | SHIP_KIND)
}

/// Ein Ereignis ohne Felder außer `type` (Hilfe der Tests).
#[cfg(test)]
pub(crate) fn events_of_type<'a>(w: &'a World, typ: &str) -> Vec<&'a crate::state::Event> {
    w.events
        .iter()
        .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some(typ))
        .collect()
}
