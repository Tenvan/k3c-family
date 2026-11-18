//! Verlust-Kaskade statt Tod (Q67) und Ereignis `disarmed` (Q69) (Go: `disarm_test.go`, W4.3a, B-122). Alle Tests laufen
//! über `Step` und warten auf RU14.

use crate::enemies::testutil::*;
use crate::state::{Event, Job, Troop, World};

/// Go: `hitUntil` lässt einen Skelett-Gegner neben `ti` los, bis `done` gilt, und sammelt die `disarmed`-Ereignisse.
fn hit_until(w: &mut World, ti: usize, done: &dyn Fn(&World) -> bool) -> Vec<Event> {
    w.troops[ti].hp = 1.0;
    let at = w.troops[ti].x - 1.0;
    spawn(w, "skeleton", at);
    let mut got = Vec::new();
    let mut tm = 0.0;
    while tm < 20.0 {
        step(w, None, DT);
        got.extend(w.events.iter().filter(|e| e["type"] == "disarmed").cloned());
        if done(w) {
            w.enemies.clear();
            return got;
        }
        tm += DT;
    }
    panic!("Treffer ohne Wirkung: {} mit {} HP", w.troops[ti].kind, w.troops[ti].hp);
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn disarm_krieger_wird_bauer_und_hebt_schwert_auf() {
    let mut w = day_world();
    add_player(&mut w);
    add_player(&mut w);
    w.troops[0].hammer = false;
    crate::unit::jobs::make_fighter(&mut w, 0, "warrior");
    w.troops[0].x = w.hub_x - 5.0;
    let events = hit_until(&mut w, 0, &|w| w.troops[0].kind != "warrior");
    let peasant_hp = w.vals.as_ref().map_or(0.0, |v| v.troops["peasant"].hp);
    let tr = &w.troops[0];
    assert!(tr.kind == "peasant" && tr.hp == peasant_hp && w.drops.len() == 1 && w.drops[0].kind == "warrior");
    assert!(
        events.len() == 1 && events[0]["kind"] == "warrior" && events[0]["cause"] == "skeleton",
        "{events:?}"
    );
    assert_eq!(
        crate::state::pending::fighters(&w),
        0,
        "entwaffneter Krieger zählt noch zum Limit"
    );
    let again = |w: &World| w.troops[0].kind == "warrior";
    assert!(
        run_until(&mut w, 30.0, &again) >= 0.0 && w.drops.is_empty(),
        "Schwert nicht wieder aufgehoben"
    );
}

#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn disarm_beruf_faellt_zu_boden() {
    let mut w = quiet_world();
    let hub = w.hub_x;
    let tr = add_peasant(&mut w, hub - 5.0, "miner");
    let events = hit_until(&mut w, tr, &|w| w.troops[tr].profession.is_empty());
    let t = &w.troops[tr];
    assert!(events.len() == 1 && events[0]["kind"] == "miner" && w.drops.len() == 1 && t.kind == "peasant");
    let again = |w: &World| w.troops[tr].profession == "miner";
    assert!(
        run_until(&mut w, 30.0, &again) >= 0.0,
        "Bergmann-Ausrüstung nicht wieder aufgehoben"
    );
}

/// B-928: Ein Treffer macht nie einen Landstreicher; ein getroffener Bauer behält Status und Münze und flieht zur Burg.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn treffer_macht_keinen_landstreicher() {
    let mut w = quiet_world();
    let hub = w.hub_x;
    let tr = add_peasant(&mut w, hub - 5.0, "");
    let coins = w.coins.len();
    let events = hit_until(&mut w, tr, &|w| w.troops[tr].fleeing);
    assert!(
        events.is_empty() && w.troops[tr].kind == "peasant",
        "Treffer: {events:?}"
    );
    w.troops[tr].hp = 0.0;
    lose_layer(&mut w, tr); // Kampf-HP aufgebraucht: keine Kaskade
    let hp = w.vals.as_ref().map_or(0.0, |v| v.troops["peasant"].hp);
    let t = &w.troops[tr];
    assert!(t.kind == "peasant" && w.coins.len() == coins && t.hp == hp && t.fleeing && w.drops.is_empty());
    w.troops[tr].x = hub - 40.0;
    run(&mut w, 30.0);
    let home = w.vals.as_ref().map_or(0.0, |v| v.hub.home_radius_units);
    assert!(
        !w.troops[tr].fleeing && (w.troops[tr].x - hub).abs() <= home + 1.0,
        "Bauer nicht an der Burg"
    );
}

/// Beim Burgfall gibt es kein `disarmed`.
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn disarm_nicht_beim_burgfall() {
    let mut w = day_world();
    crate::unit::jobs::make_fighter(&mut w, 0, "warrior");
    w.castle.hp = 0.0;
    step(&mut w, None, DT);
    assert!(w.events.iter().any(|e| e["type"] == "castleFallen"), "kein Burgfall");
    assert!(
        !w.events.iter().any(|e| e["type"] == "disarmed") && w.drops.is_empty(),
        "disarmed beim Burgfall"
    );
}

/// B-928: Die Flucht verwirft ein getragenes Material nicht (über `Step`; der Aufruf von `flee` steht in
/// `common_tests.rs`).
#[test]
#[ignore = "wartet auf RU19 (pure)"]
fn flucht_behaelt_den_trage_auftrag_ueber_step() {
    let mut w = quiet_world();
    let hub = w.hub_x;
    let tr = add_peasant(&mut w, hub - 30.0, "");
    w.troops[tr].job = Some(Job {
        r#type: "carry".into(),
        resource: "wood".into(),
        amount: 3,
        ..Job::default()
    });
    w.troops[tr].fleeing = true;
    run(&mut w, 2.0);
    let t: &Troop = &w.troops[tr];
    assert!(
        t.job.as_ref().is_some_and(|j| j.r#type == "carry" && j.amount == 3),
        "Trage-Auftrag verloren"
    );
}
