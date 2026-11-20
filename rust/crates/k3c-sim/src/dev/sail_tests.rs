//! RU36.3 (B-1157): Dev-Aktion „Abfahrbereit“: Schiff fertig, Endboss besiegt, Wechselpunkt öffnet.

use super::dev_ready_to_sail;
use crate::island::switch_testutil::{step_seconds, test_islands};
use crate::island::testutil::{count, test_data};
use crate::island::{CreateOptions, add_island_player, create_island_in};
use crate::world::ship::{harbor, ship_complete};

#[test]
fn abfahrbereit_baut_schiff_und_oeffnet_den_wechselpunkt() {
    let data = test_data();
    let (w, _) = test_islands(&data);
    let mut isl = create_island_in(&data, &w, "sail", &[0, 1], 1.0, CreateOptions::default()).expect("Insel");
    let pi = add_island_player(&mut isl, 0);
    let (wi, si) = harbor(&isl).expect("Hafen");
    assert!(
        !ship_complete(&isl.stages[wi], &isl.stages[wi].sites[si]),
        "Schiff schon fertig"
    );
    assert!(dev_ready_to_sail(&mut isl));
    assert!(isl.endboss_defeated);
    assert!(
        ship_complete(&isl.stages[wi], &isl.stages[wi].sites[si]),
        "Schiff nicht fertig"
    );
    isl.stages[wi].players[pi].x = isl.stages[wi].sites[si].x;
    let ev = step_seconds(&mut isl, &data, 3.0);
    assert_eq!(count(&ev, "islandGateOpen"), 1, "Wechselpunkt nicht offen");
}

#[test]
fn abfahrbereit_lehnt_die_letzte_insel_ab() {
    let data = test_data();
    let w = crate::island::testutil::standard_world(&data);
    let mut isl = create_island_in(&data, &w, "letzte", &[0, 1], 1.0, CreateOptions::default()).expect("Insel");
    assert!(!dev_ready_to_sail(&mut isl), "letzte Insel hat keine Abfahrt");
    assert!(!isl.endboss_defeated);
}
