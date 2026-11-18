//! Tests des Zustands-Spiegels (Go: `world_state_mirror_test.go`; K4.1a, B-383/AC-01): Boss-Phase, Warnkreis, Event mit
//! Restzeit, Inselwechsel im JSON der Welt. Die Go-Tests bauen Inseln (`create_island`: Plätze, Ruinen RU18) und Mond-Events
//! (RU19) und tragen `ignore`. `own_*` prüft `mirror_state` und `mirror_switch` an Hand-Welten.

use crate::boss::endboss::EndbossFight;
use crate::boss::testutil::boss_stage;
use crate::island::Island;
use crate::island::switch_testutil::{gate_island, step_seconds, test_islands};
use crate::island::testutil::{run_island, test_data};
use crate::state::{Enemy, World};

use super::state_mirror::{mirror_state, mirror_switch};
use super::testutil::DT;

fn mirror_json<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).expect("JSON")
}

/// Go: `protect` (boss_endboss_test.go): hält die Spieler handlungsfähig: Krone auf, viel Gold und Last Stand gegen den
/// letzten Treffer.
fn protect(w: &mut World) {
    for p in &mut w.players {
        p.crown = None;
        p.gold = 1000;
        p.last_stand_for = 1.0;
    }
}

/// Go: `mirrorSteps`: tickt die Insel `seconds` lang (Spiegel setzt `step_island`) und hält die Spieler am Leben.
fn mirror_steps(isl: &mut Island, seconds: f64) {
    let data = test_data();
    let mut tm = 0.0;
    while tm < seconds - 1e-9 {
        protect(&mut isl.stages[0]);
        run_island(isl, &data, &[], DT);
        tm += DT;
    }
}

/// Go: `endIsland` (boss_endboss_test.go): Insel mit der Kristallhöhle (Tiefe 4) und `players` Spielern, ohne Truppen und
/// Gegner; dazu die X-Lage des Baus. Braucht `create_island` (Plätze, Ruinen RU18).
fn end_island(_players: i64) -> (Island, f64) {
    panic!("RU18: end_island")
}

/// Go: `warden`: der Endboss der Kristallhöhle in der Stufe (Index).
fn warden(w: &World) -> Option<usize> {
    w.enemies.iter().position(|e| e.kind == "crystalHeartWarden")
}

#[test]
#[ignore = "wartet auf RU18 (end_island)"]
fn mirror_endboss_phase_und_warnkreis() {
    let (mut isl, x) = end_island(2);
    isl.stages[0].players[0].x = x + 4.0;
    isl.stages[0].players[1].x = x + 6.0;
    assert_eq!(mirror_json(&isl.stages[0].enemies), "[]", "Felder ohne Boss");
    mirror_steps(&mut isl, DT);
    let bi = warden(&isl.stages[0]).expect("Endboss");
    {
        let boss = &isl.stages[0].enemies[bi];
        assert!(
            boss.phase == 1 && boss.warn.is_none(),
            "Phase 1 (Beschwörung): {boss:?}"
        );
    }
    let boss = isl.stages[0].enemies.remove(bi);
    isl.stages[0].enemies = vec![Enemy {
        hp: boss.max_hp * 0.59,
        ..boss
    }];
    mirror_steps(&mut isl, DT);
    let boss = &isl.stages[0].enemies[0];
    let ok = boss.phase == 2
        && boss
            .warn
            .as_ref()
            .is_some_and(|z| z.r == 5.0 && z.x == (boss.x * 10.0).round() / 10.0 && z.r#in == boss.aoe_in);
    assert!(ok, "Phase 2: Phase {}, Warn {:?}", boss.phase, boss.warn);
    let before = boss.warn.as_ref().map_or(0.0, |z| z.r#in);
    mirror_steps(&mut isl, 1.0);
    let boss = &isl.stages[0].enemies[0];
    let now = boss.warn.as_ref().map_or(0.0, |z| z.r#in);
    assert!(
        !(now >= before && before > 1.0),
        "Warn.in sinkt nicht: {before} -> {now}"
    );
    let s = mirror_json(boss);
    assert!(s.contains(r#""phase":2"#) && s.contains(r#""warn":{"x":"#), "JSON: {s}");
}

/// Go: `moonIsland` (events_moon_test.go): Insel mit Wald und Höhle, zwei Spielern und Zyklus kurz vor Nacht `n`.
fn moon_island(n: i64) -> Island {
    crate::events::testutil::moon_island(n).1
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn mirror_event() {
    let data = test_data();
    let mut isl = moon_island(7);
    let s = mirror_json(&isl.stages[0]);
    assert!(
        !s.contains(r#""events":[{"id""#) && !s.contains(r#""nightEvents""#),
        "Event vor der Nacht"
    );
    run_island(&mut isl, &data, &[], 2.0);
    let w = &isl.stages[0];
    assert!(
        w.night_events.len() == 1
            && w.night_events[0].id == "fullMoon"
            && w.night_events[0].seconds_left == w.cycle.seconds_left,
        "Vollmond: {:?}, Zyklus {:?}",
        w.night_events,
        w.cycle
    );
    let left = w.night_events[0].seconds_left;
    run_island(&mut isl, &data, &[], 1.0); // Nacht 4 min bei Tempo 60: 4 s Spielzeit
    let w = &isl.stages[0];
    assert!(
        w.night_events[0].seconds_left < left
            && mirror_json(w).contains(r#""nightEvents":[{"id":"fullMoon","secondsLeft":"#),
        "Restzeit sinkt nicht: {left} -> {}",
        w.night_events[0].seconds_left
    );
    assert_eq!(
        isl.stages[1].night_events.len(),
        1,
        "Höhle: {:?}",
        isl.stages[1].night_events
    );
    let mut both = moon_island(91);
    run_island(&mut both, &data, &[], 2.0);
    assert_eq!(
        both.stages[0].night_events.len(),
        2,
        "Nacht 91: {:?}",
        both.stages[0].night_events
    );
    let mut day = moon_island(8);
    run_island(&mut day, &data, &[], 2.0);
    assert!(day.stages[0].night_events.is_empty(), "Event in Nacht 8");
}

#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn mirror_inselwechsel() {
    let data = test_data();
    let (world, _) = test_islands(&data);
    let mut isl = gate_island(&data, &world, 2);
    isl.endboss_defeated = false;
    step_seconds(&mut isl, &data, 1.0);
    assert!(
        !mirror_json(&isl.stages[1]).contains("islandSwitch"),
        "Wechsel vor dem Endboss"
    );
    isl.endboss_defeated = true;
    step_seconds(&mut isl, &data, 1.0);
    for (i, w) in isl.stages.iter().enumerate() {
        let ok = w.island_switch.as_ref().is_some_and(|s| s.open && !s.ready);
        assert!(ok, "Stufe {i}: {:?}", w.island_switch);
    }
    let p = isl.stages[1].island_switch.as_ref().map_or(0.0, |s| s.progress);
    assert!(p > 0.0 && p < 1.0, "Fortschritt {p}");
    step_seconds(&mut isl, &data, 3.0);
    let sw = isl.stages[1].island_switch.as_ref().expect("Wechsel");
    assert!(sw.ready && sw.progress == 1.0, "bereit: {sw:?}");
    let want = r#""islandSwitch":{"open":true,"progress":1,"ready":true,"targets":[{"id":"test-island2","name":"Test-Insel","etappe":2}],"choice":"test-island2","completed":["island1"]}"#;
    let s = mirror_json(&isl.stages[0]);
    assert!(s.contains(want), "JSON: {s}");
}

/// Eine Stufe mit Wertesatz samt Bossen; `kinds` sind die Gegner, alle als Boss.
fn boss_world(kinds: &[&str]) -> World {
    let (mut w, _) = boss_stage(4, 1);
    for kind in kinds {
        let id = w.new_id();
        w.enemies.push(Enemy {
            id,
            kind: (*kind).into(),
            boss: true,
            phase: 7, // alter Stand, den der Spiegel überschreibt
            warn: Some(crate::state::shared::WarnZone::default()),
            ..Enemy::default()
        });
    }
    w
}

fn fight(boss_id: i64, home: usize, phase: usize) -> EndbossFight {
    EndbossFight {
        boss_id,
        home,
        phase,
        ..EndbossFight::default()
    }
}

#[test]
fn own_mirror_state_setzt_phase_nur_fuer_den_laufenden_endboss() {
    let mut w = boss_world(&["forestChieftain", "goblinLeader"]);
    let (end_id, mini_id) = (w.enemies[0].id, w.enemies[1].id);
    mirror_state(&mut w, 0, Some(&fight(end_id, 0, 0)));
    assert_eq!(w.enemies[0].phase, 1, "Phase 1 ab der ersten Phase");
    assert!(
        w.enemies[0].warn.is_none(),
        "Phase 1 ist die Beschwörung: kein Warnkreis"
    );
    assert_eq!(w.enemies[1].phase, 0, "der Miniboss ist nicht der Endboss-Kampf");
    assert!(w.enemies[1].warn.is_none(), "Beschwörung, kein Flächenschlag");
    assert_ne!(mini_id, end_id);
}

#[test]
fn own_mirror_state_ohne_kampf_oder_in_anderer_stufe_ist_phase_null() {
    let mut w = boss_world(&["forestChieftain"]);
    let id = w.enemies[0].id;
    mirror_state(&mut w, 0, None);
    assert_eq!(w.enemies[0].phase, 0, "ohne Insel");
    w.enemies[0].phase = 7;
    mirror_state(&mut w, 0, Some(&fight(id, 1, 0))); // Kampf gehört zur Stufe 1
    assert_eq!(w.enemies[0].phase, 0, "Kampf einer anderen Stufe");
    w.enemies[0].phase = 7;
    mirror_state(&mut w, 0, Some(&fight(id + 100, 0, 0))); // anderer Gegner
    assert_eq!(w.enemies[0].phase, 0, "Kampf um einen anderen Gegner");
}

#[test]
fn own_mirror_state_laesst_gewoehnliche_gegner_und_leere_events_in_ruhe() {
    let mut w = boss_world(&[]);
    let id = w.new_id();
    w.enemies.push(Enemy {
        id,
        kind: "goblin".into(),
        phase: 3,
        ..Enemy::default()
    });
    w.night_events.push(crate::state::shared::ActiveEvent {
        id: "alt".into(),
        seconds_left: 5.0,
    });
    mirror_state(&mut w, 0, None);
    assert_eq!(w.enemies[0].phase, 3, "nur Bosse werden gespiegelt");
    assert!(
        w.night_events.is_empty(),
        "ohne Events im Wertesatz bleibt nichts stehen"
    );
}

#[test]
fn own_mirror_switch_setzt_den_zustand_in_allen_stufen() {
    let mut isl = Island {
        id: "x".into(),
        island_id: "island1".into(),
        stages: vec![World::default(), World::default()],
        completed: vec!["island0".into()],
        gate_open: true,
        switch_ready: true,
        switch_progress: 1.7,
        ..Island::default()
    };
    mirror_switch(&mut isl);
    for (i, w) in isl.stages.iter().enumerate() {
        let sw = w.island_switch.as_ref().expect("Wechsel");
        assert!(sw.open && sw.ready, "Stufe {i}: {sw:?}");
        assert_eq!(sw.progress, 1.0, "Fortschritt wird auf 1 begrenzt");
        assert_eq!(sw.completed, ["island0", "island1"]);
    }
    assert_eq!(isl.completed, ["island0"], "die Liste der Insel bleibt unverändert");
    isl.gate_open = false;
    mirror_switch(&mut isl);
    assert!(
        isl.stages.iter().all(|w| w.island_switch.is_none()),
        "geschlossen: kein Wechsel"
    );
    let s = mirror_json(&isl.stages[0]);
    assert!(!s.contains("islandSwitch"), "JSON: {s}");
}
