//! Port von `native/k3c-game/game/story_test.go`: Geschichtszeile (B-974) bei Schiffsfahrt, Ladeanzeige und Spielende.

use std::cell::{Cell as Counter, RefCell};
use std::rc::Rc;

use k3c_platform::InputState;

use super::play_testutil::play_game;
use super::testutil::keys1;
use super::voyage_testutil::{VoyDrawer, harbor, switched, voy_game_with};
use super::*;
use crate::game::levels::levels_tests::{AssetsOf, FakeLoader, stages};
use crate::view::frame::{END_DELAY, Frame, Outcome, Seat};
use crate::view::input::Input;
use crate::view::layer::Listener;
use crate::view::roles::AssetSource;
use crate::view::story::{END_HOLD_MAX, EndHolder};

/// Merkt je Update, was ein Listener für die Geschichtszeile sieht (B-974).
#[derive(Default)]
struct Seen {
    clocks: Vec<f64>,
    loading: Vec<bool>,
    switches: i64,
}

struct StoryListener(Rc<RefCell<Seen>>);

impl Listener for StoryListener {
    fn listen(&mut self, f: &Frame) {
        let mut s = self.0.borrow_mut();
        s.clocks.push(f.clock);
        s.loading.push(f.loading);
        s.switches += f.events.iter().filter(|e| e.type_() == "islandSwitch").count() as i64;
    }
}

fn advance(g: &mut Game, n: usize) {
    let r = InputState::new();
    for _ in 0..n {
        g.advance(&r);
    }
}

/// B-974 (Ankunft nach der Fahrt): Listener sehen den Inselwechsel genau einmal; während der Schiffsfahrt steht die Uhr
/// des Frames, die Einblendung der Ankunft beginnt also erst danach.
#[test]
fn story_waits_for_voyage() {
    let seen = Rc::new(RefCell::new(Seen::default()));
    let s2 = Rc::clone(&seen);
    let d = VoyDrawer::new(true);
    let (mut g, _, _) = voy_game_with(Some(&d), vec![harbor("b"), switched("b")], move |p| {
        p.listeners = vec![Box::new(StoryListener(s2))];
    });
    advance(&mut g, 2);
    assert!(g.voy.on, "keine Fahrt");
    let at = g.frame.clock;
    while g.voy.on {
        advance(&mut g, 1);
        assert!(
            !g.voy.on || g.frame.clock == at,
            "Uhr läuft während der Fahrt: {} statt {at}",
            g.frame.clock
        );
    }
    advance(&mut g, 1);
    let n = seen.borrow().switches;
    assert!(
        n == 1 && g.frame.clock > at,
        "Inselwechsel {n} Mal gemeldet, Uhr nach der Fahrt {} (davor {at})",
        g.frame.clock
    );
}

/// B-974: `Frame::loading` meldet den Listenern die Ladeanzeige (Einblendungen warten so lange).
#[test]
fn frame_loading_for_listeners() {
    let seen = Rc::new(RefCell::new(Seen::default()));
    let s2 = Rc::clone(&seen);
    let frame = Frame {
        seats: vec![Seat {
            player: 0,
            device: keys1(),
            ..Seat::default()
        }],
        stages: stages(&["wald"]),
        ..Frame::default()
    };
    let (mut g, _, _) = voy_game_with(None, vec![frame], move |p| {
        p.art = Some(Box::new(FakeLoader::default()));
        p.assets = vec![Box::new(AssetsOf::of(&[("wald", &["a/b"]), ("mine", &["c/d"])])) as Box<dyn AssetSource>];
        p.listeners = vec![Box::new(StoryListener(s2))];
    });
    advance(&mut g, 1); // Stufe neu: Ladeanzeige
    g.load.shown = true;
    advance(&mut g, 1); // lädt
    advance(&mut g, 1);
    let l = seen.borrow().loading.clone();
    assert!(
        l.len() == 3 && l[0] && !l[2] && !g.loading(),
        "Frame::loading je Update {l:?}, lädt am Ende {}",
        g.loading()
    );
}

/// Ein Listener, der am Spielende noch etwas zeigt (`EndHolder`), bis `left` Abfragen verbraucht sind.
struct Holder(Counter<i64>);

impl EndHolder for Holder {
    fn end_holding(&self) -> bool {
        self.0.set(self.0.get() - 1);
        self.0.get() >= 0
    }
}

/// B-974 (Review): Zeigt das HUD am Spielende noch Zeilen der Geschichte, wartet das Hauptmenü darauf, höchstens
/// `END_HOLD_MAX` Takte über `END_DELAY` hinaus.
#[test]
fn ending_waits_for_story() {
    for (hold, want) in [(40, END_DELAY + 40), (1 << 20, END_DELAY + END_HOLD_MAX)] {
        let (mut g, probe) = play_game(Vec::new());
        g.parts.end_holders = vec![Box::new(Holder(Counter::new(hold)))];
        g.frame.outcome = Outcome::Won;
        g.frame.stages = stages(&["wald"]);
        let mut n = 0;
        while !probe.closed.get() && n < 10 * END_HOLD_MAX {
            g.play(&Input::default());
            n += 1;
        }
        assert!(
            n == want && g.mode == Mode::Menu,
            "Halten {hold}: geschlossen nach {n} Takten (erwartet {want}), Modus {:?}",
            g.mode
        );
    }
}

/// Ohne Zeichner und ohne Fahrt sieht der Listener den Wechsel ebenfalls genau einmal (kein doppeltes Melden durch `sail`).
#[test]
fn switch_reported_once_without_voyage() {
    let seen = Rc::new(RefCell::new(Seen::default()));
    let s2 = Rc::clone(&seen);
    let (mut g, _, _) = voy_game_with(None, vec![harbor("b"), switched("b")], move |p| {
        p.listeners = vec![Box::new(StoryListener(s2))];
    });
    advance(&mut g, 5);
    assert_eq!(seen.borrow().switches, 1);
}
