//! Tests zu `barracks.rs` (Go: `building_barracks_test.go`, W3.1, B-116/AC-01, Sprint W3 AC-01): Kämpfer-Limit je Hub (Q29),
//! Basis plus Zuschlag je gebauter Kaserne; geprüft nur beim Waffe-Holen. Die Tests über `step` warten auf RU18 (Zahlplatz
//! `new_hub_site`, Schritte der Wirtschaft).

use super::barracks::{fighters, troop_limit};
use super::testutil::*;
use crate::state::World;
use crate::state::common::destroy_site;
use crate::unit::jobs::{make_archer, make_fighter};
use crate::unit::spawn_vagrant;
use crate::unit::warrior::{fetches_fighter_weapon, weapon_to_fetch};
use crate::world::vals;

#[test]
fn kaserne_hebt_limit_um_datenwert() {
    let mut w = bare_world();
    let l = vals(&w).buildings["barracks"].troop_limit.clone();
    assert_eq!(troop_limit(&w), l.base, "Limit ohne Kaserne");
    let barracks = built_site(&mut w, "barracks", 250.0);
    assert_eq!(troop_limit(&w), l.base + l.per_building, "Limit mit Kaserne");
    let id = w.sites[barracks].id;
    destroy_site(&mut w, id);
    assert_eq!(troop_limit(&w), l.base, "Limit nach Zerstörung");
}

/// limitWorld: ein Bauer, eine Werkstatt mit einem Bogen, das Limit voll mit Bogenschützen; dazu ein Landstreicher, der
/// nicht zählt; zwei Spieler.
fn limit_world() -> (World, usize) {
    let mut w = day_world();
    w.troops[0].hammer = false; // Waffen holt nur ein Bauer ohne Hammer (B-950)
    add_player(&mut w);
    add_player(&mut w);
    let hub = w.hub_x;
    for _ in 0..vals(&w).buildings["barracks"].troop_limit.base {
        let ti = spawn_vagrant(&mut w, hub, hub);
        make_archer(&mut w, ti);
    }
    spawn_vagrant(&mut w, hub + 30.0, hub + 30.0);
    let workshop = build_site(&mut w, "bowyer");
    w.sites[workshop].bows = 1;
    (w, workshop)
}

#[test]
#[ignore = "wartet auf RU18 (new_hub_site, step_players)"]
fn limit_voll_bogen_bleibt_im_regal() {
    let (mut w, workshop) = limit_world();
    run(&mut w, 20.0);
    assert!(
        w.troops[0].kind == "peasant" && w.sites[workshop].bows == 1 && fighters(&w) == troop_limit(&w),
        "Limit voll: Bauer ist {}, Bögen im Regal {}, Kämpfer {}/{}",
        w.troops[0].kind,
        w.sites[workshop].bows,
        fighters(&w),
        troop_limit(&w)
    );
    build_site(&mut w, "barracks");
    let done = |w: &World| w.troops[0].kind == "archer";
    assert!(
        run_until(&mut w, 30.0, &done) >= 0.0 && w.sites[workshop].bows == 0,
        "mit Kaserne: Bauer ist {}, Bögen im Regal {}",
        w.troops[0].kind,
        w.sites[workshop].bows
    );
}

/// Eine zerstörte Kaserne senkt das Limit, bestehende Kämpfer bleiben.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, step_players)"]
fn kaserne_zerstoert_kaempfer_bleiben() {
    let (mut w, _) = limit_world();
    let barracks = build_site(&mut w, "barracks");
    let done = |w: &World| w.troops[0].kind == "archer";
    run_until(&mut w, 30.0, &done);
    let before = fighters(&w);
    let id = w.sites[barracks].id;
    destroy_site(&mut w, id);
    run(&mut w, 1.0);
    let base = vals(&w).buildings["barracks"].troop_limit.base;
    assert!(
        fighters(&w) == before && before == base + 1,
        "Kämpfer {} nach Zerstörung, vorher {before}",
        fighters(&w)
    );
    let hub = w.hub_x;
    let other = spawn_vagrant(&mut w, hub, hub);
    w.troops[other].kind = "peasant".into();
    let v = vals(&w);
    if let Some(j) = weapon_to_fetch(&w, &v, other) {
        assert!(
            !fetches_fighter_weapon(&w, &v, &j),
            "über dem Limit gibt es trotzdem einen Abhol-Auftrag"
        );
    }
}

/// W4.3a (Q40): Das Limit zählt Bogenschützen und Krieger gemischt; Bauern mit Beruf und Landstreicher nicht. Bei vollem
/// Limit bleibt das Schwert im Regal.
#[test]
#[ignore = "wartet auf RU18 (new_hub_site, step_players)"]
fn limit_zaehlt_bogenschuetzen_und_krieger() {
    let mut w = day_world();
    add_player(&mut w);
    add_player(&mut w);
    let l = vals(&w).buildings["barracks"].troop_limit.clone();
    let hub = w.hub_x;
    for i in 0..l.base {
        let ti = spawn_vagrant(&mut w, hub, hub);
        make_fighter(&mut w, ti, ["archer", "warrior"][(i % 2) as usize]);
    }
    add_peasant(&mut w, hub, "miner");
    spawn_vagrant(&mut w, hub + 30.0, hub + 30.0);
    let workshop = build_site(&mut w, "workshop");
    w.sites[workshop].swords = 1;
    w.troops[0].hammer = false;
    run(&mut w, 20.0);
    assert!(
        w.troops[0].kind == "peasant" && w.sites[workshop].swords == 1 && fighters(&w) == l.base,
        "Limit voll: Bauer ist {}, Schwerter im Regal {}, Kämpfer {}/{}",
        w.troops[0].kind,
        w.sites[workshop].swords,
        fighters(&w),
        l.base
    );
    build_site(&mut w, "barracks");
    let done = |w: &World| w.troops[0].kind == "warrior";
    assert!(
        run_until(&mut w, 30.0, &done) >= 0.0 && w.sites[workshop].swords == 0,
        "mit Kaserne: Bauer ist {}, Schwerter im Regal {}",
        w.troops[0].kind,
        w.sites[workshop].swords
    );
    for _ in 0..(l.base + l.per_building - fighters(&w)) {
        let ti = spawn_vagrant(&mut w, hub, hub);
        make_fighter(&mut w, ti, "warrior");
    }
    w.sites[workshop].swords = 1;
    let extra = add_peasant(&mut w, hub, "");
    run(&mut w, 20.0);
    assert!(
        w.troops[extra].kind == "peasant" && w.sites[workshop].swords == 1 && fighters(&w) == l.base + l.per_building,
        "Limit mit Kaserne: Bauer ist {}, Schwerter im Regal {}, Kämpfer {}/{}",
        w.troops[extra].kind,
        w.sites[workshop].swords,
        fighters(&w),
        l.base + l.per_building
    );
}

/// Eigener Test (ohne Go-Gegenstück, läuft ohne `create_world`): Nur Kämpfer zählen, Landstreicher und Bauern nicht.
#[test]
fn fighters_zaehlt_nur_kaempfer() {
    let mut w = bare_world();
    let hub = w.hub_x;
    for kind in ["archer", "warrior", "eliteArcher", "vagrant", "peasant"] {
        let ti = spawn_vagrant(&mut w, hub, hub);
        w.troops[ti].kind = kind.into();
    }
    assert_eq!(fighters(&w), 3);
}
