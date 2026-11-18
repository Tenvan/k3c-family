//! Tests zu `pay.rs`, `util.rs` und dem Zahlziel in `mod.rs` (Go: die Abdeckung von `economy.go` und `economy_pay.go` liegt
//! in `building_*_test.go`, `unit_*_test.go` und den Golden; `economy.go` hat keinen eigenen Test). Eigene Tests ohne
//! `create_world`; was `site_cost`, `upgrade_payable` (RU18.3), `merchant_at` (RU18.4) oder `step_players` (RU18.5) braucht,
//! trägt `ignore`.

use super::pending::{coin_give_event, merchant_at};
use super::*;
use crate::enemies::testutil::{bare_world, built_site};
use crate::state::{Coin, ResourceNode, Troop};
use crate::world::vals;

/// Zwei Monarchen (Index 0 und 1) an `x`, je mit `gold`.
fn two_players(w: &mut World, x: f64, gold: i64) {
    for index in 0..2 {
        let id = w.new_id();
        w.players.push(Player {
            id,
            index,
            x,
            gold,
            ..Player::default()
        });
    }
}

fn add_node(w: &mut World, kind: &str, x: f64) -> i64 {
    let id = w.new_id();
    w.nodes.push(ResourceNode {
        id,
        kind: kind.into(),
        x,
        ..ResourceNode::default()
    });
    id
}

fn add_vagrant(w: &mut World, x: f64) -> i64 {
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "vagrant".into(),
        x,
        ..Troop::default()
    });
    id
}

#[test]
fn nearest_nimmt_den_naechsten_bei_gleichstand_den_fruehen() {
    let items = [10.0, 14.0, 6.0, 12.0];
    assert_eq!(nearest(&items, |x| *x, 10.0, 5.0, |_| true), Some(0), "Abstand 0");
    assert_eq!(
        nearest(&items, |x| *x, 8.0, 5.0, |_| true),
        Some(0),
        "Gleichstand 10 und 6: der frühere"
    );
    assert_eq!(
        nearest(&items, |x| *x, 8.0, 5.0, |x| *x != 10.0),
        Some(2),
        "ok schließt aus"
    );
    assert_eq!(nearest(&items, |x| *x, 100.0, 5.0, |_| true), None, "außer Reichweite");
    assert_eq!(nearest(&items, |x| *x, 8.0, 5.0, |_| false), None, "nichts ok");
}

#[test]
fn can_afford_und_spend_ohne_gold() {
    let mut s = k3c_data::hub::Stock {
        wood: 5,
        stone: 3,
        ..Default::default()
    };
    let c = k3c_data::buildings::Cost {
        gold: 99,
        wood: 5,
        stone: 3,
        ..Default::default()
    };
    assert!(can_afford(&s, &c), "Gold zählt nicht");
    spend(&mut s, &c);
    assert!(s.wood == 0 && s.stone == 0);
    assert!(!can_afford(&s, &c));
}

#[test]
fn key_of_nennt_das_ziel() {
    let mut w = bare_world();
    let si = built_site(&mut w, "workshop", 50.0);
    let id = w.sites[si].id;
    assert_eq!(key_of(&w, None), "");
    assert_eq!(key_of(&w, Some(&PayTarget::Site(id))), format!("site:{id}"));
    assert_eq!(key_of(&w, Some(&PayTarget::Troop(7))), "vagrant:7");
    assert_eq!(key_of(&w, Some(&PayTarget::Node(9))), "node:9");
    assert_eq!(key_of(&w, Some(&PayTarget::Sword(si))), format!("sword:{id}"));
    assert_eq!(
        key_of(&w, Some(&PayTarget::Offer { site: si, idx: 1 })),
        format!("offer:{id}.1")
    );
    assert_eq!(key_of(&w, Some(&PayTarget::Merchant("buy".into()))), "merchant:buy");
    let spot = CombatSpot {
        kind: SpotKind::Trigger,
        side: -1,
        x: 5.0,
    };
    let t = PayTarget::Combat {
        key: "trigger:-1".into(),
        spot,
    };
    assert_eq!(key_of(&w, Some(&t)), "combat:trigger:-1");
}

/// AC-03: Zwei Spieler zahlen gleichzeitig an einem Zahlziel (Ressource): die Münzen beider zählen, die letzte Münze
/// markiert und beendet nur die Zahlung dessen, der sie gibt; der andere bekommt beim Loslassen nichts zurück, weil die
/// Ressource markiert ist.
#[test]
fn zwei_spieler_zahlen_gleichzeitig_an_einer_ressource() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let nid = add_node(&mut w, "copperOre", 100.0);
    let cost = v.economy.gatherables["copperOre"].mark_cost;
    assert!(cost >= 2, "der Test braucht Markierkosten ab 2, nicht {cost}");
    let target = PayTarget::Node(nid);
    let key = key_of(&w, Some(&target));
    for (pi, coins) in [(0, cost - 1), (1, 0)] {
        for _ in 0..coins {
            w.players[pi].gold -= 1;
            w.players[pi].pay_key = Some(key.clone());
            w.players[pi].pay_amount += 1;
            pay_to_target(&mut w, &v, None, pi, &target);
        }
    }
    assert!(
        !w.nodes[0].marked && w.nodes[0].paid_gold == cost - 1,
        "noch nicht voll bezahlt"
    );
    w.players[1].gold -= 1;
    w.players[1].pay_key = Some(key.clone());
    w.players[1].pay_amount = 1;
    pay_to_target(&mut w, &v, None, 1, &target);
    let n = &w.nodes[0];
    assert!(n.marked && n.marked_by == Some(1), "die letzte Münze markiert: {n:?}");
    assert!(
        w.players[1].pay_key.is_none() && w.players[1].pay_amount == 0,
        "Zahlung des Markierers fertig"
    );
    assert_eq!(
        w.players[0].pay_key.as_deref(),
        Some(key.as_str()),
        "die andere Zahlung bleibt offen"
    );
    refund_pending(&mut w, &v, None, 0);
    assert!(w.coins.is_empty(), "markiert: nichts zurück");
    assert!(w.players[0].pay_key.is_none() && w.players[0].pay_amount == 0);
}

/// Zwei Spieler bezahlen denselben Landstreicher: die Münzen zählen zusammen, die letzte macht ihn zum Bauern.
#[test]
fn zwei_spieler_rekrutieren_gemeinsam_einen_landstreicher() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let tid = add_vagrant(&mut w, 100.0);
    let cost = v.troops["vagrant"].recruit_cost.gold.max(1);
    let target = PayTarget::Troop(tid);
    for n in 0..cost {
        let pi = (n % 2) as usize;
        pay_to_target(&mut w, &v, None, pi, &target);
    }
    let t = &w.troops[0];
    assert_eq!(t.kind, "peasant", "bezahlt: Bauer");
    assert_eq!(t.paid_gold, 0);
    assert_eq!(t.hp, v.troops["peasant"].hp);
    assert_eq!(t.anchor_x, w.hub_x);
    assert!(t.job.is_none());
    assert_eq!(w.events.len(), 1, "ein Ereignis recruited");
    assert_eq!(w.events[0]["type"], "recruited");
    assert_eq!(w.events[0]["player"], (cost - 1) % 2, "der Spieler der letzten Münze");
}

#[test]
fn refund_pending_ohne_zahlung_tut_nichts() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    w.players[0].pay_amount = 3; // ohne Schlüssel
    refund_pending(&mut w, &v, None, 0);
    assert!(w.coins.is_empty() && w.players[0].pay_amount == 0);
    w.players[1].pay_key = Some("node:5".into()); // Betrag 0
    refund_pending(&mut w, &v, None, 1);
    assert!(w.coins.is_empty() && w.players[1].pay_key.is_none());
}

#[test]
fn refund_pending_wirft_landstreicher_und_ressource_zurueck() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let tid = add_vagrant(&mut w, 100.0);
    w.troops[0].paid_gold = 2;
    w.players[0].pay_key = Some(format!("vagrant:{tid}"));
    w.players[0].pay_amount = 3;
    let before = w.next_id;
    refund_pending(&mut w, &v, None, 0);
    assert_eq!(w.troops[0].paid_gold, 0, "höchstens das Bezahlte");
    assert_eq!(w.coins.len(), 2);
    assert!(w.coins.iter().all(|c| c.x == 100.0), "Münzen fallen beim Spieler");
    assert_eq!(
        w.coins.iter().map(|c| c.id).collect::<Vec<_>>(),
        vec![before, before + 1],
        "IDs aufsteigend"
    );
    let nid = add_node(&mut w, "tree", 100.0);
    w.nodes[0].paid_gold = 3;
    w.players[1].pay_key = Some(format!("node:{nid}"));
    w.players[1].pay_amount = 2;
    refund_pending(&mut w, &v, None, 1);
    assert_eq!((w.nodes[0].paid_gold, w.coins.len()), (1, 4));
    assert!(w.players[1].pay_key.is_none());
}

#[test]
fn pay_node_markiert_ohne_gather_eintrag_sofort() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let nid = add_node(&mut w, "unbekannt", 100.0); // Go: `gatherOf` liefert Nullwerte, MarkCost 0
    pay_to_target(&mut w, &v, None, 0, &PayTarget::Node(nid));
    assert!(w.nodes[0].marked && w.nodes[0].marked_by == Some(0));
}

#[test]
fn pay_to_target_ignoriert_verschwundene_ziele() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    pay_to_target(&mut w, &v, None, 0, &PayTarget::Node(999));
    pay_to_target(&mut w, &v, None, 0, &PayTarget::Troop(999));
    pay_to_target(&mut w, &v, None, 0, &PayTarget::Merchant("buy".into()));
    assert!(w.coins.is_empty() && w.events.is_empty());
}

#[test]
fn coin_default_ohne_sperre() {
    let c = Coin::default();
    assert!(c.blocked_player_id.is_none() && c.lands_at == 0.0);
}

#[test]
#[ignore = "wartet auf RU18 (site_cost, upgrade_payable aus RU18.3)"]
fn pay_site_unbezahlt_wartet_auf_material_wenn_voll() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let si = built_site(&mut w, "workshop", 100.0);
    w.sites[si].state = "unpaid".into();
    let id = w.sites[si].id;
    let cost = crate::world::pending::site_cost(&w, &w.sites[si]).gold;
    for n in 0..cost {
        pay_site(&mut w, &v, None, (n % 2) as usize, id);
    }
    assert_eq!(w.sites[si].state, "waitingMaterial");
}

#[test]
#[ignore = "wartet auf RU18 (upgrade_payable aus RU18.3)"]
fn refund_site_zieht_bogen_gold_vom_gebauten_platz() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let si = built_site(&mut w, "workshop", 100.0);
    w.sites[si].bow_paid_gold = 2;
    let id = w.sites[si].id;
    assert_eq!(refund_site(&mut w, &v, None, id, 5), 2);
    assert_eq!(w.sites[si].bow_paid_gold, 0);
}

#[test]
#[ignore = "wartet auf RU18 (merchant_at aus RU18.4, coin_give_event aus RU19)"]
fn find_pay_target_und_pay_one_coin_zwei_spieler() {
    let mut w = bare_world();
    two_players(&mut w, 100.0, 10);
    let v = vals(&w);
    let tid = add_vagrant(&mut w, 100.0);
    assert_eq!(merchant_at(&w, &v, 100.0), None);
    let found = find_pay_target(&w, &v, None, &w.players[0]);
    assert_eq!(found, Some(PayTarget::Troop(tid)));
    coin_give_event(&mut w, 0, &PayTarget::Troop(tid));
    pay_one_coin(&mut w, &v, None, 0);
    pay_one_coin(&mut w, &v, None, 1);
    assert_eq!(w.players[0].gold + w.players[1].gold, 18);
}
