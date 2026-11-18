//! Portale (Port von `engine/sim/portal.go`, docs/rules/kampf.md § 2, B-927): Jedes Portal hat HP (data/biomes/*.json ›
//! portals.hp). Nur Truppen im Angriff (`Troop::attacking`, gesetzt vom Angriff der Kampfvariante, KS4) beschädigen es,
//! Gegner, Türme, Mauern und der Monarch nie. Solange Truppen ein Portal angreifen, spawnt es laufend Verteidiger
//! (portals.defense). Zerstörte Portale stehen in `World::portals_down` und im Spielstand (`HubSave::portals_down`); sind
//! alle weg, ist die Stufe dauerhaft sicher.

use serde_json::json;

use k3c_rng::pick;

use crate::boss::data::mini_boss;
use crate::enemies::BODY;
use crate::enemies::equipment_taken::drop_carried;
use crate::enemies::waves::{queue_wave, scaled, spawn_orders};
use crate::state::common::event;
use crate::state::island_ref::IslandRef;
use crate::state::links::{add_pool_points, emit, scatter_coins, unit_x};
use crate::state::save::HubSave;
use crate::state::{World, WorldValues};
use crate::unit::warrior::is_fighter;

/// Go: `World.Safe`: Alle Portale der Stufe sind zerstört (keine Wellen, kein Miniboss, kein Raub).
pub fn safe(w: &World) -> bool {
    w.portals.is_empty() && !w.portals_down.is_empty()
}

/// Go: `syncPortals`: hält HP und Verteidiger-Takt gleich lang wie `portals` (Portale kommen aus dem Level, Tests setzen
/// sie auch von Hand).
pub fn sync_portals(w: &mut World) {
    if w.portal_max_hp == 0.0 {
        w.portal_max_hp = w.biome.portals.hp;
    }
    let n = w.portals.len();
    let max = w.portal_max_hp;
    if w.portal_hp.len() < n {
        w.portal_hp.resize(n, max);
    }
    if w.portal_defense.len() < n {
        w.portal_defense.resize(n, 0.0);
    }
}

/// Go: `portalInReach`: Index des Portals in Schlagreichweite der Truppe `ti` (wie ein Krieger an einer Mauer).
pub fn portal_in_reach(w: &World, v: &WorldValues, ti: usize) -> Option<usize> {
    let t = &w.troops[ti];
    let reach = v.troops.get(t.kind.as_str()).map_or(0.0, |d| d.range) + BODY + 0.1;
    let mut best: Option<usize> = None;
    for (i, &x) in w.portals.iter().enumerate() {
        if (x - t.x).abs() <= reach && best.is_none_or(|b| (x - t.x).abs() < (w.portals[b] - t.x).abs()) {
            best = Some(i);
        }
    }
    best
}

/// Go: `stepPortals`: Truppen im Angriff schlagen das Portal in Reichweite, angegriffene Portale wehren sich in Wellen,
/// zerstörte verschwinden. `attacked` ist eine Bitmaske je Portal (ein Biom hat nur wenige Portale, höchstens 64).
pub fn step_portals(w: &mut World, v: &WorldValues, mut isl: Option<&mut IslandRef>, dt: f64) {
    if w.portals.is_empty() {
        return;
    }
    sync_portals(w);
    let mut attacked: u64 = 0;
    for ti in 0..w.troops.len() {
        let t = &mut w.troops[ti];
        t.siege_cd = (t.siege_cd - dt).max(0.0);
        if !t.attacking || !is_fighter(&v.troops, &t.kind) {
            continue;
        }
        let Some(i) = portal_in_reach(w, v, ti) else {
            continue;
        };
        attacked |= 1u64.checked_shl(i as u32).unwrap_or(0);
        if w.troops[ti].siege_cd <= 0.0 {
            let Some(d) = v.troops.get(w.troops[ti].kind.as_str()) else {
                continue;
            };
            w.portal_hp[i] -= d.damage;
            w.troops[ti].siege_cd = 1.0 / d.attacks_per_second;
        }
    }
    for i in 0..w.portals.len() {
        let hit = 1u64.checked_shl(i as u32).is_some_and(|m| attacked & m != 0);
        if hit && w.portal_hp[i] > 0.0 {
            w.portal_defense[i] -= dt;
            if w.portal_defense[i] <= 0.0 {
                let x = w.portals[i];
                defend_portal(w, v, isl.as_deref(), x);
                w.portal_defense[i] = w.biome.portals.defense.interval_seconds;
            }
        }
    }
    for i in (0..w.portals.len()).rev() {
        if w.portal_hp[i] <= 0.0 {
            destroy_portal(w, v, isl.as_deref_mut(), i);
        }
    }
}

/// Go: `defendPortal`: stellt eine Verteidiger-Welle am Portal `x` in die Warteschlange (Standard-Gegner des Portal-Pools,
/// Größe je Tiefe, bei Inseln nach Spielerzahl und Grad skaliert).
pub fn defend_portal(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, x: f64) {
    let kinds: Vec<String> = w
        .biome
        .enemies
        .portal
        .iter()
        .filter(|k| v.enemies.get(k.as_str()).is_some_and(|d| d.tier == "standard"))
        .cloned()
        .collect(); // eine Welle je Intervall
    if kinds.is_empty() {
        return;
    }
    let (size, hp, damage) = isl.map_or((1.0, 1.0, 1.0), |i| i.wave_factors());
    let mut picks: Vec<String> = Vec::new();
    for _ in 0..scaled(w.biome.portals.defense.size, size) {
        picks.push(pick(&mut w.rng, &kinds));
    }
    let plan = spawn_orders(v, &picks, &[x], &mut w.rng);
    queue_wave(w, &plan, hp, damage);
    emit(w, "portalDefense", event(json!({"x": unit_x(x), "count": picks.len()})));
}

/// Go: `destroyPortal`: entfernt das Portal `i` dauerhaft. Gegner mit diesem Portal als Ziel fliehen zum nächsten
/// stehenden; das letzte Portal macht die Stufe sicher.
pub fn destroy_portal(w: &mut World, v: &WorldValues, isl: Option<&mut IslandRef>, i: usize) {
    let x = w.portals[i];
    w.portals_down.push(x);
    w.portals.remove(i);
    w.portal_hp.remove(i);
    if i < w.portal_defense.len() {
        w.portal_defense.remove(i);
    }
    w.spawn_queue.retain(|s| s.x != x);
    let left = w.portals.len();
    emit(w, "portalDestroyed", event(json!({"x": unit_x(x), "left": left})));
    if left > 0 {
        let portals = &w.portals;
        for e in w.enemies.iter_mut().filter(|e| e.home_x == x) {
            e.home_x = nearest_of(portals, e.x);
        }
    }
    if left == 0 {
        secure_stage(w, v, isl);
    }
}

/// Go: `standingPortalNear`: das stehende Portal nächst an `x` (Aufrufer prüfen, dass eins steht).
pub fn standing_portal_near(w: &World, x: f64) -> f64 {
    nearest_of(&w.portals, x)
}

fn nearest_of(portals: &[f64], x: f64) -> f64 {
    let mut best = portals.first().copied().unwrap_or(0.0);
    for &p in portals.iter().skip(1) {
        if (p - x).abs() < (best - x).abs() {
            best = p;
        }
    }
    best
}

/// Go: `secureStage`: Das letzte Portal ist gefallen. Keine Wellen mehr, der Skill-Punkt des Miniboss geht in den
/// Fund-Pool, getragene Beute fällt zu Boden (ohne Portal gibt es keinen Raub).
pub fn secure_stage(w: &mut World, v: &WorldValues, mut isl: Option<&mut IslandRef>) {
    w.spawn_queue.clear();
    let points = v.monarch.skill_point_sources.miniboss;
    let boss = mini_boss(&v.bosses, w.biome.depth);
    let defeated = match (boss, isl.as_deref()) {
        (Some(b), Some(i)) => Some(i.defeated_bosses.contains(&b.id)),
        _ => None,
    };
    match defeated {
        None => add_pool_points(w, isl.as_deref_mut(), points),
        Some(false) => {
            // Der Miniboss entfällt: sein Punkt gilt als vergeben, ein späterer Tod gibt ihn nicht noch einmal (bossDefeated).
            add_pool_points(w, isl.as_deref_mut(), points);
            if let (Some(b), Some(i)) = (boss, isl.as_mut()) {
                i.defeated_bosses.push(b.id.clone());
            }
        }
        Some(true) => {}
    }
    for ei in 0..w.enemies.len() {
        drop_loot(w, ei);
    }
    let depth = w.biome.depth;
    emit(w, "stageSafe", event(json!({"depth": depth})));
}

/// Go: `dropLoot`: lässt den Gegner `ei` geraubtes Gold und getragene Ausrüstung fallen; er flieht nicht mehr (kein
/// Portal).
pub fn drop_loot(w: &mut World, ei: usize) {
    // `drop_carried` erwartet den Gegner außerhalb von `w.enemies` (Zustandsmodell, Vorentscheidung 7)
    let mut e = std::mem::take(&mut w.enemies[ei]);
    if e.carried_gold > 0 {
        scatter_coins(w, e.x, e.carried_gold);
        e.carried_gold = 0;
    }
    drop_carried(w, &mut e);
    e.fleeing = false;
    w.enemies[ei] = e;
}

/// Go: `applyPortals`: übernimmt die zerstörten Portale aus dem Spielstand (nach dem Anlegen aus dem Level).
pub fn apply_portals(w: &mut World, h: &HubSave) {
    w.portals_down = h.portals_down.clone();
    w.portals
        .retain(|p| !h.portals_down.iter().any(|d| (d - p).abs() < 0.01));
    w.portal_hp = Vec::new();
    w.portal_defense = Vec::new();
    sync_portals(w);
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod testutil;
