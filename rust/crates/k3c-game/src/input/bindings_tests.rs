use k3c_platform::{Key, PadButton};

use super::*;
use crate::input::test_util::{key_state, pad_state};
use crate::input::tracker::{Tracker, key_command, key_held};
use crate::view::input::{Action, Command};

fn mv(m: f64) -> Command {
    Command {
        move_: m,
        ..Default::default()
    }
}

fn skill(s: i64) -> Command {
    Command {
        skill: s,
        ..Default::default()
    }
}

fn pay() -> Command {
    Command {
        pay: true,
        ..Default::default()
    }
}

fn attack() -> Command {
    Command {
        attack: true,
        ..Default::default()
    }
}

/// AC-01: Jede Zeile der Belegung (Befehl, gehalten) ergibt den erwarteten Befehl, je Tastatur-Profil (B-893).
#[test]
fn key_command_table() {
    let galopp = Command {
        move_: 1.0,
        sprint: true,
        ..Default::default()
    };
    let sprint = Command {
        sprint: true,
        ..Default::default()
    };
    let cases: Vec<(&str, i64, Vec<Key>, Command)> = vec![
        ("keys1 nichts", 0, vec![], Command::default()),
        ("keys1 links", 0, vec![Key::A], mv(-1.0)),
        ("keys1 rechts", 0, vec![Key::D], mv(1.0)),
        ("keys1 beide Richtungen", 0, vec![Key::A, Key::D], Command::default()),
        ("keys1 Galopp", 0, vec![Key::D, Key::ShiftLeft], galopp),
        ("keys1 zahlen", 0, vec![Key::Space], pay()),
        ("keys1 Schlag", 0, vec![Key::E], attack()),
        ("keys1 Skill 1", 0, vec![Key::Q], skill(1)),
        ("keys1 Skill 2", 0, vec![Key::R], skill(2)),
        ("keys1 Skill 3", 0, vec![Key::T], skill(3)),
        ("keys1 Skill 4 (Z)", 0, vec![Key::Z], skill(4)),
        ("keys1 Skill 4 (Z auf QWERTZ)", 0, vec![Key::Y], skill(4)),
        ("keys1 niedrigster Skill gewinnt", 0, vec![Key::T, Key::R], skill(2)),
        (
            "wasd ignoriert die Pfeile",
            0,
            vec![Key::ArrowLeft, Key::Enter, Key::Numpad0],
            Command::default(),
        ),
        ("pfeile links", 1, vec![Key::ArrowLeft], mv(-1.0)),
        ("pfeile rechts", 1, vec![Key::ArrowRight], mv(1.0)),
        ("pfeile Galopp Shift", 1, vec![Key::ShiftRight], sprint),
        ("pfeile zahlen Enter", 1, vec![Key::Enter], pay()),
        ("pfeile zahlen Num-Enter", 1, vec![Key::NumpadEnter], pay()),
        ("pfeile Schlag Strg", 1, vec![Key::ControlRight], attack()),
        ("pfeile Schlag Num 0", 1, vec![Key::Numpad0], attack()),
        ("pfeile Skill 1 Einfg", 1, vec![Key::Insert], skill(1)),
        ("pfeile Skill 2 Entf", 1, vec![Key::Delete], skill(2)),
        ("pfeile Skill 3 Ende", 1, vec![Key::End], skill(3)),
        ("pfeile Skill 4 Bild ab", 1, vec![Key::PageDown], skill(4)),
        ("pfeile Skill 1 Num 1", 1, vec![Key::Numpad1], skill(1)),
        ("pfeile Skill 4 Num 4", 1, vec![Key::Numpad4], skill(4)),
        (
            "pfeile ignoriert WASD, Leertaste zahlt (B-917)",
            1,
            vec![Key::D, Key::Space, Key::E, Key::ShiftLeft],
            pay(),
        ),
    ];
    for (name, lay, keys, want) in cases {
        let got = key_command(lay, &key_state(&keys));
        assert_eq!(got, want, "{name}");
    }
}

/// AC-01: Jede Zeile der Belegung (Befehl, gehalten) am Gamepad; B ist im Spiel unbelegt.
#[test]
fn pad_command_table() {
    use PadButton::*;
    let galopp = Command {
        move_: 1.0,
        sprint: true,
        ..Default::default()
    };
    let cases: Vec<(&str, f64, Vec<PadButton>, Command)> = vec![
        ("nichts", 0.0, vec![], Command::default()),
        ("Stick in der Totzone", 0.2, vec![], Command::default()),
        ("Stick ganz rechts", 1.0, vec![], mv(1.0)),
        ("Stick ganz links", -1.0, vec![], mv(-1.0)),
        ("Steuerkreuz links", 0.0, vec![Left], mv(-1.0)),
        ("Steuerkreuz rechts", 0.0, vec![Right], mv(1.0)),
        ("Steuerkreuz geht vor Stick", 0.9, vec![Left], mv(-1.0)),
        ("Galopp RT", 1.0, vec![RT], galopp),
        ("zahlen A", 0.0, vec![A], pay()),
        ("Schlag X", 0.0, vec![X], attack()),
        ("Skill 1 LB", 0.0, vec![LB], skill(1)),
        ("Skill 2 RB", 0.0, vec![RB], skill(2)),
        ("Skill 3 LT", 0.0, vec![LT], skill(3)),
        ("Skill 4 Steuerkreuz hoch", 0.0, vec![Up], skill(4)),
        ("niedrigster Skill gewinnt", 0.0, vec![Up, RB], skill(2)),
        ("B unbelegt", 0.0, vec![B], Command::default()),
        ("View, Menu, Y unbelegt", 0.0, vec![View, Menu, Y], Command::default()),
    ];
    for (name, x, bs, want) in cases {
        let mut tr = Tracker::default();
        let mut p = pad_state(0, &bs);
        p.x = x;
        assert_eq!(tr.pad_command(&p), want, "{name}");
    }
}

/// press_once liefert die Aktionen, wenn ein frisches Gerät nach einem leeren Update held drückt.
fn press_once(held: [bool; ACTION_COUNT], pad: bool) -> Vec<Action> {
    let mut tr = Tracker::default();
    let none = [false; ACTION_COUNT];
    let mut out = Vec::new();
    tr.step(&none, pad, &mut out);
    out.clear();
    tr.step(&held, pad, &mut out);
    out
}

/// AC-01: Jede Menü-Taste beider Tastatur-Profile löst beim Drücken genau ihre Aktion(en) aus.
#[test]
fn key_action_table() {
    use Action::*;
    let cases: Vec<(i64, Key, Vec<Action>)> = vec![
        (0, Key::W, vec![Up]),
        (0, Key::S, vec![Down]),
        (0, Key::A, vec![Left]),
        (0, Key::D, vec![Right]),
        (0, Key::Space, vec![Confirm]),
        (0, Key::Escape, vec![Back, Menu]),
        (0, Key::Tab, vec![View]),
        (0, Key::Q, vec![PrevPage]),
        (0, Key::E, vec![NextPage]),
        (0, Key::Home, vec![Home]),
        (0, Key::K, vec![SkillMenu]),
        (0, Key::F11, vec![Fullscreen]),
        (0, Key::F, vec![]),
        (0, Key::F9, vec![Cheats]),
        (1, Key::F9, vec![Cheats]),
        (1, Key::F11, vec![Fullscreen]),
        (0, Key::Numpad5, vec![]),
        (0, Key::ArrowUp, vec![]),
        (1, Key::ArrowUp, vec![Up]),
        (1, Key::ArrowDown, vec![Down, SkillMenu]),
        (1, Key::ArrowLeft, vec![Left]),
        (1, Key::ArrowRight, vec![Right]),
        (1, Key::Enter, vec![Confirm]),
        (1, Key::NumpadEnter, vec![Confirm]),
        (1, Key::Backspace, vec![Back, Menu]),
        (1, Key::Escape, vec![Back, Menu]),
        (1, Key::Tab, vec![View]),
        (1, Key::Home, vec![Home]),
        (1, Key::PageUp, vec![PrevPage]),
        (1, Key::PageDown, vec![NextPage]),
        (1, Key::Numpad5, vec![SkillMenu]),
        (1, Key::K, vec![]),
        (1, Key::F, vec![]),
        (1, Key::W, vec![]),
    ];
    for (lay, key, want) in cases {
        let got = press_once(key_held(lay, &key_state(&[key])), false);
        assert_eq!(got, want, "Profil {lay} {key:?}");
    }
}

/// AC-01: Jede Menü-Taste des Gamepads löst beim Drücken ihre Aktion aus; Menu und View erst beim Loslassen.
#[test]
fn pad_action_table() {
    use Action::*;
    let cases: Vec<(PadButton, Vec<Action>)> = vec![
        (PAD_UP, vec![Up]),
        (PAD_DOWN, vec![Down, SkillMenu]), // Menü nimmt ↓, das Skill-Menü (Spiel) das andere
        (PAD_LEFT, vec![Left]),
        (PAD_RIGHT, vec![Right]),
        (PAD_A, vec![Confirm]),
        (PAD_B, vec![Back]),
        (PAD_LB, vec![PrevPage]),
        (PAD_RB, vec![NextPage]),
        (PAD_MENU, vec![]),
        (PAD_VIEW, vec![]),
        (PAD_X, vec![]),
        (PAD_RT, vec![]),
        (PAD_Y, vec![]),
        (PadButton::RS, vec![Fullscreen]),
    ];
    for (b, want) in cases {
        let mut tr = Tracker::default();
        let (idle, p) = (pad_state(0, &[]), pad_state(0, &[b]));
        let mut out = Vec::new();
        let h0 = tr.pad_held(&idle);
        tr.step(&h0, true, &mut out);
        let h1 = tr.pad_held(&p);
        out.clear();
        tr.step(&h1, true, &mut out);
        assert_eq!(out, want, "Taste {b:?}");
    }
}

/// Der Stick nach unten ist nur Menü-Richtung: Das Skill-Menü öffnet nur das Steuerkreuz (wie im Browser).
#[test]
fn stick_down_is_no_skill_menu() {
    let mut tr = Tracker::default();
    let idle = pad_state(0, &[]);
    let mut down = pad_state(0, &[]);
    down.y = 1.0;
    let mut out = Vec::new();
    let h0 = tr.pad_held(&idle);
    tr.step(&h0, true, &mut out);
    let h1 = tr.pad_held(&down);
    out.clear();
    tr.step(&h1, true, &mut out);
    assert_eq!(out, vec![Action::Down], "Stick ↓: erwartet nur Action::Down");
}

/// B-944/AC-02: Jede Menü-Aktion (`ALL_ACTIONS`, Länge `ACTION_COUNT`) ist über eine Taste mindestens eines
/// Tastatur-Profils (Profil oder `MENU_KEYS`) und über das Gamepad erreichbar; einzige Ausnahme ist `Home`
/// (Pad: Kombi View + Menu, keine Einzeltaste). `ALL_ACTIONS` folgt dem Index der Aktionen.
#[test]
fn every_action_has_key_and_pad_button() {
    assert_eq!(ALL_ACTIONS.len(), ACTION_COUNT);
    for (i, a) in ALL_ACTIONS.iter().copied().enumerate() {
        assert_eq!(a as usize, i, "ALL_ACTIONS folgt dem Index: {a:?}");
        let key = (0..LAYOUTS as i64).any(|l| !action_keys(l, a).is_empty()) || !MENU_KEYS[i].is_empty();
        assert!(key, "{a:?}: auf keiner Tastatur belegt");
        assert_eq!(PAD_ACTIONS[i].is_none(), a == Action::Home, "{a:?}: Pad-Taste");
    }
}

/// Jede Taste der Belegung wird beim Backend abgefragt; keine Taste liegt doppelt in den Befehlen eines Profils.
#[test]
fn used_keys_and_layouts() {
    for (i, l) in LAYOUT_TABLE.iter().enumerate() {
        let mut seen: Vec<Key> = Vec::new();
        let mut groups: Vec<&Vec<Key>> = vec![&l.left, &l.right, &l.sprint, &l.pay, &l.attack];
        groups.extend(l.skills.iter());
        for g in groups {
            for &k in g {
                assert!(!seen.contains(&k), "keys{}: Taste {k:?} doppelt im Befehl", i + 1);
                seen.push(k);
                assert!(
                    USED_KEYS.contains(&k),
                    "keys{}: Taste {k:?} wird nicht abgefragt",
                    i + 1
                );
            }
        }
    }
}

/// Die Marker-Tasten des Spieltests (WZ66): Einfg und LT + RT sind belegt (Skill 1 im Profil pfeile, Skill 3 und Galopp),
/// die Nebenwirkung ist gewollt; das Backend muss Einfg melden.
#[test]
fn marker_taste_einfg_wird_abgefragt() {
    assert!(USED_KEYS.contains(&Key::Insert));
}
