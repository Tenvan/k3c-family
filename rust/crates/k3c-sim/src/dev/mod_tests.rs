//! Go: `dev_test.go` (B-178, B-232, B-470): Dev-Gold, Dev-Material, Dev-Welle und Dev-Tageszeit.

use super::testutil::{coins_at, island_with_player};
use super::{dev_add_stock, dev_drop_gold, dev_set_phase, dev_start_wave};
use crate::island::step_island;
use crate::island::storage::capacity;
use crate::state::pending::values;

const DT: f64 = 1.0 / 30.0;

#[test]
fn dev_gold_faellt_und_wird_bis_zum_beutel_maximum_aufgehoben() {
    let (data, mut isl, index) = island_with_player("dev-gold");
    let max = values(&isl.stages[0]).economy.purse.max_gold;
    isl.stages[0].players[0].gold = max - 20;
    let before = isl.stages[0].coins.len();
    assert!(dev_drop_gold(&mut isl, index, 50), "Spieler nicht gefunden");
    let x = isl.stages[0].players[0].x;
    assert!(isl.stages[0].coins.len() - before == 50 && coins_at(&isl.stages[0], x) >= 50);
    step_island(&mut isl, &[], &data.difficulty, DT);
    assert_eq!(isl.stages[0].players[0].gold, max);
    assert_eq!(coins_at(&isl.stages[0], x), 30);
    assert!(!dev_drop_gold(&mut isl, 7, 1), "unbekannter Spieler angenommen");
}

#[test]
fn dev_material_fuellt_den_vorrat_bis_zum_lager_maximum() {
    let (data, mut isl, index) = island_with_player("dev-stock");
    let d = &data.difficulty;
    assert_eq!(dev_add_stock(&mut isl, d, index, "wood", 100), (100, true));
    assert_eq!(isl.stock.wood, 100);
    let r = isl.stage_ref(0, d);
    let limit = capacity(&isl.stages[0], &values(&isl.stages[0]), Some(&r)).expect("Lager-Maximum");
    isl.put_stage_ref(r);
    assert_eq!(dev_add_stock(&mut isl, d, index, "wood", 1000), (limit - 100, true));
    assert_eq!(isl.stock.wood, limit);
    assert!(
        !dev_add_stock(&mut isl, d, index, "gold", 1).1,
        "gold ist kein Baumaterial"
    );
    assert!(
        !dev_add_stock(&mut isl, d, 7, "wood", 1).1,
        "unbekannter Spieler angenommen"
    );
}

#[test]
fn dev_welle_startet_sofort() {
    let (data, mut isl, index) = island_with_player("dev-welle");
    assert!(dev_start_wave(&mut isl, &data.difficulty, index));
    let w = &isl.stages[0];
    assert!(
        w.wave == 1 && !w.spawn_queue.is_empty(),
        "Welle {}, {} Gegner geplant",
        w.wave,
        w.spawn_queue.len()
    );
    assert!(
        !dev_start_wave(&mut isl, &data.difficulty, 7),
        "unbekannter Spieler angenommen"
    );
}

#[test]
fn dev_tageszeit_springt_in_jede_phase() {
    let (data, mut isl, _) = island_with_player("dev-phase");
    step_island(&mut isl, &[], &data.difficulty, DT);
    let cases = [
        ("night", 1),
        ("night", 2),
        ("dusk", 3),
        ("day", 4),
        ("dawn", 5),
        ("day", 5),
        ("dawn", 6),
    ];
    for (phase, day) in cases {
        let wave = isl.stages[0].wave;
        assert!(dev_set_phase(&mut isl, phase), "{phase} abgelehnt");
        step_island(&mut isl, &[], &data.difficulty, DT);
        let w = &isl.stages[0];
        assert!(
            w.cycle.phase == phase && w.cycle.day == day,
            "{phase}: Phase {} Tag {}",
            w.cycle.phase,
            w.cycle.day
        );
        if phase == "night" {
            assert_eq!(w.wave, wave + 1, "Nacht ohne Welle");
        }
        let dawn = w.events.iter().any(|e| e["type"] == "dawn");
        assert_eq!(dawn, phase == "dawn", "{phase}: Ereignis dawn {dawn}");
    }
    assert!(!dev_set_phase(&mut isl, "mittag"), "unbekannte Phase angenommen");
}
