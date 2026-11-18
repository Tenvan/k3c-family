//! Tests zu `healer.rs` (Go: `building_healer_test.go` und `building_healer_w4_test.go`, W3.2, W4.3b, B-116/AC-01, B-122/AC-03):
//! Der Heilplatz heilt Bürger, Kämpfer und lebende Spieler im Radius, auch im Kampf, bis `max_hp` (Q32); Werte aus
//! data/buildings.json › healer.heal. Die Tests über `step` und `create_world` warten auf RU18.

use super::healer::step_healing;
use super::testutil::*;
use crate::state::World;
use crate::state::common::destroy_site;
use crate::unit::spawn_vagrant;
use crate::world::vals;

/// healWorld: Heilplatz, ein Landstreicher am Platz und einer außerhalb, zwei Spieler am Platz, ein Gegner in der Welt.
fn heal_world() -> (World, usize, usize, usize) {
    let mut w = day_world();
    let healer = hub_site_of(&w, "healer");
    let r = vals(&w).buildings["healer"].heal.radius_units;
    let hx = w.sites[healer].x;
    let near = spawn_vagrant(&mut w, hx, hx + r / 2.0);
    let far = spawn_vagrant(&mut w, hx + r + 4.0, hx + r + 4.0);
    for ti in [near, far] {
        w.troops[ti].hp = 1.0;
    }
    for _ in 0..2 {
        let pi = add_player(&mut w);
        w.players[pi].x = hx;
    }
    let hub = w.hub_x;
    spawn(&mut w, "skeleton", hub + 200.0);
    (w, healer, near, far)
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn heilplatz_heilt_im_radius_auch_im_kampf() {
    let (mut w, healer, near, far) = heal_world();
    built(&mut w.sites[healer]);
    run(&mut w, 1.0);
    let want = 1.0 + vals(&w).buildings["healer"].heal.hp_per_second;
    assert!(
        (w.troops[near].hp - want).abs() <= 0.2,
        "nach 1 s HP {}, laut Daten {want}",
        w.troops[near].hp
    );
    assert_eq!(w.troops[far].hp, 1.0, "außerhalb des Radius geheilt");
    w.troops[near].hp = w.troops[near].max_hp - 1.0;
    run(&mut w, 2.0);
    assert_eq!(w.troops[near].hp, w.troops[near].max_hp, "über oder nicht bis max_hp");
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn heilplatz_ohne_gebaeude_keine_heilung() {
    let (mut w, healer, near, _) = heal_world();
    run(&mut w, 1.0);
    built(&mut w.sites[healer]);
    let id = w.sites[healer].id;
    destroy_site(&mut w, id);
    run(&mut w, 1.0);
    assert_eq!(w.troops[near].hp, 1.0, "ungebaut oder zerstört geheilt");
}

/// healW4World: Heilplatz, je ein verwundeter Krieger in und außerhalb der Reichweite, ein verwundeter Spieler am Platz.
fn heal_w4_world() -> (World, usize, usize, usize) {
    let mut w = day_world();
    let healer = hub_site_of(&w, "healer");
    let r = vals(&w).buildings["healer"].heal.radius_units;
    let hx = w.sites[healer].x;
    let near = fighter_at(&mut w, "warrior", hx + r / 2.0);
    let far = fighter_at(&mut w, "warrior", hx + r + 4.0);
    let pi = add_player(&mut w);
    w.players[pi].x = hx;
    w.troops[near].hp = 10.0;
    w.troops[far].hp = 10.0;
    (w, healer, near, far)
}

#[test]
fn heal_w4_krieger_und_spieler_in_reichweite() {
    let (mut w, healer, near, far) = heal_w4_world();
    built(&mut w.sites[healer]);
    step_healing(&mut w, 1.0); // eine Sekunde Heilung, ohne dass die Krieger zum Posten laufen
    let want = 10.0 + vals(&w).buildings["healer"].heal.hp_per_second;
    assert!(
        (w.troops[near].hp - want).abs() <= 1e-9,
        "in Reichweite: Krieger {}, laut Daten {want}",
        w.troops[near].hp
    );
    assert_eq!(w.troops[far].hp, 10.0, "Krieger außerhalb der Reichweite geheilt");
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn heal_w4_ohne_heilplatz_keine_heilung() {
    let (mut w, _, near, far) = heal_w4_world();
    step_healing(&mut w, 1.0);
    run(&mut w, 1.0);
    assert!(
        w.troops[near].hp == 10.0 && w.troops[far].hp == 10.0,
        "ohne Heilplatz geheilt"
    );
}

/// Eigener Test (ohne Go-Gegenstück, läuft ohne `create_world`): Heilung im Radius bis `max_hp`, nur am gebauten Platz.
#[test]
fn step_healing_radius_und_gebaut() {
    let mut w = bare_world();
    let hx = w.hub_x;
    let h = vals(&w).buildings["healer"].heal.clone();
    let healer = built_site(&mut w, "healer", hx + 50.0);
    let near = spawn_vagrant(&mut w, hx + 50.0, hx + 50.0 + h.radius_units / 2.0);
    let far = spawn_vagrant(&mut w, hx + 50.0, hx + 50.0 + h.radius_units + 4.0);
    for ti in [near, far] {
        w.troops[ti].max_hp = 100.0;
        w.troops[ti].hp = 1.0;
    }
    w.sites[healer].state = "unpaid".into();
    step_healing(&mut w, 1.0);
    assert_eq!(w.troops[near].hp, 1.0, "ungebaut geheilt");
    w.sites[healer].state = "built".into();
    step_healing(&mut w, 1.0);
    assert!((w.troops[near].hp - (1.0 + h.hp_per_second)).abs() < 1e-9);
    assert_eq!(w.troops[far].hp, 1.0, "außerhalb des Radius geheilt");
    step_healing(&mut w, 1000.0);
    assert_eq!(w.troops[near].hp, 100.0, "über max_hp geheilt");
}
