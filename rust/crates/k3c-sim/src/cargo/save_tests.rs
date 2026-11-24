//! Spielstand der Fracht (RU62.1, AC-01): Speichern und Laden mitten im Beladen, Version 12 (RU62.10; die Fracht kam mit 10).

use super::testutil::{add_troop, cargo_data, cargo_island, run};
use super::{CargoItem, CrateOrder, KIND_COINS, KIND_TROOP};
use crate::island::testutil::{ISLAND_SPEED, test_data};
use crate::island::{ISLAND_SAVE_VERSION, Island, SaveTables, WorldDef, from_island_save_in, parse_island_save};
use crate::state::links::values_of;
use k3c_data::Data;

fn reload(data: &Data, world: &WorldDef, isl: &Island) -> Island {
    let json = serde_json::to_string(&isl.to_save("2026-11-19T00:00:00Z").expect("Spielstand")).expect("json");
    let defaults = values_of(world).expect("Wertesatz");
    let t = SaveTables {
        data,
        defaults: &defaults,
        standard_world: world,
    };
    let s = parse_island_save(&json, &t).expect("lesen");
    from_island_save_in(&t, &s, ISLAND_SPEED, world).expect("laden")
}

#[test]
fn spielstand_version_ist_10() {
    assert_eq!(ISLAND_SAVE_VERSION, 14);
}

#[test]
fn speichern_und_laden_mitten_im_beladen_erhaelt_fracht_und_auftraege() {
    let data = cargo_data();
    let (world, mut isl, steg) = cargo_island(&data, 1);
    isl.stock.wood = 5;
    isl.sync_stock();
    isl.cargo.load.push(CargoItem {
        kind: KIND_COINS.into(),
        amount: 10,
        ..CargoItem::default()
    });
    isl.cargo.load.push(CargoItem {
        kind: KIND_TROOP.into(),
        id: "warrior".into(),
        amount: 1,
        hp: 9.0,
    });
    isl.cargo.orders.push(CrateOrder {
        resource: "wood".into(),
        units: 20,
        carrier: 0,
        eta: 3.0,
    });
    add_troop(&mut isl, "peasant", steg + 12.0);
    let s = isl.to_save("x").expect("Spielstand");
    assert_eq!(s.version, 14);
    assert!(s.cargo.is_some());
    let mut loaded = reload(&data, &world, &isl);
    assert_eq!(loaded.cargo.load, isl.cargo.load);
    assert_eq!(loaded.cargo.orders.len(), 1);
    assert_eq!(
        (loaded.cargo.orders[0].resource.as_str(), loaded.cargo.orders[0].units),
        ("wood", 20)
    );
    assert_eq!(loaded.stock.wood, 5, "reserviertes Material bleibt reserviert");
    assert!(loaded.cargo.cfg.is_some());
    // nach dem Laden findet der Auftrag einen Träger (Bauern gehören zum Spielstand der Stufe) und kommt an
    let id = add_troop(&mut loaded, "peasant", steg + 12.0);
    run(&mut loaded, &data, &[], 10.0);
    assert!(loaded.cargo.orders.is_empty(), "Träger {id} hat die Kiste gebracht");
    assert_eq!(loaded.cargo.load.len(), 3);
}

#[test]
fn spielstand_ohne_fracht_hat_kein_cargo_feld() {
    let data = cargo_data();
    let (world, isl, _) = cargo_island(&data, 1);
    let json = serde_json::to_string(&isl.to_save("x").expect("Spielstand")).expect("json");
    assert!(!json.contains("\"cargo\""));
    let back = reload(&data, &world, &isl);
    assert!(back.cargo.load.is_empty() && back.cargo.cfg.is_some());
}

#[test]
fn fracht_in_einer_welt_ohne_fracht_wird_abgelehnt() {
    let data = cargo_data();
    let (world, mut isl, _) = cargo_island(&data, 1);
    isl.cargo.load.push(CargoItem {
        kind: KIND_COINS.into(),
        amount: 10,
        ..CargoItem::default()
    });
    let json = serde_json::to_string(&isl.to_save("x").expect("Spielstand")).expect("json");
    let plain = test_data(); // „test“ steht nicht unter den Fracht-Welten
    let defaults = values_of(&world).expect("Wertesatz");
    let t = SaveTables {
        data: &plain,
        defaults: &defaults,
        standard_world: &world,
    };
    let s = parse_island_save(&json, &t).expect("lesen");
    assert!(from_island_save_in(&t, &s, ISLAND_SPEED, &world).is_err());
}

#[test]
fn speichern_waehrend_eines_einwurfs_verliert_keine_muenzen() {
    use super::Bay;
    use super::testutil::{pay_cmds, stand_at};
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    stand_at(&mut isl, steg, 0, Bay::Coins);
    run(&mut isl, &data, &pay_cmds(1, &[0]), 1.0);
    assert!(isl.stages[0].players[0].gold < 30, "Einwurf läuft");
    let s = isl.to_save("x").expect("Spielstand");
    assert_eq!(
        s.players[0].gold, 30,
        "eingeworfene Münzen zählen beim Speichern zum Beutel"
    );
}
