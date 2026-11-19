//! Test-Hilfen der Schiffsfahrt und der Geschichte (Go: `voySim`, `voyDrawer`, `fixedSource`, `harbor`, `switched`, `stuck`,
//! `voyGame` aus `voyage_test.go`). Gibt es nur in Tests.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::{Canvas, InputState};
use k3c_sim::state::{SwitchState, SwitchTarget, World};

use super::testutil::{FakeSim, Log, Probe, keys1, parts};
use super::{Args, Game, Scene};
use crate::game::levels::levels_tests::stages;
use crate::view::frame::{Event, Frame, Seat, Stage};
use crate::view::input::{Input, Source};
use crate::view::screen::StepKind;
use crate::view::voyage::{Voyage as View, VoyageDrawer};

/// Was der Zeichner der Fahrt sah.
#[derive(Default)]
pub struct Seen {
    pub asked: i64,
    pub drawn: i64,
    pub last: View,
}

/// Zeichner der Fahrt, der nur zählt; `ready` bestimmt, ob die Fahrt zu zeichnen ist.
#[derive(Clone)]
pub struct VoyDrawer {
    pub ready: bool,
    pub seen: Rc<RefCell<Seen>>,
}

impl VoyDrawer {
    pub fn new(ready: bool) -> VoyDrawer {
        VoyDrawer {
            ready,
            seen: Rc::default(),
        }
    }

    pub fn asked(&self) -> i64 {
        self.seen.borrow().asked
    }

    pub fn last(&self) -> View {
        self.seen.borrow().last.clone()
    }
}

impl VoyageDrawer for VoyDrawer {
    fn voyage_ready(&self, _f: &Frame, v: &View) -> bool {
        let mut s = self.seen.borrow_mut();
        s.asked += 1;
        s.last = v.clone();
        self.ready
    }

    fn draw_voyage(&self, _dst: &mut dyn Canvas, _f: &Frame, v: &View) -> bool {
        let mut s = self.seen.borrow_mut();
        s.drawn += 1;
        s.last = v.clone();
        true
    }
}

/// Eingabe-Quelle, die in jedem Update dieselbe Eingabe liefert (der Test ändert sie von außen).
#[derive(Clone, Default)]
pub struct Fixed(pub Rc<RefCell<Input>>);

impl Source for Fixed {
    fn poll(&mut self, _raw: &InputState) -> Input {
        self.0.borrow().clone()
    }
}

fn seat(stage: i64) -> Seat {
    Seat {
        player: 0,
        device: keys1(),
        stage,
        ..Seat::default()
    }
}

fn switch(choice: &str, targets: &[&str]) -> SwitchState {
    SwitchState {
        open: true,
        choice: choice.to_string(),
        completed: vec!["z".to_string(), "a".to_string()],
        targets: targets
            .iter()
            .map(|t| SwitchTarget {
                id: (*t).to_string(),
                ..SwitchTarget::default()
            })
            .collect(),
        ..SwitchState::default()
    }
}

fn world(sw: SwitchState) -> Option<Rc<World>> {
    Some(Rc::new(World {
        island_switch: Some(sw),
        ..World::default()
    }))
}

/// Der Frame am Hafen: Insel a abgeschlossen und aktuell, Ziele b und c, Wahl `choice`.
pub fn harbor(choice: &str) -> Frame {
    Frame {
        seats: vec![seat(0)],
        stages: vec![Stage {
            level: "wald".to_string(),
            world: world(switch(choice, &["b", "c"])),
            ..Stage::default()
        }],
        ..Frame::default()
    }
}

fn island_switch_event(target: &str) -> Event {
    let v = serde_json::json!({"type": "islandSwitch", "island": 1, "target": target});
    Event {
        data: v.as_object().cloned().unwrap_or_default(),
        ..Event::default()
    }
}

/// Der Frame des Inselwechsels: Ereignis `islandSwitch` mit Ziel `target`, Stufen der neuen Insel.
pub fn switched(target: &str) -> Frame {
    Frame {
        seats: vec![seat(0)],
        stages: stages(&["mine"]),
        events: vec![island_switch_event(target)],
        ..Frame::default()
    }
}

/// Der Frame eines gemeldeten, aber nicht vollzogenen Wechsels: Ereignis, die Insel a steht weiter am Hafen.
pub fn stuck(target: &str) -> Frame {
    Frame {
        events: switched(target).events,
        ..harbor(target)
    }
}

/// Der Frame am Hafen einer tieferen Stufe: Insel a aktuell, Ziel b; der Spieler steht in Stufe 2, alle Stufen der
/// Insel tragen den Wechselpunkt (`sim.mirrorSwitch`).
pub fn deep_harbor() -> Frame {
    let sw = switch("b", &["b"]);
    Frame {
        seats: vec![seat(2)],
        stages: vec![
            Stage {
                index: 0,
                level: "wald".to_string(),
                world: world(sw.clone()),
                ..Stage::default()
            },
            Stage {
                index: 2,
                level: "moor".to_string(),
                world: world(sw),
                ..Stage::default()
            },
        ],
        ..Frame::default()
    }
}

/// Eine Schleife im Fenster mit Zeichner der Fahrt (`None` = keiner); die Simulation liefert `frames`. Spieler 0 an
/// `keys1`, Spieler 1 an `pad(0)`; die Eingabe steuert `Fixed`.
pub fn voy_game(d: Option<&VoyDrawer>, frames: Vec<Frame>) -> (Game, Probe, Fixed) {
    voy_game_with(d, frames, |_| {})
}

/// Wie `voy_game`; `tweak` ändert die Teile vor dem Bau (Loader, Asset-Quellen, Listener).
pub fn voy_game_with(
    d: Option<&VoyDrawer>,
    frames: Vec<Frame>,
    tweak: impl FnOnce(&mut super::Parts),
) -> (Game, Probe, Fixed) {
    let (sim, probe) = FakeSim::new(2);
    let mut p = parts(StepKind::Stay, &Log::default(), Vec::new());
    p.voyage = d.map(|d| Box::new(d.clone()) as Box<dyn VoyageDrawer>);
    tweak(&mut p);
    let args = Args {
        scene: Scene {
            seed: "t".to_string(),
            players: 2,
            ..Scene::default()
        },
        ..Args::default()
    };
    let mut g = Game::new(p, Box::new(sim.with_script(frames)), args).unwrap_or_else(|e| unreachable!("{e}"));
    let fixed = Fixed::default();
    g.scripted = Some(Box::new(fixed.clone()));
    (g, probe, fixed)
}
