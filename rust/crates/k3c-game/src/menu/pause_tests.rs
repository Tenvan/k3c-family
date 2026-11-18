//! Tests aus `native/k3c-game/menu/pause_test.go` (AC-04): Pause-Menü. `TestMainShowsOutcomeOfFrame` liegt in
//! `main_screen_tests.rs` (RU9.4), `TestMenusPagesAndFallbackFont` kommt mit RU12.3.

use super::pause::{PAUSE_LEAVE, PAUSE_REMOVE, new_pause};
use super::testutil::{FakeSession, KEYS1, PAD1, PAD2, PAD3, err_fake, has_cue, new_menus, press, step_through};
use crate::view::frame::{CueKind, Seat};
use crate::view::input::Action::{self, Back, Confirm, Down, Home, Menu};
use crate::view::input::DeviceID;
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Fehler;

/// Zwei Spieler: Spieler 0 an pad1, Spieler 1 an keys1 (Go: `twoSeats().Seats`).
fn two_seats() -> Vec<Seat> {
    vec![
        Seat {
            player: 0,
            device: PAD1,
            ..Default::default()
        },
        Seat {
            player: 1,
            device: KEYS1,
            ..Default::default()
        },
    ]
}

fn ins(d: DeviceID, keys: &[Action]) -> Vec<crate::view::input::Input> {
    keys.iter().map(|a| press(d, &[*a])).collect()
}

#[test]
fn pause_transitions() {
    type Case = (&'static str, Vec<Action>, StepKind, bool, Option<CueKind>);
    let cases: Vec<Case> = vec![
        ("B setzt fort", vec![Back], StepKind::Pop, false, Some(CueKind::Resume)),
        (
            "Menu setzt fort",
            vec![Menu],
            StepKind::Pop,
            false,
            Some(CueKind::Resume),
        ),
        ("Fortsetzen", vec![Confirm], StepKind::Pop, false, Some(CueKind::Resume)),
        (
            "Optionen",
            vec![Down, Confirm],
            StepKind::Push,
            true,
            Some(CueKind::Confirm),
        ),
        (
            "Tasten",
            vec![Down, Down, Confirm],
            StepKind::Push,
            true,
            Some(CueKind::Confirm),
        ),
        (
            "Spiel verlassen",
            vec![Down, Down, Down, Confirm],
            StepKind::Replace,
            true,
            Some(CueKind::Confirm),
        ),
        (
            "Home bleibt im Pause-Menü",
            vec![Down, Home],
            StepKind::Stay,
            false,
            None,
        ),
    ];
    for (name, keys, kind, next, cue) in cases {
        let mut sc = new_pause(&new_menus());
        let mut s = FakeSession {
            active: true,
            ..Default::default()
        };
        let mut st = Step::default();
        for a in keys {
            st = sc.update(&press(PAD1, &[a]), &mut s);
        }
        assert!(
            st.kind == kind && st.next.is_some() == next && cue.is_none_or(|c| has_cue(&st, c)),
            "{name}: Schritt {:?}",
            st.kind
        );
    }
}

/// Zwei Geräte: B und Esc setzen das Spiel fort, gleich welches Gerät sie drückt.
#[test]
fn pause_back_with_two_devices() {
    for (d, keys) in [(PAD2, vec![Back]), (KEYS1, vec![Back, Menu])] {
        let mut sc = new_pause(&new_menus());
        let mut s = FakeSession {
            active: true,
            seats: two_seats(),
            ..Default::default()
        };
        let st = sc.update(&press(d, &keys), &mut s);
        assert!(
            st.kind == StepKind::Pop && has_cue(&st, CueKind::Resume),
            "{d:?}: {:?}",
            st.kind
        );
    }
}

#[test]
fn pause_leave_closes_and_reports_error() {
    let m = new_menus();
    let mut s = FakeSession {
        active: true,
        close_err: Some(err_fake()),
        ..Default::default()
    };
    let mut sc = new_pause(&m);
    sc.sel = PAUSE_LEAVE;
    let st = sc.update(&press(PAD1, &[Confirm]), &mut s);
    assert!(
        s.closed == 1 && !s.active && st.kind == StepKind::Replace && st.next.is_some(),
        "Verlassen: {:?}, geschlossen {}",
        st.kind,
        s.closed
    );
    // Der Fehler steht im Hauptmenü, das die Pause ersetzt (Rust: ohne Downcast über `leave`).
    let mut s = FakeSession {
        active: true,
        close_err: Some(err_fake()),
        ..Default::default()
    };
    let main = new_pause(&m).leave(&mut s);
    assert_eq!(
        main.base.note.raw,
        err_fake().to_string(),
        "Hauptmenü: {:?}",
        main.base.note
    );
    assert!(m.borrow().notice.raw.is_empty(), "Meldung bleibt vorgemerkt");
}

#[test]
fn pause_remove_player() {
    type Case = (&'static str, DeviceID, Option<Fehler>, Vec<i64>, StepKind, CueKind);
    let cases: Vec<Case> = vec![
        (
            "Spieler 2 meldet sich ab",
            KEYS1,
            None,
            vec![1],
            StepKind::Pop,
            CueKind::Resume,
        ),
        (
            "Session lehnt ab",
            PAD1,
            Some(err_fake()),
            vec![0],
            StepKind::Stay,
            CueKind::Denied,
        ),
        ("Gerät ohne Platz", PAD3, None, vec![], StepKind::Stay, CueKind::Denied),
    ];
    for (name, dev, err, left, kind, cue) in cases {
        let mut sc = new_pause(&new_menus());
        let mut s = FakeSession {
            active: true,
            leave_err: err,
            seats: two_seats(),
            ..Default::default()
        };
        let st = step_through(&mut sc, &mut s, &ins(dev, &[Down, Down, Down, Confirm]));
        assert!(
            sc.sel == PAUSE_REMOVE && st.kind == kind && has_cue(&st, cue) && s.left == left,
            "{name}: Auswahl {}, Schritt {:?}, abgemeldet {:?}",
            sc.sel,
            st.kind,
            s.left
        );
    }
}

#[test]
fn pause_remove_disabled_with_one_player() {
    let mut sc = new_pause(&new_menus());
    let mut s = FakeSession {
        active: true,
        seats: vec![Seat {
            player: 0,
            device: PAD1,
            ..Default::default()
        }],
        ..Default::default()
    };
    step_through(&mut sc, &mut s, &ins(PAD1, &[Down, Down, Down]));
    assert_eq!(sc.sel, PAUSE_LEAVE, "Abmelden nicht übersprungen");
}
