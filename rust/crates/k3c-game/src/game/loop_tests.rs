//! Tests der Schleife `Game` (Go hat für `game.go` keinen eigenen Test): headless mit `FakeSim`, Zeichen-Reihenfolge,
//! Kamera je Zelle, `restore` nach einem Panic, Übergänge der Modi.

use std::cell::{Cell as Counter, RefCell};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;

use k3c_platform::{App, NullCanvas, STEP, run_headless};

use super::testutil::{FakeSim, Log, LogDrawer, Probe, parts};
use super::*;
use crate::view::layer::{Drawer, Layer};
use crate::view::screen::StepKind;

fn log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

fn scene_args() -> Args {
    Args {
        scene: Scene {
            seed: "t".to_string(),
            players: 2,
            ..Scene::default()
        },
        ..Args::default()
    }
}

fn game(args: Args, kind: StepKind, drawers: Vec<Box<dyn Drawer>>, log: &Log) -> (Game, Probe) {
    let (sim, probe) = FakeSim::new(2);
    let g = Game::new(parts(kind, log, drawers), Box::new(sim), args);
    (g.unwrap_or_else(|e| unreachable!("{e}")), probe)
}

/// 100 Takte im Menü und im Spiel über `run_headless` ohne Absturz; Zellen und Takt stimmen.
#[test]
fn headless_100_takte() {
    for args in [Args::default(), scene_args()] {
        let l = log();
        let (mut g, probe) = game(args, StepKind::Stay, Vec::new(), &l);
        let canvas = run_headless(&mut g, 100, None);
        assert_eq!(probe.steps.get(), 100);
        assert_eq!(g.frame().tick, 100);
        assert_eq!(g.frame().cells.len(), 2);
        assert!(canvas.rects > 0, "Hintergrund gezeichnet");
        assert!(!g.done());
    }
}

/// Eine Szene mit Seed startet das Spiel ohne Menü; ohne Seed liegt das Hauptmenü oben.
#[test]
fn start_menue_oder_spiel() {
    let l = log();
    let (g, probe) = game(Args::default(), StepKind::Stay, Vec::new(), &l);
    assert_eq!(g.mode(), Mode::Menu);
    assert!(g.screen().is_some());
    assert!(!probe.active.get());
    let (g, probe) = game(scene_args(), StepKind::Stay, Vec::new(), &l);
    assert_eq!(g.mode(), Mode::Play);
    assert!(g.screen().is_none());
    assert!(probe.active.get());
}

/// Der Vorlauf (`--tick`) rechnet Takte ohne Zeichnen.
#[test]
fn vorlauf_rechnet_takte() {
    let mut a = scene_args();
    a.scene.tick = 5;
    let (g, probe) = game(a, StepKind::Stay, Vec::new(), &log());
    assert_eq!(probe.steps.get(), 5);
    assert_eq!(g.frame().tick, 5);
}

/// Zelle 0, Zelle 1, Overlay, oberster Screen; je Zelle nur die Zellen-Ebenen in Ebenen-Reihenfolge.
#[test]
fn zeichen_reihenfolge() {
    let l = log();
    let drawers: Vec<Box<dyn Drawer>> = vec![
        Box::new(LogDrawer::new(Layer::OVERLAY, "o", &l)),
        Box::new(LogDrawer::new(Layer::HUD, "h", &l)),
        Box::new(LogDrawer::new(Layer::GROUND, "g", &l)),
    ];
    let (mut g, _) = game(Args::default(), StepKind::Stay, drawers, &l);
    run_headless(&mut g, 1, None);
    l.borrow_mut().clear();
    let mut c = NullCanvas::default();
    g.draw(&mut c, 0.0);
    assert_eq!(*l.borrow(), ["g0", "h0", "g1", "h1", "o", "screen"]);
    assert_eq!(c.clips, 3, "Zelle 0, Zelle 1, dann ohne Ausschnitt");
}

/// Jede Zelle folgt dem eigenen Monarchen (zwei Spieler, zwei Kameras).
#[test]
fn kamera_je_zelle_folgt_dem_eigenen_monarchen() {
    let (mut g, probe) = game(scene_args(), StepKind::Stay, Vec::new(), &log());
    let raw = k3c_platform::InputState::new();
    g.step(&raw, STEP);
    assert_eq!(g.frame().cells[0].camera.x, 10.0);
    assert_eq!(g.frame().cells[1].camera.x, 50.0);
    probe.xs.set([20.0, 40.0]);
    g.step(&raw, STEP);
    assert_eq!(g.frame().cells[0].camera.x, 20.0);
    assert_eq!(g.frame().cells[1].camera.x, 40.0);
    assert_eq!(g.frame().cells[0].player, 0);
    assert_eq!(g.frame().cells[1].player, 1);
}

/// `Blend::restore` läuft auch nach einem Panic in einem Drawer: Uhr des Frames und Kameras sind danach unverschoben.
#[test]
fn restore_nach_panic_im_drawer() {
    let l = log();
    let mut d = LogDrawer::new(Layer::GROUND, "p", &l);
    d.panics = true;
    let seen = Rc::clone(&d.clock);
    let (mut g, probe) = game(scene_args(), StepKind::Stay, vec![Box::new(d)], &l);
    let raw = k3c_platform::InputState::new();
    g.step(&raw, STEP);
    probe.xs.set([30.0, 70.0]);
    g.step(&raw, STEP);
    let (clock, cam) = (g.frame().clock, g.frame().cells[0].camera.x);
    let mut c = NullCanvas::default();
    let r = catch_unwind(AssertUnwindSafe(|| g.draw(&mut c, 0.5)));
    assert!(r.is_err(), "der Panic geht weiter");
    assert!(
        seen.get() < clock,
        "der Drawer sah die verschobene Uhr: {} < {clock}",
        seen.get()
    );
    assert_eq!(g.frame().clock, clock);
    assert_eq!(g.frame().cells[0].camera.x, cam);
    assert_eq!(g.frame().cells.len(), 2, "Zellen wieder im Frame");
}

/// Das Menü startet das Spiel (`Play`): Modus wechselt, die Pause endet; `Quit` beendet und schließt die Sitzung.
#[test]
fn uebergaenge_menue_spiel_ende() {
    let raw = k3c_platform::InputState::new();
    let (mut g, probe) = game(Args::default(), StepKind::Play, Vec::new(), &log());
    probe.paused.set(true);
    probe.active.set(true);
    g.step(&raw, STEP);
    assert_eq!(g.mode(), Mode::Play);
    assert!(g.screen().is_none());
    assert!(!probe.paused.get(), "Fortsetzen hebt die Pause auf");
    let (mut g, probe) = game(Args::default(), StepKind::Quit, Vec::new(), &log());
    assert!(!g.done());
    g.step(&raw, STEP);
    assert!(g.done());
    assert!(probe.closed.get(), "sim.close beim Ende");
    let before = probe.steps.get();
    g.step(&raw, STEP);
    assert_eq!(probe.steps.get(), before, "nach dem Ende kein Takt mehr");
}

/// Wie `TestNoClockWhenStatsOff`: ohne Leistungsanzeige fragt die Schleife die Wanduhr nie ab, mit ihr je Takt zweimal.
#[test]
fn keine_wanduhr_ohne_leistungsanzeige() {
    let raw = k3c_platform::InputState::new();
    let (mut g, _) = game(scene_args(), StepKind::Stay, Vec::new(), &log());
    let calls = Rc::new(Counter::new(0u32));
    let n = Rc::clone(&calls);
    g.stats.now = Box::new(move || {
        n.set(n.get() + 1);
        0.0
    });
    g.step(&raw, STEP);
    assert_eq!(calls.get(), 0);
    g.frame.settings.stats = true;
    g.step(&raw, STEP); // `timing` liest den Frame des vorigen Takts
    assert_eq!(calls.get(), 2);
}

// ---- Hauptmenü › Spiel › Pause › Ende (RU26.5, AC-01 und AC-03) ----------------------------------------------------

use std::collections::VecDeque;

use super::play_testutil::press;
use super::testutil::{keys1, pad};
use crate::view::cell::Cell;
use crate::view::frame::{END_DELAY, Frame as MenuFrame, Outcome};
use crate::view::input::{Action, Command, Input, Source};
use crate::view::screen::{Menus, Screen, Step};
use crate::view::session::{NewGame, Session};

type Queue = Rc<RefCell<VecDeque<Input>>>;

/// Eingabe-Quelle aus einer Warteschlange: ein Eintrag je Takt, danach leer.
struct Script(Queue);

impl Source for Script {
    fn poll(&mut self, _raw: &k3c_platform::InputState) -> Input {
        self.0.borrow_mut().pop_front().unwrap_or_default()
    }
}

/// Hauptmenü des Tests: Bestätigen startet ein neues Spiel.
struct MainScreen;

impl Screen for MainScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if input.has(Action::Confirm).1 && s.new_game(NewGame::default()).is_ok() {
            return Step {
                kind: StepKind::Play,
                ..Step::default()
            };
        }
        Step::default()
    }

    fn draw(&self, _dst: &mut dyn k3c_platform::Canvas, _f: &MenuFrame, _c: &Cell) {}
}

/// Pause des Tests: Bestätigen setzt das Spiel fort, Zurück beendet es (Hauptmenü).
struct PauseScreen;

impl Screen for PauseScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        if input.has(Action::Confirm).1 {
            return Step {
                kind: StepKind::Play,
                ..Step::default()
            };
        }
        if input.has(Action::Back).1 && s.close().is_ok() {
            return Step {
                kind: StepKind::Pop,
                ..Step::default()
            };
        }
        Step::default()
    }

    fn draw(&self, _dst: &mut dyn k3c_platform::Canvas, _f: &MenuFrame, _c: &Cell) {}
}

struct LoopMenus;

impl Menus for LoopMenus {
    fn main(&mut self) -> Box<dyn Screen> {
        Box::new(MainScreen)
    }

    fn pause(&mut self) -> Box<dyn Screen> {
        Box::new(PauseScreen)
    }
}

/// Eine Schleife mit `seats` Plätzen im Hauptmenü, Eingabe aus der Warteschlange.
fn loop_game(seats: usize) -> (Game, Probe, Queue) {
    let queue = Queue::default();
    let part = Part {
        source: Some(Box::new(Script(Rc::clone(&queue)))),
        menus: Some(Box::new(LoopMenus)),
        ..Part::named("loop")
    };
    let parts = assemble(vec![part]).unwrap_or_else(|e| unreachable!("{e}"));
    let (sim, probe) = FakeSim::new(seats);
    let g = Game::new(parts, Box::new(sim), Args::default()).unwrap_or_else(|e| unreachable!("{e}"));
    (g, probe, queue)
}

/// Legt die Eingabe des nächsten Takts fest und rechnet ihn über `run_headless`.
fn tick(g: &mut Game, q: &Queue, devices: Vec<crate::view::input::DeviceInput>) {
    q.borrow_mut().push_back(Input { devices });
    run_headless(g, 1, None);
}

fn confirm() -> Vec<crate::view::input::DeviceInput> {
    vec![press(keys1(), Command::default(), &[Action::Confirm])]
}

/// Hauptmenü offen, Bestätigen startet das Spiel.
#[test]
fn hauptmenue_zu_spiel() {
    let (mut g, probe, q) = loop_game(2);
    assert_eq!(g.mode(), Mode::Menu);
    assert!(g.screen().is_some());
    assert!(!probe.active.get());
    tick(&mut g, &q, confirm());
    assert_eq!(g.mode(), Mode::Play);
    assert!(g.screen().is_none());
    assert!(probe.active.get());
}

/// Das zweite Gerät tritt bei (Bestätigen), es gibt zwei Zellen, jede folgt ihrem Monarchen.
#[test]
fn beitritt_zweiter_spieler_zwei_zellen() {
    let (mut g, probe, q) = loop_game(1);
    tick(&mut g, &q, confirm());
    assert_eq!(g.frame().cells.len(), 1);
    let both = vec![
        press(keys1(), Command::default(), &[]),
        press(pad(0), Command::default(), &[Action::Confirm]),
    ];
    tick(&mut g, &q, both);
    assert_eq!(*probe.joins.borrow(), [pad(0)]);
    assert_eq!(g.frame().cells.len(), 2);
    probe.xs.set([20.0, 40.0]);
    run_headless(&mut g, 1, None);
    assert_eq!(g.frame().cells[0].camera.x, 20.0);
    assert_eq!(g.frame().cells[1].camera.x, 40.0);
    assert_eq!((g.frame().cells[0].player, g.frame().cells[1].player), (0, 1));
}

/// Die Menü-Taste öffnet die Pause, Fortsetzen geht zurück ins Spiel, Beenden ins Hauptmenü.
#[test]
fn pause_fortsetzen_beenden() {
    let (mut g, probe, q) = loop_game(2);
    tick(&mut g, &q, confirm());
    let key = |a: Action| vec![press(keys1(), Command::default(), &[a])];
    tick(&mut g, &q, key(Action::Menu));
    assert_eq!(g.mode(), Mode::Menu);
    assert!(probe.paused.get() && g.screen().is_some());
    tick(&mut g, &q, key(Action::Confirm));
    assert_eq!(g.mode(), Mode::Play, "Fortsetzen");
    assert!(!probe.paused.get() && g.screen().is_none());
    tick(&mut g, &q, key(Action::Menu));
    tick(&mut g, &q, key(Action::Back));
    assert!(probe.closed.get(), "Beenden schließt das Spiel");
    assert_eq!(g.mode(), Mode::Menu);
    assert!(g.screen().is_some(), "Hauptmenü liegt oben");
    assert!(!probe.active.get());
}

/// Ausgang gesetzt: nach `END_DELAY` Takten liegt das Hauptmenü oben.
#[test]
fn spielende_zurueck_ins_hauptmenue() {
    let (mut g, probe, q) = loop_game(2);
    tick(&mut g, &q, confirm());
    probe.outcome.set(Outcome::Won);
    run_headless(&mut g, 1, None); // der Frame mit Ausgang entsteht
    run_headless(&mut g, u32::try_from(END_DELAY - 1).unwrap_or(0), None);
    assert!(g.mode() == Mode::Play && !probe.closed.get(), "zu früh beendet");
    run_headless(&mut g, 1, None);
    assert!(probe.closed.get(), "Spiel nach {END_DELAY} Takten geschlossen");
    assert_eq!(g.mode(), Mode::Menu);
    assert!(g.screen().is_some(), "Hauptmenü liegt oben");
}

/// Jede Zelle bekommt die Eingabe ihres eigenen Geräts: Spieler 1 läuft rechts, Spieler 2 links.
#[test]
fn befehle_getrennt_je_geraet() {
    let (mut g, probe, q) = loop_game(2);
    tick(&mut g, &q, confirm());
    probe.cmds.borrow_mut().clear();
    let go = |m: f64| Command {
        move_: m,
        ..Command::default()
    };
    let both = vec![press(keys1(), go(1.0), &[]), press(pad(0), go(-1.0), &[])];
    tick(&mut g, &q, both);
    assert_eq!(*probe.cmds.borrow(), [(0, go(1.0)), (1, go(-1.0))]);
}

/// `--menu` (Aufnahme-Modus, `menu.go`): ohne laufendes Spiel gibt es keine Pause, ohne `MenuPages` keine Seite; das
/// Hauptmenü der Szene liegt oben.
#[test]
fn menue_der_szene() {
    let open = |menu: &str| {
        let (sim, _) = FakeSim::new(2);
        let args = Args {
            scene: Scene {
                menu: menu.to_string(),
                ..Scene::default()
            },
            ..Args::default()
        };
        Game::new(parts(StepKind::Stay, &log(), Vec::new()), Box::new(sim), args)
    };
    assert_eq!(
        open("pause").err().as_deref(),
        Some("💥 --menu pause: kein laufendes Spiel")
    );
    assert_eq!(
        open("saves").err().as_deref(),
        Some("💥 --menu saves: das Menü kennt diese Seite nicht")
    );
    assert_eq!(open("main").map(|g| g.mode()).ok(), Some(Mode::Menu));
}
