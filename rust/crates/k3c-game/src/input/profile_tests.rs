use k3c_platform::Key;

use super::*;
use crate::input::test_util::{KEYS1, device_ids, key_state, poll};
use crate::view::input::{Action, KeyboardSource};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_GAMEPAD, PROFILE_WASD};

/// commands sind die gehaltenen Befehle (command_keys).
const COMMANDS: [&str; 9] = [
    "left", "right", "sprint", "pay", "attack", "skill1", "skill2", "skill3", "skill4",
];

/// B-893/AC-01: Jedes Profil liefert alle Spiel-Aktionen: jeder Befehl und jede Menü-Aktion hat in wasd und pfeile
/// eine Taste, am Gamepad eine Taste oder (Home) die Kombi View + Menu.
#[test]
fn every_profile_has_all_actions() {
    for p in [PROFILE_WASD, PROFILE_ARROWS] {
        let l = layout(p);
        for c in COMMANDS {
            assert!(!command_keys(l, c).is_empty(), "{p}: Befehl {c} ohne Taste");
        }
        for a in bindings::ALL_ACTIONS {
            assert!(!action_keys(l, a).is_empty(), "{p}: Aktion {a:?} ohne Taste");
        }
    }
    for (a, b) in bindings::PAD_ACTIONS.iter().enumerate() {
        if b.is_none() {
            assert_eq!(
                bindings::ALL_ACTIONS[a],
                crate::view::input::Action::Home,
                "gamepad: Aktion {a} ohne Taste"
            );
        }
    }
    assert!(
        layout(PROFILE_GAMEPAD) == LAYOUT_WASD && layout("") == LAYOUT_WASD && layout(PROFILE_ARROWS) == LAYOUT_ARROWS,
        "Layout: Rückfall auf WASD bzw. Pfeile falsch"
    );
}

/// B-893/AC-03: set_keyboard wirkt ab dem nächsten Poll; es gibt immer genau ein Tastatur-Gerät, und eine beim Wechsel
/// gehaltene Taste löst erst nach dem Loslassen aus.
#[test]
fn set_keyboard_switches_layout() {
    let mut s = InputSource::default();
    poll(&mut s, &key_state(&[]), &[]);
    let input = poll(&mut s, &key_state(&[Key::ArrowLeft]), &[]);
    let (di, _) = input.of(KEYS1);
    assert!(
        di.command.move_ == 0.0 && input.devices.len() == 1,
        "wasd: Pfeil wirkt oder Geräte {:?}",
        device_ids(&input)
    );
    s.set_keyboard(PROFILE_ARROWS);
    let input = poll(&mut s, &key_state(&[Key::ArrowLeft]), &[]);
    let (di, _) = input.of(KEYS1);
    assert!(
        di.command.move_ == -1.0 && di.actions.is_empty(),
        "pfeile nach Wechsel: {di:?}"
    );
    poll(&mut s, &key_state(&[]), &[]);
    let input = poll(&mut s, &key_state(&[Key::Enter]), &[]);
    let (di, _) = input.of(KEYS1);
    assert_eq!(di.actions, vec![Action::Confirm], "Enter im Profil pfeile");
    let input = poll(&mut s, &key_state(&[Key::A]), &[]);
    assert_eq!(input.devices[0].command.move_, 0.0, "A wirkt im Profil pfeile");
}

/// B-917/AC-01: In jedem Tastatur-Profil löst die Leertaste aus (zahlen), unabhängig von den Richtungstasten.
#[test]
fn space_triggers_in_every_profile() {
    for p in [PROFILE_WASD, PROFILE_ARROWS] {
        assert!(
            command_keys(layout(p), "pay").contains(&Key::Space),
            "{p}: Leertaste löst nicht aus"
        );
        let mut s = InputSource::default();
        s.set_keyboard(p);
        poll(&mut s, &key_state(&[]), &[]);
        let input = poll(&mut s, &key_state(&[Key::Space]), &[]);
        let (di, _) = input.of(KEYS1);
        assert!(di.command.pay, "{p}: Leertaste setzt Pay nicht");
    }
}

/// B-945/AC-01: Im Menü gelten in jedem Profil Pfeile, Enter, Leertaste und Esc; außerhalb eines Menüs gilt nur das
/// Profil.
#[test]
fn menu_keys_in_every_profile() {
    let cases = [
        (Key::ArrowUp, Action::Up),
        (Key::ArrowDown, Action::Down),
        (Key::ArrowLeft, Action::Left),
        (Key::ArrowRight, Action::Right),
        (Key::Enter, Action::Confirm),
        (Key::Space, Action::Confirm),
        (Key::Escape, Action::Back),
    ];
    for p in [PROFILE_WASD, PROFILE_ARROWS] {
        for (key, act) in cases {
            let mut s = InputSource::default();
            s.set_keyboard(p);
            s.set_menu(true);
            poll(&mut s, &key_state(&[]), &[]);
            let input = poll(&mut s, &key_state(&[key]), &[]);
            let (di, _) = input.of(KEYS1);
            assert!(
                di.actions.contains(&act),
                "{p}: Taste {key:?} löst {act:?} im Menü nicht aus: {:?}",
                di.actions
            );
        }
    }
    let mut s = InputSource::default();
    poll(&mut s, &key_state(&[]), &[]);
    let input = poll(&mut s, &key_state(&[Key::ArrowUp]), &[]);
    assert!(
        input.devices[0].actions.is_empty(),
        "Pfeil hoch wirkt im Profil wasd außerhalb eines Menüs"
    );
}

/// Review FB4: Beim Wechsel Menü ↔ Spiel mit gehaltener Taste (Profil pfeile) löst weder das Menü aus noch zahlt das
/// Spiel, bis die Taste losgelassen ist.
#[test]
fn menu_switch_ignores_held_key() {
    let mut s = InputSource::default();
    s.set_keyboard(PROFILE_ARROWS);
    poll(&mut s, &key_state(&[]), &[]);
    s.set_menu(true); // Menü öffnet, Enter noch gehalten
    let input = poll(&mut s, &key_state(&[Key::Enter]), &[]);
    let (di, _) = input.of(KEYS1);
    assert!(
        di.actions.is_empty(),
        "gehaltenes Enter bestätigt im ersten Menü-Frame: {:?}",
        di.actions
    );
    poll(&mut s, &key_state(&[]), &[]);
    let input = poll(&mut s, &key_state(&[Key::Enter]), &[]);
    assert!(
        input.devices[0].actions.contains(&Action::Confirm),
        "frisches Enter bestätigt im Menü nicht"
    );
    s.set_menu(false); // Menü schließt, Enter noch gehalten
    for _ in 0..2 {
        let input = poll(&mut s, &key_state(&[Key::Enter]), &[]);
        assert!(
            !input.devices[0].command.pay,
            "gehaltenes Enter zahlt nach dem Schließen"
        );
    }
    poll(&mut s, &key_state(&[]), &[]);
    let input = poll(&mut s, &key_state(&[Key::Enter]), &[]);
    assert!(input.devices[0].command.pay, "frisches Enter zahlt nicht");
}
