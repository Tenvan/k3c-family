//! RU56.11 (B-1135, AC-01): F4 steht in der Tabelle und in der Tastenübersicht, fehlt die EXE, gibt es einen Hinweis und
//! kein Fenster, im Aufnahme-Modus wirkt F4 nicht; ohne Fenster kostet `devwin_tick` nichts.

use k3c_platform::Key;

use super::devkeys_tests::{dev_game, keys_down};
use crate::devtools::help::help_rows;
use crate::view::devkeys::{DEV_KEYS, DevCmd, dev_key_of};
use crate::view::frame::CueKind;

#[test]
fn f4_steht_in_tabelle_und_uebersicht() {
    assert_eq!(dev_key_of(DevCmd::Window), Some(Key::F4));
    assert!(DEV_KEYS.iter().any(|k| k.label == "F4" && k.cmd == DevCmd::Window));
    assert_eq!(help_rows().len(), DEV_KEYS.len());
}

#[test]
fn f4_ohne_exe_hinweis_statt_fenster() {
    let (mut g, _) = dev_game(false);
    keys_down(&mut g, &[Key::F4]);
    g.dev_keys();
    assert!(g.dw.child.is_none() && !g.dw.win.is_on(), "kein Fenster ohne EXE");
    let cue = g
        .cues
        .iter()
        .find(|c| c.kind == CueKind::Denied)
        .map(|c| c.text.clone());
    assert!(cue.is_some_and(|t| t.contains("k3c-devwin.exe")), "{:?}", g.cues);
}

#[test]
fn f4_im_aufnahme_modus_ignoriert() {
    let (mut g, _) = dev_game(false);
    g.args.shots = "x".to_string();
    keys_down(&mut g, &[Key::F4]);
    g.dev_keys();
    assert!(g.cues.is_empty() && !g.dw.win.is_on());
}

#[test]
fn ohne_fenster_tut_der_takt_nichts() {
    let (mut g, _) = dev_game(false);
    g.devwin_tick();
    g.devwin_tick();
    assert!(g.dw.boot && !g.dw.win.is_on() && g.dw.child.is_none());
}

#[test]
fn host_zustand_und_katalog_ohne_spiel() {
    use crate::devwin::Host;
    let (mut g, _) = dev_game(true);
    let st = g.state();
    assert_eq!(st.timescale, 1);
    let cat = g.catalog();
    assert_eq!(cat.timescales, [1, 2, 4, 8]);
    assert_eq!(cat.phases.len(), 4);
    assert!(cat.resources.iter().any(|r| r.id == "wood"));
    assert!(g.goto("w", 0, 0).is_err() && g.focus().is_err(), "ohne Pfad ack false");
}
