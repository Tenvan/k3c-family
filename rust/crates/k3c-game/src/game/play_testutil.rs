//! Test-Hilfen für Spielbetrieb, Overlays und Spielende (Go: `playGame`, `press`, `skillMenu` aus `play_test.go`).

use std::cell::Cell as Counter;
use std::rc::Rc;

use super::testutil::{FakeSim, Log, Probe, keys1, pad, parts};
use super::{Args, Game, Scene};
use crate::view::frame::{Cue, CueKind, Frame, Seat};
use crate::view::input::{Action, Command, DeviceID, DeviceInput, Input};
use crate::view::roles::{Overlay, OverlayStep};
use crate::view::screen::StepKind;
use crate::view::session::Session;

/// Eingabe eines Geräts mit Befehl und Aktionen (Go: `press`).
pub fn press(d: DeviceID, c: Command, a: &[Action]) -> DeviceInput {
    DeviceInput {
        device: d,
        command: c,
        actions: a.to_vec(),
    }
}

/// Ein Spiel im Spielmodus mit den Overlays `overlays` und zwei Plätzen (Spieler 0 an `keys1`, Spieler 1 an `pad(2)`,
/// Go: `playGame`); der Frame trägt die Plätze, aber noch keine Stufe.
pub fn play_game(overlays: Vec<Box<dyn Overlay>>) -> (Game, Probe) {
    let (sim, probe) = FakeSim::new(2);
    let mut p = parts(StepKind::Stay, &Log::default(), Vec::new());
    p.overlays = overlays;
    let args = Args {
        scene: Scene {
            seed: "t".to_string(),
            players: 2,
            ..Scene::default()
        },
        ..Args::default()
    };
    let mut g = Game::new(p, Box::new(sim), args).unwrap_or_else(|e| unreachable!("{e}"));
    g.frame.seats = vec![
        Seat {
            player: 0,
            device: keys1(),
            ..Seat::default()
        },
        Seat {
            player: 1,
            device: pad(2),
            ..Seat::default()
        },
    ];
    (g, probe)
}

/// Skill-Menü als Overlay (Go: `skillMenu`): `View` öffnet bzw. schließt es für den Spieler des Geräts; offen nimmt es
/// dessen Eingabe und lernt bei Bestätigen.
pub struct SkillMenu {
    pub open: Rc<Counter<[bool; 2]>>,
}

impl SkillMenu {
    pub fn new() -> (SkillMenu, Rc<Counter<[bool; 2]>>) {
        let open = Rc::new(Counter::new([false; 2]));
        (SkillMenu { open: Rc::clone(&open) }, open)
    }
}

impl Overlay for SkillMenu {
    fn update(&mut self, input: &Input, f: &Frame, s: &mut dyn Session) -> OverlayStep {
        let mut st = OverlayStep::default();
        let mut open = self.open.get();
        for seat in &f.seats {
            let p = usize::try_from(seat.player).unwrap_or(0);
            let (di, _) = input.of(seat.device);
            for a in di.actions {
                if a == Action::View {
                    open[p] = !open[p];
                } else if a == Action::Confirm && open[p] {
                    // Das Skill-Menü des Tests lernt immer denselben Skill; Fehler zählt der Test nicht.
                    let _ = s.learn(seat.player, "schild");
                    st.cues.push(Cue {
                        kind: CueKind::Confirm,
                        player: seat.player,
                        ..Default::default()
                    });
                }
            }
            st.consumed[p] = open[p];
        }
        self.open.set(open);
        st
    }
}
