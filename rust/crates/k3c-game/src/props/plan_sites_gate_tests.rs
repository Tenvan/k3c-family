//! Tests zum Reittier-Gatter (RU62.9, B-1000): Gatter mit Tier und Preis, leer wenn alle Monarchen das Tier haben.

use k3c_sim::state::mount_view::MountGate;
use k3c_sim::state::{Player, World};

use super::plan::{Planner, Shape};
use super::plan_sites_gate::gate_open;
use super::testutil::{StubSheets, camera};

fn gate() -> MountGate {
    MountGate {
        x: 20.0,
        range: 2.0,
        price: 30,
        mount: "camel".to_string(),
        sprite: "camel".to_string(),
    }
}

fn player(mount: &str) -> Player {
    Player {
        mount: mount.to_string(),
        ..Default::default()
    }
}

fn world(mounts: &[&str]) -> World {
    World {
        mount_gate: Some(gate()),
        players: mounts.iter().map(|m| player(m)).collect(),
        ..Default::default()
    }
}

fn plan(w: &World) -> Planner {
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.mount_gate(w, &camera(20.0));
    p
}

fn price(p: &Planner) -> Option<String> {
    p.top
        .iter()
        .find(|it| it.shape == Shape::Text)
        .map(|it| it.text.clone())
}

#[test]
fn ohne_gatter_im_zustand_wird_nichts_gezeichnet() {
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.mount_gate(&World::default(), &camera(20.0));
    assert!(p.body.is_empty() && p.top.is_empty());
}

#[test]
fn fehlt_einem_monarchen_das_tier_steht_es_mit_preis_am_gatter() {
    for mounts in [vec![""], vec!["camel", ""], vec!["sengokuHorse", "camel"]] {
        let p = plan(&world(&mounts));
        assert_eq!(price(&p).as_deref(), Some("30"), "Preis bei {mounts:?}");
        assert!(p.body.len() >= 3, "Gatter und Tier bei {mounts:?}");
    }
}

#[test]
fn haben_alle_das_tier_bleibt_das_gatter_leer() {
    let p = plan(&world(&["camel", "camel"]));
    assert_eq!(price(&p), None);
    assert_eq!(p.body.len(), 1, "nur das Gatter");
    assert!(!gate_open(&world(&["camel"]), "camel"));
}

#[test]
fn ausserhalb_des_bilds_wird_nichts_gezeichnet() {
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.mount_gate(&world(&[""]), &camera(200.0));
    assert!(p.body.is_empty());
}
