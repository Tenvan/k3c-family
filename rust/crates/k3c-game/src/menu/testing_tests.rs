//! Tests aus `native/k3c-game/menu/testing_test.go` (B-896). `TestTestingOpensPadTest` steht in `padtest_tests.rs`,
//! `TestMainTestingOnlyForDevelopers` steht hier (Go: ebenfalls in `testing_test.go`).

use std::cell::RefCell;
use std::rc::Rc;

use super::layout::MAX_ROWS;
use super::main_screen::{MAIN_TESTING, new_main};
use super::stress_presets::{load_stress_presets, parse_stress_presets};
use super::testing::{
    TST_BACK, TST_ENEMIES, TST_FACTOR, TST_GROWTH, TST_MODE, TST_PLAYERS, TST_ROWS, TST_START, TST_TIMESCALE,
    TestingScreen, change_stress, new_testing,
};
use super::testutil::{FakeSession, KEYS1, PAD1, new_menus, press, step_through, version_guard, with_menus};
use super::texts::TextKey;
use crate::view::MAX_PLAYERS;
use crate::view::input::{Action, DeviceID, Input};
use crate::view::screen::{Screen, StepKind};
use crate::view::settings::DEFAULT_LANGUAGE;
use crate::view::stress::{GROWTH_EXP, STRESS_MAX_ENEMIES, StressStarter, StressTest};

type Started = Rc<RefCell<Vec<(StressTest, DeviceID)>>>;

/// Schein-Starter (Go: `starterSession`): schreibt den Start mit, Fehler setzbar.
#[derive(Clone, Default)]
struct FakeStarter {
    started: Started,
    err: Rc<RefCell<Option<String>>>,
}

impl StressStarter for FakeStarter {
    fn start_stress(&mut self, t: &StressTest, by: DeviceID) -> Result<(), String> {
        self.started.borrow_mut().push((t.clone(), by));
        match self.err.borrow().clone() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

/// Die Seite mit Schein-Starter, schon geladen (erstes Update).
fn with_starter() -> (TestingScreen, FakeSession, FakeStarter) {
    let m = new_menus();
    let fs = FakeStarter::default();
    with_menus(&m, |x| x.set_stress_starter(Box::new(fs.clone())));
    let mut sc = new_testing(&m);
    let mut s = FakeSession::default();
    sc.update(&Input::default(), &mut s);
    (sc, s, fs)
}

fn hit(sc: &mut TestingScreen, s: &mut FakeSession, d: DeviceID, a: Action) -> StepKind {
    sc.update(&press(d, &[a]), s).kind
}

/// B-896/AC-01: Die Seite zeigt alle Einstellungen von B-896 (rollend, `MAX_ROWS` sichtbar), Start und Zurück.
#[test]
fn testing_shows_all_rows() {
    let (mut sc, mut s, _) = with_starter();
    let mut labels = Vec::new();
    for _ in 0..TST_ROWS {
        sc.fill(DEFAULT_LANGUAGE);
        {
            let pg = sc.pg.borrow();
            labels.push(pg.rows[pg.sel as usize].label.clone());
            assert!(pg.n <= MAX_ROWS, "{} sichtbare Zeilen", pg.n);
        }
        hit(&mut sc, &mut s, PAD1, Action::Down);
    }
    let want = [
        "Vorgabe",
        "Spieler",
        "Stufe (Tiefe)",
        "Seed",
        "Gegner je Welle",
        "Anstieg",
        "Faktor je Welle",
        "Ausbau vorab",
        "Truppen vorab",
        "Zeitraffer",
        "Ende nach Wellen",
        "Ende nach Minuten",
        "Modus",
        "Test starten",
        "Tasten-Test",
        "Zurück",
    ];
    assert_eq!(labels, want, "Zeilen");
}

/// B-896/AC-01: Pad und Tastatur ändern Werte (links/rechts begrenzt, A im Kreis); eine Änderung macht „eigene Werte“.
#[test]
fn testing_changes_values() {
    let (mut sc, mut s, _) = with_starter();
    hit(&mut sc, &mut s, PAD1, Action::Right); // Vorgabe 1
    assert!(
        sc.cur.preset == "vieleGegner" && sc.cur.enemies == 40,
        "Vorgabe übernommen: {:?}",
        sc.cur
    );
    sc.sel = TST_ENEMIES;
    hit(&mut sc, &mut s, KEYS1, Action::Right);
    assert!(
        sc.cur.enemies == 45 && sc.cur.preset.is_empty() && sc.preset == -1,
        "Tastatur rechts: {:?}",
        sc.cur
    );
    sc.sel = TST_GROWTH;
    hit(&mut sc, &mut s, PAD1, Action::Confirm);
    assert_eq!(sc.cur.growth, GROWTH_EXP, "Anstieg nicht gekippt");
    sc.sel = TST_PLAYERS;
    for _ in 0..5 {
        hit(&mut sc, &mut s, PAD1, Action::Right);
    }
    assert_eq!(sc.cur.players, MAX_PLAYERS as i64, "Spieler über dem Rand");
    sc.sel = TST_TIMESCALE;
    hit(&mut sc, &mut s, PAD1, Action::Confirm);
    assert_ne!(sc.cur.timescale, 4, "Zeitraffer unverändert");
    sc.sel = TST_MODE; // B-921: Modus kippt zwischen normal und rein
    hit(&mut sc, &mut s, PAD1, Action::Confirm);
    assert!(
        sc.cur.pure && sc.value_text("de", TST_MODE) == "rein (Gegner an der Burg)",
        "Modus nicht rein: {:?}",
        sc.cur
    );
    hit(&mut sc, &mut s, PAD1, Action::Confirm);
    assert!(!sc.cur.pure, "Modus nicht zurück auf normal");
}

/// B-896/AC-01: Starten merkt die Werte in den Einstellungen und startet über `StressStarter`; ohne die Rolle oder bei
/// Fehler bleibt die Seite mit Meldung.
#[test]
fn testing_start() {
    let (mut sc, mut s, fs) = with_starter();
    hit(&mut sc, &mut s, PAD1, Action::Right);
    sc.sel = TST_START;
    let kind = hit(&mut sc, &mut s, PAD1, Action::Confirm);
    {
        let st = fs.started.borrow();
        assert!(
            kind == StepKind::Play && st.len() == 1 && st[0].0.preset == "vieleGegner" && st[0].1 == PAD1,
            "Start: {kind:?}, {:?}",
            *st
        );
        assert!(
            s.settings.len() == 1 && s.settings[0].stress == st[0].0,
            "Werte nicht gemerkt: {:?}",
            s.settings
        );
    }
    *fs.err.borrow_mut() = Some("kaputt".to_string());
    let kind = hit(&mut sc, &mut s, PAD1, Action::Confirm);
    assert!(
        kind == StepKind::Stay && sc.base.note.bad,
        "Fehler beim Start: {kind:?}"
    );
    assert_eq!(sc.base.note.raw, "kaputt");

    let mut plain = new_testing(&new_menus());
    let mut s2 = FakeSession::default();
    plain.update(&Input::default(), &mut s2);
    plain.sel = TST_START;
    let kind = hit(&mut plain, &mut s2, PAD1, Action::Confirm);
    assert!(
        kind == StepKind::Stay && plain.base.note.key == TextKey::TstNoRunner,
        "ohne Rolle: {kind:?}"
    );
}

/// B-1117/AC-01: `MenuSet::set_stress_starter` (auch über die Rolle `Menus`) gibt der Testing-Seite den Starter: Start
/// ruft ihn auf, statt `TstNoRunner` zu zeigen.
#[test]
fn menu_set_gives_starter_to_testing() {
    use crate::view::screen::Menus;
    let mut set = super::new_set(None, None);
    let fs = FakeStarter::default();
    Menus::set_stress_starter(&mut set, Box::new(fs.clone()));
    let mut sc = new_testing(&set.shared);
    let mut s = FakeSession::default();
    sc.update(&Input::default(), &mut s);
    sc.sel = TST_START;
    let kind = hit(&mut sc, &mut s, PAD1, Action::Confirm);
    assert_eq!(kind, StepKind::Play, "Start");
    assert_eq!(fs.started.borrow().len(), 1);
    assert_ne!(sc.base.note.key, TextKey::TstNoRunner);
}

/// Die Werte folgen den Kurven: linear + Faktor, exponentiell × Faktor, gedeckelt.
#[test]
fn testing_value_texts() {
    let mut sc = new_testing(&new_menus());
    sc.update(&Input::default(), &mut FakeSession::default());
    sc.cur = StressTest {
        players: 1,
        growth: GROWTH_EXP.to_string(),
        factor: 1.5,
        timescale: 4,
        ..StressTest::default()
    }
    .normalized();
    let got = [
        sc.value_text("de", TST_GROWTH),
        sc.value_text("de", TST_FACTOR),
        sc.value_text("de", TST_TIMESCALE),
        sc.value_text("de", super::testing::TST_WAVES),
        sc.value_text("de", super::testing::TST_SEED),
    ];
    assert_eq!(
        got.join("|"),
        "exponentiell|× 1.5|4×|keine|Name des Spielstands",
        "Werte {got:?}"
    );
}

/// Die Faktor-Anzeige lautet wie `FormatFloat(f, 'f', -1, 64)`: 1.5, 2 (ohne Nachkomma), 0.25.
#[test]
fn testing_factor_display_matches_go() {
    let mut sc = new_testing(&new_menus());
    for (growth, f, want) in [
        (GROWTH_EXP, 1.5, "× 1.5"),
        (GROWTH_EXP, 2.0, "× 2"),
        ("linear", 0.25, "+ 0.25"),
        ("linear", 5.0, "+ 5"),
    ] {
        sc.cur.growth = growth.to_string();
        sc.cur.factor = f;
        assert_eq!(sc.value_text("de", TST_FACTOR), want, "Faktor {f}");
    }
}

/// Faktor: linear in 1er-, exponentiell in 0,1er-Schritten; A springt über den Höchstwert auf 0.
#[test]
fn testing_factor_steps() {
    let lin = StressTest {
        factor: 5.0,
        ..StressTest::default()
    };
    assert_eq!(change_stress(lin.clone(), TST_FACTOR, 1, false).factor, 6.0);
    let exp = StressTest {
        growth: GROWTH_EXP.to_string(),
        factor: 1.5,
        ..StressTest::default()
    };
    assert_eq!(change_stress(exp, TST_FACTOR, -1, false).factor, 1.4);
    let top = StressTest {
        factor: 50.0,
        ..StressTest::default()
    };
    assert_eq!(change_stress(top.clone(), TST_FACTOR, 1, true).factor, 0.0);
    assert_eq!(change_stress(top, TST_FACTOR, 1, false).factor, 50.0);
}

/// Zurück: B und Esc (Back, Menu, Home) sowie die Zeile Zurück gehen ins Hauptmenü.
#[test]
fn testing_back_pops() {
    for a in [Action::Back, Action::Menu, Action::Home] {
        let (mut sc, mut s, _) = with_starter();
        assert_eq!(hit(&mut sc, &mut s, PAD1, a), StepKind::Pop, "{a:?}");
    }
    let (mut sc, mut s, _) = with_starter();
    sc.sel = TST_BACK;
    assert_eq!(hit(&mut sc, &mut s, KEYS1, Action::Confirm), StepKind::Pop);
}

/// Kaputte Vorgaben: keine Vorgaben, keine ohne Id oder Name, Werte begrenzt.
#[test]
fn parse_stress_presets_cases() {
    assert!(parse_stress_presets(b"{").is_err(), "kaputte Datei ohne Fehler");
    let raw = br#"{"presets":[{"id":"a","name":"A","enemies":999,"players":9},{"name":"ohne Id"}]}"#;
    let ps = parse_stress_presets(raw);
    let ok = ps.as_ref().is_ok_and(|ps| {
        ps.len() == 1
            && ps[0].stress.enemies == STRESS_MAX_ENEMIES
            && ps[0].stress.players == MAX_PLAYERS as i64
            && ps[0].stress.preset == "a"
    });
    assert!(ok, "Vorgaben {ps:?}");
    let real = load_stress_presets();
    assert!(
        real.as_ref().is_ok_and(|r| r.len() >= 3),
        "data/stresstests.json: {real:?}"
    );
}

/// Der Name folgt der Sprache; ohne englischen Namen gilt der deutsche.
#[test]
fn preset_name_per_language() {
    let raw = br#"{"presets":[{"id":"a","name":"Eins","nameEn":"One"},{"id":"b","name":"Zwei"}]}"#;
    let ps = parse_stress_presets(raw).unwrap_or_default();
    assert_eq!(ps.len(), 2);
    assert_eq!((ps[0].name("de"), ps[0].name("en")), ("Eins", "One"));
    assert_eq!((ps[1].name("de"), ps[1].name("en")), ("Zwei", "Zwei"));
}

/// Die Karte Testing gibt es nur im Entwickler-Modus; die Navigation überspringt sie sonst (Go:
/// `TestMainTestingOnlyForDevelopers`).
#[test]
fn main_testing_only_for_developers() {
    let _g = version_guard();
    let mut sc = new_main(&new_menus());
    sc.dev = false;
    sc.fill("de");
    {
        let pg = sc.pg.borrow();
        assert!(
            pg.rows[..pg.n].iter().all(|r| r.label != "Testing"),
            "Testing ohne Entwickler-Modus sichtbar"
        );
    }
    assert!(sc.off(MAIN_TESTING), "Testing ohne Entwickler-Modus wählbar");
    let mut s = FakeSession::default();
    let mut sc = new_main(&new_menus());
    let down = press(PAD1, &[Action::Down]);
    let st = step_through(
        &mut sc,
        &mut s,
        &[down.clone(), down.clone(), down, press(PAD1, &[Action::Confirm])],
    );
    assert!(
        st.kind == StepKind::Push && st.next.is_some() && sc.sel == MAIN_TESTING,
        "Hauptmenü › Testing: {:?}, Auswahl {}",
        st.kind,
        sc.sel
    );
}
