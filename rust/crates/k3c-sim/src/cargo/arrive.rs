//! Abfahrt und Ankunft (RU62.1): Mit `switch_ready` gehen offene Träger-Aufträge zurück in den Vorrat der alten Insel
//! (`release_for_departure`); `next_island` setzt die Fracht auf die neue Insel (`board_next`): Material kommt zusätzlich
//! zum Startvorrat in den Vorrat, Truppen stehen am Hafen, Beutel liegen als Münzen am Steg.

use serde_json::json;

use super::orders::cancel_order;
use super::step::refund_pay;
use super::{CargoItem, KIND_COINS, KIND_CRATE, KIND_TROOP};
use crate::economy::sites::scatter_coins;
use crate::island::Island;
use crate::island::storage::stock_field;
use crate::state::common::event;
use crate::state::links::emit;
use crate::state::{Troop, World};
use crate::world::ship::harbor;
use crate::world::vals;

/// Der Wechsel ist bereit: laufende Einwürfe werden erstattet, offene Aufträge geben ihr Material in den Vorrat der Insel
/// zurück (der Bauer trägt die Kiste nicht mit, sie bleibt auf der alten Insel).
pub fn release_for_departure(isl: &mut Island) {
    if isl.cargo.cfg.is_none() {
        return;
    }
    let indexes: Vec<i64> = isl.cargo.pay.keys().copied().collect();
    for index in indexes {
        refund_pay(isl, index);
    }
    while !isl.cargo.orders.is_empty() {
        cancel_order(isl, isl.cargo.orders.len() - 1);
    }
    isl.sync_stock();
}

/// Stellt eine Truppe der Art `kind` mit `hp` bei `x` in die Stufe (am Hafen, ohne Auftrag).
pub fn spawn_troop(w: &mut World, kind: &str, hp: f64, x: f64) {
    let max_hp = vals(w).troops.get(kind).map_or(hp, |t| t.hp.max(hp));
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: kind.to_string(),
        x,
        hp,
        max_hp,
        anchor_x: x,
        target_x: x,
        ..Troop::default()
    });
}

/// Gibt eine Ladung an Land: Material in den Vorrat, Truppe bei `x`, Münzen bei `x` auf den Boden. Der Vorrat der Insel
/// wird nicht gedeckelt (die Fracht kommt zusätzlich zum Startvorrat).
pub fn unload(isl: &mut Island, wi: usize, x: f64, item: &CargoItem) {
    match item.kind.as_str() {
        KIND_CRATE => {
            if let Some(f) = stock_field(&mut isl.stock, &item.id) {
                *f += item.amount;
            }
        }
        KIND_TROOP => spawn_troop(&mut isl.stages[wi], &item.id, item.hp, x),
        KIND_COINS => scatter_coins(&mut isl.stages[wi], x, item.amount),
        _ => {}
    }
}

/// Der Hafen der Insel: Stufe und Steg-X des Schiffs; ohne Hafen die Burg der ersten Stufe.
pub fn landing(isl: &Island) -> (usize, f64) {
    match harbor(isl).and_then(|(wi, si)| isl.stages[wi].sites.get(si).map(|s| (wi, s.x))) {
        Some(p) => p,
        None => (0, isl.stages.first().map_or(0.0, |w| w.hub_x)),
    }
}

/// `next_island`: setzt die Fracht der alten Insel `cur` auf die neue Insel `next`.
pub fn board_next(next: &mut Island, cur: &Island) {
    if next.stages.is_empty() || cur.cargo.load.is_empty() {
        return;
    }
    let (wi, x) = landing(next);
    for item in &cur.cargo.load {
        unload(next, wi, x, item);
    }
    next.cargo.arrived.clone_from(&cur.cargo.load);
    next.sync_stock();
}

/// Meldet die angekommene Fracht einmal (Ereignis `cargoArrived`, erster Takt der neuen Insel).
pub fn emit_arrival(isl: &mut Island) {
    if isl.cargo.arrived.is_empty() {
        return;
    }
    let wi = landing(isl).0;
    let items: Vec<serde_json::Value> = isl
        .cargo
        .arrived
        .iter()
        .map(|i| json!({ "kind": i.kind, "id": i.id, "amount": i.amount }))
        .collect();
    isl.cargo.arrived.clear();
    emit(&mut isl.stages[wi], "cargoArrived", event(json!({ "items": items })));
}
