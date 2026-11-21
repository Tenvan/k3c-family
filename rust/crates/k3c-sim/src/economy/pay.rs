//! Münzen zahlen und erstatten (Port von `engine/sim/economy.go`: `payOneCoin`, `paySite`, `refundPending`, `refundSite`
//! und `engine/sim/economy_pay.go`: `payToTarget`, `payVagrant`).

use serde_json::json;

use super::links::{coin_give_event, pay_merchant, pay_upgrade, refund_merchant};
use super::{PayTarget, clear_pending, find_pay_target, key_of, offer_target, site_payable};
use crate::building::upgrades::{elite_offer_at, pay_any_offer};
use crate::combat::spots::{pay_combat, refund_combat};
use crate::state::common::event;
use crate::state::island_ref::IslandRef;
use crate::state::node_mark::mark_node;
use crate::state::{Coin, World, WorldValues};
use crate::unit::professions::refund_offer;
use crate::unit::warrior::{near_full_target, pay_sword, refund_sword};
use crate::world::links::{site_cost, upgrade_payable};
use crate::world::vein::gather_of;

/// Go: `refundPending`: wirft gezahltes, aber nicht vollendetes Gold des Spielers `pi` zurück auf den Boden und zieht es
/// beim Ziel ab.
pub fn refund_pending(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize) {
    let (key, amount) = (w.players[pi].pay_key.clone(), w.players[pi].pay_amount);
    clear_pending(&mut w.players[pi]);
    let Some(key) = key.filter(|_| amount > 0) else {
        return;
    };
    let (kind, id_text) = key.split_once(':').unwrap_or((key.as_str(), ""));
    let id = id_text.parse::<i64>().unwrap_or(0);
    let back = match kind {
        "site" => refund_site(w, v, isl, id, amount),
        "offer" => refund_offer(w, id_text, amount),
        "merchant" => refund_merchant(w, amount),
        "sword" => refund_sword(w, id, amount),
        "combat" => refund_combat(w, id_text, amount),
        "vagrant" => match w.troop_mut(id).filter(|t| t.is_wild()) {
            Some(t) => {
                let back = amount.min(t.paid_gold);
                t.paid_gold -= back;
                back
            }
            None => 0,
        },
        _ => match w.node_mut(id).filter(|n| !n.marked) {
            Some(n) => {
                let back = amount.min(n.paid_gold);
                n.paid_gold -= back;
                back
            }
            None => 0,
        },
    };
    let x = w.players[pi].x;
    for _ in 0..back {
        let id = w.new_id();
        w.coins.push(Coin {
            id,
            x,
            ..Coin::default()
        });
    }
}

/// Go: `refundSite`: zieht bis zu `amount` Münzen aus Bau, Ausbau oder Bogen des Platzes `id` zurück und liefert die Anzahl.
pub fn refund_site(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, id: i64, amount: i64) -> i64 {
    let Some(s) = w.site(id) else {
        return 0;
    };
    if !site_payable(w, v, isl, s) {
        return 0;
    }
    let unpaid = s.state == "unpaid";
    let upgrade = !unpaid && upgrade_payable(w, s);
    let Some(s) = w.site_mut(id) else {
        return 0;
    };
    let paid = if unpaid {
        &mut s.paid_gold
    } else if upgrade {
        &mut s.upgrade_paid
    } else {
        &mut s.bow_paid_gold
    };
    let back = amount.min(*paid);
    *paid -= back;
    back
}

/// Go: `payOneCoin`: eine Münze des Spielers `pi` an das Ziel in Reichweite; ohne Ziel fällt sie zu Boden.
pub fn pay_one_coin(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize) {
    let target = find_pay_target(w, v, isl, &w.players[pi]);
    let x = w.players[pi].x;
    // Am fertig bezahlten Bauplatz oder vollen Angebot nichts fallen lassen (sonst verliert man beim Festhalten Münzen).
    if target.is_none() && (near_full_target(w, v, x) || elite_offer_at(w, v, isl, x, false).is_some()) {
        return;
    }
    if let Some(PayTarget::Merchant(side)) = &target {
        pay_merchant(w, v, isl, pi, side);
        return;
    }
    w.players[pi].gold -= 1;
    let Some(target) = target else {
        let (id, now) = (w.new_id(), w.time);
        let p = &w.players[pi];
        w.coins.push(Coin {
            id,
            x,
            blocked_player_id: Some(p.id),
            blocked_until: now + v.economy.drop_pickup_delay_seconds,
            ..Coin::default()
        });
        return;
    };
    if !matches!(target, PayTarget::Offer { .. }) {
        coin_give_event(w, pi, &target); // pay_offer meldet selbst (to: offer)
    }
    let key = key_of(w, Some(&target));
    if w.players[pi].pay_key.as_deref() != Some(key.as_str()) {
        refund_pending(w, v, isl, pi);
    }
    w.players[pi].pay_key = Some(key);
    w.players[pi].pay_amount += 1;
    pay_to_target(w, v, isl, pi, &target);
}

/// Go: `paySite`: eine Münze auf Bau, Ausbau oder Bogen des Platzes `id`.
pub fn pay_site(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, id: i64) {
    let Some(s) = w.site(id) else {
        return;
    };
    if s.state == "unpaid" {
        let cost = site_cost(w, isl, s).gold;
        if let Some(s) = w.site_mut(id) {
            s.paid_gold += 1;
            if s.paid_gold >= cost {
                s.state = "waitingMaterial".into();
            }
        }
    } else if upgrade_payable(w, s) {
        pay_upgrade(w, isl, id);
    } else if let Some(s) = w.site_mut(id) {
        s.bow_paid_gold += 1;
    }
    if w.site(id).is_some_and(|s| !site_payable(w, v, isl, s)) {
        clear_pending(&mut w.players[pi]);
    }
}

/// Go: `payToTarget` bucht die Münze beim Ziel (Angebot, Schwert, Kampfvariante, Bauplatz, Landstreicher oder Ressource).
pub fn pay_to_target(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>, pi: usize, target: &PayTarget) {
    match target {
        PayTarget::Offer { site, idx } => {
            if let Some(o) = offer_target(w, v, *site, *idx) {
                pay_any_offer(w, v, isl, pi, &o);
            }
        }
        PayTarget::Sword(si) => pay_sword(w, v, isl, pi, *si),
        PayTarget::Combat { spot, .. } => pay_combat(w, v, pi, spot),
        PayTarget::Site(id) => pay_site(w, v, isl, pi, *id),
        PayTarget::Troop(id) => pay_vagrant(w, v, pi, *id),
        PayTarget::Node(id) => pay_node(w, v, pi, *id),
        PayTarget::Merchant(_) => {} // der Händler zahlt über `pay_merchant`, nie hier
    }
}

/// Der Zweig `default` von `payToTarget`: eine Markier-Münze auf die Ressource `id`; voll bezahlt wird sie markiert.
fn pay_node(w: &mut World, v: &WorldValues, pi: usize, id: i64) {
    let Some(n) = w.nodes.iter_mut().find(|n| n.id == id) else {
        return;
    };
    n.paid_gold += 1;
    let mark_cost = gather_of(v, &n.kind).map_or(0, |g| g.mark_cost);
    if n.paid_gold >= mark_cost {
        mark_node(n, &w.players[pi]);
        clear_pending(&mut w.players[pi]);
    }
}

/// Go: `payVagrant`: eine Rekrutier-Münze für den Landstreicher `id`; voll bezahlt wird er zum Bauern.
fn pay_vagrant(w: &mut World, v: &WorldValues, pi: usize, id: i64) {
    let hub_x = w.hub_x;
    let Some(t) = w.troop_mut(id) else {
        return;
    };
    t.paid_gold += 1;
    // ponytail: `recruitCost?.gold ?? 1` in TS; ein ausdrückliches gold: 0 in data/troops.json würde hier 1 kosten.
    let cost = v.troops.get("vagrant").map_or(0, |t| t.recruit_cost.gold);
    let cost = if cost == 0 { 1 } else { cost };
    if t.paid_gold < cost {
        return;
    }
    let hp = v.troops.get("peasant").map_or(0.0, |t| t.hp);
    t.serve();
    t.hp = hp;
    t.max_hp = hp;
    t.anchor_x = hub_x;
    t.target_x = t.x;
    t.job = None;
    t.paid_gold = 0;
    let index = w.players[pi].index;
    w.events.push(event(json!({"type": "recruited", "player": index})));
    clear_pending(&mut w.players[pi]);
}
