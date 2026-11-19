//! Abfahrt und Ankunft der Fracht (RU62.1, AC-01): Material, Truppen, Beutel; offene Träger-Aufträge gehen zurück.

use super::testutil::{add_troop, cargo_data, cargo_island, count, run};
use super::{CargoItem, CrateOrder, KIND_COINS, KIND_CRATE, KIND_TROOP};
use crate::island::{Island, next_island};
use k3c_data::Data;

fn item(kind: &str, id: &str, amount: i64, hp: f64) -> CargoItem {
    CargoItem {
        kind: kind.to_string(),
        id: id.to_string(),
        amount,
        hp,
    }
}

/// Fährt los: Endboss besiegt, Spieler am Steg, rechnet bis der Wechsel bereit ist.
fn depart(isl: &mut Island, data: &Data) {
    isl.endboss_defeated = true;
    for _ in 0..40 {
        run(isl, data, &[], 1.0);
        if isl.switch_ready {
            return;
        }
    }
}

#[test]
fn fracht_kommt_zusaetzlich_zum_startvorrat_an() {
    let data = cargo_data();
    let (_, mut isl, _) = cargo_island(&data, 1);
    isl.cargo.load.push(item(KIND_CRATE, "wood", 20, 0.0));
    isl.cargo.load.push(item(KIND_TROOP, "archer", 1, 7.0));
    isl.cargo.load.push(item(KIND_COINS, "", 10, 0.0));
    depart(&mut isl, &data);
    assert!(isl.switch_ready, "Abfahrt wie sonst, auch mit Fracht");
    let def = isl.next_def().expect("Ziel");
    let next = next_island(&data, &isl, &def).expect("neue Insel");
    let start = next.values().hub.island_start_stock.wood;
    assert_eq!(next.stock.wood, start + 20, "Material zusätzlich zum Startvorrat");
    let w = &next.stages[0];
    assert!(
        w.troops.iter().any(|t| t.kind == "archer" && t.hp == 7.0),
        "Truppe am Hafen"
    );
    assert_eq!(w.coins.len(), 10, "Beutel liegt als Münzen am Steg");
    assert!(next.cargo.load.is_empty(), "an Bord ist nichts mehr");
    assert!(next.cargo.cfg.is_some());
}

#[test]
fn ankunft_wird_einmal_gemeldet() {
    let data = cargo_data();
    let (_, mut isl, _) = cargo_island(&data, 1);
    isl.cargo.load.push(item(KIND_COINS, "", 10, 0.0));
    depart(&mut isl, &data);
    let def = isl.next_def().expect("Ziel");
    let mut next = next_island(&data, &isl, &def).expect("neue Insel");
    let ev = run(&mut next, &data, &[], 0.3);
    assert_eq!(count(&ev, "cargoArrived"), 1);
    let ev = run(&mut next, &data, &[], 0.3);
    assert_eq!(count(&ev, "cargoArrived"), 0);
}

#[test]
fn offener_traeger_auftrag_gibt_das_material_der_alten_insel_zurueck() {
    let data = cargo_data();
    let (_, mut isl, steg) = cargo_island(&data, 1);
    isl.stock.stone = 10;
    isl.sync_stock();
    isl.cargo.orders.push(CrateOrder {
        resource: "stone".into(),
        units: 20,
        carrier: 0,
        eta: 0.0,
    });
    add_troop(&mut isl, "peasant", steg + 400.0); // Träger weit weg: die Kiste erreicht den Steg nicht rechtzeitig
    depart(&mut isl, &data);
    assert!(isl.switch_ready);
    assert!(isl.cargo.orders.is_empty());
    assert_eq!(isl.stock.stone, 30, "Vorrat der alten Insel");
    let def = isl.next_def().expect("Ziel");
    let next = next_island(&data, &isl, &def).expect("neue Insel");
    assert!(next.cargo.load.is_empty() && next.cargo.orders.is_empty());
    assert_eq!(
        next.stock.stone,
        next.values().hub.island_start_stock.stone,
        "die Kiste fährt nicht mit"
    );
}

#[test]
fn abfahrt_ohne_fracht_und_ohne_laderaum_wie_heute() {
    let data = cargo_data();
    let (_, mut isl, _) = cargo_island(&data, 1);
    depart(&mut isl, &data);
    assert!(isl.switch_ready);
    let def = isl.next_def().expect("Ziel");
    let next = next_island(&data, &isl, &def).expect("neue Insel");
    assert_eq!(next.stock.wood, next.values().hub.island_start_stock.wood);
    assert!(next.stages[0].coins.is_empty());
}
