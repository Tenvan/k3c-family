//! Niederlage-Modi (Go: `island_defeat_test.go`, B-102, K2.2b, docs/rules/stufen.md § 4). Die sechs Go-Tests bauen eine
//! Insel (`defeat_island`) oder lassen die Burg fallen und warten auf RU14. Die Tests darunter (ohne Go-Gegenstück) prüfen
//! die Rechnung von `resource_losses` und `halve_stock`, die ohne `World`-Aufbau laufen.

use k3c_data::hub::Stock;

use super::defeat::{halve_stock, resource_losses};
use super::testutil::*;
use std::sync::Arc;

use super::{Island, step_island};
use crate::state::{Player, PlayerCommand, World, WorldValues};

/// Go: `castleFallen` (world.go) an der Stufe `stage` der Insel; kommt mit RU14.
fn castle_fallen(_isl: &mut Island, _stage: usize) {
    panic!("RU14: castle_fallen")
}

/// Go: `fell(p)` und der Kronenraub mit `leaveThroughPortal` (crown.go, enemies.go); kommt mit RU14/RU15.
fn crown_theft(_isl: &mut Island) {
    panic!("RU15: crown_theft")
}

/// Go: `DefaultOptionsFor(grade).Defeat` (island_options.go).
fn default_defeat_for(data: &k3c_data::Data, grade: &str) -> String {
    super::options::default_options_for(grade, &data.difficulty).defeat
}

#[test]
#[ignore = "wartet auf RU14"]
fn niederlage_gold_material() {
    let data = test_data();
    let mut isl = defeat_island(&data, "resources");
    let (sites, troops) = (built_sites(&isl.stages[0]), isl.stages[0].troops.len());
    isl.stages[0].castle.hp = 0.0;
    castle_fallen(&mut isl, 0);
    let (g0, g1) = (isl.stages[0].players[0].gold, isl.stages[1].players[0].gold);
    assert!(
        g0 == 20 && g1 == 15,
        "Gold {g0}/{g1}, erwartet 20/15 (alle Spieler der Insel −50 %)"
    );
    assert_eq!(
        isl.stock,
        Stock {
            wood: 10,
            stone: 5,
            ..Stock::default()
        },
        "Vorrat −50 %"
    );
    let w = &isl.stages[0];
    assert!(
        built_sites(w) == sites && w.troops.len() == troops && w.castle.hp == w.castle.max_hp && !isl.over,
        "Bauten, Truppen, Burg oder Over verändert"
    );
}

#[test]
#[ignore = "wartet auf RU14"]
fn niederlage_stufenverlust() {
    let data = test_data();
    let mut isl = defeat_island(&data, "stage");
    isl.stages[0].castle.hp = 0.0;
    castle_fallen(&mut isl, 0);
    let (g0, g1) = (isl.stages[0].players[0].gold, isl.stages[1].players[0].gold);
    assert!(
        g0 == 20 && g1 == 30,
        "Gold {g0}/{g1}, erwartet 20/30 (nur Spieler der Stufe)"
    );
    assert_eq!(
        isl.stock,
        Stock {
            wood: 10,
            stone: 5,
            ..Stock::default()
        },
        "Vorrat −50 %"
    );
    let w = &isl.stages[0];
    assert!(
        built_sites(w) == 0 && w.troops.len() == vagrants(w) && w.castle.hp == w.castle.max_hp && !isl.over,
        "Hub nicht zurückgesetzt: {} Bauten, {} Truppen, Burg {}, Over {}",
        built_sites(w),
        w.troops.len(),
        w.castle.hp,
        isl.over
    );
}

/// Kronenraub ist die Niederlage (kampf.md § 6): Game Over, die Insel steht danach.
#[test]
#[ignore = "wartet auf RU14"]
fn niederlage_kronenraub() {
    let data = test_data();
    let mut isl = defeat_island(&data, "stage");
    crown_theft(&mut isl);
    assert!(isl.over, "Over nach dem Kronenraub");
    let n = isl.stages[0]
        .events
        .iter()
        .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some("gameOver"))
        .count();
    assert_eq!(n, 1, "{n}× gameOver");
    step_island(&mut isl, &[], &data.difficulty, DT);
    let before = island_state(&isl);
    let moves = [
        PlayerCommand {
            move_x: 1.0,
            ..Default::default()
        },
        PlayerCommand {
            move_x: 1.0,
            pay: true,
            ..Default::default()
        },
    ]; // auch der Spieler in der Höhle bewegt sich nicht
    for _ in 0..90 {
        step_island(&mut isl, &moves, &data.difficulty, DT);
    }
    assert_eq!(island_state(&isl), before, "nach dem Game Over ändert sich der Zustand");
    for (i, s) in isl.stages.iter().enumerate() {
        assert!(
            s.events.is_empty(),
            "Stufe {i}: Ereignisse nach dem Game Over: {:?}",
            s.events
        );
    }
}

#[test]
#[ignore = "wartet auf RU14"]
fn niederlage_standard_je_grad() {
    let data = test_data();
    let want = [
        ("dev", "resources"),
        ("easy", "resources"),
        ("normal", "resources"),
        ("hard", "stage"),
        ("ultra", "stage"),
    ];
    for (g, w) in want {
        let got = default_defeat_for(&data, g);
        assert_eq!(got, w, "Grad {g}: Standard {got:?}, erwartet {w:?}");
    }
    let mut isl = defeat_island(&data, "stage");
    isl.options.grade = "ultra".into();
    isl.options.defeat = "stage".into(); // Überschreiben je Raum
    isl.stages[0].castle.hp = 0.0;
    castle_fallen(&mut isl, 0);
    assert!(!isl.over && built_sites(&isl.stages[0]) == 0, "Ultra mit Stufenverlust");
}

#[test]
#[ignore = "wartet auf RU14"]
fn niederlage_ohne_insel_stufenverlust() {
    let mut w = crate::enemies::testutil::quiet_world();
    crate::enemies::testutil::build_site(&mut w, "tower");
    w.troops.push(crate::state::Troop {
        kind: "archer".into(),
        x: w.hub_x,
        hp: 10.0,
        max_hp: 10.0,
        ..Default::default()
    });
    let pi = crate::enemies::testutil::add_player(&mut w);
    w.players[pi].gold = 40;
    w.castle.hp = 0.0;
    panic!("RU14: castleFallen(w) ohne Insel");
}

#[test]
#[ignore = "wartet auf RU14"]
fn niederlage_deterministisch() {
    let data = test_data();
    let mut runs = Vec::new();
    for _ in 0..2 {
        let mut isl = defeat_island(&data, "stage");
        for _ in 0..30 {
            step_island(&mut isl, &[], &data.difficulty, DT);
        }
        isl.stages[0].castle.hp = 0.0;
        for _ in 0..30 {
            step_island(&mut isl, &[], &data.difficulty, DT);
        }
        runs.push(island_state(&isl));
    }
    assert_eq!(runs[0], runs[1], "gleicher Seed, anderer Zustand");
}

// Ab hier: Tests ohne Go-Gegenstück, sie laufen schon.

fn island_mit_gold(gold: [i64; 2]) -> Island {
    let mut isl = Island {
        stages: vec![World::default(), World::default()],
        stock: Stock {
            wood: 20,
            stone: 10,
            ..Stock::default()
        },
        ..Island::default()
    };
    isl.stages[0].players.push(Player {
        gold: gold[0],
        ..Player::default()
    });
    isl.stages[1].players.push(Player {
        gold: gold[1],
        ..Player::default()
    });
    isl.stages[0].vals = Some(Arc::new(WorldValues::default())); // `stage_ref` liest den Zuschlag je Spieler daraus
    isl
}

#[test]
fn resource_losses_halbiert_gold_aller_spieler_und_den_vorrat() {
    let mut isl = island_mit_gold([40, 30]);
    resource_losses(&mut isl);
    assert_eq!((isl.stages[0].players[0].gold, isl.stages[1].players[0].gold), (20, 15));
    assert_eq!(
        isl.stock,
        Stock {
            wood: 10,
            stone: 5,
            ..Stock::default()
        }
    );
    assert!(!isl.over, "Bauten und Truppen bleiben, die Insel läuft weiter");
}

#[test]
fn halve_stock_rundet_zur_null_und_betrifft_jeden_posten() {
    let mut s = Stock {
        wood: 7,
        stone: 1,
        copper: 0,
        iron: 9,
        crystal: -3,
    };
    halve_stock(&mut s);
    assert_eq!(
        s,
        Stock {
            wood: 3,
            stone: 0,
            copper: 0,
            iron: 4,
            crystal: -1
        }
    );
}

#[test]
fn defeat_losses_im_modus_resources_nutzt_den_bezug_der_stufe() {
    let mut isl = island_mit_gold([40, 30]);
    isl.options.defeat = "resources".into();
    let mut r = isl.stage_ref(0, &Default::default());
    assert!(r.defeat_resources);
    std::mem::swap(&mut isl.stock, &mut isl.stages[0].stock);
    super::defeat::defeat_losses(&mut isl.stages[0], Some(&mut r));
    std::mem::swap(&mut isl.stock, &mut isl.stages[0].stock);
    isl.put_stage_ref(r);
    assert_eq!((isl.stages[0].players[0].gold, isl.stages[1].players[0].gold), (20, 15));
    assert_eq!(
        isl.stock,
        Stock {
            wood: 10,
            stone: 5,
            ..Stock::default()
        }
    );
}
