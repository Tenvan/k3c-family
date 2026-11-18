//! Tests zu `mod.rs`, `jobs.rs` und `flee.rs` (Go: `unit_common.go`, `unit_flee.go` haben keine eigene Testdatei; die
//! Abdeckung stehen in `shelter_tests.rs`, `disarm_tests.rs`, `hammer_tests.rs`).

use k3c_rng::Rng;

use super::flee::{enemy_within, flee};
use super::hammer::spawn_vagrant_with;
use super::jobs::{make_archer, release_job};
use super::{ARRIVE, shelter_x, walk_to, wander};
use crate::enemies::testutil::*;
use crate::state::{Job, ResourceNode, Troop};

fn peasant(w: &mut crate::state::World, v: &crate::state::WorldValues, x: f64) -> usize {
    let hub = w.hub_x;
    let ti = spawn_vagrant_with(w, v, hub, x);
    w.troops[ti].kind = "peasant".into();
    ti
}

#[test]
fn landstreicher_startet_mit_hp_der_daten_und_am_camp() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let ti = spawn_vagrant_with(&mut w, &v, 50.0, 53.0);
    let t = &w.troops[ti];
    let hp = v.troops["vagrant"].hp;
    assert_eq!(
        (t.kind.as_str(), t.hp, t.max_hp, t.anchor_x, t.target_x, t.x),
        ("vagrant", hp, hp, 50.0, 53.0, 53.0)
    );
}

#[test]
fn walk_to_naehert_sich_und_meldet_die_ankunft() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let ti = peasant(&mut w, &v, 100.0);
    let speed = v.troops["peasant"].speed;
    assert!(!walk_to(&mut w, &v, ti, 120.0, 0.1));
    assert!((w.troops[ti].x - (100.0 + speed * 0.1)).abs() < 1e-9);
    w.troops[ti].x = 120.0 - ARRIVE / 2.0;
    assert!(walk_to(&mut w, &v, ti, 120.0, 0.1));
}

/// Die zwei Würfe von `wander` fallen in der Reihenfolge Ziel, dann Wartezeit.
#[test]
fn wander_wuerfelt_erst_das_ziel_dann_die_wartezeit() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    w.rng = Rng::new("wander");
    let mut probe = Rng::new("wander");
    let ti = peasant(&mut w, &v, 100.0);
    w.troops[ti].target_x = 100.0;
    w.troops[ti].cooldown = 0.0;
    wander(&mut w, &v, ti, 100.0, 5.0, 0.1);
    let (a, b) = (probe.next(), probe.next());
    assert_eq!(w.troops[ti].target_x, 100.0 + (a * 2.0 - 1.0) * 5.0);
    assert_eq!(w.troops[ti].cooldown, 1.0 + b * 3.0);
}

#[test]
fn wander_springt_nicht_ueber_den_radius_hinaus() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let ti = peasant(&mut w, &v, 100.0);
    w.troops[ti].target_x = 190.0; // weit außerhalb: zurück zur Mitte
    w.troops[ti].cooldown = 5.0; // und noch kein neuer Wurf
    wander(&mut w, &v, ti, 100.0, 5.0, 0.1);
    assert_eq!(w.troops[ti].target_x, 100.0);
}

#[test]
fn shelter_x_streut_ueber_fuenf_plaetze_und_bleibt_gemerkt() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    for _ in 0..7 {
        peasant(&mut w, &v, 100.0);
    }
    let xs: Vec<f64> = (0..7).map(|ti| shelter_x(&mut w, ti)).collect();
    let hub = w.hub_x;
    assert_eq!(xs, [-2.0, -1.0, 0.0, 1.0, 2.0, -2.0, -1.0].map(|d| hub + d));
    w.troops.remove(0);
    assert_eq!(
        shelter_x(&mut w, 0),
        hub - 1.0,
        "der Platz bleibt, auch wenn der Index der Truppe wechselt"
    );
}

#[test]
fn release_job_gibt_knoten_und_platz_frei_wenn_die_truppe_sie_belegte() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let ti = peasant(&mut w, &v, 100.0);
    let tid = w.troops[ti].id;
    let nid = w.new_id();
    w.nodes.push(ResourceNode {
        id: nid,
        kind: "tree".into(),
        worker_id: Some(tid),
        ..ResourceNode::default()
    });
    let s = built_site(&mut w, "wall", 10.0);
    w.sites[s].worker_id = Some(tid + 1000); // ein anderer Bauer baut dort
    let sid = w.sites[s].id;
    w.troops[ti].job = Some(Job {
        r#type: "gather".into(),
        node_id: nid,
        ..Job::default()
    });
    release_job(&mut w, ti);
    assert!(w.troops[ti].job.is_none() && w.nodes[0].worker_id.is_none());
    w.troops[ti].job = Some(Job {
        r#type: "build".into(),
        site_id: sid,
        ..Job::default()
    });
    release_job(&mut w, ti);
    assert_eq!(
        w.sites[s].worker_id,
        Some(tid + 1000),
        "fremder Bauer bleibt eingetragen"
    );
    release_job(&mut w, ti); // ohne Auftrag: nichts
}

/// B-928: Die Flucht verwirft ein getragenes Material nicht.
#[test]
fn flucht_behaelt_den_trage_auftrag() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let at = w.hub_x - 30.0;
    let ti = peasant(&mut w, &v, at);
    w.troops[ti].job = Some(Job {
        r#type: "carry".into(),
        resource: "wood".into(),
        amount: 3,
        ..Job::default()
    });
    w.troops[ti].fleeing = true;
    for _ in 0..60 {
        flee(&mut w, &v, ti, 1.0 / 30.0);
    }
    let j = w.troops[ti].job.as_ref().expect("Trage-Auftrag verloren");
    assert!(j.r#type == "carry" && j.amount == 3);
}

#[test]
fn flucht_endet_am_schutzplatz_ohne_gegner_in_hoerweite() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let at = w.hub_x - 40.0;
    let ti = peasant(&mut w, &v, at);
    w.troops[ti].fleeing = true;
    let mut ticks = 0;
    while flee(&mut w, &v, ti, 0.1) {
        ticks += 1;
        assert!(ticks < 2000, "Bauer kommt nicht an");
    }
    assert!(!w.troops[ti].fleeing && w.troops[ti].shelter == 0);
    assert!((w.troops[ti].x - w.hub_x).abs() <= 2.0 + ARRIVE);
}

#[test]
fn flucht_dauert_an_solange_ein_gegner_in_hoerweite_ist() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let at = w.hub_x;
    let ti = peasant(&mut w, &v, at);
    w.troops[ti].fleeing = true;
    let hub_x = w.hub_x;
    let e = spawn(&mut w, "skeleton", hub_x + 1.0);
    assert!(enemy_within(&w, w.troops[ti].x, 8.0));
    assert!(flee(&mut w, &v, ti, 0.1), "Gegner in Hörweite");
    w.enemies[e].hp = 0.0;
    assert!(!enemy_within(&w, w.troops[ti].x, 8.0), "besiegte Gegner zählen nicht");
}

#[test]
fn make_archer_waehlt_die_schwaechere_seite() {
    let mut w = bare_world();
    let v = w.vals.clone().expect("Wertesatz");
    let hub = w.hub_x;
    let (a, b) = (peasant(&mut w, &v, hub), peasant(&mut w, &v, hub));
    make_archer(&mut w, a);
    assert_eq!(w.troops[a].anchor_x, hub - 1.0, "bei Gleichstand links");
    make_archer(&mut w, b);
    assert_eq!(w.troops[b].anchor_x, hub + 1.0, "links ist besetzt: rechts");
    let t: &Troop = &w.troops[b];
    assert!(t.kind == "archer" && t.job.is_none() && t.profession.is_empty());
}
