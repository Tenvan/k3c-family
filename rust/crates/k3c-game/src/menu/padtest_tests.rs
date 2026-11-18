//! Tests aus `native/k3c-game/menu/padtest_test.go` (AC-03) und AC-02: Tasten-Test zeigt Tasten, Achsen und Aktion je
//! Gerät. `TestTestingOpensPadTest` gehört zu RU12.3 (Testing-Menü).

use k3c_platform::{InputState, PadAxis, PadButton};

use super::layout::MAX_ROWS;
use super::padtest::{ACTION_NAMES, HOLD_LAST, PadTestScreen, new_pad_test};
use super::padtest_read::{InputPads, PadRaw, PadSource, axis, clip};
use super::testutil::{FakeSession, KEYS1, PAD1, PAD2, has_cue, new_menus, press, with_menus};
use super::texts::TextKey;
use crate::view::frame::CueKind;
use crate::view::input::Action::{self, Back, Cheats, Confirm, Left, Menu, Right, Up};
use crate::view::input::{DeviceInput, Input};
use crate::view::screen::{Screen, StepKind};
use crate::view::settings::DEFAULT_LANGUAGE;

/// Schein-Quelle mit festen Pads (Go: ersetztes `readPads`).
struct FakePads(Vec<PadRaw>);

impl PadSource for FakePads {
    fn read(&self) -> Vec<PadRaw> {
        self.0.clone()
    }
}

fn raw(id: i64, name: &str, buttons: &str, axes: &str, rawb: &str) -> PadRaw {
    PadRaw {
        id,
        name: name.to_string(),
        sdl: "0300".to_string(),
        standard: true,
        buttons: buttons.to_string(),
        axes: axes.to_string(),
        raw: rawb.to_string(),
    }
}

fn screen(pads: Vec<PadRaw>) -> PadTestScreen {
    let m = new_menus();
    with_menus(&m, |x| x.set_pad_source(Box::new(FakePads(pads))));
    new_pad_test(&m)
}

/// Alle Zeilen der Seite als "Beschriftung|Wert".
fn page_text(sc: &PadTestScreen) -> Vec<String> {
    sc.fill(DEFAULT_LANGUAGE);
    let pg = sc.pg.borrow();
    pg.rows[..pg.n]
        .iter()
        .map(|r| format!("{}|{}", r.label, r.value))
        .collect()
}

fn multi(devs: &[(crate::view::input::DeviceID, &[Action])]) -> Input {
    Input {
        devices: devs
            .iter()
            .map(|(d, a)| DeviceInput {
                device: *d,
                actions: a.to_vec(),
                ..Default::default()
            })
            .collect(),
    }
}

/// B-944/AC-01: Die Testseite zeigt Tasten, Achsen und die zugeordnete Aktion je Pad; Links verlässt sie nicht.
#[test]
fn pad_test_shows_buttons_axes_action() {
    let mut sc = screen(vec![raw(0, "Xbox Controller", "A LB", "LS 0.50 0 · RS 0 0", "0 4")]);
    let mut s = FakeSession::default();
    let st = sc.update(&press(PAD1, &[Left]), &mut s);
    assert_eq!(st.kind, StepKind::Stay, "Links verlässt die Seite");
    sc.fill(DEFAULT_LANGUAGE);
    let text = page_text(&sc).join("\n");
    for want in ["A LB → Links", "LS 0.50 0", "Xbox Controller", "roh 0 4"] {
        assert!(text.contains(want), "{want:?} fehlt in\n{text}");
    }
    assert!(sc.pg.borrow().n <= MAX_ROWS, "{} Zeilen", sc.pg.borrow().n);
}

/// B-944: B, Esc (Back und Menu zugleich) und A auf Zurück gehen zurück (Pop mit Cue Back).
#[test]
fn pad_test_back_keys() {
    let cases = [
        ("B", press(PAD1, &[Back])),
        ("Esc", press(KEYS1, &[Back, Menu])),
        ("A", press(PAD1, &[Confirm])),
    ];
    for (name, input) in cases {
        let st = screen(Vec::new()).update(&input, &mut FakeSession::default());
        assert_eq!(st.kind, StepKind::Pop, "{name}: soll zurück");
        assert!(has_cue(&st, CueKind::Back), "{name}: Cue Back fehlt");
    }
}

/// Andere Tasten zeigt die Seite nur an, sie verlässt sie nicht.
#[test]
fn pad_test_other_keys_stay() {
    let mut sc = screen(Vec::new());
    let st = sc.update(&press(KEYS1, &[Up, Right, Cheats]), &mut FakeSession::default());
    assert_eq!(st.kind, StepKind::Stay);
}

/// AC-02: Tastatur und zwei Pads gleichzeitig; jede Zeile zeigt das eigene Gerät, die Aktion steht je Gerät und
/// verschwindet nach `HOLD_LAST` Updates.
#[test]
fn pad_test_per_device_rows_and_hold() {
    let mut sc = screen(vec![
        raw(0, "Pad A", "A", "LS 0.50 0 · RS 0 0", "–"),
        raw(1, "Pad B", "B RB", "LS 0 -0.30 · RS 0 0", "–"),
    ]);
    let mut s = FakeSession::default();
    sc.update(&multi(&[(KEYS1, &[Up]), (PAD1, &[Left]), (PAD2, &[Right])]), &mut s);
    let rows = sc.rows();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0], ("Tastatur".to_string(), "→ Hoch".to_string()));
    assert_eq!(rows[1], ("Pad 1 · Pad A".to_string(), "A → Links".to_string()));
    assert_eq!(rows[2], ("  roh –".to_string(), "LS 0.50 0 · RS 0 0".to_string()));
    assert_eq!(rows[3], ("Pad 2 · Pad B".to_string(), "B RB → Rechts".to_string()));
    assert_eq!(rows[4].1, "LS 0 -0.30 · RS 0 0");

    // 45 Updates danach steht die Aktion noch, im 46. ist sie weg.
    for _ in 0..HOLD_LAST {
        sc.update(&Input::default(), &mut s);
    }
    assert_eq!(sc.rows()[0].1, "→ Hoch");
    assert_eq!(sc.rows()[1].1, "A → Links");
    sc.update(&Input::default(), &mut s);
    assert_eq!(sc.rows()[0].1, "→ –");
    assert_eq!(sc.rows()[1].1, "A → –");
    assert_eq!(sc.rows()[3].1, "B RB → –");
}

/// Ohne Pad steht "keines verbunden"; mehr als zwei Pads zeigt die Seite nicht (höchstens `MAX_ROWS` Zeilen).
#[test]
fn pad_test_no_pad_and_many_pads() {
    let mut sc = screen(Vec::new());
    sc.update(&Input::default(), &mut FakeSession::default());
    assert_eq!(page_text(&sc)[1], "Gamepad|keines verbunden");

    let mut sc = screen((0..4).map(|i| raw(i, "P", "A", "LS 0 0 · RS 0 0", "–")).collect());
    sc.update(&Input::default(), &mut FakeSession::default());
    assert_eq!(page_text(&sc).len(), 6, "Tastatur, zwei Pads je zwei Zeilen, Zurück");
}

/// Die Reihenfolge von `Action` entspricht der Namensliste (Go: `actionNames`).
#[test]
fn action_names_follow_action_order() {
    let pairs = [
        (Action::Up, "Hoch"),
        (Action::Down, "Runter"),
        (Action::Left, "Links"),
        (Action::Right, "Rechts"),
        (Action::Confirm, "Bestätigen"),
        (Action::Back, "Zurück"),
        (Action::Menu, "Menü"),
        (Action::View, "View"),
        (Action::PrevPage, "Seite zurück"),
        (Action::NextPage, "Seite vor"),
        (Action::Home, "Home"),
        (Action::SkillMenu, "Skill-Menü"),
        (Action::Fullscreen, "Vollbild"),
        (Action::Cheats, "Cheats"),
    ];
    assert_eq!(pairs.len(), ACTION_NAMES.len());
    for (a, name) in pairs {
        assert_eq!(ACTION_NAMES[a as usize], name);
    }
}

/// `InputPads` baut Xbox-Namen und Achsen aus dem Schnappschuss der Plattform.
#[test]
fn input_pads_use_xbox_names() {
    let mut st = InputState::new();
    let p = st.pad_mut(3);
    p.set_button(PadButton::A, true);
    p.set_button(PadButton::Home, true);
    p.set_button(PadButton::Up, true);
    p.set_axis(PadAxis::LX, 0.5);
    p.set_axis(PadAxis::RY, -0.03);
    let pads = InputPads::new();
    let reader = pads.clone();
    assert!(reader.read().is_empty());
    pads.snapshot(&st);
    let got = reader.read();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].id, 3);
    assert_eq!(got[0].name, "Pad 3");
    assert_eq!(got[0].buttons, "A ↑ Guide");
    assert_eq!(got[0].axes, "LS 0.50 0 · RS 0 0");
    assert_eq!((got[0].sdl.as_str(), got[0].raw.as_str()), ("–", "–"));
}

#[test]
fn axis_and_clip_helpers() {
    assert_eq!(axis(0.049), "0");
    assert_eq!(axis(-0.049), "0");
    assert_eq!(axis(0.05), "0.05");
    assert_eq!(axis(-1.0), "-1.00");
    assert_eq!(clip("Größenänderung", 5), "Größe");
    assert_eq!(clip("kurz", 10), "kurz");
}

#[test]
fn pad_test_page_texts() {
    let mut sc = screen(vec![raw(0, "Xbox", "–", "LS 0 0 · RS 0 0", "–")]);
    sc.update(&Input::default(), &mut FakeSession::default());
    sc.fill(DEFAULT_LANGUAGE);
    let pg = sc.pg.borrow();
    assert_eq!(pg.title, super::texts::tr(DEFAULT_LANGUAGE, TextKey::TstPads));
    assert_eq!(pg.sel, pg.n as i64 - 1);
    assert_eq!(pg.note, "Pad 1: Xbox · 0300");
}
