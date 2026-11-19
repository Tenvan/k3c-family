//! Inselwechsel (Port von `engine/sim/island_switch.go`, B-103, B-637, docs/rules/stufen.md § 1): Der Sieg über den Endboss
//! öffnet den Wechselpunkt am Steg des Hafens (Stufe mit dem fertigen Schiff, ship.go), wenn es eine nächste Insel gibt.
//! Stehen alle lebenden, gesteuerten Spieler der Insel dort `hub.travel.seconds` lang, ist der Wechsel bereit
//! (`switch_ready`); den Austausch der Insel macht der Raum (B-345) mit `next_island`.

use serde_json::Value;

use super::travel::insert_player;
use super::{CreateOptions, Island, IslandBuild, IslandDef, create_island_at};
use crate::state::links::{crowned, emit, to_hub, unit_x};
use crate::state::{Event, World};
use crate::world::ship::{harbor, ship_complete};
use k3c_data::Data;

impl Island {
    /// Go: `isLast`: Die Insel ist die letzte ihrer Welt: keine Ziele mehr (kein Wechselpunkt, Kampagnensieg möglich,
    /// B-961).
    pub fn is_last(&self) -> bool {
        self.target_indexes().is_empty()
    }

    /// Go: `NextDef` liefert die Daten des gewählten Ziels (Wahl, sonst erstes Ziel); `None` auf der letzten Insel.
    pub fn next_def(&self) -> Option<IslandDef> {
        let i = self.chosen_index()?;
        self.world.islands.get(i as usize).cloned()
    }
}

/// Ereignis mit den Feldern `fields`.
fn event(fields: &[(&str, Value)]) -> Event {
    let mut ev = Event::new();
    for (k, v) in fields {
        ev.insert((*k).to_string(), v.clone());
    }
    ev
}

/// Go: `stepIslandSwitch`: Wechselpunkt öffnen und den gemeinsamen Fortschritt rechnen (nach dem Tick aller Stufen).
pub fn step_island_switch(isl: &mut Island, dt: f64) {
    if !isl.endboss_defeated || isl.is_last() || isl.switch_ready {
        return;
    }
    isl.apply_choices();
    let ship = harbor(isl).and_then(|(wi, si)| {
        let w = &isl.stages[wi];
        let s = w.sites.get(si)?;
        ship_complete(w, s).then_some((wi, s.x))
    });
    let Some((wi, x)) = ship else {
        // Schiff verloren (Stufenverlust): der Wechselpunkt schließt
        isl.gate_open = false;
        isl.switch_progress = 0.0;
        return;
    };
    let island = Value::from(isl.number + 1);
    if !isl.gate_open {
        isl.gate_open = true;
        let ev = event(&[("island", island.clone()), ("x", Value::from(unit_x(x)))]);
        emit(&mut isl.stages[wi], "islandGateOpen", ev);
    }
    if !all_at_gate(isl, wi, x) {
        isl.switch_progress = 0.0;
        return;
    }
    isl.switch_progress += dt / isl.values().hub.travel.seconds;
    if isl.switch_progress >= 1.0 {
        isl.switch_ready = true;
        crate::cargo::arrive::release_for_departure(isl); // RU62.1: offene Träger lassen ihre Kiste zurück
        let ev = event(&[("island", island), ("target", Value::from(isl.chosen_target()))]);
        emit(&mut isl.stages[wi], "islandSwitch", ev);
    }
}

/// Go: `allAtGate`: Alle lebenden, gesteuerten Spieler der Insel stehen in der Stufe `wi` am Steg bei `x`; tote und
/// freie zählen nicht, einer in einer anderen Stufe lässt den Wechsel warten. Ohne solche Spieler kein Wechsel.
pub fn all_at_gate(isl: &Island, wi: usize, x: f64) -> bool {
    let range = isl.values().hub.travel.range_units;
    let mut n = 0;
    for (si, s) in isl.stages.iter().enumerate() {
        for p in &s.players {
            if !crowned(p) || p.free {
                continue;
            }
            if si != wi || (p.x - x).abs() > range {
                return false;
            }
            n += 1;
        }
    }
    n > 0
}

/// Go: `NextIsland` baut die Insel `def` (das gewählte Ziel, `next_def`) in der Welt der alten Insel: Seed aus der
/// Spiel-ID und dem Listenplatz des Ziels, Startvorrat, eigene Hubs, Optionen der alten Insel, Insel-ID aus `def`. Die
/// abgeschlossenen Inseln ziehen mit (die alte kommt dazu). Die Spieler ziehen mit (Index, Gold, Skills und Slots bleiben)
/// und stehen mit Krone an der Burg der ersten Stufe; ihre Skill-Basis ist die Zahl der mitgebrachten Skills.
pub fn next_island(data: &Data, cur: &Island, def: &IslandDef) -> Result<Island, String> {
    let number = match cur.chosen_index() {
        Some(n) if cur.world.islands.get(n as usize).is_some_and(|d| d.id == def.id) => n,
        _ => {
            return Err(format!(
                "insel: {:?} ist nicht das Ziel des Wechsels in Welt {:?}",
                def.id, cur.world.id
            ));
        }
    };
    let build = IslandBuild {
        data,
        world: &cur.world,
        first: def,
        hub: None,
    };
    let opts: CreateOptions = def.create_options(&data.biomes);
    let seed = format!("{}:{}", cur.id, number);
    let mut next = create_island_at(&build, &seed, &def.depths, cur.cycle_speed, 0.0, &opts)?;
    next.completed = cur.completed.clone();
    if let Some(d) = cur.world.islands.get(cur.number as usize)
        && cur.endboss_defeated
        && !next.completed.contains(&d.id)
    {
        next.completed.push(d.id.clone());
    }
    next.stock = next.values().hub.island_start_stock.clone();
    next.sync_stock();
    next.id = cur.id.clone();
    next.number = number;
    next.island_id = def.id.clone();
    next.scaling = def.depth_scaling.clone();
    next.options = cur.options.clone();
    next.next_player = cur.next_player;
    if let Some(first) = next.stages.first_mut() {
        bring_players(cur, first);
    }
    crate::cargo::arrive::board_next(&mut next, cur); // RU62.1: die Fracht kommt zusätzlich zum Startvorrat an
    Ok(next)
}

/// Die Spieler der alten Insel ziehen in die erste Stufe der neuen (Schleife von `NextIsland`).
fn bring_players(cur: &Island, first: &mut World) {
    for p in cur.players() {
        let mut q = p.clone();
        q.id = first.new_id();
        q.pay_cooldown = 0.5;
        q.paying = false;
        q.pay_key = None;
        q.pay_amount = 0;
        q.cooldowns = Vec::new();
        q.skill_base = q.skills.len() as i64;
        q.crown = None;
        let pos = insert_player(&mut first.players, q);
        to_hub(first, pos);
    }
}
