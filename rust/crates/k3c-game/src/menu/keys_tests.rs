//! Tests aus `native/k3c-game/menu/keys_test.go`: `move`, `window`, `cycle`, `key_of`, `run`.
//! `TestTextsComplete` gehört zu RU9.3, `TestGeomFitsWindow` zu RU9.1.

use super::keys::{Base, Handler, Key, cycle, first, key_of, move_sel, run, window};
use super::testutil::{FakeSession, KEYS1, PAD1, PAD2, press};
use crate::view::input::{Action, DeviceID, DeviceInput, Input};
use crate::view::screen::{Screen, StepKind};
use crate::view::session::Session;

type Off<'a> = Option<&'a dyn Fn(i64) -> bool>;

#[test]
fn move_cases() {
    let off_one = |i: i64| i == 1;
    let only_two = |i: i64| i != 2;
    let cases: [(&str, i64, i64, i64, bool, Off, i64); 8] = [
        ("runter", 0, 1, 3, false, None, 1),
        ("unten bleibt", 2, 1, 3, false, None, 2),
        ("oben bleibt", 0, -1, 3, false, None, 0),
        ("im Kreis nach oben", 0, -1, 3, true, None, 2),
        ("im Kreis nach unten", 2, 1, 3, true, None, 0),
        ("überspringt ausgegraut", 0, 1, 3, false, Some(&off_one), 2),
        ("nur ausgegraut darüber", 2, -1, 3, false, Some(&only_two), 2),
        ("leere Liste", 0, 1, 0, false, None, 0),
    ];
    for (name, sel, dir, n, wrap, off, want) in cases {
        assert_eq!(move_sel(sel, dir, n, wrap, off), want, "{name}");
    }
    assert_eq!(first(3, Some(&off_one)), 0);
    assert_eq!(first(2, Some(&|_| true)), 0);
}

#[test]
fn window_and_cycle() {
    for (sel, n, vis, want) in [
        (0, 3, 7, 0),
        (0, 12, 7, 0),
        (5, 12, 7, 2),
        (11, 12, 7, 5),
        (3, 12, 0, 0),
    ] {
        assert_eq!(window(sel, n, vis), want, "window({sel}, {n}, {vis})");
    }
    assert!(
        cycle(3, 1, 4) == 0 && cycle(0, -1, 4) == 3 && cycle(1, 1, 0) == 0,
        "cycle springt nicht im Kreis"
    );
}

#[test]
fn key_of_maps_actions() {
    let want = [
        (Action::Up, Key::Up),
        (Action::Down, Key::Down),
        (Action::Left, Key::Left),
        (Action::Right, Key::Right),
        (Action::Confirm, Key::OK),
        (Action::Back, Key::Back),
        (Action::Menu, Key::Back),
        (Action::Home, Key::Home),
        (Action::View, Key::Delete),
        (Action::SkillMenu, Key::None),
        (Action::Fullscreen, Key::None),
        (Action::NextPage, Key::None),
    ];
    for (a, k) in want {
        assert_eq!(key_of(a), k, "Aktion {a:?}");
    }
}

/// Zählt die Tasten, die `run` an `handle` gibt.
#[derive(Default)]
struct Probe {
    base: Base,
    got: Vec<Key>,
}

impl Handler for Probe {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }
    fn handle(&mut self, _d: DeviceID, k: Key, _s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        self.got.push(k);
        (StepKind::Stay, None)
    }
}

/// B am Pad, Esc (Back und Menu zugleich) an der Tastatur und Home: Back und Home; ein zweites Back desselben Geräts im
/// selben Update zählt nicht.
#[test]
fn key_of_maps_back_menu_home() {
    let mut s = FakeSession::default();
    let mut p = Probe::default();
    run(&mut p, &press(PAD1, &[Action::Back]), &mut s);
    assert_eq!(p.got, [Key::Back], "B am Pad");
    p.got.clear();
    run(&mut p, &press(KEYS1, &[Action::Back, Action::Menu]), &mut s);
    assert_eq!(
        p.got,
        [Key::Back],
        "Esc an der Tastatur liefert Back und Menu, zählt einmal"
    );
    p.got.clear();
    run(&mut p, &press(PAD1, &[Action::Home]), &mut s);
    assert_eq!(p.got, [Key::Home]);
    p.got.clear();
    let two = Input {
        devices: vec![
            DeviceInput {
                device: KEYS1,
                actions: vec![Action::Menu, Action::Back],
                ..Default::default()
            },
            DeviceInput {
                device: PAD1,
                actions: vec![Action::Back],
                ..Default::default()
            },
        ],
    };
    run(&mut p, &two, &mut s);
    assert_eq!(p.got, [Key::Back, Key::Back], "je Gerät ein Back");
}

#[test]
fn run_remembers_device_and_counts_version() {
    let mut s = FakeSession::default();
    let mut p = Probe::default();
    run(&mut p, &press(PAD2, &[Action::Down, Action::Up]), &mut s);
    assert!(
        p.base.by_ok && p.base.by == PAD2 && p.base.ver == 1,
        "Gerät gemerkt, Version einmal erhöht"
    );
    run(&mut p, &press(KEYS1, &[Action::Up]), &mut s);
    assert!(p.base.by == KEYS1 && p.base.ver == 2);
}

/// Go: Hauptmenü, Hinweis "bedient: Pad 2" bzw. "operated by: Keyboard" (braucht Hauptmenü und Texte).
#[test]
#[ignore = "wartet auf RU9.4"]
fn run_remembers_operating_device() {}
