//! Laderaum, Stufenverlust und Determinismus der Fracht (RU62.1, AC-01).

use super::testutil::{add_troop, cargo_data, cargo_island, count, pay_cmds, run, stand_at};
use super::{Bay, CargoItem, CrateOrder, KIND_COINS, KIND_CRATE, KIND_TROOP};
use crate::island::Island;

fn item(kind: &str, id: &str, amount: i64) -> CargoItem {
    CargoItem {
        kind: kind.to_string(),
        id: id.to_string(),
        amount,
        hp: if kind == KIND_TROOP { 7.0 } else { 0.0 },
    }
}

fn set_ship_level(isl: &mut Island, level: i64) {
    for s in &mut isl.stages[0].sites {
        if s.kind == "ship" {
            s.level = level;
        }
    }
}

#[test]
fn laderaum_vergroessert_die_plaetze() {
    let data = cargo_data();
    let (_, mut isl, _) = cargo_island(&data, 1);
    run(&mut isl, &data, &[], 0.2);
    assert_eq!(
        isl.stages[0].cargo.as_ref().map(|c| (c.slots, c.hold)),
        Some((6, false))
    );
    set_ship_level(&mut isl, 4);
    run(&mut isl, &data, &[], 0.2);
    assert_eq!(
        isl.stages[0].cargo.as_ref().map(|c| (c.slots, c.hold)),
        Some((10, true))
    );
}

#[test]
fn laderaum_ist_vierte_schiffsstufe_und_haelt_die_abfahrt_nicht_auf() {
    let data = cargo_data();
    let (_, mut isl, _) = cargo_island(&data, 1);
    let v = crate::world::vals(&isl.stages[0]);
    let levels = &v.buildings["ship"].levels;
    assert_eq!(levels.len(), 4, "Kiel, Rumpf, Segel, Laderaum");
    assert_eq!((levels[3].cost.gold, levels[3].cost.wood), (20, 60));
    let w = &isl.stages[0];
    let s = w.sites.iter().find(|s| s.kind == "ship").expect("Schiff");
    assert!(crate::world::ship::ship_complete(w, s), "ohne Laderaum fertig");
    assert!(
        crate::world::ship::ship_next_cost(w, None, s).is_some_and(|c| c.gold == 20),
        "Laderaum als nächster Abschnitt"
    );
    // Ausbau zum Laderaum läuft: Abfahrt bleibt möglich
    for s in &mut isl.stages[0].sites {
        if s.kind == "ship" {
            s.upgrade = "waitingWorker".into();
        }
    }
    let w = &isl.stages[0];
    let s = w.sites.iter().find(|s| s.kind == "ship").expect("Schiff");
    assert!(crate::world::ship::ship_complete(w, s));
}

#[test]
fn verlust_des_laderaums_gibt_plaetze_in_umgekehrter_reihenfolge_zurueck() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    set_ship_level(&mut isl, 4);
    let l = &mut isl.cargo.load;
    l.push(item(KIND_COINS, "", 10)); // 0
    l.push(item(KIND_COINS, "", 10)); // 1
    l.push(item(KIND_COINS, "", 10)); // 2
    l.push(item(KIND_COINS, "", 10)); // 3
    l.push(item(KIND_COINS, "", 10)); // 4
    l.push(item(KIND_COINS, "", 10)); // 5
    l.push(item(KIND_CRATE, "wood", 20)); // 6
    l.push(item(KIND_TROOP, "archer", 1)); // 7
    l.push(item(KIND_COINS, "", 10)); // 8
    l.push(item(KIND_CRATE, "stone", 20)); // 9
    run(&mut isl, &data, &[], 0.2);
    assert_eq!(isl.cargo.load.len(), 10);
    set_ship_level(&mut isl, 3); // Abschnitt verloren
    let ev = run(&mut isl, &data, &[], 0.2);
    assert_eq!(isl.cargo.load.len(), 6);
    assert_eq!(count(&ev, "cargoReturned"), 4);
    assert_eq!((isl.stock.stone, isl.stock.wood), (20, 20), "Material in den Vorrat");
    assert!(
        isl.stages[0]
            .troops
            .iter()
            .any(|t| t.kind == "archer" && t.hp == 7.0 && (t.x - steg).abs() < 5.0),
        "Truppe an Land"
    );
    assert_eq!(isl.stages[0].coins.len(), 10, "Münzen am Steg");
}

#[test]
fn verlust_des_laderaums_storniert_zuerst_offene_auftraege() {
    let data = cargo_data();
    let (_, mut isl, _) = cargo_island(&data, 1);
    set_ship_level(&mut isl, 4);
    for _ in 0..6 {
        isl.cargo.load.push(item(KIND_COINS, "", 10));
    }
    isl.cargo.orders.push(CrateOrder {
        resource: "copper".into(),
        units: 20,
        carrier: 0,
        eta: 0.0,
    });
    run(&mut isl, &data, &[], 0.2);
    set_ship_level(&mut isl, 3);
    run(&mut isl, &data, &[], 0.2);
    assert!(isl.cargo.orders.is_empty());
    assert_eq!(isl.stock.copper, 20);
    assert_eq!(isl.cargo.load.len(), 6);
}

#[test]
fn traeger_faellt_aus_ein_neuer_uebernimmt() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    isl.cargo.orders.push(CrateOrder {
        resource: "wood".into(),
        units: 20,
        carrier: 0,
        eta: 0.0,
    });
    let first = add_troop(&mut isl, "peasant", steg + 40.0);
    run(&mut isl, &data, &[], 0.2);
    assert_eq!(isl.cargo.orders[0].carrier, first);
    isl.stages[0].troops.retain(|t| t.id != first);
    let second = add_troop(&mut isl, "peasant", steg + 50.0);
    run(&mut isl, &data, &[], 0.2);
    assert_eq!(isl.cargo.orders[0].carrier, second);
}

#[test]
fn fracht_ist_deterministisch() {
    let run_once = || {
        let data = cargo_data();
        let (_, mut isl, steg) = cargo_island(&data, 2);
        isl.stock.wood = 100;
        isl.sync_stock();
        add_troop(&mut isl, "archer", steg + 4.0);
        add_troop(&mut isl, "peasant", steg + 20.0);
        stand_at(&mut isl, steg, 0, Bay::Coins);
        stand_at(&mut isl, steg, 1, Bay::Crates);
        run(&mut isl, &data, &pay_cmds(2, &[0, 1]), 12.0);
        (
            serde_json::to_string(&isl.stages[0]).expect("json"),
            isl.cargo.load.clone(),
            isl.stock.clone(),
        )
    };
    let (a, b) = (run_once(), run_once());
    assert_eq!(a, b);
    assert!(a.1.len() >= 2, "Beutel und Kiste");
}
