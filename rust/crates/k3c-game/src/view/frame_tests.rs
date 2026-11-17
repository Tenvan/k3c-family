//! Port von `native/k3c-game/view/frame_test.go`.

use std::rc::Rc;

use k3c_sim::state::{Event as SimEvent, Player, World};
use serde_json::json;

use super::super::input::{DeviceID, DeviceKind};
use super::*;

fn player(index: i64, x: f64) -> Player {
    Player {
        index,
        x,
        ..Default::default()
    }
}

fn sim_event(v: serde_json::Value) -> SimEvent {
    v.as_object().cloned().unwrap_or_default()
}

fn test_frame() -> (Frame, Rc<World>, Rc<World>) {
    let top = Rc::new(World {
        players: vec![player(0, 1.0), player(1, 2.0)],
        ..World::default()
    });
    let deep = Rc::new(World {
        players: vec![player(1, 7.0)],
        ..World::default()
    });
    let f = Frame {
        stages: vec![
            Stage {
                index: 0,
                world: Some(top.clone()),
                ..Default::default()
            },
            Stage {
                index: 2,
                depth: 1,
                world: Some(deep.clone()),
                ..Default::default()
            },
        ],
        seats: vec![
            Seat {
                player: 0,
                monarch: 0,
                stage: 0,
                ..Default::default()
            },
            Seat {
                player: 1,
                monarch: 1,
                stage: 2,
                device: DeviceID::new(DeviceKind::Gamepad, 1),
                ..Default::default()
            },
        ],
        events: vec![
            Event {
                stage: 0,
                data: sim_event(json!({"type": "night", "day": 1})),
                ..Default::default()
            },
            Event {
                stage: 2,
                data: sim_event(json!({"type": "coin"})),
                ..Default::default()
            },
            Event {
                stage: 0,
                data: sim_event(json!({"day": 2})),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    (f, top, deep)
}

fn cell(player: i64) -> Cell {
    Cell {
        player,
        ..Default::default()
    }
}

#[test]
fn frame_resolves_seats() {
    let (f, top, deep) = test_frame();
    assert!(
        f.world(&cell(0)).is_some_and(|w| std::ptr::eq(w, Rc::as_ptr(&top))),
        "Zelle 0 sieht nicht die Oberfläche"
    );
    assert!(
        f.world(&cell(1)).is_some_and(|w| std::ptr::eq(w, Rc::as_ptr(&deep))),
        "Zelle 1 sieht nicht ihre Stufe"
    );
    assert!(f.world(&cell(-1)).is_none(), "Fenster-Zelle hat eine Welt");
    let m = f.monarch(1);
    assert!(
        m.is_some_and(|m| m.x == 7.0),
        "Monarch 1 = {m:?}, erwartet den aus Stufe 2"
    );
    assert!(f.monarch(5).is_none(), "unbekannter Spieler hat einen Monarchen");
    let s = f.seat(1);
    assert!(s.is_some_and(|s| s.device.to_string() == "pad2"), "Seat 1 = {s:?}");
}

#[test]
fn frame_missing_stage() {
    let mut f = Frame {
        seats: vec![Seat {
            player: 0,
            stage: 3,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert!(
        f.world(&cell(0)).is_none() && f.monarch(0).is_none(),
        "fehlende Stufe liefert Welt"
    );
    f.stages = vec![Stage {
        index: 3,
        ..Default::default()
    }];
    assert!(f.monarch(0).is_none(), "Stufe ohne Welt liefert Monarchen");
}

#[test]
fn frame_events() {
    let (f, _, _) = test_frame();
    let got: Vec<&Event> = f.events_of(0).collect();
    assert!(
        got.len() == 2 && got[0].type_() == "night" && got[1].type_().is_empty(),
        "events_of(0) = {got:?}"
    );
    assert_eq!(f.events_of(9).count(), 0, "Ereignisse einer fremden Stufe");
    // Abbruch beim ersten Ereignis
    assert_eq!(f.events_of(0).next().map(Event::type_), Some("night"));
    // ohne Vec: der Iterator borrowt nur (die Go-Messung AllocsPerRun entfällt, Rust legt hier nichts an)
    let n: usize = f.events_of(2).map(|e| e.type_().len()).sum();
    assert_eq!(n, 4);
}
