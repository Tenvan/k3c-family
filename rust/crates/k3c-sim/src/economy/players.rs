//! Spielerschritt der Wirtschaft (Port von `engine/sim/economy.go`: `stepPlayers`, `movePlayer`, `toHub`, `collectCoins`,
//! `collectPickups`): Bewegung, Zahlen, Münzen und Truhen einsammeln.

use serde_json::json;

use super::fountain::fountain_coins;
use super::pay::{pay_one_coin, refund_pending};
use super::pending::{chest_skill_point, merchant_at, mount_of, speed_of, step_attack, step_skills};
use super::{PayTarget, find_pay_target, key_of};
use crate::crown::crowned;
use crate::state::common::{event, sign};
use crate::state::island_ref::IslandRef;
use crate::state::pending::{add_pool_points, emit, unit_x};
use crate::state::purse::purse_max_of;
use crate::state::{PlayerCommand, World, WorldValues};
use crate::world::vals;

/// Go: `stepPlayers`: Bewegung, Zahlen, Einsammeln der Spieler (Reihenfolge: Spieler 0, 1, …).
pub fn step_players(w: &mut World, commands: &[PlayerCommand], mut isl: Option<&mut IslandRef>, dt: f64) {
    let v = vals(w);
    for pi in 0..w.players.len() {
        let index = usize::try_from(w.players[pi].index).ok();
        let cmd = index.and_then(|i| commands.get(i)).cloned().unwrap_or_default();
        w.players[pi].pay_cooldown = (w.players[pi].pay_cooldown - dt).max(0.0);
        if !crowned(&w.players[pi]) {
            // ohne Krone nur laufen (Aufheben: stepCrowns), kein Schlag, kein Skill, kein Zahlen (kampf.md § 5)
            w.players[pi].paying = false;
            w.players[pi].skill_held = 0; // mit der Krone zählt der erste Druck wieder als neu (B-524)
            refund_pending(w, &v, isl.as_deref(), pi);
            move_player(w, &v, pi, &cmd, dt);
            continue;
        }
        move_player(w, &v, pi, &cmd, dt);
        step_attack(w, &v, pi, &cmd, dt);
        step_skills(w, &v, pi, &cmd, dt);
        w.players[pi].paying = cmd.pay;
        step_pay(w, &v, isl.as_deref(), pi, &cmd);
    }
    collect_coins(w, &v, isl.as_deref_mut());
    collect_pickups(w, &v, isl);
}

/// Der Zahl-Teil von `step_players` für den lebenden Monarchen `pi`: eine Münze im Takt, Erstattung beim Loslassen.
fn step_pay(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, cmd: &PlayerCommand) {
    let p = &w.players[pi];
    if cmd.pay && p.pay_cooldown <= 0.0 && (p.gold > 0 || merchant_at(w, v, p.x) == Some("sell")) {
        pay_one_coin(w, v, isl, pi);
        w.players[pi].pay_cooldown = v.economy.pay_interval_seconds;
    }
    // Aufgehört zu halten (oder das Ziel gewechselt), bevor der Betrag voll war: Münzen kommen zurück.
    if let Some(key) = w.players[pi].pay_key.as_deref() {
        let same = cmd.pay && {
            let target: Option<PayTarget> = find_pay_target(w, v, isl, &w.players[pi]);
            key_of(w, target.as_ref()) == key
        };
        if !same {
            refund_pending(w, v, isl, pi);
        }
    }
}

/// Go: `movePlayer`: Beschleunigung zum Zieltempo, Sprint auf dem Reittier, Grenzen der Welt.
pub fn move_player(w: &mut World, v: &WorldValues, pi: usize, cmd: &PlayerCommand, dt: f64) {
    let mut mult = 1.0;
    if cmd.sprint {
        mult = v.monarch.sprint_multiplier * mount_of(v, pi).sprint_factor; // Sprint auf dem Reittier
    }
    let target = cmd.move_x * speed_of(w, v, pi) * mult;
    let width = w.width_units;
    let p = &mut w.players[pi];
    p.vx += (target - p.vx) * (1.0 - (-v.monarch.acceleration * dt).exp());
    p.x = (p.x + p.vx * dt).max(0.0).min(width);
    if p.vx.abs() > 0.05 {
        p.facing = sign(p.vx) as i64;
    }
}

/// Go: `toHub`: stellt den Monarchen `pi` nach einem Burgfall oder Stufenwechsel neben die Burg (er behält seine Krone
/// oder deren Verlust).
pub fn to_hub(w: &mut World, pi: usize) {
    let hub_x = w.hub_x;
    let p = &mut w.players[pi];
    p.vx = 0.0;
    p.x = hub_x + 3.0;
    if p.index % 2 == 0 {
        p.x = hub_x - 3.0;
    }
}

/// Go: `collectCoins`: Münzen am Boden wandern zum ersten Monarchen in Reichweite, der sie tragen kann; Münzen in der
/// Luft (Fontäne) und für den Werfer gesperrte bleiben liegen.
pub fn collect_coins(w: &mut World, v: &WorldValues, isl: Option<&mut IslandRef>) {
    let reach = v.economy.pickup_range_units; // je Aufruf einmal: Münzen × Spieler
    let ro: Option<&IslandRef> = isl.as_deref();
    let mut coins = std::mem::take(&mut w.coins);
    let mut collected = 0;
    coins.retain(|c| {
        if w.time < c.lands_at {
            return true; // noch in der Luft (Fontäne)
        }
        let taker = (0..w.players.len()).find(|&pi| {
            let p = &w.players[pi];
            if (p.x - c.x).abs() > reach {
                return false;
            }
            let blocked = c.blocked_player_id == Some(p.id) && w.time < c.blocked_until;
            crowned(p) && p.gold < purse_max_of(w, p, ro) && !blocked
        });
        let Some(pi) = taker else {
            return true;
        };
        w.players[pi].gold += 1;
        collected += 1; // Siegvariante gold (island/victory.rs)
        let (index, x) = (w.players[pi].index, c.x);
        emit(w, "coinPickup", event(json!({"player": index, "x": unit_x(x)})));
        false
    });
    w.coins = coins;
    if let Some(i) = isl {
        i.gold_collected += collected;
    }
}

/// Go: `collectPickups`: Truhen und Skill-Punkte am Boden gehen an den ersten Monarchen in 1,5-facher Reichweite.
pub fn collect_pickups(w: &mut World, v: &WorldValues, mut isl: Option<&mut IslandRef>) {
    let r = v.economy.pickup_range_units * 1.5;
    let mut pickups = std::mem::take(&mut w.pickups);
    pickups.retain(|pk| {
        let taker = (0..w.players.len()).find(|&i| crowned(&w.players[i]) && (w.players[i].x - pk.x).abs() <= r);
        let Some(pi) = taker else {
            return true;
        };
        let index = w.players[pi].index;
        if pk.kind == "chest" {
            let gold = w.rng.int(v.economy.chest_gold[0], v.economy.chest_gold[1]);
            fountain_coins(w, v, pk.x, gold);
            w.events
                .push(event(json!({"type": "chest", "player": index, "gold": gold})));
            chest_skill_point(w, v, isl.as_deref_mut(), pi);
        } else {
            add_pool_points(w, isl.as_deref_mut(), v.monarch.skill_point_sources.hidden);
            let total = w.skill_points;
            w.events
                .push(event(json!({"type": "skillPoint", "player": index, "total": total})));
        }
        false
    });
    w.pickups = pickups;
}
