//! Händler (Port von `engine/sim/merchant.go`; docs/rules/buerger.md § 1, B-121, Q36, Q55): einer je Insel, nur im Hub der
//! Tiefe `hub.merchant.onlyDepth`. Er kommt bei dawn an jedem `everyDays`-ten Tag (mit gebauter Taverne irgendwo auf der
//! Insel `everyDaysWithTavern`), handelt ein Material, per `w.rng` gewählt aus den freigeschalteten (Hub-Stufe ≥ Stufe in
//! `economy.merchant.materials`), und reist `stayDays` später beim dawn ab. Zwei Zahlziele: Kaufen (Hub `+buyOffsetUnits`,
//! `gold` Münzen → `material` in den Vorrat) und Verkaufen (Hub `+sellOffsetUnits`, A halten: `material` aus dem Vorrat →
//! `gold` in die Börse).
//!
//! Zustandsmodell: Gos `w.island` ist `Option<&IslandRef>` (`None` = nil). `MerchantData` steht in `k3c_data::economy`,
//! `Merchant` in `state/shared.rs`, `MerchantSave` im Spielstand der Insel; hier nichts doppelt. `tavern_on_island` prüft die
//! eigene Stufe live in `w.sites` und die übrigen über `IslandRef::tavern_elsewhere`.

use serde_json::json;

use crate::economy::{add_stock, clear_pending, give_gold, refund_pending};
use crate::island::storage::{resource_full, stock_field};
use crate::links::{emit, merchant_raid_end, merchant_raid_start, pure, unit_x};
use crate::state::common::event;
use crate::state::island_ref::IslandRef;
use crate::state::links::hit_event;
use crate::state::{Coin, Merchant, World, WorldValues};
use crate::world::{hub_of, vals};

#[cfg(test)]
mod merchant_tests;
#[cfg(test)]
mod raid_target_tests;
#[cfg(test)]
mod testutil;

/// Go: `merchantID`, die ID des Händlers als Ziel. Feste ID statt `w.new_id()`, damit seine Ankunft die IDs der übrigen
/// Entitäten (und die Golden-Läufe) nicht verschiebt; es gibt höchstens einen Händler je Stufe.
pub const MERCHANT_ID: i64 = -1;

/// Go: `merchantDawn`: Abreise, dann Ankunft nach Rhythmus (aus `step_cycle` beim dawn).
pub fn merchant_dawn(w: &mut World, mut isl: Option<&mut IslandRef>) {
    // Go prüft `pure` zuerst; es hat keine Nebenwirkung, die Tiefe zuerst spart den Aufruf.
    if w.biome.depth != hub_of(w).merchant.only_depth || pure(w) {
        return;
    }
    let v = vals(w);
    let (m, day) = (&v.economy.merchant, w.cycle.day);
    if w.merchant.as_ref().is_some_and(|mm| day >= mm.leaves) {
        merchant_leaves(w, isl.as_deref(), "merchantLeft");
    }
    let every = if tavern_on_island(w, isl.as_deref()) {
        m.every_days_with_tavern
    } else {
        m.every_days
    };
    if w.merchant.is_some() || every <= 0 || day % every != 0 {
        return;
    }
    merchant_arrive(w, &v, isl.as_deref_mut());
    merchant_raid_start(w, isl.as_deref()); // jeder 4. Besuch (events_merchant.go)
}

/// Go: `merchantArrive`: Der Händler kommt an (Material per `w.rng`, Besuch zählt); auch für die Dev-Aktion `event`.
pub fn merchant_arrive(w: &mut World, v: &WorldValues, isl: Option<&mut IslandRef>) {
    let m = &v.economy.merchant;
    // Go schneidet `m.Materials[:n]` und würfelt `Int(0, len-1)`; n = 0 (keine Materialien) ist dort ein Panic.
    let n = (m.materials.len() as i64).min(w.hub_level.max(1));
    let pick = w.rng.int(0, n - 1);
    let resource = usize::try_from(pick)
        .ok()
        .and_then(|i| m.materials.get(i))
        .cloned()
        .unwrap_or_default();
    w.merchant = Some(Merchant {
        resource: resource.clone(),
        leaves: w.cycle.day + m.stay_days,
        hp: m.hp,
        max_hp: m.hp,
        ..Merchant::default()
    });
    if let Some(isl) = isl {
        isl.merchant_visits += 1; // Besuche zählen, Rhythmus des Händler-Überfalls (K3.2)
    }
    let x = unit_x(merchant_x(w, "buy"));
    emit(w, "merchantArrived", event(json!({"resource": resource, "x": x})));
}

/// Go: `merchantLeaves`: Der Händler reist ab (Typ `merchantLeft`) oder flieht (`merchantFled`); ein halber Kauf fällt als
/// Münzen.
pub fn merchant_leaves(w: &mut World, isl: Option<&IslandRef>, typ: &str) {
    merchant_raid_end(w, isl, typ == "merchantLeft");
    let (paid, x) = (w.merchant.as_ref().map_or(0, |m| m.buy_paid), merchant_x(w, "buy"));
    for _ in 0..paid {
        let id = w.new_id();
        w.coins.push(Coin {
            id,
            x,
            ..Coin::default()
        });
    }
    w.merchant = None;
    emit(w, typ, event(json!({})));
}

/// Go: `damageMerchant`: Schaden am Händler; bei 0 HP flieht er sofort.
pub fn damage_merchant(w: &mut World, damage: f64) {
    let Some(m) = w.merchant.as_mut() else {
        return;
    };
    m.hp -= damage;
    let hp = m.hp;
    hit_event(w, "merchant", MERCHANT_ID, merchant_x(w, "buy"), damage);
    if hp <= 0.0 {
        merchant_leaves(w, None, "merchantFled");
    }
}

/// Go: `tavernOnIsland`: Steht irgendwo auf der Insel eine gebaute Taverne? Ohne Insel zählt nur die eigene Stufe.
pub fn tavern_on_island(w: &World, isl: Option<&IslandRef>) -> bool {
    isl.is_some_and(|i| i.tavern_elsewhere) || w.sites.iter().any(|s| s.kind == "tavern" && s.state == "built")
}

/// Go: `merchantX`: Ort des Zahlziels `side` ("buy" oder sonst "sell") am Hub.
pub fn merchant_x(w: &World, side: &str) -> f64 {
    let m = &hub_of(w).merchant;
    if side == "sell" {
        w.hub_x + m.sell_offset_units
    } else {
        w.hub_x + m.buy_offset_units
    }
}

/// Go: `merchantAt`: "buy" oder "sell", wenn `x` in Zahl-Reichweite eines Zahlziels des anwesenden Händlers liegt.
pub fn merchant_at(w: &World, v: &WorldValues, x: f64) -> Option<&'static str> {
    w.merchant.as_ref()?;
    ["buy", "sell"]
        .into_iter()
        .find(|side| (merchant_x(w, side) - x).abs() <= v.economy.pay_range_units)
}

/// Go: `payMerchant`: ein Takt A halten am Händler (statt `pay_one_coin`): Kaufen nimmt eine Münze, mit `gold` Münzen kommt
/// `material` in den Vorrat; Verkaufen tauscht `material` aus dem Vorrat gegen `gold`. Geht nichts (Vorrat leer oder voll),
/// fällt keine Münze.
pub fn pay_merchant(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, side: &str) {
    let m = &v.economy.merchant;
    let Some(resource) = w.merchant.as_ref().map(|mm| mm.resource.clone()) else {
        return;
    };
    if side == "sell" {
        sell(w, v, isl, pi, &resource);
        return;
    }
    let paid = w.merchant.as_ref().map_or(0, |mm| mm.buy_paid);
    if w.players[pi].gold <= 0 || paid >= m.gold || resource_full(w, v, isl, &resource) {
        return;
    }
    let key = "merchant:buy";
    if w.players[pi].pay_key.as_deref() != Some(key) {
        refund_pending(w, v, isl, pi); // senkt `buy_paid` über `refund_merchant`
        w.players[pi].pay_key = Some(key.to_string());
    }
    let p = &mut w.players[pi];
    p.gold -= 1;
    p.pay_amount += 1;
    let (index, x) = (p.index, p.x);
    let done = w.merchant.as_mut().is_some_and(|mm| {
        mm.buy_paid += 1;
        mm.buy_paid >= m.gold
    });
    emit(
        w,
        "coinGive",
        event(json!({"player": index, "x": unit_x(x), "to": "merchant"})),
    );
    if done {
        if let Some(mm) = w.merchant.as_mut() {
            mm.buy_paid = 0;
        }
        add_stock(w, v, isl, &resource, m.material);
        clear_pending(&mut w.players[pi]);
        let ev = json!({"player": index, "resource": resource, "amount": m.material, "gold": -m.gold});
        emit(w, "traded", event(ev));
    }
}

/// Der Verkaufen-Zweig von `pay_merchant`: `material` aus dem Vorrat gegen `gold` in die Börse.
fn sell(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, resource: &str) {
    let m = &v.economy.merchant;
    match stock_field(&mut w.stock, resource) {
        Some(f) if *f >= m.material => *f -= m.material,
        _ => return,
    }
    let (index, x) = (w.players[pi].index, w.players[pi].x);
    give_gold(w, isl, pi, m.gold, x);
    let ev = json!({"player": index, "resource": resource, "amount": -m.material, "gold": m.gold});
    emit(w, "traded", event(ev));
}

/// Go: `refundMerchant`: zieht bis zu `amount` Münzen aus dem laufenden Kauf zurück und liefert die Anzahl.
pub fn refund_merchant(w: &mut World, amount: i64) -> i64 {
    let Some(m) = w.merchant.as_mut() else {
        return 0;
    };
    let back = amount.min(m.buy_paid);
    m.buy_paid -= back;
    back
}
