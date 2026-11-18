//! Tests zu `world/travel.rs` (Go: `world_travel.go` hat keinen eigenen Test; die Reise deckt `rules_test.go` mit `Step`,
//! das ab RU14.5 portiert wird). Hier eigene Tests (`own_`) auf einer Hand-Stufe ohne `create_world`.

use std::sync::Arc;

use k3c_level::types::Entity;

use super::testutil::{bare_world, built_site};
use super::travel::{TravelPoint, ready_at, step_travel, travel_points};
use crate::state::{Player, World};

const DT: f64 = 0.5;

/// Eine Hand-Stufe der Tiefe 0 mit Reise-Werten (Reichweite 2, Dauer 4 s) und einem Tiefen-Eingang bei 300.
fn travel_world() -> World {
    let mut w = bare_world();
    let mut v = (*w.vals.clone().expect("Wertesatz")).clone();
    v.hub.travel.range_units = 2.0;
    v.hub.travel.seconds = 4.0;
    w.vals = Some(Arc::new(v));
    w.biome.depth = 0;
    w.level.entities.push(Entity {
        kind: "exit".into(),
        x: 300.0,
        ..Entity::default()
    });
    w
}

fn add(w: &mut World, x: f64, free: bool) {
    w.players.push(Player {
        x,
        free,
        ..Player::default()
    });
}

fn pt(x: f64, to_depth: i64, via: &str) -> TravelPoint {
    TravelPoint {
        x,
        to_depth,
        via: via.into(),
    }
}

#[test]
fn own_travel_points_exit_fuehrt_nach_unten() {
    let w = travel_world();
    assert_eq!(travel_points(&w), vec![pt(300.0, 1, "exit")]);
}

#[test]
fn own_travel_points_ohne_tiefere_stufe_kein_exit() {
    let mut w = travel_world();
    w.biome.depth = 999;
    assert!(travel_points(&w).is_empty());
}

#[test]
fn own_travel_points_nur_gebaute_treppen() {
    let mut w = travel_world();
    w.level.entities.clear();
    w.biome.depth = 1;
    built_site(&mut w, "stairsDown", 100.0);
    built_site(&mut w, "stairsUp", 120.0);
    built_site(&mut w, "wall", 140.0);
    let open = built_site(&mut w, "stairsDown", 160.0);
    w.sites[open].state = "empty".into();
    assert_eq!(
        travel_points(&w),
        vec![pt(100.0, 2, "stairsDown"), pt(120.0, 0, "stairsUp")]
    );
}

#[test]
fn own_treppe_hinauf_gibt_es_nicht_ueber_der_oberwelt() {
    let mut w = travel_world();
    w.level.entities.clear();
    built_site(&mut w, "stairsUp", 100.0);
    assert!(travel_points(&w).is_empty());
}

#[test]
fn own_ready_at_braucht_entscheider_in_reichweite() {
    let mut w = travel_world();
    assert!(!ready_at(&w, &[], 300.0, 2.0));
    add(&mut w, 301.0, false);
    add(&mut w, 303.0, false);
    assert!(ready_at(&w, &[0], 300.0, 2.0));
    assert!(!ready_at(&w, &[0, 1], 300.0, 2.0));
}

#[test]
fn own_step_travel_fortschritt_waechst_und_endet_bei_eins() {
    let mut w = travel_world();
    add(&mut w, 300.0, false);
    step_travel(&mut w, DT);
    let t = w.travel.clone().expect("Reise");
    assert_eq!((t.x, t.to_depth, t.via.as_str()), (300.0, 1, "exit"));
    assert!((t.progress - 0.125).abs() < 1e-12);
    for _ in 0..20 {
        step_travel(&mut w, DT);
    }
    assert_eq!(w.travel.as_ref().map(|t| t.progress), Some(1.0));
}

#[test]
fn own_step_travel_ohne_spieler_keine_reise() {
    let mut w = travel_world();
    step_travel(&mut w, DT);
    assert!(w.travel.is_none());
}

#[test]
fn own_step_travel_verlassen_loescht_die_reise() {
    let mut w = travel_world();
    add(&mut w, 300.0, false);
    step_travel(&mut w, DT);
    assert!(w.travel.is_some());
    w.players[0].x = 250.0;
    step_travel(&mut w, DT);
    assert!(w.travel.is_none());
}

#[test]
fn own_step_travel_alle_entscheider_muessen_stehen() {
    let mut w = travel_world();
    add(&mut w, 300.0, false);
    add(&mut w, 250.0, false);
    step_travel(&mut w, DT);
    assert!(w.travel.is_none());
    w.players[1].x = 299.5;
    step_travel(&mut w, DT);
    assert!(w.travel.is_some());
}

#[test]
fn own_step_travel_freie_und_kronenlose_zaehlen_nicht() {
    let mut w = travel_world();
    add(&mut w, 300.0, false);
    add(&mut w, 10.0, true); // frei (B-059): reist mit, zählt nicht
    step_travel(&mut w, DT);
    assert!(w.travel.is_some());
    // ohne Krone (gefallen) kein Entscheider; bleibt nur dieser eine, gibt es keinen Wechsel
    w.players[0].crown = Some(Default::default());
    step_travel(&mut w, DT);
    assert!(w.travel.is_none());
}

#[test]
fn own_step_travel_anderer_punkt_beginnt_von_vorn() {
    let mut w = travel_world();
    w.level.entities.push(Entity {
        kind: "exit".into(),
        x: 400.0,
        ..Entity::default()
    });
    add(&mut w, 300.0, false);
    step_travel(&mut w, DT);
    step_travel(&mut w, DT);
    assert!((w.travel.as_ref().map_or(0.0, |t| t.progress) - 0.25).abs() < 1e-12);
    w.players[0].x = 400.0;
    step_travel(&mut w, DT);
    let t = w.travel.clone().expect("Reise");
    assert_eq!(t.x, 400.0);
    assert!((t.progress - 0.125).abs() < 1e-12);
}

#[test]
fn own_step_travel_no_travel_rechnet_nichts() {
    let mut w = travel_world();
    w.no_travel = true;
    add(&mut w, 300.0, false);
    step_travel(&mut w, DT);
    assert!(w.travel.is_none());
}
