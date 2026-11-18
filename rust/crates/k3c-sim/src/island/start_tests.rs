//! Tests H1 (Go: `island_start_test.go`, B-177): Startvorrat Holz einer neuen Insel. Drei der vier Go-Tests bauen eine Insel
//! und warten auf RU14; der Test des Lager-Maximums rechnet nur mit den Daten und läuft.

use k3c_data::hub::Stock;

use super::testutil::*;
use super::{add_island_player, step_island};
use crate::state::PlayerCommand;

#[test]
#[ignore = "wartet auf RU14"]
fn island_start_stock() {
    let data = test_data();
    let a = new_island(&data, "start-a", &[0, 1, 2]);
    assert_eq!(
        a.stock,
        Stock {
            wood: 100,
            ..Stock::default()
        },
        "Startvorrat, erwartet nur 100 Holz"
    );
    let b = new_island(&data, "start-a", &[0, 1, 2]);
    assert_eq!(a.stock, b.stock, "gleicher Seed, anderer Vorrat");
}

/// Braucht außerdem `to_save`/`from_island_save` (RU10.2).
#[test]
#[ignore = "wartet auf RU14"]
fn island_start_stock_not_added_on_load() {
    let data = test_data();
    let mut isl = new_island(&data, "start-b", &[0, 1]);
    isl.stock = Stock {
        wood: 7,
        stone: 3,
        ..Stock::default()
    };
    panic!("RU10.2: from_island_save(isl.to_save(..), ISLAND_SPEED), Vorrat 7 Holz und 3 Stein ohne Startvorrat");
}

#[test]
fn island_start_stock_within_storage_limit() {
    let data = test_data();
    let s = &data.hub.island_start_stock;
    let max = data.economy.storage.base_per_hub;
    for (name, v) in [
        ("wood", s.wood),
        ("stone", s.stone),
        ("copper", s.copper),
        ("iron", s.iron),
        ("crystal", s.crystal),
    ] {
        assert!(
            (0..=max).contains(&v),
            "islandStartStock.{name} = {v}, erlaubt 0 bis {max} (Lager-Maximum je Hub)"
        );
    }
}

/// Beide Mauern und ein Turm sind ohne vorheriges Holzfällen bezahlbar: Gold ist gezahlt, das Material liegt im Vorrat.
#[test]
#[ignore = "wartet auf RU14"]
fn island_start_builds_walls_and_tower_without_chopping() {
    let data = test_data();
    let mut isl = new_island(&data, "start-c", &[0, 1, 2]);
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0);
    let (mut walls, mut towers) = (0, 0);
    let mut want = Vec::new();
    for (i, s) in isl.stages[0].sites.iter().enumerate() {
        match s.kind.as_str() {
            "wall" if walls < 2 => {
                walls += 1;
                want.push(i);
            }
            "tower" if towers < 1 => {
                towers += 1;
                want.push(i);
            }
            _ => {}
        }
    }
    assert_eq!(
        want.len(),
        3,
        "Bauplätze gefunden: {}, erwartet 2 Mauern und 1 Turm",
        want.len()
    );
    let mut cost = 0;
    for &i in &want {
        let b = &data.buildings[&isl.stages[0].sites[i].kind];
        let s = &mut isl.stages[0].sites[i];
        s.paid_gold = b.cost.gold;
        s.state = "waitingMaterial".into();
        cost += b.cost.wood;
    }
    step_island(
        &mut isl,
        &vec![PlayerCommand::default(); 2],
        &data.difficulty,
        1.0 / 30.0,
    );
    for &i in &want {
        let s = &isl.stages[0].sites[i];
        assert_eq!(
            s.state, "waitingWorker",
            "{}: Zustand {}, erwartet waitingWorker (Material bezahlt)",
            s.kind, s.state
        );
    }
    assert_eq!(isl.stock.wood, 100 - cost, "Holz");
}
