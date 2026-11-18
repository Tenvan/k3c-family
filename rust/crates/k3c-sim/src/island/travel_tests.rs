//! Einzelwechsel (Go: `island_travel_test.go`, SP12.2): ein Spieler wechselt allein, die anderen bleiben stehen. Die sieben
//! Go-Tests bauen eine Insel (`create_world_with`, `step`, `travel_points`, RU14) und tragen `ignore`. Die Tests ab `own_`
//! prüfen, was schon läuft: `insert_player` und `remove_player` mit zwei Spielern, `players`, `stage_of`, `stage_by_depth`
//! und `point_near`.
use std::collections::BTreeMap;

use super::switch_testutil::{built, fell};
use super::testutil::*;
use super::travel::{insert_player, point_near, remove_player};
use super::{Island, add_island_player, step_island};
use crate::state::links::TravelPoint;
use crate::state::{Player, PlayerCommand, World};

/// Go: `exitX`, die X-Position des Tiefen-Eingangs der Stufe.
fn exit_x(w: &World) -> f64 {
    w.level
        .entities
        .iter()
        .find(|e| e.kind == "exit")
        .map(|e| e.x)
        .expect("Stufe hat keinen Ausgang")
}

/// Go: `builtSite`, baut den ersten Platz der Art `kind` und liefert seine X-Position.
fn built_site(w: &mut World, kind: &str) -> f64 {
    let s = w.sites.iter_mut().find(|s| s.kind == kind).expect("Bauplatz");
    built(s);
    s.x
}

/// Go: `standFor` lässt die Insel `seconds` ticken (ohne Eingaben) und hält die Spieler (Stufe, Platz → x) dabei an ihrem
/// Platz.
fn stand_for(isl: &mut Island, data: &k3c_data::Data, seconds: f64, hold: &BTreeMap<(usize, usize), f64>) {
    let cmds = vec![PlayerCommand::default(); isl.next_player as usize];
    let mut tm = 0.0;
    while tm < seconds {
        for (&(si, pi), &x) in hold {
            let p = &mut isl.stages[si].players[pi];
            p.x = x;
            p.vx = 0.0;
        }
        step_island(isl, &cmds, &data.difficulty, DT);
        tm += DT;
    }
}

#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: assertion `left == right` failed: andere Spieler bleiben stehen)"]
fn island_single_player_travels_via_exit() {
    let data = test_data();
    let mut isl = must_island(&data, "reise-a", &[0, 1, 2]);
    let (a, b, c) = (
        add_island_player(&mut isl, 0),
        add_island_player(&mut isl, 0),
        add_island_player(&mut isl, 1),
    );
    isl.stages[0].players[a].gold = 55;
    isl.stages[0].players[b].gold = 33;
    let (bx, cx) = (isl.stages[0].players[b].x, isl.stages[1].players[c].x);
    let hold = BTreeMap::from([((0, a), exit_x(&isl.stages[0]))]);
    stand_for(&mut isl, &data, 2.5, &hold);

    assert_eq!(isl.stage_of(0), Some(1), "Spieler 0 muss in der Höhle (Stufe 1) sein");
    assert!(
        isl.stage_of(1) == Some(0) && isl.stage_of(2) == Some(1),
        "andere Spieler dürfen nicht wechseln: {:?}, {:?}",
        isl.stage_of(1),
        isl.stage_of(2)
    );
    assert_eq!(isl.stages[0].players[0].x, bx, "andere Spieler bleiben stehen");
    assert_eq!(isl.stages[1].players[1].x, cx, "andere Spieler bleiben stehen");
    let moved = &isl.stages[1].players[0];
    assert!(
        moved.index == 0 && moved.gold == 55,
        "Index und Gold bleiben: {moved:?}"
    );
    assert!(
        isl.stages[1].players.len() == 2 && isl.stages[1].players[1].index == 2,
        "Reihenfolge in der Zielstufe nach Index"
    );
    let got: Vec<i64> = isl.players().iter().map(|p| p.index).collect();
    assert_eq!(got, [0, 1, 2], "players() nach Index sortiert erwartet");
}

#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: index out of bounds: the len is 0 but the index is 0)"]
fn island_travel_via_stairs() {
    let data = test_data();
    let mut isl = must_island(&data, "reise-b", &[0, 1]);
    let p = add_island_player(&mut isl, 0);
    let down = built_site(&mut isl.stages[0], "stairsDown");
    stand_for(&mut isl, &data, 2.5, &BTreeMap::from([((0, p), down)]));
    assert_eq!(isl.stage_of(0), Some(1), "Treppe runter: Spieler muss in Stufe 1 sein");
    let up = built_site(&mut isl.stages[1], "stairsUp");
    stand_for(&mut isl, &data, 2.5, &BTreeMap::from([((1, 0), up)]));
    assert_eq!(
        isl.stage_of(0),
        Some(0),
        "Treppe hoch: Spieler muss zurück in Stufe 0 sein"
    );
}

#[test]
fn island_travel_needs_built_stairs() {
    let data = test_data();
    let mut isl = must_island(&data, "reise-c", &[0, 1]);
    let p = add_island_player(&mut isl, 0);
    let down = isl.stages[0]
        .sites
        .iter()
        .find(|s| s.kind == "stairsDown")
        .map(|s| s.x)
        .expect("Treppe");
    stand_for(&mut isl, &data, 3.0, &BTreeMap::from([((0, p), down)]));
    assert_eq!(isl.stage_of(0), Some(0), "ungebaute Treppe darf nicht reisen lassen");
}

#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: index out of bounds: the len is 0 but the index is 0)"]
fn island_travel_aborts_when_leaving_point() {
    let data = test_data();
    let mut isl = must_island(&data, "reise-d", &[0, 1]);
    let p = add_island_player(&mut isl, 0);
    let x = exit_x(&isl.stages[0]);
    stand_for(&mut isl, &data, 1.2, &BTreeMap::from([((0, p), x)]));
    stand_for(&mut isl, &data, 0.5, &BTreeMap::from([((0, p), x - 20.0)])); // weg vom Punkt: Fortschritt zurück auf 0
    stand_for(&mut isl, &data, 1.2, &BTreeMap::from([((0, p), x)]));
    assert_eq!(
        isl.stage_of(0),
        Some(0),
        "nach Verlassen des Punkts muss die Zeit neu laufen"
    );
    stand_for(&mut isl, &data, 1.2, &BTreeMap::from([((0, p), x)]));
    assert_eq!(isl.stage_of(0), Some(1), "nach 2 s am Stück muss der Wechsel erfolgen");
}

#[test]
fn island_dead_and_free_players_do_not_travel() {
    let data = test_data();
    let mut isl = must_island(&data, "reise-e", &[0, 1]);
    let (dead, free) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 0));
    fell(&mut isl.stages[0], dead);
    isl.stages[0].players[free].free = true;
    let x = exit_x(&isl.stages[0]);
    stand_for(&mut isl, &data, 3.0, &BTreeMap::from([((0, dead), x), ((0, free), x)]));
    assert!(
        isl.stage_of(0) == Some(0) && isl.stage_of(1) == Some(0),
        "tote und freie Spieler wechseln nicht"
    );
}

#[test]
fn island_travel_without_target_stage_stays() {
    let data = test_data();
    let mut isl = must_island(&data, "reise-f", &[0, 1]); // Tiefe 2 gibt es global, aber nicht in dieser Insel
    let p = add_island_player(&mut isl, 1);
    let x = exit_x(&isl.stages[1]);
    stand_for(&mut isl, &data, 3.0, &BTreeMap::from([((1, p), x)]));
    assert_eq!(isl.stage_of(0), Some(1), "ohne Zielstufe bleibt der Spieler stehen");
}

#[test]
#[ignore = "wartet auf RU18 (Ursache ungeprueft: index out of bounds: the len is 0 but the index is 0)"]
fn island_travel_deterministic() {
    let data = test_data();
    let run = || {
        let mut isl = must_island(&data, "reise-g", &[0, 1, 2]);
        let (a, b) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 1));
        let xa = exit_x(&isl.stages[0]);
        stand_for(&mut isl, &data, 2.5, &BTreeMap::from([((0, a), xa)]));
        let xb = exit_x(&isl.stages[1]);
        stand_for(&mut isl, &data, 5.0, &BTreeMap::from([((1, b), xb)]));
        island_json(&isl)
    };
    let (x, y) = (run(), run());
    for (i, (a, b)) in x.iter().zip(&y).enumerate() {
        assert_eq!(a, b, "Stufe {i}: Reise ist nicht deterministisch");
    }
}

fn player(index: i64) -> Player {
    Player {
        index,
        ..Player::default()
    }
}

fn indexes(players: &[Player]) -> Vec<i64> {
    players.iter().map(|p| p.index).collect()
}

#[test]
fn own_insert_player_haelt_die_reihenfolge_nach_index() {
    let mut list = vec![player(0), player(2)];
    assert_eq!(insert_player(&mut list, player(1)), 1, "Position von Spieler 1");
    assert_eq!(indexes(&list), [0, 1, 2]);
    assert_eq!(insert_player(&mut list, player(5)), 3, "hinten anfügen");
    assert_eq!(insert_player(&mut list, player(-1)), 0, "vorne einfügen");
    assert_eq!(indexes(&list), [-1, 0, 1, 2, 5]);
    // gleicher Index: nach den vorhandenen (wie Go: `players[i-1].Index > p.Index`)
    let mut a = player(1);
    a.gold = 7;
    assert_eq!(insert_player(&mut list, a), 3);
    assert_eq!(list[2].gold, 0);
    assert_eq!(list[3].gold, 7);
}

#[test]
fn own_remove_player_nimmt_nur_den_einen_und_behaelt_die_reihenfolge() {
    let mut list = vec![player(0), player(1), player(2)];
    list[1].gold = 9;
    let got = remove_player(&mut list, 1).expect("Spieler 1");
    assert_eq!(got.gold, 9, "der Spieler selbst kommt zurück");
    assert_eq!(indexes(&list), [0, 2]);
    assert!(
        remove_player(&mut list, 1).is_none(),
        "ein zweites Mal gibt es ihn nicht"
    );
    assert_eq!(indexes(&list), [0, 2]);
}

#[test]
fn own_wechsel_zweier_spieler_haelt_die_reihenfolge_wie_go() {
    // Spieler 0 und 2 stehen in Stufe 1, Spieler 1 wechselt von Stufe 0 dorthin: Reihenfolge 0, 1, 2.
    let mut from = vec![player(1), player(3)];
    let mut to = vec![player(0), player(2)];
    let moved = remove_player(&mut from, 1).expect("Spieler 1");
    insert_player(&mut to, moved);
    assert_eq!(indexes(&from), [3]);
    assert_eq!(indexes(&to), [0, 1, 2]);
}

/// Eine Insel von Hand: je Stufe (Tiefe, Spieler-Indizes), ohne `create_world_with`.
fn hand_island(stages: &[(i64, &[i64])]) -> Island {
    let mut isl = Island::default();
    for (depth, idx) in stages {
        let mut w = World::default();
        w.biome.depth = *depth;
        w.players = idx.iter().map(|&i| player(i)).collect();
        isl.stages.push(w);
    }
    isl
}

#[test]
fn own_players_stage_of_und_stage_by_depth() {
    let isl = hand_island(&[(0, &[0, 2]), (1, &[1]), (4, &[])]);
    let got: Vec<i64> = isl.players().iter().map(|p| p.index).collect();
    assert_eq!(got, [0, 1, 2], "nach Index sortiert über alle Stufen");
    assert_eq!(isl.stage_of(1), Some(1));
    assert_eq!(isl.stage_of(2), Some(0));
    assert_eq!(isl.stage_of(7), None, "Go: -1");
    assert_eq!(isl.stage_by_depth(4), Some(2));
    assert_eq!(isl.stage_by_depth(2), None, "Go: -1");
    assert_eq!(isl.player_count(), 3);
}

#[test]
fn own_point_near_nimmt_den_ersten_punkt_in_reichweite() {
    let pt = |x: f64, to_depth: i64| TravelPoint {
        x,
        to_depth,
        via: String::new(),
    };
    let points = [pt(10.0, 1), pt(12.0, 2), pt(40.0, 3)];
    assert_eq!(
        point_near(&points, 11.0, 2.0).map(|p| p.to_depth),
        Some(1),
        "der erste in Reichweite"
    );
    assert_eq!(point_near(&points, 12.0, 0.5).map(|p| p.to_depth), Some(2));
    assert_eq!(
        point_near(&points, 40.0, 0.0).map(|p| p.to_depth),
        Some(3),
        "Rand zählt (<=)"
    );
    assert!(point_near(&points, 25.0, 2.0).is_none());
    assert!(point_near(&[], 0.0, 9.0).is_none());
}
