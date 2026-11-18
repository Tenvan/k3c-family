//! Inselwechsel (Go: `island_switch_test.go`, B-103, K2.3a). Alle zehn Go-Tests bauen eine Insel mit `create_island_in`
//! (`create_world_with`, RU14) oder lesen die Standard-Welt (`standard_world`, Geschichte: RU19) und tragen `ignore`. Die Tests ab
//! `own_` prüfen, was schon läuft: die Schritt-Abkürzungen von `step_island_switch`, `next_island` mit einem Ziel, das keines
//! ist, und das Zusammenspiel von `is_last`, `next_def` und der Wahl.
use super::switch_testutil::*;
use super::testutil::*;
use super::{Island, IslandDef, IslandOptions, add_island_player, next_island, set_options};
use crate::enemies::spawn_enemy;
use crate::state::World;

#[test]
fn inselwechsel_erst_nach_endboss() {
    let data = test_data();
    let (w, _) = test_islands(&data);
    let mut isl = gate_island(&data, &w, 2);
    isl.endboss_defeated = false;
    let ev = step_seconds(&mut isl, &data, 3.0);
    assert!(
        count(&ev, "islandGateOpen") == 0 && !isl.switch_ready,
        "Wechselpunkt vor dem Endboss"
    );
    isl.endboss_defeated = true;
    let ev = step_seconds(&mut isl, &data, 3.0);
    assert!(
        count(&ev, "islandGateOpen") == 1 && count(&ev, "islandSwitch") == 1 && isl.switch_ready,
        "{}x islandGateOpen, {}x islandSwitch, bereit {}",
        count(&ev, "islandGateOpen"),
        count(&ev, "islandSwitch"),
        isl.switch_ready
    );
    let def = isl.next_def();
    assert_eq!(def.map(|d| d.id), Some("test-island2".to_string()), "nächste Insel");
    assert_eq!(count(&ev, "victory"), 0, "victory vor der letzten Insel");
}

#[test]
fn inselwechsel_letzte_insel_ohne_wechsel() {
    let data = test_data();
    let mut isl = gate_island(&data, &standard_world(&data), 2);
    let ev = step_seconds(&mut isl, &data, 3.0);
    assert!(
        count(&ev, "islandGateOpen") == 0 && !isl.switch_ready && count(&ev, "victory") == 1,
        "letzte Insel: {}x islandGateOpen, bereit {}, {}x victory",
        count(&ev, "islandGateOpen"),
        isl.switch_ready,
        count(&ev, "victory")
    );
}

#[test]
fn inselwechsel_alle_lebenden_am_punkt() {
    type Setup = fn(&mut Island);
    let cases: Vec<(&str, usize, Setup, bool)> = vec![
        ("2 Spieler", 2, |_| {}, true),
        ("4 Spieler", 4, |_| {}, true),
        ("einer fehlt", 4, |i| i.stages[0].players[3].x += 20.0, false),
        (
            "einer tot",
            2,
            |i| {
                i.stages[0].players[1].x += 20.0;
                fell(&mut i.stages[0], 1);
                if let Some(c) = i.stages[0].players[1].crown.as_mut() {
                    c.x += 40.0;
                }
            },
            true,
        ),
        (
            "einer frei",
            2,
            |i| {
                i.stages[0].players[1].x += 20.0;
                i.stages[0].players[1].free = true;
            },
            true,
        ),
        (
            "einer in anderer Stufe",
            2,
            |i| {
                add_island_player(i, 1);
            },
            false,
        ),
    ];
    let data = test_data();
    for (name, n, setup, ready) in cases {
        let (w, _) = test_islands(&data);
        let mut isl = gate_island(&data, &w, n);
        setup(&mut isl);
        step_seconds(&mut isl, &data, data.hub.travel.seconds + 0.5);
        assert_eq!(
            isl.switch_ready, ready,
            "{name}: bereit {}, erwartet {ready}",
            isl.switch_ready
        );
    }
}

#[test]
fn inselwechsel_abbruch_beim_weggehen() {
    let data = test_data();
    let secs = data.hub.travel.seconds;
    let (w, _) = test_islands(&data);
    let mut isl = gate_island(&data, &w, 2);
    let gate = isl.stages[0].players[0].x;
    step_seconds(&mut isl, &data, 0.75 * secs);
    isl.stages[0].players[0].x = gate + 20.0;
    step_seconds(&mut isl, &data, DT);
    isl.stages[0].players[0].x = gate;
    step_seconds(&mut isl, &data, 0.75 * secs);
    assert!(!isl.switch_ready, "Fortschritt läuft nach dem Weggehen weiter");
    step_seconds(&mut isl, &data, 0.5 * secs);
    assert!(isl.switch_ready, "nach vollem Fortschritt nicht bereit");
}

/// Go: `switchedIsland`: Insel 1 (Grad hard, Ziel gold) mit zwei Spielern, Gold und Skills, Wechsel zur Test-Insel.
fn switched_island(data: &k3c_data::Data) -> (Island, Island) {
    let (w, def) = test_islands(data);
    let mut cur = gate_island(data, &w, 2);
    let o = IslandOptions {
        grade: "hard".into(),
        goal: "gold".into(),
        defeat: "resources".into(),
    };
    set_options(&mut cur, o, false).expect("Optionen");
    cur.stages[0].players[0].gold = 33;
    cur.stages[0].players[1].gold = 44;
    cur.stages[0].players[0].skills = vec!["a".into(), "b".into()];
    cur.stages[0].players[0].slots = vec!["a".into()];
    fell(&mut cur.stages[0], 1);
    let next = next_island(data, &cur, &def).expect("nächste Insel");
    (cur, next)
}

#[test]
fn inselwechsel_neue_insel() {
    let data = test_data();
    let (cur, next) = switched_island(&data);
    assert!(
        next.number == 1
            && next.id == cur.id
            && next.seed != cur.seed
            && next.options == cur.options
            && next.stages.len() == 2,
        "Insel {}, ID {:?}, Seed {:?}, Optionen {:?}, {} Stufen",
        next.number,
        next.id,
        next.seed,
        next.options,
        next.stages.len()
    );
    assert_eq!(next.stock, data.hub.island_start_stock, "Vorrat, erwartet Startvorrat");
    for w in &next.stages {
        assert_eq!(built_sites(w), 0, "Tiefe {}: gebaute Plätze", w.biome.depth);
    }
}

#[test]
fn inselwechsel_spieler_ziehen_mit() {
    let data = test_data();
    let (_, next) = switched_island(&data);
    let first = &next.stages[0];
    assert!(
        first.players.len() == 2 && next.stages[1].players.is_empty(),
        "Spieler in Stufe 0: {}, in Stufe 1: {}",
        first.players.len(),
        next.stages[1].players.len()
    );
    for (i, p) in first.players.iter().enumerate() {
        assert!(
            p.index == i as i64 && p.crown.is_none() && (p.x - first.hub_x).abs() <= 3.0,
            "Spieler {i}: Index {}, Krone {:?}, x {} (Burg {})",
            p.index,
            p.crown,
            p.x,
            first.hub_x
        );
    }
    let p0 = &first.players[0];
    assert!(
        p0.gold == 33 && first.players[1].gold == 44 && p0.skills == ["a", "b"] && p0.slots == ["a"],
        "Gold {}/{}, Skills {:?}, Slots {:?}",
        p0.gold,
        first.players[1].gold,
        p0.skills,
        p0.slots
    );
}

#[test]
fn inselwechsel_skill_basis() {
    let data = test_data();
    let (_, mut next) = switched_island(&data);
    assert_eq!(
        available_points(&next.stages[0], 0),
        0,
        "mitgebrachte Skills zählen gegen den neuen Pool"
    );
    add_island_pool(&mut next, 0, 2);
    assert_eq!(
        available_points(&next.stages[0], 0),
        2,
        "erwartet 2 aus dem Pool der neuen Insel"
    );
    let pi = add_island_player(&mut next, 0);
    assert_eq!(
        next.stages[0].players[pi].index, 2,
        "neuer Spieler mit Index 2 erwartet"
    );
}

#[test]
fn inselwechsel_tabelle_der_insel() {
    let data = test_data();
    let (mut cur, mut next) = switched_island(&data);
    let goblin = |isl: &mut Island| {
        on_stage(isl, &data, 1, |w: &mut World, r| {
            let ei = spawn_enemy(w, "goblin", 0.0, Some(r));
            w.enemies[ei].max_hp
        })
    };
    let (old, neu) = (goblin(&mut cur), goblin(&mut next));
    let base = data.enemies["goblin"].hp;
    assert!(
        old == (base * 1.5).round() && neu == (base * 2.0).round(),
        "Goblin in Tiefe 1: Insel 1 {old}, Test-Insel {neu} (Basis {base})"
    );
}

/// Die Standard-Welt hat eine Geschichte (`story`); `standard_world` meldet bis RU19 den Platzhalter.
#[test]
fn inselwechsel_daten() {
    let data = test_data();
    let d = &standard_world(&data).islands[0];
    assert_eq!(d.depths, [0, 1, 2, 3, 4], "Insel 1: Tiefen");
    assert!(
        data.bosses.end_bosses.iter().any(|b| b.depth == 4),
        "Insel 1: Endboss in Tiefe 4"
    );
    for depth in &d.depths {
        assert!(
            data.bosses.bosses.iter().any(|b| b.kind == "mini" && b.depth == *depth),
            "Insel 1: kein Miniboss in Tiefe {depth}"
        );
    }
}

/// Der Rest von `inselwechsel_daten` (Go: Welten, die das Laden ablehnt) läuft ohne Standard-Welt.
#[test]
fn own_inselwechsel_ungueltige_welten_werden_abgelehnt() {
    let bad = [
        (
            "doppelt",
            test_world(&format!("{},{}", test_island("a", ""), test_island("a", ""))),
        ),
        (
            "Faktor 0",
            test_world(&test_island("a", r#","depthScaling":{"hp":0,"damage":1,"speed":1}"#)),
        ),
        ("Tiefe fehlt", test_world(&test_island("a", r#","depths":[4,9]"#))),
        ("leer", test_world("")),
    ];
    for (name, raw) in bad {
        assert!(parse_world(&raw, "test").is_err(), "{name}: kein Fehler beim Laden");
    }
}

#[test]
fn inselwechsel_deterministisch() {
    let data = test_data();
    let (_, mut a) = switched_island(&data);
    let (_, mut b) = switched_island(&data);
    step_seconds(&mut a, &data, 5.0);
    step_seconds(&mut b, &data, 5.0);
    assert_eq!(island_json(&a), island_json(&b), "gleicher Seed, andere neue Insel");
}

/// Eine Test-Welt aus Hand: Insel 1 (Etappe 1) und eine zweite Insel.
fn two_island_island(endboss: bool) -> (Island, IslandDef) {
    let mut w = super::save_tests::world_of(&["island1", "test-island2"]);
    w.id = "test".into();
    let def = w.islands[1].clone();
    let isl = Island {
        world: w,
        endboss_defeated: endboss,
        ..Island::default()
    };
    (isl, def)
}

#[test]
fn own_step_island_switch_ohne_endboss_oder_auf_letzter_insel_tut_nichts() {
    let (mut isl, _) = two_island_island(false);
    super::step_island_switch(&mut isl, 1.0);
    assert!(
        !isl.gate_open && !isl.switch_ready && isl.switch_progress == 0.0,
        "vor dem Endboss"
    );
    let mut last = Island {
        world: super::save_tests::world_of(&["island1"]),
        endboss_defeated: true,
        ..Island::default()
    };
    super::step_island_switch(&mut last, 1.0);
    assert!(!last.gate_open && last.is_last(), "letzte Insel: kein Wechselpunkt");
    let (mut ready, _) = two_island_island(true);
    ready.switch_ready = true;
    ready.pending_choice.insert(0, "test-island2".into());
    super::step_island_switch(&mut ready, 1.0);
    assert_eq!(
        ready.pending_choice.len(),
        1,
        "bereit: Wahlen werden nicht mehr übernommen"
    );
}

#[test]
fn own_next_def_ist_das_erste_ziel_und_folgt_der_wahl() {
    let (mut isl, def) = two_island_island(true);
    assert_eq!(isl.next_def().map(|d| d.id), Some(def.id.clone()));
    assert!(!isl.is_last());
    isl.chosen = "gibtsnicht".into();
    assert_eq!(isl.chosen_target(), def.id, "unbekannte Wahl gilt als erstes Ziel");
    let last = Island {
        world: super::save_tests::world_of(&["island1"]),
        ..Island::default()
    };
    assert!(last.next_def().is_none() && last.chosen_target().is_empty());
}

#[test]
fn own_next_island_lehnt_ein_nicht_gewaehltes_ziel_ab() {
    let data = test_data();
    let (isl, def) = two_island_island(true);
    let other = IslandDef {
        id: "gibtsnicht".into(),
        ..def.clone()
    };
    let err = next_island(&data, &isl, &other).expect_err("kein Ziel");
    assert_eq!(
        err,
        "insel: \"gibtsnicht\" ist nicht das Ziel des Wechsels in Welt \"test\""
    );
    let last = Island {
        world: super::save_tests::world_of(&["island1"]),
        ..Island::default()
    };
    assert!(next_island(&data, &last, &def).is_err(), "letzte Insel hat kein Ziel");
}

#[test]
fn own_step_island_leere_insel_ruft_nichts_auf() {
    // Ohne Stufen rechnen Reise, Wechsel und Sieg nichts und fassen keinen Platzhalter an.
    let mut isl = Island::default();
    isl.options.goal = "endboss".into();
    super::step_island_travel(&mut isl, 0.1);
    super::check_victory(&mut isl);
    assert!(!isl.won);
}
