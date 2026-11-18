//! Lager-Maximum der Insel (Port von `engine/sim/island_storage.go`, B-113, materialien-gebaeude.md § 1): je Rohstoff 300 je
//! Hub (Stufe) plus 300 je gebautem Lager. Gilt nur, wenn die Welt zu einer Insel gehört; Campaign und einzelne Welten sind
//! unbegrenzt.
//!
//! Zustandsmodell (Vorentscheidung 4 und 5): Gos `w.island` ist `Option<&IslandRef>` (`None` = `w.island == nil`), der Vorrat
//! einer Insel liegt während des Schritts in `w.stock` (die Insel tauscht ihn per `swap`); alle Funktionen lesen nur `w.stock`.
//! Das Maximum braucht Stufenzahl, Dev-Bonus und die Lager der anderen Stufen: Sie stehen im `IslandRef` (`stages`,
//! `stock_bonus`, `storages_elsewhere`), die Lager der eigenen Stufe liest `capacity` live aus `w.sites`. Der Wertesatz `v`
//! kommt vom Aufrufer (Go: `w.values()`). Keine Würfel-Aufrufe.

use k3c_data::hub::Stock;
use serde_json::json;

use crate::state::common::{event, is_dangerous};
use crate::state::island_ref::IslandRef;
use crate::state::{World, WorldValues};
use crate::unit::jobs::build_job;
use crate::unit::{gather_pressure, walk_to};

/// Go: `stockField` ist der Zähler im Vorrat zu einem Rohstoff, `None` bei unbekanntem Namen.
pub fn stock_field<'a>(s: &'a mut Stock, resource: &str) -> Option<&'a mut i64> {
    match resource {
        "wood" => Some(&mut s.wood),
        "stone" => Some(&mut s.stone),
        "copper" => Some(&mut s.copper),
        "iron" => Some(&mut s.iron),
        "crystal" => Some(&mut s.crystal),
        _ => None,
    }
}

/// Lesende Form von `stock_field` (Go gibt den Zeiger, auch zum Lesen): der Wert, `None` bei unbekanntem Namen.
pub fn stock_get(s: &Stock, resource: &str) -> Option<i64> {
    match resource {
        "wood" => Some(s.wood),
        "stone" => Some(s.stone),
        "copper" => Some(s.copper),
        "iron" => Some(s.iron),
        "crystal" => Some(s.crystal),
        _ => None,
    }
}

/// Go: `capacity` ist das Maximum je Rohstoff; `None` = unbegrenzt (keine Insel).
pub fn capacity(w: &World, v: &WorldValues, isl: Option<&IslandRef>) -> Option<i64> {
    let isl = isl?;
    let st = &v.economy.storage;
    let own = w
        .sites
        .iter()
        .filter(|s| s.kind == "storage" && s.state == "built")
        .count() as i64;
    Some(st.base_per_hub * isl.stages + isl.stock_bonus + st.per_storage * (isl.storages_elsewhere + own))
}

/// Go: `purseMax` ist das Gold-Maximum eines Spielers (Börse): der Grundwert plus der Dev-Bonus der Insel (`devMaxStock`).
pub fn purse_max(v: &WorldValues, isl: Option<&IslandRef>) -> i64 {
    match isl {
        None => v.economy.purse.max_gold,
        Some(i) => v.economy.purse.max_gold + i.stock_bonus,
    }
}

/// Go: `depositPoint` ist die Abgabestelle für einen Träger bei `x`: die Burg oder das nächste gebaute Lager. Bei Gleichstand
/// gewinnt die Burg, dann das frühere Lager in `w.sites`.
pub fn deposit_point(w: &World, x: f64) -> f64 {
    let mut best = w.hub_x;
    for s in &w.sites {
        if s.kind == "storage" && s.state == "built" && (s.x - x).abs() < (best - x).abs() {
            best = s.x;
        }
    }
    best
}

/// Go: `carryToStock` bringt das Material zur Abgabestelle und legt höchstens bis zum Maximum ab. Der Rest bleibt im Job, der
/// Bauer wartet dort und versucht es jeden Tick erneut. Bei Gefahr geht er zur Burg (wie ohne Insel).
pub fn carry_to_stock(w: &mut World, v: &WorldValues, ti: usize, isl: Option<&IslandRef>, dt: f64) {
    let x = if is_dangerous(w) {
        w.hub_x
    } else {
        deposit_point(w, w.troops[ti].x)
    };
    if !walk_to(w, v, ti, x, dt) {
        return;
    }
    let Some((resource, amount)) = w.troops[ti].job.as_ref().map(|j| (j.resource.clone(), j.amount)) else {
        return;
    };
    let (taken, ok) = add_stock_capped(w, v, isl, &resource, amount);
    if ok && taken > 0 {
        w.events.push(event(
            json!({"type": "gathered", "resource": resource, "amount": taken}),
        ));
        if let Some(j) = w.troops[ti].job.as_mut() {
            j.amount -= taken;
        }
    }
    if !ok || w.troops[ti].job.as_ref().is_none_or(|j| j.amount <= 0) {
        gather_pressure(w);
        w.troops[ti].job = None;
        return;
    }
    // Maximum voll: Das Material bleibt beim Träger, aber ein wartender Bauplatz geht vor, sonst blockiert ein wartender
    // Träger den Bau, der Platz schaffen würde. Nur Bauaufträge: Wer zum Bogenschützen wird, verlöre das Material.
    if let Some(j) = build_job(w, ti) {
        let t = &mut w.troops[ti];
        t.carried = t.job.take();
        t.job = Some(j);
    }
}

/// Go: `resourceFull` sagt, ob der Rohstoff in einer Insel am Maximum steht (ohne Insel nie).
pub fn resource_full(w: &World, v: &WorldValues, isl: Option<&IslandRef>, resource: &str) -> bool {
    let Some(field) = stock_get(&w.stock, resource) else {
        return false;
    };
    capacity(w, v, isl).is_some_and(|limit| field >= limit)
}

/// Go: `addStockCapped` legt höchstens bis zur Kapazität ab und liefert die aufgenommene Menge. Liegt der Vorrat schon
/// darüber (Lager zerstört), wird nichts aufgenommen, der Überschuss bleibt. `false`: kein Baumaterial.
pub fn add_stock_capped(
    w: &mut World,
    v: &WorldValues,
    isl: Option<&IslandRef>,
    resource: &str,
    amount: i64,
) -> (i64, bool) {
    let limit = capacity(w, v, isl);
    let Some(field) = stock_field(&mut w.stock, resource) else {
        return (0, false);
    };
    let taken = match limit {
        Some(l) => amount.min(l - *field).max(0),
        None => amount,
    };
    *field += taken;
    (taken, true)
}
