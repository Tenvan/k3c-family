//! Tests aus `native/k3c-game/menu/main_test.go`, dazu `TestMainShowsOutcomeOfFrame` (`pause_test.go`) und die zwei
//! Hauptmenü-Tests aus `table_test.go`. Tests, die einen Screen aus RU12.2 oder RU12.3 öffnen (Testing, Lizenzen), warten darauf.

use super::MenuSet;
use super::keys::Note;
use super::main_screen::{MAIN_CONTINUE, MAIN_LICENSES, MAIN_LOAD, MAIN_NEW, MAIN_QUIT, new_main, outcome_note};
use super::pause::{PAUSE_LEAVE, new_pause};
use super::testutil::{
    FakeSession, KEYS1, PAD1, PAD2, err_fake, has_cue, new_menus, press, save, step_through, version_guard,
};
use super::texts::{TextKey, tr};
use super::version::{set_version, version};
use crate::view::frame::{CueKind, Outcome, Seat};
use crate::view::input::{Action, DeviceInput, Input};
use crate::view::screen::{Screen, StepKind};
use crate::view::session::{SaveInfo, Session};

fn upd(sc: &mut dyn Screen, s: &mut dyn Session, a: Action) -> StepKind {
    sc.update(&press(PAD1, &[a]), s).kind
}

#[test]
fn main_selection_starts_at_continue_only_with_loadable_save() {
    let cases: [(&str, Vec<SaveInfo>, i64); 3] = [
        ("keine Stände", vec![], MAIN_NEW),
        ("nur kaputte", vec![save("alt", 1, false)], MAIN_NEW),
        (
            "ladbarer Stand",
            vec![save("alt", 1, false), save("spiel-1", 4, true)],
            MAIN_CONTINUE,
        ),
    ];
    for (name, saves, want) in cases {
        let mut sc = new_main(&new_menus());
        sc.update(
            &Input::default(),
            &mut FakeSession {
                saves,
                ..Default::default()
            },
        );
        assert_eq!(sc.sel, want, "{name}");
    }
}

#[test]
fn main_moves_and_skips_disabled_continue() {
    use Action::{Back, Down, Home, Up};
    let cases: [(&str, Vec<SaveInfo>, Vec<Action>, i64); 6] = [
        ("runter", vec![], vec![Down], MAIN_LOAD),
        ("hoch über ausgegrautes Weiter bleibt", vec![], vec![Up], MAIN_NEW),
        ("unten bleibt am Rand", vec![], vec![Down; 6], MAIN_QUIT),
        (
            "hoch zu Weiter mit Stand",
            vec![save("a", 1, true)],
            vec![Down, Up],
            MAIN_CONTINUE,
        ),
        ("Home springt zum Anfang", vec![], vec![Down, Down, Home], MAIN_NEW),
        ("B im Hauptmenü bleibt", vec![], vec![Down, Back], MAIN_LOAD),
    ];
    for (name, saves, keys, want) in cases {
        let mut sc = new_main(&new_menus());
        let mut s = FakeSession {
            saves,
            ..Default::default()
        };
        for a in keys {
            assert_eq!(
                upd(&mut sc, &mut s, a),
                StepKind::Stay,
                "{name}: Taste {a:?} verlässt das Hauptmenü"
            );
        }
        assert_eq!(sc.sel, want, "{name}");
    }
}

/// B und Esc sind im Hauptmenü ohne Wirkung (die Wurzel), an Pad und Tastatur.
#[test]
fn main_ignores_back() {
    let mut sc = new_main(&new_menus());
    let mut s = FakeSession::default();
    let esc = Input {
        devices: vec![DeviceInput {
            device: KEYS1,
            actions: vec![Action::Back, Action::Menu],
            ..Default::default()
        }],
    };
    let st = sc.update(&esc, &mut s);
    assert!(
        st.kind == StepKind::Stay && st.cues.is_empty(),
        "Esc verlässt das Hauptmenü"
    );
    let st = sc.update(&press(PAD1, &[Action::Back]), &mut s);
    assert!(
        st.kind == StepKind::Stay && st.cues.is_empty(),
        "B verlässt das Hauptmenü"
    );
}

/// Weiter spielt, Beenden beendet, Lizenzen öffnet die Tabelle; jeder Schritt trägt den Cue Confirm.
#[test]
fn main_confirm_play_quit_licenses() {
    let cases = [
        ("Weiter spielt", MAIN_CONTINUE, StepKind::Play, false),
        ("Lizenzen öffnen die Tabelle", MAIN_LICENSES, StepKind::Push, true),
        ("Beenden", MAIN_QUIT, StepKind::Quit, false),
    ];
    for (name, sel, kind, has_next) in cases {
        let mut sc = new_main(&new_menus());
        let mut s = FakeSession {
            saves: vec![save("spiel-1", 2, true)],
            ..Default::default()
        };
        sc.update(&Input::default(), &mut s);
        sc.sel = sel;
        let st = sc.update(&press(PAD2, &[Action::Confirm]), &mut s);
        assert!(
            st.kind == kind && st.next.is_some() == has_next && has_cue(&st, CueKind::Confirm),
            "{name}: Schritt {:?}",
            st.kind
        );
    }
}

/// Neues Spiel (Weltauswahl), Laden (Liste) und Optionen öffnen ihre Screens; die Tabelle läuft erst, wenn RU12.3 das
/// Testing-Menü (`new_testing`) liefert.
#[test]
#[ignore = "wartet auf RU12.3"]
fn main_confirm_transitions() {}

#[test]
fn main_continue_uses_confirming_device() {
    let mut sc = new_main(&new_menus());
    let mut s = FakeSession {
        saves: vec![save("spiel-2", 5, true)],
        ..Default::default()
    };
    let st = sc.update(&press(PAD2, &[Action::Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Play && s.conts == 1 && s.devs.len() == 1 && s.devs[0][0] == PAD2,
        "Weiter: {:?}, Geräte {:?}",
        st.kind,
        s.devs
    );
    assert_eq!(sc.latest.name, "spiel-2");
}

#[test]
fn main_continue_error_stays_with_message() {
    let mut sc = new_main(&new_menus());
    let mut s = FakeSession {
        saves: vec![save("a", 1, true)],
        ..Default::default()
    };
    sc.update(&Input::default(), &mut s);
    s.saves.clear(); // inzwischen gelöscht: Continue scheitert
    let st = sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Stay && has_cue(&st, CueKind::Denied) && sc.base.note.bad && !sc.base.note.raw.is_empty(),
        "Fehler: {:?}, Meldung {:?}",
        st.kind,
        sc.base.note
    );
    sc.update(&Input::default(), &mut s);
    assert!(
        sc.sel == MAIN_NEW && sc.off(MAIN_CONTINUE),
        "nach dem Fehler neu gelesen: Auswahl {}",
        sc.sel
    );
}

#[test]
fn main_list_error_is_shown() {
    let mut sc = new_main(&new_menus());
    sc.update(
        &Input::default(),
        &mut FakeSession {
            list_err: Some(err_fake()),
            ..Default::default()
        },
    );
    assert!(
        sc.base.note.bad && sc.base.note.raw == err_fake().to_string(),
        "Meldung {:?}",
        sc.base.note
    );
}

/// Nach dem Laden-Menü (Liste, in der gelöscht wurde) liest das Hauptmenü die Spielstände neu.
#[test]
fn main_rereads_list_after_load_menu() {
    let mut sc = new_main(&new_menus());
    let mut s = FakeSession {
        saves: vec![save("a", 1, true)],
        ..Default::default()
    };
    sc.update(&Input::default(), &mut s);
    sc.sel = MAIN_LOAD;
    sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    s.saves.clear(); // in der Liste gelöscht
    sc.update(&Input::default(), &mut s);
    assert!(sc.off(MAIN_CONTINUE), "Weiter bleibt nach Löschen wählbar");
}

#[test]
fn main_shows_notice_once() {
    let shared = new_menus();
    shared.borrow_mut().notice = Note {
        key: TextKey::Lost,
        set: true,
        bad: true,
        ..Note::default()
    };
    let mut set = MenuSet { shared };
    assert_eq!(set.main_screen().base.note.key, TextKey::Lost, "Meldung fehlt");
    assert!(!set.main_screen().base.note.set, "Meldung erscheint zweimal");
}

/// Version im Hauptmenü (B-808 AC-02): unter dem Titel, ohne Angabe „unbekannt“. Die Lizenzdatei prüft RU12.3.
#[test]
fn main_shows_version() {
    let _g = version_guard();
    let old = version();
    for (v, want) in [("", "Version unbekannt"), ("v0.9-3-gabc", "Version v0.9-3-gabc")] {
        set_version(v);
        let sc = new_main(&new_menus());
        sc.fill("de");
        let got = sc.pg.borrow().sub.clone();
        set_version(&old);
        assert_eq!(got, want, "Version {v:?}");
    }
}

/// Erstes Bild vor dem ersten Update (B-813 AC-03): „Weiter“ zeigt „lädt“, nie „Kein Spielstand vorhanden“.
#[test]
fn main_first_draw_shows_loading() {
    let fresh = new_main(&new_menus());
    fresh.fill("de");
    let pg = fresh.pg.borrow();
    let row = &pg.rows[MAIN_CONTINUE as usize];
    assert!(
        row.value == tr("de", TextKey::ContinueLoading) && !row.off,
        "vor dem ersten Update: {:?}",
        row.value
    );
}

/// Erstes Bild nach „Spiel verlassen“ (B-813 AC-03, Go: `TestMainFirstDrawAfterLeave`): Das Hauptmenü kennt den
/// Spielstand schon vor seinem ersten Update; ein Hauptmenü ohne gelesene Liste zeigt „lädt“, nie „Kein Spielstand
/// vorhanden“. Rust hat kein Downcast auf `Box<dyn Screen>`: Das Hauptmenü kommt aus `PauseScreen::leave`.
#[test]
fn main_first_draw_after_leave() {
    let mut s = FakeSession {
        active: true,
        saves: vec![save("burg", 3, true)],
        seats: vec![Seat {
            player: 0,
            device: PAD1,
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut pause = new_pause(&new_menus());
    pause.sel = PAUSE_LEAVE;
    let st = pause.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Replace && st.next.is_some(),
        "Verlassen: {:?}",
        st.kind
    );
    let main = pause.leave(&mut s);
    main.fill("de");
    {
        let pg = main.pg.borrow();
        let row = &pg.rows[MAIN_CONTINUE as usize];
        assert!(
            row.value.contains("burg") && !row.off,
            "erstes Bild: Weiter {:?} (aus {})",
            row.value,
            row.off
        );
    }
    let fresh = new_main(&new_menus());
    fresh.fill("de");
    let pg = fresh.pg.borrow();
    let row = &pg.rows[MAIN_CONTINUE as usize];
    assert!(
        row.value == tr("de", TextKey::ContinueLoading) && !row.off,
        "vor dem ersten Update: {:?}",
        row.value
    );
}

#[test]
fn main_shows_outcome_of_frame() {
    for (name, o, key, bad) in [
        ("Niederlage", Outcome::Lost, TextKey::Lost, true),
        ("Sieg", Outcome::Won, TextKey::Won, false),
    ] {
        let mut sc = new_main(&new_menus());
        sc.ended.set(o);
        sc.fill("de");
        {
            let pg = sc.pg.borrow();
            assert!(
                pg.note == tr("de", key) && pg.bad == bad,
                "{name}: Meldung {:?} (rot {})",
                pg.note,
                pg.bad
            );
        }
        sc.base.note = Note {
            raw: "eigene".to_string(),
            set: true,
            ..Note::default()
        };
        sc.fill("de");
        assert_eq!(sc.pg.borrow().note, "eigene", "{name}: eigene Meldung verdrängt");
    }
    let sc = new_main(&new_menus());
    sc.fill("de");
    assert_eq!(sc.pg.borrow().note, "", "ohne Ausgang");
    assert_eq!(outcome_note(Outcome::None), Note::default());
}

/// Die Hinweiszeile nennt das bedienende Gerät, in beiden Sprachen (Go: `TestRunRemembersOperatingDevice`, RU9.2).
#[test]
fn run_remembers_operating_device() {
    let mut sc = new_main(&new_menus());
    let mut s = FakeSession::default();
    step_through(&mut sc, &mut s, &[press(PAD2, &[Action::Down])]);
    sc.fill("de");
    assert!(
        sc.pg.borrow().hint.ends_with("bedient: Pad 2"),
        "Hinweis {:?}",
        sc.pg.borrow().hint
    );
    step_through(&mut sc, &mut s, &[press(KEYS1, &[Action::Up])]);
    sc.fill("en");
    assert!(
        sc.pg.borrow().hint.ends_with("operated by: Keyboard"),
        "hint {:?}",
        sc.pg.borrow().hint
    );
}

/// Haupt- und Pause-Menü geben ihre Illustration an die Zeilen weiter (Go: `TestMenuRowsCarryArt`, RU9.2).
#[test]
fn menu_rows_carry_art() {
    let _g = version_guard();
    let sc = new_main(&new_menus());
    sc.fill("de");
    for (i, a) in super::art::MAIN_ART.iter().enumerate() {
        assert_eq!(sc.pg.borrow().rows[i].art, *a, "Hauptmenü Zeile {i}");
    }
}
