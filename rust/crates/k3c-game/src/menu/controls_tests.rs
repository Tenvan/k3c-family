//! Tests aus `native/k3c-game/menu/controls_test.go` (`TestOptionsOpenControls` steht in `options_tests.rs`), dazu B
//! und Esc.

use super::controls::{CTL_BACK, new_controls, profile_text};
use super::testutil::{FakeSession, KEYS1, PAD1, has_cue, new_menus, press};
use crate::view::frame::CueKind;
use crate::view::input::{Action, DeviceInput, Input};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_GAMEPAD, PROFILE_WASD};
use crate::view::screen::{Screen, StepKind};

/// B-893/AC-03: Umschalten wirkt sofort (`set_settings` mit dem neuen Profil); AC-02: Spieler 2 bekommt die Tastatur
/// nicht, solange Spieler 1 sie hat, wohl aber, nachdem Spieler 1 auf Gamepad gewechselt hat.
#[test]
fn controls_switch_profiles() {
    let mut sc = new_controls(&new_menus(), false);
    let mut s = FakeSession::default();
    let mut press_pad = |sc: &mut super::controls::ControlsScreen, a: Action| {
        sc.update(&press(PAD1, &[a]), &mut s);
    };
    press_pad(&mut sc, Action::Right); // Spieler 1: WASD → Pfeile
    assert_eq!(sc.cur.controls[0], PROFILE_ARROWS, "Spieler 1 nach rechts");
    press_pad(&mut sc, Action::Down);
    press_pad(&mut sc, Action::Right); // Spieler 2: Gamepad, Tastatur vergeben → bleibt Gamepad
    assert_eq!(
        sc.cur.controls[1], PROFILE_GAMEPAD,
        "Spieler 2 bekommt die Tastatur: {:?}",
        sc.cur.controls
    );
    press_pad(&mut sc, Action::Up);
    press_pad(&mut sc, Action::Right); // Spieler 1: Pfeile → Gamepad
    press_pad(&mut sc, Action::Down);
    press_pad(&mut sc, Action::Right); // Spieler 2: Gamepad → WASD
    assert!(
        sc.cur.controls[0] == PROFILE_GAMEPAD && sc.cur.controls[1] == PROFILE_WASD,
        "nach Tausch: {:?}",
        sc.cur.controls
    );
    assert_eq!(s.settings.len(), 3, "nur echte Wechsel gespeichert");
    for set in &s.settings {
        assert!(
            set.controls[0] == PROFILE_GAMEPAD || set.controls[1] == PROFILE_GAMEPAD,
            "zwei Tastatur-Spieler gespeichert: {:?}",
            set.controls
        );
    }
}

/// Zurück mit A auf der letzten Zeile, mit B überall; die Zeilen zeigen die Profile.
#[test]
fn controls_back_and_texts() {
    let mut sc = new_controls(&new_menus(), false);
    let mut s = FakeSession::default();
    sc.update(&press(PAD1, &[Action::Up]), &mut s);
    assert_eq!(sc.sel, CTL_BACK, "hoch springt ans Ende");
    let step = sc.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert_eq!(step.kind, StepKind::Pop, "A auf Zurück");
    let mut sc = new_controls(&new_menus(), false);
    assert_eq!(
        sc.update(&press(PAD1, &[Action::Back]), &mut s).kind,
        StepKind::Pop,
        "B"
    );
    assert_eq!(profile_text("de", PROFILE_ARROWS), "Tastatur Pfeile");
    assert_eq!(profile_text("en", PROFILE_GAMEPAD), "Gamepad");
}

/// B am Pad, Esc (Back und Menu zugleich) an der Tastatur und Home schließen die Steuerung, auch aus dem Pause-Menü.
#[test]
fn controls_back_and_esc_close_also_from_pause() {
    let esc = Input {
        devices: vec![DeviceInput {
            device: KEYS1,
            actions: vec![Action::Back, Action::Menu],
            ..Default::default()
        }],
    };
    for dim in [false, true] {
        for input in [press(PAD1, &[Action::Back]), esc.clone(), press(PAD1, &[Action::Home])] {
            let step = new_controls(&new_menus(), dim).update(&input, &mut FakeSession::default());
            assert!(
                step.kind == StepKind::Pop && has_cue(&step, CueKind::Back),
                "dim {dim}: {:?}",
                step.kind
            );
        }
    }
}
