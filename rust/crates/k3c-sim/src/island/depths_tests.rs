//! Fünf Stufen (Go: `island_depths_test.go`, W2.2, B-115/AC-03, Sprint W2 AC-03): Die Insel führt fünf Stufen; Eingang und
//! Treppen verbinden benachbarte. Beide Tests bauen eine Insel (`create_world_with`, `step`, `travel_points`) und warten auf
//! RU14.
use super::add_island_player;
use super::switch_testutil::built;
use super::testutil::*;
use crate::state::pending::travel_points;
use crate::state::{PlayerCommand, World};

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn insel_fuenf_stufen() {
    let data = test_data();
    let mut isl = must_island(&data, "fuenf", &[0, 1, 2, 3, 4]);
    assert_eq!(isl.stages.len(), 5, "{} Stufen", isl.stages.len());
    for (i, w) in isl.stages.iter_mut().enumerate() {
        want_stairs(w, i > 0, i < 4);
        let depth = i as i64;
        for pt in travel_points(w) {
            assert!(
                pt.to_depth == depth - 1 || pt.to_depth == depth + 1,
                "Tiefe {i}: Reisepunkt nach {}",
                pt.to_depth
            );
        }
        // Oberste: Eingang und Treppe runter; tiefste: nur Treppe hoch; dazwischen alle drei.
        let n = travel_points(w).len();
        let want = match i {
            0 => 2,
            4 => 1,
            _ => 3,
        };
        assert_eq!(n, want, "Tiefe {i}: {n} Reisepunkte");
    }
}

/// Go: `wantStairs` prüft die Treppen-Plätze und den Tiefen-Eingang einer Stufe und baut die Treppen.
fn want_stairs(w: &mut World, up: bool, down: bool) {
    let (mut has_up, mut has_down) = (false, false);
    for s in &mut w.sites {
        has_up |= s.kind == "stairsUp";
        has_down |= s.kind == "stairsDown";
        if s.kind == "stairsUp" || s.kind == "stairsDown" {
            built(s);
        }
    }
    let exits = w.level.entities.iter().filter(|e| e.kind == "exit").count();
    assert!(
        up == has_up && down == has_down && exits == 1,
        "Tiefe {}: stairsUp {has_up}, stairsDown {has_down}, {exits} Eingänge",
        w.biome.depth
    );
}

/// Ein Spieler geht über den Eingang von Tiefe 3 nach 4 und über die Treppe zurück.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn insel_wechsel_tiefe_3_und_4() {
    let data = test_data();
    let mut isl = must_island(&data, "tief", &[0, 1, 2, 3, 4]);
    let pi = add_island_player(&mut isl, 3);
    let index = isl.stages[3].players[pi].index;
    if let Some(e) = isl.stages[3].level.entities.iter().find(|e| e.kind == "exit") {
        let x = e.x;
        isl.stages[3].players[pi].x = x;
    }
    let cmds = [PlayerCommand::default()];
    let secs = data.hub.travel.seconds + 0.5;
    run_island(&mut isl, &data, &cmds, secs);
    assert_eq!(isl.stage_of(index), Some(4), "nicht in Tiefe 4");
    let Some(up) = isl.stages[4].sites.iter_mut().find(|s| s.kind == "stairsUp") else {
        panic!("Tiefe 4 hat keine Treppe hoch");
    };
    built(up);
    let up_x = up.x;
    isl.stages[4].players[0].x = up_x;
    run_island(&mut isl, &data, &cmds, secs);
    assert!(
        isl.stage_of(index) == Some(3) && isl.stages[3].players.len() == 1,
        "nicht zurück in Tiefe 3, Stufe {:?}",
        isl.stage_of(index)
    );
}
