//! Tests zu `merchant/mod.rs` (Go: `merchant_test.go`, W4.2, B-121, Sprint W4 AC-03): Händler nach Rhythmus, Tausch in beide
//! Richtungen, Abreise nach einem Tag. Alle Werte aus data/economy.json › merchant und data/hub.json › merchant.
//!
//! Die Go-Tests laufen über `step` und `to_dawn`; sie brauchen `emit`, `unit_x` (events.go) und `pure` (dev.go) aus RU19 und
//! tragen `ignore`. Die Tests mit Präfix `own_` haben kein Go-Gegenstück und laufen schon: Sie prüfen die Teile ohne Ereignis.

use super::testutil::*;
use super::*;
use crate::enemies::testutil::build_site;
use crate::state::PlayerCommand;
use crate::world::values_room::default_values;

fn pay_cmd() -> PlayerCommand {
    PlayerCommand {
        pay: true,
        ..PlayerCommand::default()
    }
}

/// (d) Ankunft am Tag laut Rhythmus, nicht davor, Abreise beim nächsten dawn; mit Taverne früher; nur in Tiefe 0.
#[test]
#[ignore = "wartet auf RU18 (step_passives)"]
fn merchant_rhythmus_und_abreise() {
    let m = default_values().economy.merchant.clone();
    let mut w = merchant_world(0);
    let present = merchant_days(&mut w, 2 * m.every_days);
    for day in 2..=2 * m.every_days + 1 {
        let want = day % m.every_days == 0; // bleibt stay_days = 1 Tag
        let got = present.get(&day).copied().unwrap_or(false);
        assert_eq!(got, want, "ohne Taverne Tag {day}: da={got}, erwartet {want}");
    }

    let mut w = merchant_world(0);
    build_site(&mut w, "tavern");
    let present = merchant_days(&mut w, m.every_days_with_tavern);
    let at = |d: i64| present.get(&d).copied().unwrap_or(false);
    assert!(
        !at(m.every_days_with_tavern - 1) && at(m.every_days_with_tavern),
        "mit Taverne: {present:?}, erwartet Ankunft an Tag {}",
        m.every_days_with_tavern
    );

    let mut w = merchant_world(1);
    for (day, da) in merchant_days(&mut w, m.every_days) {
        assert!(!da, "Händler in Tiefe 1 an Tag {day}");
    }
}

/// (d) Gleicher Seed → gleiches Material, nur aus den freigeschalteten.
#[test]
#[ignore = "wartet auf RU18 (step_passives)"]
fn merchant_material_aus_rng_und_freigeschaltet() {
    let m = default_values().economy.merchant.clone();
    let pick = |level: i64| -> String {
        let mut w = merchant_world(0);
        w.hub_level = level;
        merchant_days(&mut w, m.every_days - 1); // bis zum Ankunftstag
        w.merchant.as_ref().expect("kein Händler").resource.clone()
    };
    assert_eq!(pick(1), m.materials[0], "Hub-Stufe 1");
    let (a, b) = (pick(m.materials.len() as i64), pick(m.materials.len() as i64));
    assert!(a == b && m.materials.contains(&a), "gleicher Seed: {a} und {b}");
}

/// (d) Kaufen: 10 × gold Münzen → 10 × material; Verkaufen umgekehrt; andere Materialien bleiben.
#[test]
#[ignore = "wartet auf RU18 (speed_of)"]
fn merchant_tauscht_in_beide_richtungen() {
    let m = default_values().economy.merchant.clone();
    let trades = 10;
    let (mut w, ps) = trade_world("stone", "buy", "");
    w.players[ps[0]].gold = trades * m.gold;
    steps(&mut w, &[pay_cmd()], 30.0);
    let open = w.merchant.as_ref().map_or(0, |mm| mm.buy_paid);
    assert!(
        w.players[ps[0]].gold == 0 && w.stock.stone == trades * m.material && w.stock.wood == 0 && open == 0,
        "Kaufen: Gold {}, Stein {}, Holz {}, offen {open}",
        w.players[ps[0]].gold,
        w.stock.stone,
        w.stock.wood
    );

    let (mut w, ps) = trade_world("stone", "sell", "");
    w.stock = k3c_data::hub::Stock {
        stone: trades * m.material,
        wood: 50,
        ..Default::default()
    };
    w.players[ps[0]].gold = 0;
    steps(&mut w, &[pay_cmd()], 30.0);
    assert!(
        w.players[ps[0]].gold == trades * m.gold && w.stock.stone == 0 && w.stock.wood == 50 && w.coins.is_empty(),
        "Verkaufen: Gold {}, Stein {}, Holz {}, Münzen {}",
        w.players[ps[0]].gold,
        w.stock.stone,
        w.stock.wood,
        w.coins.len()
    );
}

/// (d) Nach einem Tag ist der Händler weg; ein halber Kauf fällt als Münzen zu Boden.
#[test]
#[ignore = "wartet auf RU18 (step_passives)"]
fn merchant_reist_ab_halber_kauf_zu_boden() {
    let m = default_values().economy.merchant.clone();
    let (mut w, _) = trade_world("wood", "buy", "");
    w.players.clear(); // niemand hebt die Münzen auf
    let day = w.cycle.day;
    let mm = w.merchant.as_mut().expect("Händler");
    mm.leaves = day + m.stay_days;
    mm.buy_paid = m.gold - 1;
    let paid = mm.buy_paid;
    to_dawn(&mut w);
    assert!(
        w.merchant.is_none() && paid != 0 && w.coins.len() as i64 == paid,
        "Abreise: Händler {:?}, offen {paid}, Münzen {}",
        w.merchant,
        w.coins.len()
    );
}

/// (e) Zwei Spieler tauschen gleichzeitig (einer kauft, einer verkauft); das Ergebnis hängt nicht von der Reihenfolge ab.
#[test]
#[ignore = "wartet auf RU18 (speed_of)"]
fn merchant_zwei_spieler_reihenfolge_egal() {
    let m = default_values().economy.merchant.clone();
    let run = |swap: bool| {
        let sides = if swap { ["sell", "buy"] } else { ["buy", "sell"] };
        let (mut w, ps) = trade_world("stone", sides[0], sides[1]);
        w.stock = k3c_data::hub::Stock {
            stone: 5 * m.material,
            ..Default::default()
        };
        for (i, &pi) in ps.iter().enumerate() {
            w.players[pi].gold = if sides[i] == "buy" { 5 * m.gold } else { 0 };
        }
        steps(&mut w, &[pay_cmd(), pay_cmd()], 20.0);
        let gold = ps.iter().map(|&pi| w.players[pi].gold).sum::<i64>();
        (w.stock.clone(), gold)
    };
    let ((s1, g1), (s2, g2)) = (run(false), run(true));
    // Am Ende ist alles verkauft: 5 Käufe (+5 × material) und 10 Verkäufe, zusammen 10 × gold in den Börsen.
    assert!(
        s1 == s2 && g1 == g2 && g1 == 10 * m.gold && s1.stone == 0,
        "Reihenfolge: Vorrat {s1:?}/{s2:?}, Gold {g1}/{g2}"
    );
}

/// Eigener Test: `merchant_at` kennt beide Zahlziele nur bei anwesendem Händler (Kaufen 16, Verkaufen 20, Reichweite 2).
#[test]
fn own_merchant_at_findet_die_zahlziele() {
    let mut w = merchant_world(0);
    let v = vals(&w);
    let (buy, sell) = (merchant_x(&w, "buy"), merchant_x(&w, "sell"));
    assert!(buy < sell, "Kaufen {buy}, Verkaufen {sell}");
    assert_eq!(merchant_at(&w, &v, buy), None, "ohne Händler");
    w.merchant = Some(Merchant {
        resource: "wood".into(),
        leaves: 99,
        ..Merchant::default()
    });
    let r = v.economy.pay_range_units;
    assert_eq!(merchant_at(&w, &v, buy), Some("buy"));
    assert_eq!(merchant_at(&w, &v, buy + r), Some("buy"), "Rand der Reichweite");
    assert_eq!(merchant_at(&w, &v, sell), Some("sell"));
    assert_eq!(merchant_at(&w, &v, w.hub_x), None, "fern der Ziele");
}

/// Eigener Test: `refund_merchant` zieht höchstens den offenen Kauf zurück.
#[test]
fn own_refund_merchant_zieht_hoechstens_den_offenen_kauf_zurueck() {
    let mut w = merchant_world(0);
    assert_eq!(refund_merchant(&mut w, 3), 0, "ohne Händler");
    w.merchant = Some(Merchant {
        buy_paid: 3,
        ..Merchant::default()
    });
    assert_eq!(refund_merchant(&mut w, 2), 2);
    assert_eq!(refund_merchant(&mut w, 5), 1);
    assert_eq!(w.merchant.as_ref().map(|m| m.buy_paid), Some(0));
    assert_eq!(refund_merchant(&mut w, 1), 0);
}

/// Eigener Test: `tavern_on_island` zählt eine gebaute Taverne der eigenen Stufe und eine der anderen Stufen
/// (`tavern_elsewhere`); ohne Insel nur die eigene.
#[test]
fn own_tavern_on_island_zaehlt_eigene_und_andere_stufen() {
    let mut w = merchant_world(0);
    let other = IslandRef {
        tavern_elsewhere: true,
        ..IslandRef::default()
    };
    assert!(!tavern_on_island(&w, None) && !tavern_on_island(&w, Some(&IslandRef::default())));
    assert!(tavern_on_island(&w, Some(&other)), "Taverne einer anderen Stufe");
    build_site(&mut w, "tavern");
    assert!(tavern_on_island(&w, None), "gebaute Taverne der eigenen Stufe");
}

/// Eigener Test: Außerhalb der Tiefe `onlyDepth` kommt kein Händler, `merchant_dawn` endet vor `pure` und `emit`.
#[test]
fn own_merchant_dawn_in_anderer_tiefe_tut_nichts() {
    let mut w = merchant_world(1);
    w.cycle.day = default_values().economy.merchant.every_days;
    merchant_dawn(&mut w, None);
    assert!(w.merchant.is_none() && w.events.is_empty());
}

/// Eigener Test: Verkaufen ohne Material im Vorrat und Kaufen ohne Gold ändern nichts (keine Münze fällt, kein Ereignis).
#[test]
fn own_pay_merchant_ohne_vorrat_oder_gold_aendert_nichts() {
    let (mut w, ps) = trade_world("stone", "sell", "buy");
    let v = vals(&w);
    w.players[ps[1]].gold = 0;
    pay_merchant(&mut w, &v, None, ps[0], "sell");
    pay_merchant(&mut w, &v, None, ps[1], "buy");
    assert!(w.players[ps[0]].gold == v.economy.purse.start_gold && w.players[ps[1]].gold == 0);
    assert!(w.stock.stone == 0 && w.coins.is_empty() && w.events.is_empty());
    assert_eq!(w.merchant.as_ref().map(|m| m.buy_paid), Some(0));
}

/// Eigener Test: `trade_world` stellt die Spieler an die Zahlziele.
#[test]
fn own_trade_world_stellt_spieler_an_die_zahlziele() {
    let (w, ps) = trade_world("wood", "buy", "sell");
    assert_eq!(ps.len(), 2);
    assert_eq!(w.players[ps[0]].x, merchant_x(&w, "buy"));
    assert_eq!(w.players[ps[1]].x, merchant_x(&w, "sell"));
}
