//! Beladen an den drei Ladeplätzen (RU62.1, AC-01): Beutel, Truppe, Kiste; Ablehnung ohne Abzug; zwei Spieler im selben Takt.

use super::testutil::{add_troop, cargo_data, cargo_island, count, pay_cmds, run, stand_at};
use super::{Bay, KIND_COINS, KIND_CRATE, KIND_TROOP};
use crate::island::testutil::test_data;

#[test]
fn muenz_platz_belegt_einen_platz_mit_einem_beutel() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    stand_at(&mut isl, steg, 0, Bay::Coins);
    let ev = run(&mut isl, &data, &pay_cmds(1, &[0]), 2.4);
    run(&mut isl, &data, &pay_cmds(1, &[]), 0.1); // Taste los: ein angefangener zweiter Einwurf wird erstattet
    assert_eq!(isl.cargo.load.len(), 1);
    assert_eq!(
        (isl.cargo.load[0].kind.as_str(), isl.cargo.load[0].amount),
        (KIND_COINS, 10)
    );
    assert_eq!(isl.stages[0].players[0].gold, 20, "genau purseCoins abgezogen");
    assert_eq!(count(&ev, "cargoLoaded"), 1);
    assert!(isl.stages[0].coins.is_empty(), "keine Münze fällt auf den Boden");
    let st = isl.stages[0].cargo.as_ref().expect("Zustand");
    assert_eq!((st.slots, st.used, st.reserved), (6, 1, 0));
    assert_eq!(st.bays.len(), 3);
}

#[test]
fn truppen_platz_ruft_die_naechste_truppe_an_bord() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    let far = add_troop(&mut isl, "warrior", steg - 30.0);
    let near = add_troop(&mut isl, "archer", steg + 5.0);
    add_troop(&mut isl, "peasant", steg + 6.0); // Bürger zählen nicht
    stand_at(&mut isl, steg, 0, Bay::Troops);
    run(&mut isl, &data, &pay_cmds(1, &[0]), 1.1);
    run(&mut isl, &data, &pay_cmds(1, &[]), 0.1);
    assert_eq!(isl.cargo.load.len(), 1);
    assert_eq!(
        (isl.cargo.load[0].kind.as_str(), isl.cargo.load[0].id.as_str()),
        (KIND_TROOP, "archer")
    );
    assert!(
        isl.stages[0].troops.iter().all(|t| t.id != near),
        "die Truppe ist an Bord"
    );
    assert!(isl.stages[0].troops.iter().any(|t| t.id == far), "die ferne bleibt");
    assert_eq!(isl.stages[0].players[0].gold, 25, "troopCoins");
}

#[test]
fn material_platz_reserviert_ab_auftrag_und_belegt_erst_bei_ankunft() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    isl.stock.wood = 50;
    isl.stock.stone = 90; // der größte Vorrat
    isl.sync_stock();
    let peasant = add_troop(&mut isl, "peasant", steg + 40.0);
    stand_at(&mut isl, steg, 0, Bay::Crates);
    run(&mut isl, &data, &pay_cmds(1, &[0]), 1.5);
    assert_eq!(isl.stock.stone, 70, "Material ab Auftrag reserviert");
    assert_eq!(
        isl.cargo.orders.len(),
        1,
        "der Platz ist reserviert, die Kiste noch nicht an Bord"
    );
    assert_eq!(isl.cargo.orders[0].carrier, peasant);
    assert!(isl.cargo.load.is_empty());
    let st = isl.stages[0].cargo.as_ref().expect("Zustand");
    assert_eq!((st.used, st.reserved), (0, 1));
    let ev = run(&mut isl, &data, &pay_cmds(1, &[]), 20.0);
    assert!(isl.cargo.orders.is_empty() && isl.cargo.load.len() == 1);
    assert_eq!(
        (isl.cargo.load[0].kind.as_str(), isl.cargo.load[0].id.as_str()),
        (KIND_CRATE, "stone")
    );
    assert_eq!(count(&ev, "cargoLoaded"), 1);
    assert_eq!(isl.stock.wood, 50, "das andere Material bleibt");
}

#[test]
fn voller_laderaum_lehnt_ab_ohne_abzug() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    for _ in 0..6 {
        isl.cargo.load.push(super::CargoItem {
            kind: KIND_COINS.into(),
            amount: 10,
            ..super::CargoItem::default()
        });
    }
    stand_at(&mut isl, steg, 0, Bay::Coins);
    let ev = run(&mut isl, &data, &pay_cmds(1, &[0]), 4.0);
    assert_eq!(isl.stages[0].players[0].gold, 30, "keine Münze abgezogen");
    assert_eq!(isl.cargo.load.len(), 6);
    assert_eq!(count(&ev, "cargoFull"), 1, "einmal je gehaltener Taste");
    let st = isl.stages[0].cargo.as_ref().expect("Zustand");
    assert!(st.bays.iter().all(|b| b.state == "full"));
}

#[test]
fn vorrat_unter_einer_kiste_und_keine_truppe_lehnen_ab() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    isl.stock.wood = 19;
    isl.sync_stock();
    stand_at(&mut isl, steg, 0, Bay::Crates);
    let ev = run(&mut isl, &data, &pay_cmds(1, &[0]), 3.0);
    assert_eq!((isl.stages[0].players[0].gold, isl.stock.wood), (30, 19));
    assert!(isl.cargo.orders.is_empty());
    assert_eq!(count(&ev, "cargoRejected"), 1);
    stand_at(&mut isl, steg, 0, Bay::Troops);
    run(&mut isl, &data, &pay_cmds(1, &[]), 0.5);
    let ev = run(&mut isl, &data, &pay_cmds(1, &[0]), 3.0);
    assert_eq!(isl.stages[0].players[0].gold, 30, "keine Truppe: abgelehnt");
    assert_eq!(count(&ev, "cargoRejected"), 1);
    assert!(isl.cargo.load.is_empty());
}

#[test]
fn loslassen_erstattet_die_eingeworfenen_muenzen() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    stand_at(&mut isl, steg, 0, Bay::Coins);
    run(&mut isl, &data, &pay_cmds(1, &[0]), 1.0); // vier bis fünf Münzen, noch nicht voll
    assert!(isl.stages[0].players[0].gold < 30 && isl.cargo.load.is_empty());
    run(&mut isl, &data, &pay_cmds(1, &[]), 0.2);
    assert_eq!(isl.stages[0].players[0].gold, 30, "alles zurück");
    assert!(isl.cargo.pay.is_empty());
}

#[test]
fn zwei_spieler_im_selben_takt_ein_platz_frei() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 2);
    for _ in 0..5 {
        isl.cargo.load.push(super::CargoItem {
            kind: KIND_COINS.into(),
            amount: 10,
            ..super::CargoItem::default()
        });
    }
    stand_at(&mut isl, steg, 0, Bay::Coins);
    stand_at(&mut isl, steg, 1, Bay::Coins);
    let ev = run(&mut isl, &data, &pay_cmds(2, &[0, 1]), 4.0);
    assert_eq!(isl.cargo.load.len(), 6, "ein Platz, ein Beutel");
    let golds: Vec<i64> = isl.stages[0].players.iter().map(|p| p.gold).collect();
    assert_eq!(
        golds,
        vec![20, 30],
        "Spieler 1 lädt, die Münzen von Spieler 2 bleiben bei ihm"
    );
    assert_eq!(count(&ev, "cargoFull"), 2, "je Spieler einmal");
}

#[test]
fn ohne_segel_und_auf_der_letzten_insel_keine_ladeplaetze() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    stand_at(&mut isl, steg, 0, Bay::Coins);
    isl.stages[0].sites.iter_mut().for_each(|s| {
        if s.kind == "ship" {
            s.level = 2; // Segel fehlt
        }
    });
    run(&mut isl, &data, &pay_cmds(1, &[0]), 3.0);
    assert!(isl.cargo.load.is_empty() && isl.stages[0].cargo_bays.is_empty());
    isl.stages[0].sites.iter_mut().for_each(|s| {
        if s.kind == "ship" {
            s.level = 3;
        }
    });
    isl.number = 1; // letzte Insel der Test-Welt
    assert!(isl.is_last());
    run(&mut isl, &data, &pay_cmds(1, &[0]), 3.0);
    assert!(isl.cargo.load.is_empty(), "auf der letzten Insel gibt es keine Fracht");
}

#[test]
fn standard_traegt_keine_fracht() {
    let data = test_data();
    let isl = crate::island::testutil::new_island(&data, "std", &[0, 1]);
    assert!(isl.cargo.cfg.is_none(), "standard steht nicht in cargo.json");
    assert!(
        isl.stages
            .iter()
            .all(|w| w.cargo.is_none() && w.cargo_bays.is_empty() && !w.cargo_hold)
    );
    let json = serde_json::to_string(&isl.stages[0]).expect("json");
    assert!(
        !json.contains("\"cargo\""),
        "der Zustand der Standard-Welt bleibt unverändert"
    );
}
