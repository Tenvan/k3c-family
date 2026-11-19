//! Tests der Verdrahtung des Stresstests in Rust: Starter des Menüs (zwei Spieler), Halt beim Übergang Menü zu Spiel,
//! Geräte der Spieler und Schreibfehler des Berichts. Helfer aus `stress_tests.rs`.

use std::cell::Cell as Counter;
use std::rc::Rc;

use k3c_platform::{Canvas, InputState};

use super::tests::{cfg, step_with, stress_game_of};
use super::*;
use crate::game::packages::Libs;
use crate::game::play_testutil::play_game_with;
use crate::game::testutil::{FakeSim, keys1, pad};
use crate::game::{Args, Mode};
use crate::view::cell::Cell as ViewCell;
use crate::view::input::Input;
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;

/// B-918: Jeder Übergang Menü → Spiel hebt einen hängenden Halt auf; ohne Menü-Übergang bleibt der Halt Sache der
/// Entwickler-Taste (Go: `step` mit `StepPlay` und `StepStay`; hier über einen Takt der Schleife).
#[test]
fn step_from_menu_to_play_clears_halt() {
    let (mut g, p, _, _) = stress_game_of("halt");
    p.active.set(true);
    p.dev.borrow_mut().halt = true;
    g.mode = Mode::Menu;
    let pause = g.parts.menus.pause();
    g.stack.push(pause);
    g.stack.push(Box::new(StarterScreen::new(None, StepKind::Play)));
    g.advance(&InputState::new());
    assert!(
        g.mode == Mode::Play && !p.dev.borrow().halt,
        "Modus {:?}, Halt {}",
        g.mode,
        p.dev.borrow().halt
    );
    p.dev.borrow_mut().halt = true;
    g.advance(&InputState::new());
    assert!(p.dev.borrow().halt, "Halt ohne Menü-Übergang aufgehoben");
}

/// Screen, der im ersten `update` (falls gesetzt) den Stresstest über den Starter des Menüs anfordert und `kind` liefert.
struct StarterScreen {
    start: Option<(Box<dyn StressStarter>, StressTest)>,
    kind: StepKind,
    result: Rc<Counter<Option<bool>>>,
}

impl StarterScreen {
    fn new(start: Option<(Box<dyn StressStarter>, StressTest)>, kind: StepKind) -> StarterScreen {
        StarterScreen {
            start,
            kind,
            result: Rc::default(),
        }
    }
}

impl Screen for StarterScreen {
    fn update(&mut self, _input: &Input, _s: &mut dyn Session) -> Step {
        if let Some((st, t)) = self.start.as_mut() {
            self.result.set(Some(st.start_stress(t, keys1()).is_ok()));
        }
        Step {
            kind: self.kind,
            ..Step::default()
        }
    }

    fn draw(&self, _dst: &mut dyn Canvas, _f: &Frame, _c: &ViewCell) {}
}

/// Der Starter des Menüs startet den Test mit zwei Spielern aus dem `update` des Screens: Spiel samt Geräten, F2/F10,
/// danach liefert der Screen `Play` (Test gegen `FakeSim`).
#[test]
fn starter_of_menu_starts_stress_with_two_players() {
    let (mut g, p, _, _) = stress_game_of("menu");
    p.active.set(true);
    g.mode = Mode::Menu;
    let scr = StarterScreen::new(Some((g.stress_starter(), cfg())), StepKind::Play);
    let result = Rc::clone(&scr.result);
    g.stack.push(Box::new(scr));
    g.advance(&InputState::new());
    assert_eq!(result.get(), Some(true), "Starter lehnt ab");
    assert!(g.stress.is_some(), "Test läuft nicht");
    assert_eq!(g.mode, Mode::Play);
    let games = p.games.borrow();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].devices, vec![keys1(), pad(STRESS_PAD_BASE + 1)]);
    assert_eq!(g.dev.debug, DEBUG_VALUES);
}

/// B-1117/AC-02: `wire_stress_starter` (Aufruf von `build_game_with` nach `Game::new`) gibt dem Menü den Starter.
#[test]
fn wire_gives_menu_the_starter() {
    use crate::game::testutil::{Log, parts};
    let log = Log::default();
    let (sim, _) = FakeSim::new(2);
    let mut g = Game::new(
        parts(StepKind::Stay, &log, Vec::new()),
        Box::new(sim.with_dev()),
        Args::default(),
    )
    .unwrap_or_else(|e| unreachable!("{e}"));
    assert!(
        !log.borrow().iter().any(|l| l == "stress-starter"),
        "vor der Verdrahtung"
    );
    g.wire_stress_starter();
    assert!(
        log.borrow().iter().any(|l| l == "stress-starter"),
        "Starter nicht im Menü"
    );
}

/// B-1117/AC-02: Die echten Pakete (Menü samt Testing-Seite) bauen mit der Verdrahtung ohne Fehler.
#[test]
fn build_game_with_real_packages_wires_menu() {
    let (sim, _) = FakeSim::new(2);
    let g = crate::game::launch::build_game_with(&Args::default(), Box::new(sim.with_dev()), &Libs::default());
    assert!(g.is_ok(), "{:?}", g.err());
}

/// Ohne Entwickler-Rolle lehnt der Starter ab, mit dem Fehlertext von Go; es entsteht kein Spiel.
#[test]
fn starter_without_dev_role_refuses() {
    let (sim, probe) = FakeSim::new(2);
    let (mut g, p) = play_game_with(sim, probe, Vec::new());
    p.games.borrow_mut().clear();
    let scr = StarterScreen::new(Some((g.stress_starter(), cfg())), StepKind::Stay);
    let result = Rc::clone(&scr.result);
    g.stack.push(Box::new(scr));
    g.advance(&InputState::new());
    assert_eq!(result.get(), Some(false));
    assert!(g.stress.is_none() && p.games.borrow().is_empty());
}

/// Zwei Spieler: Gerät `by` als Spieler 1, dann Platzhalter-Pads ab `STRESS_PAD_BASE + 1`; Seed nur bei `seed > 0`.
#[test]
fn stress_devices_and_game_for_players() {
    let t = StressTest {
        players: 3,
        seed: 0,
        ..cfg()
    };
    assert_eq!(stress_devices(&t, keys1()), vec![keys1(), pad(1001), pad(1002)]);
    let ng = stress_game(&t, keys1());
    assert!(ng.seed.is_empty() && ng.grade == "normal" && ng.name == "stresstest");
    assert_eq!(stress_game(&cfg(), keys1()).seed, "42");
}

/// Ein Fehler beim Schreiben des Berichts steht als Hinweis und beendet den Test ohne Halt.
#[test]
fn report_write_failure_is_a_cue() {
    let (mut g, p, _, dir) = stress_game_of("blocked");
    std::fs::create_dir_all(dir.parent().unwrap()).unwrap();
    std::fs::write(&dir, "kein Ordner").unwrap();
    g.start_stress(&cfg(), keys1()).unwrap();
    step_with(&mut g, 1, 0, &[]);
    p.active.set(false);
    g.stress_step();
    assert!(g.stress.is_none() && !p.dev.borrow().halt);
    assert!(
        g.cues.iter().any(|c| c.text.starts_with("Stresstest-Bericht: ")),
        "Hinweise {:?}",
        g.cues
    );
    let _ = std::fs::remove_file(&dir);
}
