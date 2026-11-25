//! Port von `native/k3c-game/devtools/cheats_test.go` (ohne `window`: die Zeichenhilfe liefert RU29.3), dazu der Abgleich
//! der Aktionsnamen mit `engine/room/dev*.go`.

use std::cell::RefCell;
use std::rc::Rc;

use super::cheatlist::cheat_groups;
use super::{DevOverlay, new};
use crate::view::devkeys::{DevAction, DevSession};
use crate::view::frame::{CueKind, Frame, Seat};
use crate::view::input::Action::{Back, Cheats, Confirm, Down, Right, Up};
use crate::view::input::{Action, DeviceID, DeviceInput, DeviceKind, Input};
use crate::view::roles::OverlayStep;

const KEYS1: DeviceID = DeviceID {
    kind: DeviceKind::Keyboard,
    index: 0,
};
const PAD1: DeviceID = DeviceID {
    kind: DeviceKind::Gamepad,
    index: 0,
};
const PAD2: DeviceID = DeviceID {
    kind: DeviceKind::Gamepad,
    index: 1,
};

/// FakeDev ist eine Entwickler-Rolle, die die Dev-Aktionen mitschreibt.
#[derive(Default)]
struct FakeDev {
    players: Vec<i64>,
    acts: Vec<DevAction>,
}

impl DevSession for FakeDev {
    fn dev(&mut self, p: i64, a: &DevAction) -> Result<(), String> {
        self.players.push(p);
        self.acts.push(a.clone());
        Ok(())
    }
    fn halt(&mut self, _on: bool) {}
    fn halted(&self) -> bool {
        false
    }
    fn step_once(&mut self) {}
    fn reload(&mut self) -> Result<(), String> {
        Ok(())
    }
}

fn press(d: DeviceID, a: &[Action]) -> DeviceInput {
    DeviceInput {
        device: d,
        actions: a.to_vec(),
        ..Default::default()
    }
}

fn input(ds: Vec<DeviceInput>) -> Input {
    Input { devices: ds }
}

fn seat(player: i64, device: DeviceID) -> Seat {
    Seat {
        player,
        device,
        ..Default::default()
    }
}

fn frame(seats: Vec<Seat>) -> Frame {
    Frame {
        seats,
        ..Default::default()
    }
}

/// Ein Update mit der Entwickler-Rolle `dev` (`None`: Session ohne Rolle).
fn update(o: &mut DevOverlay, i: &Input, f: &Frame, dev: Option<&mut FakeDev>) -> OverlayStep {
    let mut d = dev.map(|d| d as &mut dyn DevSession);
    o.update_with(i, f, &mut d)
}

fn overlay() -> Rc<RefCell<DevOverlay>> {
    Rc::new(RefCell::new(DevOverlay {
        lib: None,
        menus: Default::default(),
        toggles: Default::default(),
        cues: Vec::new(),
        shapes: Vec::new(),
        lines: Vec::new(),
    }))
}

/// open_cheats öffnet mit F9 der linken Tastatur-Hälfte das Menü von Spieler 2 (Spieler 1 am Pad).
fn open_cheats() -> (Rc<RefCell<DevOverlay>>, FakeDev, Frame) {
    let o = overlay();
    let mut s = FakeDev::default();
    let f = frame(vec![seat(0, PAD1), seat(1, KEYS1)]);
    let st = update(
        &mut o.borrow_mut(),
        &input(vec![press(KEYS1, &[Cheats])]),
        &f,
        Some(&mut s),
    );
    let m = o.borrow();
    assert!(
        st.consumed[1] && !st.consumed[0] && m.menus[1].open && !m.menus[0].open,
        "F9: verbraucht {:?}, offen {}/{}",
        st.consumed,
        m.menus[0].open,
        m.menus[1].open
    );
    drop(m);
    (o, s, f)
}

/// Jede Gruppe hat Einträge, jede Aktion ist eine Aktion des Raums, die Bosse kommen aus data/bosses.json.
#[test]
fn cheat_groups_known_actions_and_bosses() {
    let known = [
        "gold",
        "material",
        "maxStock",
        "fillStock",
        "cheapBuild",
        "buildAll",
        "castleFall",
        "readyToSail",
        "shieldPlus",
        "spawn",
        "nextPhase",
        "phase",
        "timescale",
        "timescaleAuto",
        "grade",
        "night",
        "wave",
        "boss",
        "bossPhase",
        "event",
    ];
    let names = ["Vorrat", "Ausbau", "Zeit", "Grad", "Kampf"];
    let groups = cheat_groups();
    assert_eq!(groups.len(), names.len(), "Gruppen");
    let mut bosses = 0;
    for (i, g) in groups.iter().enumerate() {
        assert!(g.name == names[i] && !g.entries.is_empty(), "Gruppe {i}: {}", g.name);
        for e in &g.entries {
            assert!(
                known.contains(&e.act.action.as_str()) && !e.label.is_empty(),
                "{}: {e:?}",
                g.name
            );
            if e.act.action == "boss" {
                bosses += 1;
            }
        }
    }
    assert!(bosses > 0, "keine Bosse aus data/bosses.json");
}

/// Jede Aktion des Menüs steht als Fall im Rust-Raum (`k3c-room/src/room/dev*.rs`, als Text). Bis RU41 prüfte der Test
/// gegen `engine/room/dev*.go`; die Go-Engine ist entfernt.
#[test]
fn cheat_actions_exist_in_room() {
    let rust_room = [
        include_str!("../../../k3c-room/src/room/dev_extra.rs"),
        include_str!("../../../k3c-room/src/room/dev.rs"),
        include_str!("../../../k3c-room/src/room/dev_combat.rs"),
    ]
    .concat();
    for g in cheat_groups() {
        for e in &g.entries {
            let quoted = format!("\"{}\"", e.act.action);
            assert!(
                rust_room.contains(&quoted),
                "Aktion {} fehlt in k3c-room/src/room/dev*.rs",
                e.act.action
            );
        }
    }
}

/// F9 öffnet das Menü des Spielers und verbraucht seine Eingabe, der andere spielt weiter; Bestätigen löst für diesen
/// Spieler aus, ein Schalter kehrt seinen Stand um, rechts wechselt die Gruppe.
#[test]
fn cheat_overlay_per_player() {
    let (o, mut s, f) = open_cheats();
    update(
        &mut o.borrow_mut(),
        &input(vec![press(KEYS1, &[Confirm])]),
        &f,
        Some(&mut s),
    );
    assert!(
        s.acts.len() == 1 && s.players[0] == 1 && s.acts[0].action == "gold" && s.acts[0].amount == 10,
        "Gold: {:?} an {:?}",
        s.acts,
        s.players
    );
    // hoch: letzter Eintrag (Baukosten 1)
    update(
        &mut o.borrow_mut(),
        &input(vec![press(KEYS1, &[Up, Confirm])]),
        &f,
        Some(&mut s),
    );
    let a = &s.acts[1];
    assert!(
        a.action == "cheapBuild" && a.on == Some(true) && o.borrow().toggles.get("cheapBuild") == Some(&true),
        "Schalter: {a:?}"
    );
    update(
        &mut o.borrow_mut(),
        &input(vec![press(KEYS1, &[Right, Right, Down, Confirm])]),
        &f,
        Some(&mut s),
    );
    let a = &s.acts[2];
    assert!(a.action == "phase" && a.phase == "day", "Zeit, zweiter Eintrag: {a:?}");
}

/// Erneut F9 schließt (das Update ist noch verbraucht), danach geht die Eingabe wieder ans Spiel.
#[test]
fn cheat_overlay_closes() {
    let (o, mut s, f) = open_cheats();
    let st = update(
        &mut o.borrow_mut(),
        &input(vec![press(KEYS1, &[Cheats])]),
        &f,
        Some(&mut s),
    );
    assert!(
        !o.borrow().menus[1].open && st.consumed[1],
        "zweites F9: verbraucht {:?}",
        st.consumed
    );
    let st = update(
        &mut o.borrow_mut(),
        &input(vec![press(KEYS1, &[Confirm])]),
        &f,
        Some(&mut s),
    );
    assert!(
        !st.consumed[1] && s.acts.is_empty(),
        "geschlossen: verbraucht {:?}",
        st.consumed
    );
}

/// B schließt das Menü (im Spiel ohne Menü ist B unbelegt), je Gerät.
#[test]
fn cheat_overlay_back_closes_own_menu() {
    let o = overlay();
    let mut s = FakeDev::default();
    let f = frame(vec![seat(0, PAD1), seat(1, PAD2)]);
    update(
        &mut o.borrow_mut(),
        &input(vec![press(PAD1, &[Cheats]), press(PAD2, &[Cheats])]),
        &f,
        Some(&mut s),
    );
    assert!(o.borrow().menus[0].open && o.borrow().menus[1].open);
    let st = update(
        &mut o.borrow_mut(),
        &input(vec![press(PAD2, &[Back])]),
        &f,
        Some(&mut s),
    );
    let m = o.borrow();
    assert!(m.menus[0].open && !m.menus[1].open && st.consumed[0] && st.consumed[1]);
}

/// Spielt die Tastatur nicht, öffnet F9 das Menü des ersten Spielers.
#[test]
fn cheat_key_fallback() {
    let o = overlay();
    let mut f = frame(vec![seat(0, PAD1), seat(1, PAD2)]);
    update(&mut o.borrow_mut(), &input(vec![press(KEYS1, &[Cheats])]), &f, None);
    assert!(
        o.borrow().menus[0].open && !o.borrow().menus[1].open,
        "zwei Pads: {}/{}",
        o.borrow().menus[0].open,
        o.borrow().menus[1].open
    );
    let o = overlay();
    f.seats.truncate(1);
    update(&mut o.borrow_mut(), &input(vec![press(KEYS1, &[Cheats])]), &f, None);
    assert!(o.borrow().menus[0].open, "nur Pad: Menü von Spieler 1 nicht offen");
    f.recording = true;
    let o = overlay();
    update(&mut o.borrow_mut(), &input(vec![press(PAD1, &[Cheats])]), &f, None);
    assert!(!o.borrow().menus[0].open, "Aufnahme-Modus: Menü offen");
}

/// Ohne Entwickler-Rolle meldet das Menü einen Fehler statt auszulösen.
#[test]
fn cheat_without_dev_session() {
    let o = overlay();
    let f = frame(vec![seat(0, PAD1)]);
    update(&mut o.borrow_mut(), &input(vec![press(PAD1, &[Cheats])]), &f, None);
    let st = update(&mut o.borrow_mut(), &input(vec![press(PAD1, &[Confirm])]), &f, None);
    let m = o.borrow();
    assert!(
        m.menus[0].bad && st.cues.first().map(|c| c.kind) == Some(CueKind::Denied),
        "Status {:?}, Cues {:?}",
        m.menus[0].status,
        st.cues
    );
    assert_eq!(m.menus[0].status, super::cheats::ERR_NO_DEV);
}

/// `new` liefert einen Drawer auf `Layer::OVERLAY` und ein Overlay; ohne Spiel (keine Zellen) bleibt Draw still.
#[test]
fn new_yields_overlay_layer_and_overlay() {
    let parts = new(None);
    assert_eq!(parts.drawers.len(), 1);
    assert_eq!(parts.drawers[0].layer(), crate::view::layer::Layer::OVERLAY);
}

/// Die Umrechnung vormultiplizierter Farben in gerade Farben.
#[test]
fn straight_color_conversion() {
    assert_eq!(super::straight([0, 0, 0, 0]), [0.0; 4]);
    assert_eq!(super::straight([255, 255, 255, 255]), [1.0, 1.0, 1.0, 1.0]);
    let c = super::straight([10, 12, 18, 220]);
    assert!((c[3] - 220.0 / 255.0).abs() < 1e-6 && (c[0] - 10.0 / 220.0).abs() < 1e-6);
}

/// Fenster der Einträge um die Auswahl (Go: `window` in `TestCheatWithoutDevSession`).
#[test]
fn window_of_entries() {
    use super::drawcheats::window;
    assert_eq!(window(20, 30, 9), 16);
    assert_eq!((window(2, 30, 9), window(29, 30, 9), window(3, 5, 9)), (0, 21, 0));
}

/// Ein offenes Menü zeichnet ohne Schrift nur das Feld und die Auswahlzeile, ohne Absturz.
#[test]
fn cheat_menu_draws_without_font() {
    use crate::view::cell::{Cell, rect, window_cell};
    use crate::view::layer::Drawer;
    let o = overlay();
    o.borrow_mut().menus[0].open = true;
    let drawer = super::DevDrawer(Rc::clone(&o));
    let c = Cell {
        index: 0,
        player: 0,
        rect: rect(0, 0, 320, 360),
        ..Cell::default()
    };
    let f = Frame {
        cells: vec![c],
        ..Frame::default()
    };
    let mut dst = k3c_platform::NullCanvas::default();
    drawer.draw(&mut dst, &f, &window_cell(c.rect));
    assert_eq!(dst.rects, 2, "Feld und Auswahlzeile");
}
