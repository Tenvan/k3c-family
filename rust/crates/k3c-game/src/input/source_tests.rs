use k3c_platform::{InputState, Key, PadAxis, PadButton};

use super::*;
use crate::input::bindings::{PAD_A, PAD_B, PAD_DOWN, PAD_LB, PAD_LEFT, PAD_MENU, PAD_RT, PAD_VIEW, PAD_X};
use crate::input::test_util::{KEYS1, device_ids, key_state, pad_id, pad_state, poll};
use crate::view::input::{Action, Command};

fn odd(id: u32) -> PadState {
    PadState {
        id,
        name: "Lenkrad".to_string(),
        ..Default::default()
    }
}

/// AC-02: Geräte-Liste bei kein Pad, Pad kommt, zwei Pads, Pad geht und kommt mit neuer ID wieder: Tastatur zuerst,
/// dann Pads nach Pad-ID, nie ein fester Pad-Index; Pads ohne Standard-Belegung fehlen.
#[test]
fn devices_hot_plug() {
    let cases: Vec<(&str, Vec<PadState>, Vec<DeviceID>)> = vec![
        ("kein Pad", vec![], vec![KEYS1]),
        ("Pad kommt", vec![pad_state(5, &[])], vec![KEYS1, pad_id(5)]),
        (
            "zweites Pad, unsortiert gemeldet",
            vec![pad_state(5, &[]), pad_state(2, &[])],
            vec![KEYS1, pad_id(2), pad_id(5)],
        ),
        ("Pad 2 geht", vec![pad_state(5, &[])], vec![KEYS1, pad_id(5)]),
        (
            "Pad kommt mit neuer ID",
            vec![pad_state(7, &[]), pad_state(5, &[])],
            vec![KEYS1, pad_id(5), pad_id(7)],
        ),
        (
            "Pad ohne Standard-Belegung",
            vec![pad_state(5, &[]), odd(9)],
            vec![KEYS1, pad_id(5)],
        ),
        ("alle Pads weg", vec![], vec![KEYS1]),
    ];
    let mut s = InputSource::default();
    for (name, pads, want) in cases {
        let got = device_ids(&poll(&mut s, &key_state(&[]), &pads));
        assert_eq!(got, want, "{name}");
    }
    assert!(
        s.pads.is_empty() && s.warned.is_empty(),
        "Zustand getrennter Pads bleibt: {} tracker, {} Meldungen",
        s.pads.len(),
        s.warned.len()
    );
}

/// Ein neu verbundenes Pad löst im ersten Update nichts aus, auch wenn es eine frühere ID wieder bekommt.
#[test]
fn reconnected_pad_starts_fresh() {
    let mut s = InputSource::default();
    poll(&mut s, &key_state(&[]), &[pad_state(1, &[])]);
    poll(&mut s, &key_state(&[]), &[]);
    let input = poll(&mut s, &key_state(&[]), &[pad_state(1, &[PAD_A])]);
    let (di, ok) = input.of(pad_id(1));
    assert!(
        ok && di.actions.is_empty() && di.command.pay,
        "wiederverbundenes Pad: {di:?}"
    );
}

/// Pad ohne Standard-Belegung: eine Meldung, solange es verbunden ist; nach Trennen und Wiederkommen wieder eine.
#[test]
fn non_standard_pad_warns_once() {
    let mut s = InputSource::default();
    let odd = [odd(4)];
    for _ in 0..3 {
        poll(&mut s, &key_state(&[]), &odd);
    }
    assert_eq!(s.warned, vec![4u32], "Meldungen: {:?}", s.warned);
    poll(&mut s, &key_state(&[]), &[]);
    assert!(s.warned.is_empty(), "Meldung bleibt nach Trennen: {:?}", s.warned);
}

/// poll_twice liefert die Eingabe des zweiten Updates (nach einem leeren ersten Update mit denselben Geräten).
fn poll_twice(k: &KeyState, pads: &[PadState]) -> Input {
    let mut s = InputSource::default();
    let idle: Vec<PadState> = pads.iter().map(|p| pad_state(p.id, &[])).collect();
    poll(&mut s, &key_state(&[]), &idle);
    poll(&mut s, k, pads)
}

/// Drei Spieler gleichzeitig ohne Übersprechen: Jedes Gerät bekommt nur seine Tasten, auch wenn alle zugleich drücken.
#[test]
fn no_crosstalk() {
    let mut right = pad_state(3, &[PAD_RT, PAD_X]);
    right.x = 1.0;
    let input = poll_twice(
        &key_state(&[Key::A, Key::Space]),
        &[pad_state(1, &[PAD_A, PAD_LEFT]), right],
    );
    let want: Vec<(DeviceID, Command, Vec<Action>)> = vec![
        (
            KEYS1,
            Command {
                move_: -1.0,
                pay: true,
                ..Default::default()
            },
            vec![Action::Left, Action::Confirm],
        ),
        (
            pad_id(1),
            Command {
                move_: -1.0,
                pay: true,
                ..Default::default()
            },
            vec![Action::Left, Action::Confirm],
        ),
        (
            pad_id(3),
            Command {
                move_: 1.0,
                sprint: true,
                attack: true,
                ..Default::default()
            },
            vec![Action::Right],
        ),
    ];
    for (dev, cmd, actions) in want {
        let (di, ok) = input.of(dev);
        assert!(
            ok && di.command == cmd && di.actions == actions,
            "{dev}: {di:?}, erwartet {cmd:?} {actions:?}"
        );
    }
}

/// Poll legt nach dem ersten Aufruf nichts mehr an, auch mit zwei Pads, Aktionen und Wiederholung. Rust: ohne
/// `unsafe` (Workspace-Lint) gibt es keinen zählenden Allokator; geprüft wird, dass Puffer und Aktionslisten nach dem
/// Warmlaufen weder wachsen noch neu angelegt werden (Kapazität und Zeiger bleiben gleich).
#[test]
fn poll_allocs() {
    let mut s = InputSource::default();
    let pressed = [
        pad_state(1, &[PAD_A, PAD_DOWN, PAD_VIEW, PAD_MENU]),
        pad_state(2, &[PAD_B, PAD_LB]),
    ];
    let idle = [pad_state(1, &[]), pad_state(2, &[])];
    let (k_pressed, k_idle) = (key_state(&[Key::Space, Key::Escape, Key::ArrowUp]), key_state(&[]));
    for _ in 0..3 {
        let mut p = pressed.clone();
        s.poll_raw(&k_pressed, &mut p);
        let mut i = idle.clone();
        s.poll_raw(&k_idle, &mut i);
    }
    let snapshot = |s: &InputSource| {
        (
            s.buf.capacity(),
            s.buf.as_ptr(),
            s.buf
                .iter()
                .map(|d| (d.actions.capacity(), d.actions.as_ptr()))
                .collect::<Vec<_>>(),
            s.pads.capacity(),
        )
    };
    let before = snapshot(&s);
    let (mut p, mut i) = (pressed.clone(), idle.clone());
    for n in 1..=100 {
        if n % 2 == 0 {
            s.poll_raw(&k_pressed, &mut p);
        } else {
            s.poll_raw(&k_idle, &mut i);
        }
    }
    assert!(
        snapshot(&s) == before,
        "Poll legt Speicher neu an oder lässt ihn wachsen"
    );
}

/// Neu in Rust (kein Go-Test): `Source::poll` übersetzt den Plattform-Schnappschuss (Tasten, Pad-Tasten, linker Stick).
#[test]
fn poll_reads_platform_state() {
    let mut s = InputSource::default();
    let mut raw = InputState::new();
    raw.pad_mut(2);
    let first = s.poll(&raw);
    assert_eq!(device_ids(&first), vec![KEYS1, pad_id(2)]);
    raw.keys.press(Key::A);
    let pad = raw.pad_mut(2);
    pad.set_button(PadButton::A, true);
    pad.set_axis(PadAxis::LX, 1.0);
    let input = s.poll(&raw);
    let (kb, _) = input.of(KEYS1);
    assert_eq!(kb.command.move_, -1.0);
    assert_eq!(kb.actions, vec![Action::Left]);
    let (pd, _) = input.of(pad_id(2));
    assert!(pd.command.pay && pd.command.move_ == 1.0, "Pad: {pd:?}");
    assert_eq!(pd.actions, vec![Action::Right, Action::Confirm]);
}
