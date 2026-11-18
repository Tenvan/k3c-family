//! Wellen und Geschosse (Port von `engine/sim/enemies_waves.go`, ursprünglich `waves.ts` und der Rest von `enemies.ts`).
//!
//! Die Würfe von `plan_wave` und `spawn_orders` haben eine feste Reihenfolge (Schleife, Schwarm, Elite); sie ist Teil
//! des Ergebnisses (Entscheidung 012) und steht hier an den Stellen von Go.

use k3c_data::biome::Biome;
use k3c_rng::{Rng, pick};
use serde_json::json;

use super::equipment_taken::drop_carried;
use super::guard::mark_cleared;
use crate::economy::add_stock;
use crate::state::common::{apply_damage_by, event, sign};
use crate::state::island_ref::IslandRef;
use crate::state::pending::{
    MERCHANT_ID, add_pool, boss_defeated, emit, full_moon_extra, hit_splash, merchant_x, moon_drop, pure, scaled_gold,
    scatter_coins, trigger_boss, unit_x, values,
};
use crate::state::{Enemy, QueuedSpawn, World, WorldValues};

/// Go: `spawnOrder`: ein Platz einer Welle.
#[derive(Debug, Clone, PartialEq)]
pub struct SpawnOrder {
    pub kind: String,
    pub x: f64,
    /// Sekunden nach Wellenstart.
    pub delay: f64,
}

/// Go: `scaled` (island_options.go): skaliert eine Gegnerzahl, gerundet, mindestens 1 bei Zahl > 0. RU10 führt die
/// Funktion mit den Insel-Optionen; hier steht sie, weil `plan_wave` sie ruft.
pub fn scaled(n: i64, size: f64) -> i64 {
    if n <= 0 || size == 1.0 {
        return n;
    }
    ((n as f64 * size).round() as i64).max(1)
}

/// Go: `planWave`: stellt eine Welle zusammen: Anzahl laut Tabelle, Gegnertypen aus dem Biom (nachts mit Nacht-Gegnern),
/// gleichmäßig auf die Portale verteilt und über `spawnSpreadSeconds` gestaffelt.
pub fn plan_wave(
    v: &WorldValues,
    b: &Biome,
    wave: i64,
    r: &mut Rng,
    portals: &[f64],
    night: bool,
    size: f64,
) -> Vec<SpawnOrder> {
    let Some(first) = v.waves.table.first() else {
        return Vec::new();
    };
    if portals.is_empty() {
        return Vec::new();
    }
    let row = v.waves.table.iter().rfind(|t| wave >= t.from_wave).unwrap_or(first);
    let night_kinds: &[String] = if night { &b.enemies.night } else { &[] };
    let (mut standard, mut elite) = (Vec::new(), Vec::new());
    for k in b.enemies.portal.iter().chain(night_kinds) {
        match v.enemies.get(k.as_str()).map(|d| d.tier.as_str()) {
            Some("standard") => standard.push(k.clone()),
            Some("elite") => elite.push(k.clone()),
            _ => {}
        }
    }
    let mut picks: Vec<String> = Vec::new();
    if !standard.is_empty() {
        for _ in 0..scaled(r.int(row.standard[0], row.standard[1]), size) {
            picks.push(pick(r, &standard));
        }
    }
    if !elite.is_empty() {
        for _ in 0..scaled(r.int(row.elite[0], row.elite[1]), size) {
            picks.push(pick(r, &elite));
        }
    }
    spawn_orders(v, &picks, portals, r)
}

/// Go: `spawnOrders`: verteilt die Plätze reihum auf die Portale und staffelt sie über `spawnSpreadSeconds`.
pub fn spawn_orders(v: &WorldValues, picks: &[String], portals: &[f64], r: &mut Rng) -> Vec<SpawnOrder> {
    let mut out = Vec::with_capacity(picks.len());
    for (i, kind) in picks.iter().enumerate() {
        let delay = i as f64 / (picks.len() as f64).max(1.0) * v.waves.spawn_spread_seconds + r.next();
        let x = portals[i % portals.len()];
        out.push(SpawnOrder {
            kind: kind.clone(),
            x,
            delay,
        });
        // Schwarm-Platz (`swarm`): swarmSize Gegner am selben Portal, die übrigen je eine Zufalls-Sekunde später.
        if let Some(d) = v
            .enemies
            .get(kind.as_str())
            .filter(|d| d.traits.iter().any(|t| t == "swarm"))
        {
            for _ in 1..d.swarm_size {
                out.push(SpawnOrder {
                    kind: kind.clone(),
                    x,
                    delay: delay + r.next(),
                });
            }
        }
    }
    out
}

/// Go: `startWave`.
pub fn start_wave(w: &mut World, isl: Option<&mut IslandRef>) {
    if pure(w) {
        return; // reiner Stressmodus (B-921): keine Welle außer den gesetzten Gegnern
    }
    w.wave += 1;
    w.last_wave_at = w.time;
    // nur Inseln skalieren nach Spieleranzahl und Grad
    let (size, hp, damage) = isl.as_deref().map_or((1.0, 1.0, 1.0), IslandRef::wave_factors);
    let v = values(w);
    let night = w.cycle.phase == "night";
    let mut plan = plan_wave(&v, &w.biome, w.wave, &mut w.rng, &w.portals, night, size);
    let extra = full_moon_extra(w, &plan); // Vollmond (events_moon.go)
    plan.extend(extra);
    queue_wave(w, &plan, hp, damage);
    w.events
        .push(event(json!({"type": "wave", "wave": w.wave, "count": plan.len()})));
    trigger_boss(w, isl);
}

/// Go: `queueWave`: stellt die Plätze einer Welle mit den Faktoren der Insel in die Warteschlange (`start_wave`,
/// Händler-Überfall).
pub fn queue_wave(w: &mut World, plan: &[SpawnOrder], hp: f64, damage: f64) {
    for s in plan {
        w.spawn_queue.push(QueuedSpawn {
            kind: s.kind.clone(),
            x: s.x,
            at: w.time + s.delay,
            hp_factor: hp,
            damage_factor: damage,
        });
    }
}

/// Go: `targetX`: die Position des Ziels eines Geschosses (Gegner, Spieler, Truppen, Bauplätze, Burg).
pub fn target_x(w: &World, id: i64) -> Option<f64> {
    if let Some(e) = w.enemies.iter().find(|e| e.id == id) {
        return Some(e.x);
    }
    if let Some(p) = w.players.iter().find(|p| p.id == id) {
        return Some(p.x);
    }
    if let Some(t) = w.troop(id) {
        return Some(t.x);
    }
    if let Some(s) = w.site(id) {
        return Some(s.x);
    }
    if id == MERCHANT_ID && w.merchant.is_some() {
        return Some(merchant_x(w, "buy")); // Händler (merchant.go)
    }
    (w.castle.id == id).then_some(w.castle.x)
}

/// Go: `stepProjectiles`.
pub fn step_projectiles(w: &mut World, dt: f64) {
    let mut projectiles = std::mem::take(&mut w.projectiles);
    projectiles.retain_mut(|pr| {
        let Some(x) = target_x(w, pr.target_id) else {
            return false;
        };
        if (x - pr.x).abs() <= pr.speed * dt {
            if pr.splash > 0.0 {
                hit_splash(w, pr, x); // Splitter-Wurf (boss_abilities.go)
            } else {
                apply_damage_by(w, pr.target_id, pr.damage, &pr.cause);
            }
            return false;
        }
        pr.x += sign(x - pr.x) * pr.speed * dt;
        true
    });
    let added = std::mem::replace(&mut w.projectiles, projectiles);
    w.projectiles.extend(added); // was die Treffer an neuen Geschossen auslösten
}

/// Go: `removeDeadEnemies`: Besiegte Gegner droppen Gold (plus geklautes Gold), manchmal auch die Stufen-Ressource; ein
/// Boss gibt stattdessen seine Belohnung (`boss_defeated`, ohne Würfel für den Drop). Während der Abrechnung ist
/// `w.enemies` leer; die Überlebenden kommen danach zurück.
pub fn remove_dead_enemies(w: &mut World, mut isl: Option<&mut IslandRef>) {
    if w.enemies.iter().all(|e| e.hp > 0.0) {
        return; // meistens stirbt niemand: keine neuen Listen je Tick
    }
    let v = values(w);
    let all = std::mem::take(&mut w.enemies);
    let mut kept = Vec::with_capacity(all.len());
    for mut e in all {
        if e.hp > 0.0 {
            kept.push(e);
            continue;
        }
        if e.boss {
            boss_defeated(w, &e, isl.as_deref_mut());
        } else {
            enemy_drop(w, &v, isl.as_deref(), &mut e);
        }
        if e.guard {
            mark_cleared(w, &e); // keine Wirkung auf den Aggressionspool, kein Respawn
            continue;
        }
        let percent = w.biome.cycle.percent_per_kill;
        add_pool(w, percent);
    }
    w.enemies = kept;
}

/// Go: `enemyDrop`: Gold laut Daten plus geklautes Gold als Münzen, mit Chance die Stufen-Ressource in den Vorrat.
pub fn enemy_drop(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, e: &mut Enemy) {
    let gold = v.enemies.get(e.kind.as_str()).map_or([0, 0], |d| d.gold);
    let rolled = w.rng.int(gold[0], gold[1]);
    // Blutmond: Drop mal Faktor, geklautes Gold nicht
    let dropped = moon_drop(w, scaled_gold(w, rolled)) + e.carried_gold;
    scatter_coins(w, e.x, dropped);
    drop_carried(w, e);
    emit(
        w,
        "kill",
        event(json!({"kind": e.kind, "x": unit_x(e.x), "gold": dropped})),
    );
    let drop = &v.economy.enemy_resource_drop;
    if w.rng.next() < drop.chance {
        let resource = w.biome.primary_resource.clone(); // nur im seltenen Treffer
        if add_stock(w, v, isl, &resource, drop.amount) {
            w.events.push(event(
                json!({"type": "gathered", "resource": resource, "amount": drop.amount}),
            ));
        }
    }
}
