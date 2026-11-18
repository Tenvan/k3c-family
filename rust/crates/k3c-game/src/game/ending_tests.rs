//! Tests des Spielendes in `play.rs` (`ending_test.go`): `TestEndingClosesAfterDelay` und
//! `TestEndingResetsAfterLeavingThroughPause`. `TestEndingWaitsForStory` gehört zu RU30.2.

use std::cell::Cell as Counter;
use std::rc::Rc;

use k3c_sim::state::World;

use super::Game;
use super::play_testutil::{play_game, press};
use super::stack::{Mode, Stack};
use super::testutil::pad;
use crate::view::frame::{END_DELAY, Frame, Outcome, Stage};
use crate::view::input::{Action, Command, Input};
use crate::view::roles::{Overlay, OverlayStep};
use crate::view::session::Session;

/// Zählt die Aufrufe (nach dem Schließen des Spiels darf keiner mehr kommen).
struct CountingOverlay(Rc<Counter<u32>>);

impl Overlay for CountingOverlay {
    fn update(&mut self, _input: &Input, _f: &Frame, _s: &mut dyn Session) -> OverlayStep {
        self.0.set(self.0.get() + 1);
        OverlayStep::default()
    }
}

/// Ein Frame im Spiel mit Ausgang `o`.
fn ended_frame(g: &mut Game, o: Outcome) {
    g.frame.outcome = o;
    g.frame.stages = vec![Stage {
        index: 0,
        world: Some(Rc::new(World::default())),
        ..Stage::default()
    }];
}

/// Spielende: `END_DELAY` Takte nach `Frame::outcome` schließt game das Spiel und legt das Hauptmenü auf; im Takt des
/// Schließens läuft nichts mehr (kein Overlay, kein Befehl, kein Beitritt).
#[test]
fn ending_closes_after_delay() {
    let calls = Rc::new(Counter::new(0));
    let (mut g, probe) = play_game(vec![Box::new(CountingOverlay(Rc::clone(&calls)))]);
    ended_frame(&mut g, Outcome::Won);
    let input = Input {
        devices: vec![press(pad(4), Command::default(), &[Action::Confirm])],
    };
    for _ in 0..END_DELAY - 1 {
        g.play(&input);
    }
    assert!(
        !probe.closed.get() && g.mode == Mode::Play,
        "zu früh geschlossen, Modus {:?}",
        g.mode
    );
    let (joins, before) = (probe.joins.borrow().len(), calls.get());
    g.play(&input);
    assert!(probe.closed.get(), "nach {END_DELAY} Takten nicht geschlossen");
    assert!(g.mode == Mode::Menu && g.stack.len() == 1 && g.end_in == 0);
    assert_eq!(calls.get(), before, "Overlay nach dem Schließen");
    assert_eq!(probe.joins.borrow().len(), joins, "Beitritt nach dem Schließen");
}

/// Review-Befund: Spielende erkannt, dann über die Pause „Spiel verlassen“ und ein neues Spiel: Der Zähler beginnt neu,
/// das neue Spiel endet nicht nach wenigen Takten. Der Ausgang des alten Spiels bleibt im Menü-Frame ohne Stufe stehen.
#[test]
fn ending_resets_after_leaving_through_pause() {
    let (mut g, probe) = play_game(Vec::new());
    ended_frame(&mut g, Outcome::Lost);
    for _ in 0..END_DELAY - 2 {
        g.play(&Input::default());
    }
    // Pause › Spiel verlassen › Hauptmenü
    g.frame = Frame {
        outcome: Outcome::Lost,
        seats: g.frame.seats.clone(),
        ..Frame::default()
    };
    g.play(&Input::default());
    assert_eq!(g.end_in, 0, "Zähler nach dem Verlassen");
    // neues Spiel läuft
    g.mode = Mode::Play;
    g.stack = Stack::new();
    ended_frame(&mut g, Outcome::None);
    for _ in 0..END_DELAY * 2 {
        g.play(&Input::default());
    }
    assert!(
        !probe.closed.get() && g.mode == Mode::Play,
        "neues Spiel beendet, Modus {:?}",
        g.mode
    );
}
