//! Takt der Fracht (RU62.1): nach den Stufen und vor dem Wechsel (`step_island`). Reihenfolge: Ankunft melden, Plätze
//! (Verlust des Laderaums gibt Fracht zurück), Träger-Aufträge, Einwürfe der Spieler (feste Reihenfolge nach Spieler-Index),
//! Zustand für die Anzeige. Ohne `cfg` (Insel ohne Fracht, auch `standard`) kehrt der Takt sofort zurück, ohne zu allozieren.

use k3c_data::cargo::CargoFile;
use serde_json::json;

use super::arrive::{emit_arrival, unload};
use super::load::{Refusal, check, perform};
use super::orders::{cancel_order, step_orders};
use super::view::publish;
use super::{Bay, PayState};
use crate::island::Island;
use crate::state::PlayerCommand;
use crate::state::common::event;
use crate::state::links::{crowned, emit};
use crate::world::ship::{harbor, ship_complete, ship_hold_built};
use crate::world::vals;

/// Die Lage des Schiffs für den Takt.
pub struct Ship {
    /// Stufe des Hafens.
    pub wi: usize,
    /// Steg-X des Schiffs.
    pub x: f64,
    /// Das Grundschiff (bis zum Segel) steht.
    pub complete: bool,
    /// Der Abschnitt Laderaum steht.
    pub hold: bool,
}

fn ship_of(isl: &Island) -> Option<Ship> {
    let (wi, si) = harbor(isl)?;
    let w = isl.stages.get(wi)?;
    let s = w.sites.get(si)?;
    Some(Ship {
        wi,
        x: s.x,
        complete: ship_complete(w, s),
        hold: ship_hold_built(w, s),
    })
}

/// Plätze gesamt: Grundschiff, mit Laderaum mehr.
pub fn slots_of(cfg: &CargoFile, hold: bool) -> i64 {
    cfg.base_slots + if hold { cfg.hold.extra_slots } else { 0 }
}

/// Go-frei, neu in Rust: der Takt der Fracht einer Insel.
pub fn step_cargo(isl: &mut Island, commands: &[PlayerCommand], dt: f64) {
    let Some(cfg) = isl.cargo.cfg.clone() else {
        return;
    };
    emit_arrival(isl);
    let Some(ship) = ship_of(isl) else {
        return;
    };
    let slots = slots_of(&cfg, ship.hold);
    let trimmed = trim_overflow(isl, &ship, slots);
    step_orders(isl, ship.wi, ship.x + Bay::Crates.offset(&cfg), cfg.carrier_speed, dt);
    let active = ship.complete && !isl.is_last();
    if active {
        step_pays(isl, commands, &ship, &cfg, slots, dt);
    } else {
        refund_all(isl);
    }
    if trimmed || !isl.cargo.orders.is_empty() || !isl.cargo.load.is_empty() || active {
        isl.sync_stock();
    }
    publish(isl, &ship, &cfg, slots, active);
}

/// Verlust des Laderaums: Aufträge (zuletzt erteilte zuerst), dann Fracht in umgekehrter Lade-Reihenfolge geben ihren Platz
/// zurück (Material in den Vorrat, Truppen an Land, Münzen am Steg). Liefert `true`, wenn etwas zurückging.
fn trim_overflow(isl: &mut Island, ship: &Ship, slots: i64) -> bool {
    let mut changed = false;
    while isl.cargo.taken() as i64 > slots && !isl.cargo.orders.is_empty() {
        cancel_order(isl, isl.cargo.orders.len() - 1);
        changed = true;
    }
    while isl.cargo.load.len() as i64 > slots {
        let Some(item) = isl.cargo.load.pop() else {
            break;
        };
        unload(isl, ship.wi, ship.x, &item);
        let ev = event(json!({ "kind": item.kind, "id": item.id, "amount": item.amount }));
        emit(&mut isl.stages[ship.wi], "cargoReturned", ev);
        changed = true;
    }
    changed
}

/// Erstattet jeden laufenden Einwurf (Ladeplätze nicht nutzbar).
fn refund_all(isl: &mut Island) {
    isl.cargo.rejected.clear();
    while let Some(index) = isl.cargo.pay.keys().next().copied() {
        refund_pay(isl, index);
    }
}

/// Der Spieler mit dem Index `index` (in irgendeiner Stufe): (Stufe, Platz).
fn find_player(isl: &Island, index: i64) -> Option<(usize, usize)> {
    isl.stages
        .iter()
        .enumerate()
        .find_map(|(wi, w)| w.players.iter().position(|p| p.index == index).map(|pi| (wi, pi)))
}

/// Gibt die bisher eingeworfenen Münzen des Spielers `index` zurück (in den Beutel, was nicht passt, fällt auf den Boden)
/// und beendet seinen Einwurf.
pub fn refund_pay(isl: &mut Island, index: i64) {
    let Some(st) = isl.cargo.pay.remove(&index) else {
        return;
    };
    if st.paid <= 0 {
        return;
    }
    if let Some((wi, pi)) = find_player(isl, index) {
        // zurück in den Beutel ohne Grenze: die Münzen kamen aus ihm (kein Verlust bei Lager-Bonus oder Respec)
        isl.stages[wi].players[pi].gold += st.paid;
    }
}

/// Der Ladeplatz in Reichweite von `px`, falls einer (feste Reihenfolge).
fn bay_at(cfg: &CargoFile, ship_x: f64, px: f64) -> Option<Bay> {
    Bay::ALL
        .into_iter()
        .find(|b| (px - (ship_x + b.offset(cfg))).abs() <= cfg.bay_range_units)
}

/// Der Platz in `stages[wi].players` mit dem kleinsten Index über `after` (feste Reihenfolge nach Spieler-Index).
fn next_player(isl: &Island, wi: usize, after: Option<i64>) -> Option<usize> {
    let mut best: Option<(usize, i64)> = None;
    for (pi, p) in isl.stages[wi].players.iter().enumerate() {
        if after.is_some_and(|a| p.index <= a) {
            continue;
        }
        if best.is_none_or(|(_, bi)| p.index < bi) {
            best = Some((pi, p.index));
        }
    }
    best.map(|(pi, _)| pi)
}

/// Einwürfe aller Spieler der Hafen-Stufe, nach Spieler-Index; Spieler anderer Stufen brechen ihren Einwurf ab.
fn step_pays(isl: &mut Island, commands: &[PlayerCommand], ship: &Ship, cfg: &CargoFile, slots: i64, dt: f64) {
    let others: Vec<i64> = isl
        .cargo
        .pay
        .keys()
        .copied()
        .filter(|i| find_player(isl, *i).is_none_or(|(wi, _)| wi != ship.wi))
        .collect();
    for index in others {
        refund_pay(isl, index);
    }
    let mut after = None;
    while let Some(pi) = next_player(isl, ship.wi, after) {
        let index = isl.stages[ship.wi].players[pi].index;
        after = Some(index);
        let pay = usize::try_from(index)
            .ok()
            .and_then(|i| commands.get(i))
            .is_some_and(|c| c.pay);
        step_player(isl, ship, cfg, slots, pi, pay, dt);
    }
}

/// Der Einwurf eines Spielers `pi` der Hafen-Stufe in diesem Takt.
fn step_player(isl: &mut Island, ship: &Ship, cfg: &CargoFile, slots: i64, pi: usize, pay: bool, dt: f64) {
    let (index, px, alive) = {
        let p = &isl.stages[ship.wi].players[pi];
        (p.index, p.x, crowned(p) && !p.free)
    };
    let bay = if pay && alive { bay_at(cfg, ship.x, px) } else { None };
    if isl.cargo.pay.get(&index).is_some_and(|s| Some(s.bay) != bay) {
        refund_pay(isl, index); // losgelassen, weggegangen oder anderer Ladeplatz: Münzen zurück
    }
    let Some(bay) = bay else {
        isl.cargo.rejected.remove(&index);
        return;
    };
    let bay_x = ship.x + bay.offset(cfg);
    if !isl.cargo.pay.contains_key(&index) {
        if let Err(r) = check(isl, ship.wi, slots, bay, bay_x) {
            reject(isl, ship.wi, index, bay, r);
            return;
        }
        isl.cargo.rejected.remove(&index);
        isl.cargo.pay.insert(index, PayState { bay, paid: 0, acc: 0.0 });
    }
    let interval = vals(&isl.stages[ship.wi]).economy.pay_interval_seconds.max(0.05);
    let price = bay.price(cfg);
    let Some(mut st) = isl.cargo.pay.remove(&index) else {
        return;
    };
    st.acc -= dt;
    while st.acc <= 0.0 && st.paid < price && isl.stages[ship.wi].players[pi].gold > 0 {
        isl.stages[ship.wi].players[pi].gold -= 1;
        st.paid += 1;
        st.acc += interval;
    }
    st.acc = st.acc.max(0.0);
    if st.paid < price {
        isl.cargo.pay.insert(index, st);
        return;
    }
    // voll bezahlt: noch einmal prüfen (ein anderer Spieler kann im selben Takt den letzten Platz genommen haben)
    match check(isl, ship.wi, slots, bay, bay_x) {
        Ok(()) => {
            if !finish(isl, ship, cfg, index, bay, bay_x) {
                isl.stages[ship.wi].players[pi].gold += st.paid;
            }
        }
        Err(r) => {
            isl.stages[ship.wi].players[pi].gold += st.paid;
            reject(isl, ship.wi, index, bay, r);
        }
    }
}

/// Lädt und meldet es (Ereignis `cargoLoaded` für Beutel und Truppe, `cargoOrdered` für die Kiste).
fn finish(isl: &mut Island, ship: &Ship, cfg: &CargoFile, index: i64, bay: Bay, bay_x: f64) -> bool {
    let Some(item) = perform(isl, ship.wi, cfg, bay, bay_x) else {
        return false;
    };
    let (name, slot) = if item.kind == super::KIND_CRATE {
        ("cargoOrdered", isl.cargo.orders.len() as i64 - 1)
    } else {
        ("cargoLoaded", isl.cargo.load.len() as i64 - 1)
    };
    let ev = event(json!({
        "player": index, "bay": bay.name(), "kind": item.kind, "id": item.id, "amount": item.amount, "index": slot,
    }));
    emit(&mut isl.stages[ship.wi], name, ev);
    true
}

/// Meldet die Ablehnung einmal je gehaltener Taste: `cargoFull` bei vollem Laderaum, sonst `cargoRejected`.
fn reject(isl: &mut Island, wi: usize, index: i64, bay: Bay, r: Refusal) {
    if !isl.cargo.rejected.insert(index) {
        return;
    }
    let ev = event(json!({ "player": index, "bay": bay.name(), "reason": r.name() }));
    let name = if r == Refusal::Full {
        "cargoFull"
    } else {
        "cargoRejected"
    };
    emit(&mut isl.stages[wi], name, ev);
}
