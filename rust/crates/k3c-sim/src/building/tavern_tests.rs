//! Tests zu `tavern.rs` (Go: `building_tavern_test.go`, W3.1, B-116/AC-01, Sprint W3 AC-01): Die gebaute Taverne liefert bei
//! jedem Morgengrauen Landstreicher bis zum Maximum (Q30), Werte aus data/buildings.json › tavern.vagrants. Der Test über
//! `step` wartet auf RU18 (Zahlplatz `new_hub_site`) und RU19 (`pure`).

use super::tavern::{vagrants_at, wander_radius};
use super::testutil::*;
use crate::enemies::testutil::step;
use crate::state::World;
use crate::state::common::destroy_site;
use crate::unit::spawn_vagrant;
use crate::world::base::base;
use crate::world::cycle::{cycle_at, cycle_phases};
use crate::world::vals;

/// Go: `toDawn` rechnet bis zum nächsten Tagesanbruch (Beginn des Morgengrauens).
fn to_dawn(w: &mut World) {
    let dn = base().expect("eingebettete Daten").day_night.clone();
    let (phases, length) = cycle_phases(&dn);
    let now = w.time * w.cycle_speed;
    let mut target = (now / length).floor() * length + phases[3].start;
    if target <= now + 2.0 * DT * w.cycle_speed {
        target += length;
    }
    w.time = target / w.cycle_speed - 2.0 * DT;
    w.cycle = cycle_at(&dn, w.time * w.cycle_speed);
    for _ in 0..10 {
        step(w, None, DT);
        if has_event(&w.events, "dawn") {
            return;
        }
    }
    panic!("kein dawn");
}

#[test]
#[ignore = "wartet auf RU18.4 (preset_for)"]
fn taverne_landstreicher_je_dawn_bis_max() {
    let mut w = quiet_world();
    add_player(&mut w);
    add_player(&mut w);
    let v = vals(&w).buildings["tavern"].vagrants.clone();
    let tavern = hub_site_of(&w, "tavern");
    let tx = w.sites[tavern].x;
    to_dawn(&mut w);
    assert_eq!(vagrants_at(&w, tx), 0, "ungebaut: Landstreicher an der Taverne");
    built(&mut w.sites[tavern]);
    for day in 1..=(v.max + 1) {
        to_dawn(&mut w);
        let want = (day * v.per_dawn).min(v.max);
        assert_eq!(vagrants_at(&w, tx), want, "Tag {day}");
    }
    for t in &w.troops {
        assert!(
            t.anchor_x != tx || (t.x - tx).abs() <= v.wander_units,
            "Landstreicher bei {}, außerhalb von {tx} ± {}",
            t.x,
            v.wander_units
        );
    }
    let id = w.sites[tavern].id;
    destroy_site(&mut w, id);
    w.troops.clear();
    to_dawn(&mut w);
    assert_eq!(vagrants_at(&w, tx), 0, "zerstört: Landstreicher an der Taverne");
}

/// Eigener Test (ohne Go-Gegenstück, läuft ohne `create_world`): `vagrants_at` zählt die an `x` gebundenen Landstreicher,
/// `wander_radius` gibt Taverne und Camp ihren Radius.
#[test]
fn vagrants_at_und_wander_radius() {
    let mut w = bare_world();
    let v = vals(&w);
    let hub = w.hub_x;
    built_site(&mut w, "tavern", hub + 50.0);
    let at_tavern = spawn_vagrant(&mut w, hub + 50.0, hub + 51.0);
    let at_camp = spawn_vagrant(&mut w, hub + 90.0, hub + 90.0);
    let peasant = spawn_vagrant(&mut w, hub + 50.0, hub + 50.0);
    w.troops[peasant].kind = "peasant".into();
    assert_eq!(vagrants_at(&w, hub + 50.0), 1, "Bauern zählen nicht");
    assert_eq!(vagrants_at(&w, hub + 90.0), 1);
    assert_eq!(vagrants_at(&w, hub + 10.0), 0);
    assert_eq!(
        wander_radius(&w, at_tavern),
        v.buildings["tavern"].vagrants.wander_units
    );
    assert_eq!(wander_radius(&w, at_camp), v.economy.recruit_camp.wander_units);
}
