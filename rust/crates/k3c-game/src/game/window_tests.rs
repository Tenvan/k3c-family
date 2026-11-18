//! Tests von `window.rs`: `TestWindowRect` und `TestFullscreenToggles` aus `play_test.go`, dazu der Vollbild-Wunsch.

use super::play_testutil::{play_game, press};
use super::testutil::pad;
use super::window::{toggled_window, window_rect};
use crate::view::input::{Action, Command, Input};
use crate::view::settings::{LANGUAGE_ENGLISH, Settings, WINDOW_BORDERLESS, WINDOW_FRAMED, WINDOW_FULLSCREEN};

/// `ActionFullscreen` schaltet zwischen Vollbild und Fenster (aus rahmenlos: Vollbild) und lässt den Rest stehen; nicht
/// im Aufnahme-Modus und nicht ohne Frame (leere Einstellungen). B-826 AC-03.
#[test]
fn fullscreen_toggles() {
    let (mut g, probe) = play_game(Vec::new());
    let input = Input {
        devices: vec![press(pad(2), Command::default(), &[Action::Fullscreen])],
    };
    g.toggle_window(&input);
    assert!(probe.set.borrow().is_empty(), "ohne Frame umgeschaltet");
    g.frame.settings = Settings {
        music: 0.5,
        effects: 1.0,
        language: LANGUAGE_ENGLISH.to_string(),
        ..Settings::default()
    };
    let down = Input {
        devices: vec![press(pad(2), Command::default(), &[Action::Down])],
    };
    g.toggle_window(&down);
    g.toggle_window(&input);
    let mut want = g.frame.settings.clone();
    want.window = WINDOW_FULLSCREEN.to_string();
    assert_eq!(*probe.set.borrow(), [want], "Vollbild an");
    for (from, to) in [
        (WINDOW_FULLSCREEN, WINDOW_FRAMED),
        (WINDOW_BORDERLESS, WINDOW_FULLSCREEN),
    ] {
        assert_eq!(toggled_window(from), to, "F aus {from:?}");
    }
    g.args.shots = "reports/x".to_string();
    g.toggle_window(&input);
    assert_eq!(probe.set.borrow().len(), 1, "im Aufnahme-Modus umgeschaltet");
}

/// Rahmenlos füllt den Monitor; das Fenster behält die Startgröße (höchstens 90 % Monitorgröße) und steht mittig.
#[test]
fn window_rect_cases() {
    let cases = [
        (WINDOW_BORDERLESS, 1280, 720, (0, 0, 1920, 1080)),
        (WINDOW_FRAMED, 1280, 720, (320, 180, 1280, 720)),
        (WINDOW_FRAMED, 2560, 1440, (96, 54, 1728, 972)),
    ];
    for (mode, w, h, want) in cases {
        assert_eq!(window_rect(mode, 1920, 1080, w, h), want, "{mode:?} {w}x{h}");
    }
}

/// Neu in Rust: Der Vollbild-Wunsch folgt `Settings::window` (Vollbild und rahmenlos: an, Fenster: aus); vor dem ersten
/// Frame und im Aufnahme-Modus gibt es keinen Wunsch; `apply_window` merkt den Modus nur bei Änderung.
#[test]
fn fullscreen_wish_follows_settings() {
    let (mut g, _) = play_game(Vec::new());
    assert_eq!(g.fullscreen_wish(), None, "vor dem ersten Frame");
    g.frame.settings = Settings {
        language: LANGUAGE_ENGLISH.to_string(),
        ..Settings::default()
    };
    for (mode, want) in [
        (WINDOW_FRAMED, false),
        (WINDOW_BORDERLESS, true),
        (WINDOW_FULLSCREEN, true),
    ] {
        g.frame.settings.window = mode.to_string();
        assert_eq!(g.fullscreen_wish(), Some(want), "Modus {mode:?}");
        g.apply_window();
        assert_eq!(g.window, mode);
    }
    g.args.shots = "reports/x".to_string();
    assert_eq!(g.fullscreen_wish(), None, "Aufnahme-Modus");
    g.frame.settings.window = WINDOW_FRAMED.to_string();
    g.apply_window();
    assert_eq!(g.window, WINDOW_FULLSCREEN, "Aufnahme-Modus merkt nichts");
}
