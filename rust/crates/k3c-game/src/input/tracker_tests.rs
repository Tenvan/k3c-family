use k3c_platform::PadButton;

use super::*;
use crate::input::bindings::{PAD_A, PAD_B, PAD_DOWN, PAD_MENU, PAD_UP, PAD_VIEW, PAD_Y};
use crate::input::test_util::pad_state;
use crate::view::input::Action;

type Btn = PadButton;

/// PadStep ist ein Update eines Gamepads mit den erwarteten Aktionen.
#[derive(Clone)]
struct PadStep {
    bs: Vec<Btn>,
    x: f64,
    y: f64,
    want: Vec<Action>,
}

fn at(want: &[Action], bs: &[Btn]) -> PadStep {
    PadStep {
        bs: bs.to_vec(),
        x: 0.0,
        y: 0.0,
        want: want.to_vec(),
    }
}

fn stick(x: f64, y: f64, want: &[Action]) -> PadStep {
    PadStep {
        bs: vec![],
        x,
        y,
        want: want.to_vec(),
    }
}

/// times wiederholt ein Update n-mal.
fn times(n: i64, s: PadStep) -> Vec<PadStep> {
    vec![s; n as usize]
}

fn seq(parts: Vec<Vec<PadStep>>) -> Vec<PadStep> {
    parts.concat()
}

fn one(s: PadStep) -> Vec<PadStep> {
    vec![s]
}

const NONE: &[Action] = &[];
const HOME: &[Action] = &[Action::Home];
const MENU: &[Action] = &[Action::Menu];
const VIEW_A: &[Action] = &[Action::View];
const BACK: &[Action] = &[Action::Back];
const UP: &[Action] = &[Action::Up];
const DOWN: &[Action] = &[Action::Down];
/// Steuerkreuz ↓ gedrückt: auch das Skill-Menü
const DOWN_S: &[Action] = &[Action::Down, Action::SkillMenu];
const CONF: &[Action] = &[Action::Confirm];
const CHEAT: &[Action] = &[Action::Cheats];

/// run_pad spielt die Updates nach einem leeren ersten Update ab und prüft die Aktionen je Update.
fn run_pad(name: &str, steps: Vec<PadStep>) {
    let mut tr = Tracker::default();
    let idle = pad_state(0, &[]);
    let h = tr.pad_held(&idle);
    tr.step(&h, true, &mut Vec::new());
    for (i, s) in steps.iter().enumerate() {
        let mut p = pad_state(0, &s.bs);
        p.x = s.x;
        p.y = s.y;
        tr.pad_command(&p);
        let h = tr.pad_held(&p);
        let mut got = Vec::new();
        tr.step(&h, true, &mut got);
        assert_eq!(got, s.want, "{name}, Update {}", i + 1);
    }
}

/// Fälle der Kombi View + Menu (Teil 1 von `pad_menu_view_home`, wegen der Funktionslänge getrennt).
fn chord_cases() -> Vec<(&'static str, Vec<PadStep>)> {
    vec![
        (
            "View, dann Menu",
            seq(vec![
                one(at(NONE, &[PAD_VIEW])),
                one(at(HOME, &[PAD_VIEW, PAD_MENU])),
                one(at(NONE, &[PAD_VIEW])),
                one(at(NONE, &[])),
            ]),
        ),
        (
            "Menu, dann View",
            seq(vec![
                one(at(NONE, &[PAD_MENU])),
                one(at(HOME, &[PAD_MENU, PAD_VIEW])),
                one(at(NONE, &[])),
            ]),
        ),
        (
            "beide im selben Update",
            seq(vec![
                one(at(HOME, &[PAD_MENU, PAD_VIEW])),
                times(3, at(NONE, &[PAD_MENU, PAD_VIEW])),
                one(at(NONE, &[])),
            ]),
        ),
        (
            "nach der Kombi Menu neu drücken",
            seq(vec![
                one(at(HOME, &[PAD_MENU, PAD_VIEW])),
                one(at(NONE, &[PAD_VIEW])),
                one(at(HOME, &[PAD_MENU, PAD_VIEW])),
                one(at(NONE, &[])),
            ]),
        ),
    ]
}

/// Fälle für Menu kurz, View allein, B und Y (Teil 2 von `pad_menu_view_home`).
fn single_cases() -> Vec<(&'static str, Vec<PadStep>)> {
    vec![
        (
            "Menu kurz",
            seq(vec![times(3, at(NONE, &[PAD_MENU])), one(at(MENU, &[]))]),
        ),
        (
            "Menu genau 600 ms",
            seq(vec![times(MENU_SHORT, at(NONE, &[PAD_MENU])), one(at(MENU, &[]))]),
        ),
        (
            "Menu zu lang",
            seq(vec![times(MENU_SHORT + 1, at(NONE, &[PAD_MENU])), one(at(NONE, &[]))]),
        ),
        (
            "View allein",
            seq(vec![times(30, at(NONE, &[PAD_VIEW])), one(at(VIEW_A, &[]))]),
        ),
        (
            "B zurück",
            seq(vec![
                one(at(BACK, &[PAD_B])),
                times(20, at(NONE, &[PAD_B])),
                one(at(NONE, &[])),
            ]),
        ),
        (
            "View + Y Cheat-Menü",
            seq(vec![
                one(at(NONE, &[PAD_VIEW])),
                one(at(CHEAT, &[PAD_VIEW, PAD_Y])),
                times(5, at(NONE, &[PAD_VIEW, PAD_Y])),
                one(at(NONE, &[])),
            ]),
        ),
        (
            "Y allein",
            seq(vec![
                one(at(NONE, &[PAD_Y])),
                times(5, at(NONE, &[PAD_Y])),
                one(at(NONE, &[])),
            ]),
        ),
    ]
}

/// AC-03: View + Menu gemeinsam liefert nur Action::Home, nie zusätzlich Menu oder View; Menu kurz und View allein
/// lösen beim Loslassen aus; B ist Action::Back (Zurück in Menüs, im Spiel wertet game sie nicht aus).
#[test]
fn pad_menu_view_home() {
    let cases: Vec<_> = chord_cases().into_iter().chain(single_cases()).collect();
    for (name, steps) in cases {
        run_pad(name, steps);
    }
}

/// Gehaltene Richtungen wiederholen nach REPEAT_DELAY alle REPEAT_EVERY Updates; andere Aktionen nie (auch nicht das
/// Skill-Menü auf derselben Taste).
#[test]
fn repeat() {
    let steps = seq(vec![
        one(at(DOWN_S, &[PAD_DOWN])),
        times(REPEAT_DELAY - 1, at(NONE, &[PAD_DOWN])),
        one(at(DOWN, &[PAD_DOWN])),
        times(REPEAT_EVERY - 1, at(NONE, &[PAD_DOWN])),
        one(at(DOWN, &[PAD_DOWN])),
        one(at(NONE, &[])),
        one(at(DOWN_S, &[PAD_DOWN])),
    ]);
    run_pad("Steuerkreuz runter gehalten", steps);
    run_pad(
        "A gehalten",
        seq(vec![one(at(CONF, &[PAD_A])), times(40, at(NONE, &[PAD_A]))]),
    );
}

/// Stick und Steuerkreuz sind im Menü gleichwertig; der Stick schaltet mit Hysterese (an über 0,5, aus unter 0,3).
#[test]
fn stick_nav() {
    let cases: Vec<(&str, Vec<PadStep>)> = vec![
        (
            "Stick hoch mit Hysterese",
            seq(vec![
                one(stick(0.0, -0.45, NONE)),
                one(stick(0.0, -0.6, UP)),
                one(stick(0.0, -0.35, NONE)),
                one(stick(0.0, -0.55, NONE)),
                one(stick(0.0, -0.25, NONE)),
                one(stick(0.0, -0.6, UP)),
            ]),
        ),
        (
            "Stick runter, links, rechts",
            seq(vec![
                one(stick(0.0, 0.7, DOWN)),
                one(stick(0.0, 0.0, NONE)),
                one(stick(-0.7, 0.0, &[Action::Left])),
                one(stick(0.0, 0.0, NONE)),
                one(stick(0.7, 0.0, &[Action::Right])),
            ]),
        ),
        (
            "Steuerkreuz und Stick zugleich: eine Flanke",
            seq(vec![
                one(at(UP, &[PAD_UP])),
                one(PadStep {
                    bs: vec![PAD_UP],
                    x: 0.0,
                    y: -1.0,
                    want: vec![],
                }),
                one(stick(0.0, -1.0, NONE)),
                one(stick(0.0, 0.0, NONE)),
            ]),
        ),
    ];
    for (name, steps) in cases {
        run_pad(name, steps);
    }
}

/// Totzone des Lauf-Sticks mit Hysterese: an ab 0,25 (wie der Browser), aus erst unter 0,15; dazwischen der Rohwert.
#[test]
fn stick_move_hysteresis() {
    let cases: [(f64, bool, bool); 9] = [
        (0.2, false, false),
        (0.3, true, true),
        (0.2, true, true),
        (0.1, false, false),
        (0.2, false, false),
        (-0.3, true, true),
        (-0.16, true, true),
        (-0.14, false, false),
        (f64::NAN, false, false),
    ];
    let mut tr = Tracker::default();
    let m = tr.stick_move(0.25);
    assert_eq!(m, 0.25, "Schwelle 0,25: {m}");
    tr.stick_move(0.0);
    for (i, (x, moving, want_pos)) in cases.into_iter().enumerate() {
        let m = tr.stick_move(x);
        assert!(
            tr.moving == moving && (m != 0.0) == want_pos && (m == 0.0 || m.is_sign_negative() == x.is_sign_negative()),
            "Schritt {i} (x={x}): Move {m}, moving {}",
            tr.moving
        );
    }
    assert_eq!(tr.stick_move(1.0), 1.0, "voller Ausschlag");
    // Rohwert wie im Browser (kein Umskalieren ab der Schwelle)
    for x in [0.25, 0.5, -0.8] {
        let m = tr.stick_move(x);
        assert_eq!(m, x, "x={x}: Move {m}, erwartet den Rohwert");
    }
    assert_eq!(tr.stick_move(1.2), 1.0, "über 1");
}

/// Ein Gerät, das mit gehaltenen Tasten kommt, löst nichts aus, bis die Tasten einmal losgelassen sind.
#[test]
fn first_update_ignores_held() {
    let mut tr = Tracker::default();
    let steps: Vec<(Vec<Btn>, &[Action])> = vec![
        (vec![PAD_A, PAD_MENU, PAD_VIEW], NONE),
        (vec![PAD_A, PAD_MENU, PAD_VIEW], NONE),
        (vec![PAD_A], NONE),
        (vec![], NONE),
        (vec![PAD_A], CONF),
    ];
    for (i, (bs, want)) in steps.into_iter().enumerate() {
        let p = pad_state(0, &bs);
        let h = tr.pad_held(&p);
        let mut got = Vec::new();
        tr.step(&h, true, &mut got);
        assert_eq!(got, want, "Update {}", i + 1);
    }
}

/// HomeChord: beide gehalten und mindestens eine gerade gedrückt.
#[test]
fn home_chord_table() {
    // (held_view, held_menu, just_view, just_menu, want)
    let cases: [(bool, bool, bool, bool, bool); 6] = [
        (true, true, true, false, true),
        (true, true, false, true, true),
        (true, true, true, true, true),
        (true, true, false, false, false),
        (true, false, true, false, false),
        (false, true, false, true, false),
    ];
    for (i, (hv, hm, jv, jm, want)) in cases.into_iter().enumerate() {
        assert_eq!(home_chord(hv, hm, jv, jm), want, "Fall {i}");
    }
}
