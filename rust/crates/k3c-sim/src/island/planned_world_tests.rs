//! Feste Objekte einer geplanten Insel (Go: `island_planned_world_test.go`, B-461/AC-07): Sie stehen an der geplanten Stelle
//! und wirken in der Sim wie gewürfelte, mit 2 Spielern. Alle vier Tests bauen eine Insel und rechnen sie
//! (`create_world_with`, `step`, RU14) und tragen `ignore`; die Marke nennt den letzten Sprint, der fehlt.
#![allow(clippy::unimplemented)] // Rümpfe der ignorierten Tests: warten auf den genannten Sprint

use std::collections::BTreeMap;

use k3c_level::types::{PlannedChunk, PlannedObject};

use super::testutil::*;
use super::{CreateOptions, Island, add_island_player, create_island, step_island};
use crate::pending::vagrants_at;
use crate::state::PlayerCommand;

/// Go: `fixedPlan` setzt im Wald je ein festes Objekt: Baum (Platz +2), Truhe (+1), Camp (−1, Chunk-Mitte), Portal (+6).
fn fixed_plan() -> BTreeMap<i64, Vec<PlannedChunk>> {
    let chunk = |slot: i64, kind: &str, objects: Vec<PlannedObject>| PlannedChunk {
        slot,
        kind: kind.into(),
        objects,
        ..PlannedChunk::default()
    };
    let object = |kind: &str, at: f64| PlannedObject { kind: kind.into(), at };
    BTreeMap::from([(
        0,
        vec![
            chunk(2, "forest", vec![object("tree", 10.0)]),
            chunk(1, "forest", vec![object("chest", 20.0)]),
            chunk(-1, "recruitCamp", vec![]),
            chunk(6, "portal", vec![]),
        ],
    )])
}

/// Go: `plannedIsland` baut die Insel „fest“ mit der festen Planung.
fn planned_island() -> (Island, k3c_data::Data) {
    let data = test_data();
    let opts = CreateOptions {
        planned: Some(fixed_plan()),
        ..CreateOptions::default()
    };
    let isl = create_island(&data, &standard_world(&data), "fest", &[0], ISLAND_SPEED, opts).expect("Insel");
    (isl, data)
}

/// Go: `hasX`: Eine Stelle in `xs` liegt bei `want` (Abstand unter 0,01).
fn has_x(xs: &[f64], want: f64) -> bool {
    xs.iter().any(|x| (x - want).abs() < 0.01)
}

/// Go: `hasNear`: Eine Stelle in `xs` liegt höchstens `tol` von `x` entfernt.
fn has_near(xs: &[f64], x: f64, tol: f64) -> bool {
    xs.iter().any(|v| (v - x).abs() <= tol)
}

/// Go: `DevSetPhase` (dev.go): setzt die Tageszeit der Insel.
fn dev_set_phase(isl: &mut Island, phase: &str) -> bool {
    crate::dev::dev_set_phase(isl, phase)
}

/// Go: `TestFesteObjekteStehenWieGeplant`: Feste Objekte stehen an der geplanten Stelle und wirken in der Sim wie
/// gewürfelte (der Landstreicher des Camps braucht `vagrants_at`, RU18).
#[test]
fn feste_objekte_stehen_wie_geplant() {
    let (isl, _) = planned_island();
    let w = &isl.stages[0];
    let trees: Vec<f64> = w.nodes.iter().filter(|n| n.kind == "tree").map(|n| n.x).collect();
    let camps: Vec<f64> = w.camps.iter().map(|c| c.x).collect();
    let chests: Vec<f64> = w.pickups.iter().filter(|p| p.kind == "chest").map(|p| p.x).collect();
    assert!(
        has_x(&trees, w.hub_x + 110.0)
            && has_x(&chests, w.hub_x + 70.0)
            && has_x(&camps, w.hub_x - 75.0)
            && has_x(&w.portals, w.hub_x + 325.0),
        "Baum {trees:?}, Truhe {chests:?}, Camp {camps:?}, Portale {:?}; Hub {}",
        w.portals,
        w.hub_x
    );
    assert!(
        vagrants_at(w, w.hub_x - 75.0) > 0,
        "das feste Camp liefert keine Landstreicher"
    );
}

/// Go: `TestFesteTruheIstAufhebbar`.
#[test]
fn feste_truhe_ist_aufhebbar() {
    let (mut isl, data) = planned_island();
    let p0 = add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0);
    let x = isl.stages[0].hub_x + 70.0;
    isl.stages[0].players[p0].x = x;
    step_island(
        &mut isl,
        &[PlayerCommand::default(), PlayerCommand::default()],
        &data.difficulty,
        DT,
    );
    assert_eq!(
        isl.chests_opened, 1,
        "geöffnete Truhen {}, erwartet 1",
        isl.chests_opened
    );
}

/// Go: `TestFesterBaumLaesstSichMarkieren`.
#[test]
fn fester_baum_laesst_sich_markieren() {
    let (mut isl, data) = planned_island();
    add_island_player(&mut isl, 0);
    let p1 = add_island_player(&mut isl, 0);
    let x = isl.stages[0].hub_x + 110.0;
    let cmds = [
        PlayerCommand::default(),
        PlayerCommand {
            pay: true,
            ..PlayerCommand::default()
        },
    ];
    let mut tm = 0.0;
    while tm < 3.0 {
        let p = &mut isl.stages[0].players[p1];
        (p.x, p.vx) = (x, 0.0);
        step_island(&mut isl, &cmds, &data.difficulty, DT);
        tm += DT;
    }
    let tree = isl.stages[0]
        .nodes
        .iter()
        .find(|n| n.kind == "tree" && (n.x - x).abs() < 0.01)
        .expect("fester Baum fehlt");
    assert!(tree.marked, "der feste Baum ist nach dem Bezahlen nicht markiert");
}

/// Go: `TestWelleKommtAusDemFestenPortal` (die Nacht setzt `dev_set_phase`, RU19).
#[test]
fn welle_kommt_aus_dem_festen_portal() {
    let (mut isl, data) = planned_island();
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0);
    assert!(dev_set_phase(&mut isl, "night"), "Nacht lässt sich nicht setzen");
    let cmds = [PlayerCommand::default(), PlayerCommand::default()];
    let mut tm = 0.0;
    while tm < 60.0 {
        step_island(&mut isl, &cmds, &data.difficulty, DT);
        let w = &isl.stages[0];
        if let Some(first) = w.enemies.first() {
            assert!(
                w.enemies.iter().any(|e| has_near(&w.portals, e.x, 3.0)),
                "erster Gegner bei {}, Portale {:?}",
                first.x,
                w.portals
            );
            return;
        }
        tm += DT;
    }
    panic!("keine Welle in 60 s");
}

// Ab hier: Test ohne Go-Gegenstück, er läuft schon.

#[test]
fn own_feste_planung_der_hilfe_ist_im_wald_gesetzt() {
    let plan = fixed_plan();
    assert_eq!(plan[&0].len(), 4);
    assert_eq!(plan[&0][0].objects[0].kind, "tree");
    assert!(has_x(&[1.0, 5.0], 5.004) && !has_x(&[1.0], 5.0));
    assert!(has_near(&[10.0], 12.0, 3.0) && !has_near(&[10.0], 14.0, 3.0));
}
