//! Tests zu den Zielen der Gegner (Go: `enemies_targets_test.go`, K1.2, B-128): Tor und übrige Gebäude als Ziele.

use super::target::{TargetKind, blocking_wall, choose_target};
use super::testutil::*;
use super::{BODY, is_barrier};
use crate::state::common::sign;
use crate::state::{Troop, World, WorldValues};
use std::sync::Arc;

/// Go: `targetOf`: das Ziel, das `ei` jetzt wählt (mit Mauer oder Tor wie in `step_enemies`), oder 0.
fn target_of(w: &World, ei: usize) -> i64 {
    let v: Arc<WorldValues> = w.vals.clone().expect("Wertesatz");
    let e = &w.enemies[ei];
    let dir = sign(w.hub_x - e.x);
    let wall = if e.has("ignoresWalls") {
        None
    } else {
        blocking_wall(w, e, dir)
    };
    choose_target(w, &v, ei, dir, wall).map_or(0, |c| c.id)
}

/// Go: `workshopAt`.
fn workshop_at(w: &mut World, x: f64) -> usize {
    built_site(w, "workshop", x)
}

#[test]
fn blocking_wall_nimmt_die_naechste_mauer_vor_dem_gegner() {
    let mut w = bare_world();
    let at = w.hub_x - 20.0;
    let near = built_site(&mut w, "wall", at);
    let at = w.hub_x - 40.0;
    built_site(&mut w, "wall", at);
    let at = w.hub_x - 10.0;
    built_site(&mut w, "workshop", at);
    let at = w.hub_x - 50.0;
    let e = spawn(&mut w, "skeleton", at);
    let dir = sign(w.hub_x - w.enemies[e].x);
    assert_eq!(dir, 1.0);
    let wall = blocking_wall(&w, &w.enemies[e], dir).expect("Mauer");
    assert_eq!(wall.id, w.sites[1].id, "die äußere Mauer ist die nächste");
    assert!(is_barrier(&w.sites[near]));
    // Hinter der Mauer (zur Burg hin) gibt es keine mehr.
    w.enemies[e].x = w.hub_x - 10.0;
    assert!(blocking_wall(&w, &w.enemies[e], dir).is_none());
}

/// (e) Nahkämpfer erreichen kein Gebäude hinter dem Tor, auch nicht mit `prefersBuildings`; sie greifen das Tor an.
#[test]
fn gegner_nahkampf_erreicht_kein_gebaeude_hinter_dem_tor() {
    let mut w = bare_world();
    let at = w.hub_x - 20.0;
    let gate = built_site(&mut w, "wall", at);
    let gate_x = w.sites[gate].x;
    let shop = workshop_at(&mut w, gate_x + 0.5);
    for kind in ["skeleton", "caveTroll"] {
        let e = spawn(&mut w, kind, gate_x - BODY);
        assert_eq!(
            target_of(&w, e),
            w.sites[gate].id,
            "{kind} greift das Tor an (Werkstatt {} dahinter)",
            w.sites[shop].id
        );
    }
}

#[test]
fn gegner_ohne_ziel_in_reichweite_waehlt_nichts() {
    let mut w = bare_world();
    let at = w.hub_x - 100.0;
    let e = spawn(&mut w, "skeleton", at);
    assert_eq!(target_of(&w, e), 0);
}

#[test]
fn gebaeude_sind_ziel_wenn_nichts_anderes_in_reichweite_ist() {
    let mut w = bare_world();
    let at = w.hub_x - 20.0;
    let shop = workshop_at(&mut w, at);
    let shop_id = w.sites[shop].id;
    let at = w.sites[shop].x - 1.0;
    let skeleton = spawn(&mut w, "skeleton", at);
    let at = w.sites[shop].x - 1.0;
    let troll = spawn(&mut w, "caveTroll", at);
    assert_eq!(target_of(&w, skeleton), shop_id);
    assert_eq!(target_of(&w, troll), shop_id);
}

#[test]
fn die_burg_ist_ein_ziel_im_burgradius() {
    let mut w = bare_world();
    let r = w.vals.as_ref().map_or(0.0, |v| v.hub.castle_radius_units);
    let at = w.hub_x - r;
    let e = spawn(&mut w, "skeleton", at);
    assert_eq!(target_of(&w, e), w.castle.id);
    let v = w.vals.clone().expect("Wertesatz");
    let c = choose_target(&w, &v, e, 1.0, None).expect("Ziel");
    assert_eq!(c.kind, TargetKind::Castle);
    assert_eq!(c.kind.as_str(), "castle");
}

/// (d) `prefersBuildings` (Höhlentroll) nimmt die Werkstatt vor einem Krieger im selben Abstand; ein Gegner ohne die
/// Eigenschaft nimmt den Krieger und die Werkstatt erst, wenn nichts anderes in Reichweite ist.
#[test]
fn gegner_gebaeude_als_ziel() {
    let mut w = bare_world();
    let at = w.hub_x - 20.0;
    let shop = workshop_at(&mut w, at);
    let (shop_id, shop_x) = (w.sites[shop].id, w.sites[shop].x);
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: "warrior".into(),
        x: shop_x - 2.0,
        hp: 50.0,
        max_hp: 50.0,
        ..Troop::default()
    });
    let troll = spawn(&mut w, "caveTroll", shop_x - 1.0);
    let skeleton = spawn(&mut w, "skeleton", shop_x - 1.0);
    assert_eq!(target_of(&w, troll), shop_id, "Höhlentroll greift die Werkstatt an");
    assert_eq!(target_of(&w, skeleton), id, "Skelett greift den Krieger an");
    w.troops.clear();
    assert_eq!(
        target_of(&w, skeleton),
        shop_id,
        "Skelett ohne andere Ziele greift die Werkstatt an"
    );
}

/// (c) Gegner mit `ignoresWalls` (auch mit Phasen) laufen an der gebauten Mauer (dem Tor) vorbei, ohne sie anzugreifen.
#[test]
fn gegner_ignores_walls_laeuft_an_der_mauer_vorbei() {
    for kind in ["bat", "mineGhost"] {
        let mut w = day_world();
        let gate = line_site(&mut w, "wall", -1, 1);
        crate::world::testutil::sturdy(&mut w.sites[gate]);
        let (gx, gmax) = (w.sites[gate].x, w.sites[gate].max_hp);
        let e = spawn(&mut w, kind, gx - 4.0);
        step_for(&mut w, 3.0);
        assert!(
            w.enemies[e].x > gx && w.sites[gate].hp >= gmax,
            "{kind}: aufgehalten oder angegriffen"
        );
    }
}

/// (f) Jeder Tod eines Gegners meldet genau ein `kill` mit Art und Ort, auch fünf in einem Tick und an der Obergrenze.
#[test]
fn gegner_kill_je_tod() {
    let mut w = quiet_world();
    for i in 0..5 {
        let at = w.hub_x - 20.0 - f64::from(i) * 2.0;
        let e = spawn(&mut w, "skeleton", at);
        w.enemies[e].hp = 0.0;
    }
    step(&mut w, None, DT);
    assert_eq!(count_events(&w, "kill"), 5, "ein kill je Tod");
    for k in super::events_of_type(&w, "kill") {
        assert_eq!(k["kind"], "skeleton");
        assert!(k["x"].is_number());
    }
    for i in 0..5 {
        let at = w.hub_x - 20.0 - f64::from(i);
        let e = spawn(&mut w, "skeleton", at);
        w.enemies[e].hp = 0.0;
    }
    w.events.clear();
    remove_dead_enemies(&mut w);
    // Die Obergrenze je Tick (`capEvents`, RU19) verwirft Treffer, nie `kill`: wird mit RU19 vervollständigt.
    assert_eq!(count_events(&w, "kill"), 5);
}
