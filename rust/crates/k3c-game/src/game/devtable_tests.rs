//! Port von `native/k3c-game/game/devtable_test.go`. `devSim` (Go) heißt hier `FakeSim::with_dev`
//! (`dev_testutil::DevLog`); `TestOverlayHidesConsumedPlayer` steht in `overlay_tests`.

use k3c_platform::Key;

use super::devkeys_tests::{dev_game, keys_down};
use super::*;
use crate::input::action_keys;
use crate::view::devkeys::{DEBUG_GEOMETRY, DEBUG_OFF, DevCmd, dev_key_of};

fn key_of(c: DevCmd) -> Key {
    dev_key_of(c).unwrap_or(Key::A)
}

// AC-01 (B-888): Jede Taste der Tabelle wird ausgewertet: game liest sie selbst, oder input liefert sie als
// Geräte-Aktion auf der linken Tastatur-Hälfte (F9, F11); jede Taste kommt genau einmal vor.
#[test]
fn dev_key_table_evaluated() {
    let (mut g, _) = dev_game(false);
    let mut seen: Vec<&str> = Vec::new();
    let mut set = g.frame.settings.clone();
    for k in &DEV_KEYS {
        assert!(!seen.contains(&k.label), "{} doppelt", k.label);
        seen.push(k.label);
        if let Some(a) = k.action {
            assert!(
                action_keys(0, a).contains(&k.key),
                "{}: input liefert die Aktion {a:?} nicht",
                k.label
            );
            assert!(
                !g.dev_command(k.cmd, &mut set),
                "{}: game wertet eine Geräte-Aktion selbst aus",
                k.label
            );
            continue;
        }
        assert!(g.dev_command(k.cmd, &mut set), "{}: kein Befehl in game", k.label);
    }
}

// AC-02: F1 und F10 schalten den Frame-Zustand, F12 merkt das nächste Bild vor; im Aufnahme-Modus wirkt keine Taste.
#[test]
fn dev_key_view_toggles() {
    let (mut g, _) = dev_game(false);
    keys_down(
        &mut g,
        &[key_of(DevCmd::Help), key_of(DevCmd::Debug), key_of(DevCmd::Shot)],
    );
    g.dev_keys();
    assert!(
        g.dev.help && g.dev.debug == DEBUG_GEOMETRY && g.shot_next,
        "nach F1, F10, F12: {:?}, Bild {}",
        g.dev,
        g.shot_next
    );
    keys_down(&mut g, &[key_of(DevCmd::Debug)]);
    g.dev_keys();
    g.dev_keys();
    assert_eq!(g.dev.debug, DEBUG_OFF, "F10 dreimal");
    g.args.shots = "x".to_string();
    g.dev.help = false;
    keys_down(&mut g, &[key_of(DevCmd::Help)]);
    g.dev_keys();
    assert!(!g.dev.help, "F1 im Aufnahme-Modus gewirkt");
}

// AC-02: F7 hält an, erneut F7 rechnet einen Takt; F8 beendet den Halt, danach schaltet es den Zeitraffer über den
// ersten Spieler weiter; F6 lädt neu und meldet einen Fehler als Cue.
#[test]
fn dev_key_session() {
    let (mut g, probe) = dev_game(true);
    g.frame.timescale = 2;
    fn press(g: &mut Game, c: DevCmd) {
        keys_down(g, &[key_of(c)]);
        g.dev_keys();
    }
    press(&mut g, DevCmd::Pause);
    press(&mut g, DevCmd::Pause);
    {
        let l = probe.dev.borrow();
        assert!(l.halt && l.steps == 1, "F7, F7: halt {}, Schritte {}", l.halt, l.steps);
    }
    press(&mut g, DevCmd::Speed);
    {
        let l = probe.dev.borrow();
        assert!(!l.halt && l.actions.is_empty(), "F8 im Halt: halt {}", l.halt);
    }
    press(&mut g, DevCmd::Speed);
    {
        let l = probe.dev.borrow();
        assert!(
            l.actions.len() == 1 && l.actions[0].action == "timescale" && l.actions[0].factor == 4 && l.players[0] == 0,
            "F8: {:?} an {:?}",
            l.actions,
            l.players
        );
    }
    probe.dev.borrow_mut().fail_next = Some("kaputt".to_string());
    press(&mut g, DevCmd::Load);
    assert_eq!(probe.dev.borrow().reloads, 1, "F6: Ladevorgänge");
    assert!(
        g.cues.len() == 1 && g.cues[0].kind == CueKind::Denied && g.cues[0].text == "kaputt",
        "F6: Cues {:?}",
        g.cues
    );
}

// Ohne Entwickler-Rolle, ohne laufendes Spiel oder unter einem Menü geschieht bei F5 bis F8 nichts.
#[test]
fn dev_key_session_needs_running_game() {
    let (mut g, _) = dev_game(false);
    keys_down(&mut g, &[key_of(DevCmd::Load), key_of(DevCmd::Pause)]);
    g.dev_keys();
    assert!(g.cues.is_empty(), "ohne Entwickler-Rolle: {:?}", g.cues);
    let (mut g, probe) = dev_game(true);
    g.mode = crate::game::Mode::Menu;
    keys_down(
        &mut g,
        &[key_of(DevCmd::Load), key_of(DevCmd::Pause), key_of(DevCmd::Speed)],
    );
    g.dev_keys();
    let l = probe.dev.borrow();
    assert!(
        l.reloads == 0 && !l.halt && l.actions.is_empty(),
        "unter einem Menü gewirkt"
    );
}
