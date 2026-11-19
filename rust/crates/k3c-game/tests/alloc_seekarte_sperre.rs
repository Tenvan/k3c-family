//! RU61.2 (B-1006/AC-03): Die Sperre durch gehaltenes A an der Seekarte allokiert nicht. Port des Go-Tests
//! `TestSeekarteSperreOhneAllokationen`: Nach dem Bestätigen hält Spieler 1 A; jedes weitere `Hud::update` mit
//! gehaltener Sperre (`update_sea`, Zweig `sea.hold`) zählt 0 Allokationen (`k3c-alloc-probe`, je Thread).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::rc::Rc;

use k3c_alloc_probe::{CountingAlloc, measure};
use k3c_game::hud::Hud;
use k3c_game::menu::placeholder::PlaceholderSession;
use k3c_game::replay::neutral::NeutralSimView;
use k3c_game::view::frame::{Frame, RouteIsland, Seat, Stage};
use k3c_game::view::input::{Action, DeviceID, DeviceInput, DeviceKind, Input};
use k3c_game::view::roles::Overlay;
use k3c_game::view::settings::default_settings;
use k3c_sim::state::{Player, Site, SwitchState, SwitchTarget, World};

#[global_allocator]
static ALLOC: CountingAlloc = CountingAlloc;

fn input(pay: bool, confirm: bool) -> Input {
    let device = DeviceID::new(DeviceKind::Keyboard, 0);
    let mut d = DeviceInput {
        device,
        actions: if confirm { vec![Action::Confirm] } else { vec![] },
        ..DeviceInput::default()
    };
    d.command.pay = pay;
    Input { devices: vec![d] }
}

#[test]
fn gehaltenes_a_an_der_seekarte_allokiert_nicht() {
    let target = |id: &str| SwitchTarget {
        id: id.to_string(),
        name: id.to_uppercase(),
        etappe: 2,
    };
    let w = World {
        players: vec![Player {
            index: 0,
            id: 1,
            x: 50.0,
            ..Default::default()
        }],
        sites: vec![Site {
            kind: "ship".to_string(),
            x: 50.0,
            ..Default::default()
        }],
        island_switch: Some(SwitchState {
            open: true,
            choice: "b".to_string(),
            completed: vec!["a".to_string()],
            targets: vec![target("b"), target("c")],
            ..SwitchState::default()
        }),
        ..Default::default()
    };
    let isl = |id: &str, etappe| RouteIsland {
        id: id.to_string(),
        name: id.to_uppercase(),
        etappe,
        ..RouteIsland::default()
    };
    let f = Frame {
        stages: vec![Stage {
            world: Some(Rc::new(w)),
            ..Default::default()
        }],
        seats: vec![Seat {
            player: 0,
            device: DeviceID::new(DeviceKind::Keyboard, 0),
            monarch: 0,
            stage: 0,
            lost: false,
        }],
        settings: default_settings(),
        route: vec![isl("a", 1), isl("b", 2), isl("c", 2), isl("d", 3)],
        ..Default::default()
    };
    let mut h = Hud::new(None, Box::new(NeutralSimView));
    h.listen(&f);
    let mut s = PlaceholderSession::default();
    let st = h.update(&input(true, true), &f, &mut s);
    assert!(st.consumed[0], "Bestätigen am Steg verbraucht die Eingabe");
    for _ in 0..8 {
        h.update(&input(true, false), &f, &mut s); // Aufwärmen: Puffer wachsen
    }
    let held = input(true, false);
    let (st, n) = measure(|| h.update(&held, &f, &mut s));
    assert!(st.consumed[0], "gehaltenes A bleibt verbraucht");
    assert_eq!(n.allocs, 0, "Sperre allokiert: {n:?}");
}
