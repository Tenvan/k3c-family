//! Tests zu `island_world.rs` und `scope.rs` (Port von `engine/balance/island_world_test.go`, BS11).

use k3c_data::balance_targets::{FILE, load_targets};
use k3c_sim::island::add_island_player;
use serde_json::json;

use crate::figures::figures_of;
use crate::island_world::{create_in, load_world, shared, worlds};
use crate::metrics::{Collector, Metrics};
use crate::run::{Matrix, Report, RunResult, Scenario, new_island, run};
use crate::scope::{Restrict, Scope};

fn scenario(world: &str, players: i64) -> Scenario {
    Scenario {
        seed: "1".into(),
        players,
        days: 1,
        world: world.into(),
        ..Scenario::default()
    }
}

/// BS11.1/AC-01 (B-642/AC-01): Mit Welt hat die Insel alle Tiefen der ersten Insel, ohne Welt wie bisher eine Stufe.
#[test]
fn welt_baut_alle_tiefen() {
    let isl = new_island(&scenario("standard", 2)).expect("Insel");
    let want = &shared().expect("Daten").standard.islands[0].depths;
    assert!(
        isl.stages.len() == 5 && isl.stages.len() == want.len(),
        "Stufen {}, Tiefen {want:?}",
        isl.stages.len()
    );
    for (i, w) in isl.stages.iter().enumerate() {
        assert_eq!(w.biome.depth, want[i], "Tiefe der Stufe {i}");
    }
    assert_eq!(isl.stages[0].players.len(), 2, "Monarchen auf Stufe 0");
    let old = new_island(&scenario("", 2)).expect("ohne Welt");
    assert_eq!(old.stages.len(), 1, "ohne Welt eine Stufe");
    assert!(
        new_island(&scenario("gibtsnicht", 1)).is_err(),
        "unbekannte Welt muss ein Fehler sein"
    );
}

/// Die Welt `standard` steht in der Liste der Welten.
#[test]
fn welten_nennt_standard() {
    assert!(worlds().expect("Welten").iter().any(|w| w == "standard"));
}

/// Ein Lauf ohne Welt schreibt keine Insel-Felder; mit Welt stehen die Stufen im Ergebnis.
#[test]
fn insel_lauf_metriken() {
    let m = |world: &str| Matrix {
        seeds: vec!["1".into()],
        players: vec![2],
        bots: vec!["saver".into()],
        depths: vec![0],
        days: 1,
        world: world.into(),
        ..Matrix::default()
    };
    let r = run(&m("standard")).expect("Lauf");
    let res = &r.results[0];
    let met = res.metrics.as_ref().filter(|_| res.valid).expect("gültiger Lauf");
    assert_eq!(met.stages.len(), 5, "Stufen");
    let s = &met.stages[0];
    assert!(
        s.arrival_tick == Some(0) && s.arrival_day == Some(1),
        "Stufe 0 beginnt mit Ankunft Tag 1: {s:?}"
    );
    assert!(
        met.stages[1].arrival_tick.is_none(),
        "Stufe 1 ohne Spieler: {:?}",
        met.stages[1]
    ); // saver steigt nicht ab
    let plain = run(&m("")).expect("Lauf").json().expect("JSON");
    for k in ["stages", "bosses", "endbossDay", "\"world\""] {
        assert!(!plain.contains(k), "Lauf ohne Welt enthält {k}");
    }
}

/// BS11.1/AC-02: Der Sammler führt Ankunft, Bosse, Endboss und Burgfall je Stufe nach festem Ereignisverlauf.
#[test]
fn sammler_insel_ereignisse() {
    let mut isl = new_island(&scenario("standard", 1)).expect("Insel");
    let mut c = Collector::new(&isl, 0);
    add_island_player(&mut isl, 2); // ein zweiter Monarch steht auf Stufe 2
    c.observe_island(&isl, 100);
    let ev = |v: serde_json::Value| v.as_object().cloned().expect("Ereignis");
    c.island_event(
        &isl,
        &isl.stages[2],
        &ev(json!({"type": "bossDefeated", "boss": "mini2"})),
        120,
    );
    c.island_event(&isl, &isl.stages[2], &ev(json!({"type": "castleFallen"})), 130);
    isl.endboss_defeated = true;
    c.observe_island(&isl, 200);
    let m = c.result(&isl);
    let s = &m.stages[2];
    assert!(
        s.arrival_tick == Some(100) && s.castle_fall_tick == Some(130),
        "Stufe 2: {s:?}"
    );
    assert!(
        m.bosses.len() == 1 && m.bosses[0].boss == "mini2" && m.bosses[0].tick == 120,
        "Bosse {:?}",
        m.bosses
    );
    assert_eq!(m.endboss_tick, Some(200), "Endboss");
    check_insel_bericht(&m);
}

/// Kennzahlen und Markdown zeigen Tiefen, Bosse und Endboss (Go: `checkInselBericht`).
fn check_insel_bericht(m: &Metrics) {
    let r = Report {
        matrix: Matrix::default(),
        results: vec![RunResult {
            scenario: Scenario::default(),
            valid: true,
            error: String::new(),
            ticks: 0,
            metrics: Some(m.clone()),
            replay: None,
        }],
    };
    let f = figures_of(&r).expect("Kennzahlen");
    let isl = f.island.as_ref().expect("Insel-Kennzahlen");
    assert!(
        isl.depths.len() == 5 && isl.endboss.defeated == 100.0 && isl.bosses.len() == 1,
        "Kennzahlen {f:?}"
    );
    let mut b = String::new();
    isl.markdown(&mut b);
    assert!(
        b.contains("| mini2 | 0 | 100") && b.contains("| 4 |"),
        "Markdown ohne Tiefen und Bosse: {b}"
    );
}

/// BS11.4: `Scope.world` lässt alle Ziele die ganze Insel spielen; jede andere Einschränkung ohne Welt entfernt die Insel-Ziele.
#[test]
fn scope_welt() {
    let raw = shared().expect("Daten").source.read(FILE).expect("Datei der Ziele");
    let ts = load_targets(&raw).expect("Ziele");
    let w = ts
        .restrict(&Scope {
            days: Some(40),
            bot: "coop2".into(),
            world: "standard".into(),
            ..Scope::default()
        })
        .expect("Umfang mit Welt");
    assert_eq!(w.targets.len(), ts.targets.len(), "Zahl der Ziele");
    for g in &w.targets {
        assert!(
            g.world == "standard" && g.days == 40 && g.bot == "coop2",
            "Ziel {:?}: {g:?}",
            g.name
        );
    }
    let one = ts
        .restrict(&Scope {
            days: Some(40),
            ..Scope::default()
        })
        .expect("Umfang ohne Welt");
    for g in &one.targets {
        assert!(g.world.is_empty(), "Ziel {:?} behält die Welt", g.name);
    }
}

/// BS11.5: Der Insel-Lauf ist deterministisch: gleiche Matrix, gleiche Bytes.
#[test]
fn insel_lauf_deterministisch() {
    let m = Matrix {
        seeds: vec!["k3c-dev".into()],
        players: vec![2],
        bots: vec!["coop2".into()],
        depths: vec![0],
        days: 1,
        world: "standard".into(),
        ..Matrix::default()
    };
    let a = run(&m).expect("Lauf").json().expect("JSON");
    let b = run(&m).expect("Lauf").json().expect("JSON");
    assert_eq!(a, b, "zweiter Insel-Lauf liefert andere Bytes");
}

/// B-692: Der Balancing-Lauf löst `extends` und `overrides` der Welt auf, Eltern aus data/worlds.
#[test]
fn welt_mit_overrides_im_lauf() {
    let mut w = load_world("standard").expect("Welt");
    w.id = "teuer".into();
    w.extends = "standard".into();
    w.overrides.insert(
        "economy".into(),
        serde_json::json!({"purse": {"startGold": 27, "maxGold": 40}}),
    );
    let mut isl = create_in(&w, "1").expect("Insel");
    let pi = add_island_player(&mut isl, 0);
    assert_eq!(isl.stages[0].players[pi].gold, 27, "Startgold aus den Overrides");
    w.extends = "fehlt".into();
    let err = create_in(&w, "1").expect_err("unbekannte Eltern-Welt");
    assert!(err.to_string().contains("extends"), "Fehler {err}");
}
