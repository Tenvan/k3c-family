//! Träger-Aufträge (RU62.1): Ein freier Bauer trägt die Kiste zum Steg. Der Weg wird gerechnet, nicht gelaufen: Der Auftrag
//! zählt `eta` herunter (Weg des Bauern zum Steg durch `carrierSpeed`), die Anzeige kann den Träger daran entlang führen.
//! Fällt der Träger aus (tot), sucht der Auftrag einen neuen; gibt es keinen freien Bauern, wartet er mit dem Platz.

use serde_json::json;

use super::{CargoItem, KIND_CRATE};
use crate::island::Island;
use crate::island::storage::stock_field;
use crate::state::common::event;
use crate::state::links::emit;

/// Rechnet die Aufträge einen Takt: Träger suchen, Weg zählen, Ankunft am Steg belegt den Platz. `x` ist der Ladeplatz für
/// Material.
pub fn step_orders(isl: &mut Island, wi: usize, x: f64, speed: f64, dt: f64) {
    let mut i = 0;
    while i < isl.cargo.orders.len() {
        if isl.cargo.orders[i].carrier != 0 && isl.stages[wi].troop(isl.cargo.orders[i].carrier).is_none() {
            isl.cargo.orders[i].carrier = 0; // Träger gefallen: ein neuer wird gesucht
        }
        if isl.cargo.orders[i].carrier == 0 {
            assign(isl, wi, i, x, speed);
        }
        if isl.cargo.orders[i].carrier == 0 {
            i += 1;
            continue;
        }
        isl.cargo.orders[i].eta -= dt;
        if isl.cargo.orders[i].eta > 0.0 {
            i += 1;
            continue;
        }
        let o = isl.cargo.orders.remove(i);
        let item = CargoItem {
            kind: KIND_CRATE.to_string(),
            id: o.resource.clone(),
            amount: o.units,
            hp: 0.0,
        };
        let index = isl.cargo.load.len() as i64;
        isl.cargo.load.push(item);
        let ev = event(json!({
            "kind": KIND_CRATE, "id": o.resource, "amount": o.units, "index": index, "carrier": o.carrier,
        }));
        emit(&mut isl.stages[wi], "cargoLoaded", ev);
    }
}

/// Der nächste freie Bauer (Bürger ohne Hammer, ohne Arbeit, ohne Last, nicht schon Träger) übernimmt den Auftrag `i`.
fn assign(isl: &mut Island, wi: usize, i: usize, x: f64, speed: f64) {
    let w = &isl.stages[wi];
    let mut best: Option<(i64, f64)> = None;
    for t in &w.troops {
        let taken = isl.cargo.orders.iter().any(|o| o.carrier == t.id);
        if t.kind != "peasant" || t.hammer || t.job.is_some() || t.carried.is_some() || taken {
            continue;
        }
        let d = (t.x - x).abs();
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some((t.id, d));
        }
    }
    if let Some((id, d)) = best {
        isl.cargo.orders[i].carrier = id;
        isl.cargo.orders[i].eta = d / speed.max(0.1);
    }
}

/// Der Auftrag `i` fällt weg: das reservierte Material geht in den Vorrat der Insel zurück.
pub fn cancel_order(isl: &mut Island, i: usize) {
    let o = isl.cargo.orders.remove(i);
    if let Some(f) = stock_field(&mut isl.stock, &o.resource) {
        *f += o.units;
    }
}
