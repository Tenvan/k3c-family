//! Test-Hilfen der Schleife: `FakeSim` (eine Simulation mit zwei Plätzen, frei steuerbar), Test-Drawer, -Screens, -Menüs
//! und -Quelle, dazu `parts` (zusammengebaut mit `assemble`). Gibt es nur in Tests.

use std::cell::{Cell as Counter, RefCell};
use std::rc::Rc;

use k3c_platform::{Canvas, InputState};
use k3c_sim::state::{Player, World};

#[path = "dev_testutil.rs"]
pub mod dev_testutil;

use self::dev_testutil::{DevHandle, DevLog};
use super::sim::Sim;
use super::{Parts, assemble};
use crate::view::cell::Cell;
use crate::view::devkeys::DevSession;
use crate::view::frame::{Frame, Outcome, Seat, Stage};
use crate::view::input::{Command, DeviceID, DeviceKind, Input, Source};
use crate::view::layer::{Drawer, Layer};
use crate::view::screen::{Menus, Screen, Step, StepKind};
use crate::view::session::{Fehler, NewGame, SaveInfo, Session, WorldInfo};
use crate::view::settings::Settings;

/// Protokoll der Zeichen-Aufrufe und Zustände von außen sichtbar.
pub type Log = Rc<RefCell<Vec<String>>>;

/// Was der Test von außen an der Simulation sieht und steuert.
#[derive(Clone, Default)]
pub struct Probe {
    /// Welt-x der Monarchen (Index = Spieler)
    pub xs: Rc<Counter<[f64; 2]>>,
    pub closed: Rc<Counter<bool>>,
    pub paused: Rc<Counter<bool>>,
    pub active: Rc<Counter<bool>>,
    /// Takte der Simulation
    pub steps: Rc<Counter<u64>>,
    /// Ausgang, den `step` in den Frame schreibt
    pub outcome: Rc<Counter<Outcome>>,
    /// Befehle in Reihenfolge der Aufrufe: (Spieler, Befehl)
    pub cmds: Rc<RefCell<Vec<(i64, Command)>>>,
    /// Geräte, die `join` aufnahm
    pub joins: Rc<RefCell<Vec<DeviceID>>>,
    /// Neuzuordnungen: (Spieler, Gerät)
    pub rebinds: Rc<RefCell<Vec<(i64, DeviceID)>>>,
    /// gelernte Skills
    pub learned: Rc<RefCell<Vec<String>>>,
    /// Einstellungen, die `set_settings` bekam
    pub set: Rc<RefCell<Vec<Settings>>>,
    /// Aufrufe der Entwickler-Rolle (`FakeSim::with_dev`)
    pub dev: Rc<RefCell<DevLog>>,
    /// Spiele, die `new_game` bekam
    pub games: Rc<RefCell<Vec<NewGame>>>,
    /// Spielstände, die `list` meldet
    pub saves: Rc<RefCell<Vec<SaveInfo>>>,
    /// Namen, die `delete` angenommen hat
    pub deleted: Rc<RefCell<Vec<String>>>,
    /// Name des laufenden Spiels: `delete` lehnt ihn ab, solange es aktiv ist
    pub running: Rc<RefCell<String>>,
}

/// Tastatur 1 (`keys1`).
pub fn keys1() -> DeviceID {
    DeviceID::default()
}

/// Gamepad `n` (Pad-ID der Plattform; `pad(0)` heißt „pad1“).
pub fn pad(n: i64) -> DeviceID {
    DeviceID::new(DeviceKind::Gamepad, n)
}

/// Ein Spiel mit zwei Plätzen; `step` baut je Takt einen Frame mit einer Welt und zwei Monarchen.
pub struct FakeSim {
    pub probe: Probe,
    seats: Vec<Seat>,
    tick: u64,
    /// Frames, die `step` der Reihe nach liefert (danach den letzten ohne Ereignisse); leer = der eigene Frame
    script: Vec<Frame>,
    /// Entwickler-Rolle (`with_dev`), sonst keine
    dev: Option<DevHandle>,
}

impl FakeSim {
    /// `n` Plätze (höchstens 2), Monarchen bei x = 10 und 50.
    pub fn new(n: usize) -> (FakeSim, Probe) {
        let probe = Probe::default();
        probe.xs.set([10.0, 50.0]);
        let seats = (0..n.min(2))
            .map(|i| Seat {
                player: i64::try_from(i).unwrap_or(0),
                monarch: i64::try_from(i).unwrap_or(0),
                device: if i == 0 { keys1() } else { pad(0) },
                ..Seat::default()
            })
            .collect();
        let sim = FakeSim {
            probe: probe.clone(),
            seats,
            tick: 0,
            script: Vec::new(),
            dev: None,
        };
        (sim, probe)
    }

    /// Hängt die Entwickler-Rolle ein; ihre Aufrufe stehen in `Probe::dev` (Go: `devSim`).
    pub fn with_dev(mut self) -> FakeSim {
        self.dev = Some(DevHandle(Rc::clone(&self.probe.dev)));
        self
    }

    /// Liefert je Takt den nächsten Frame der Liste (danach den letzten ohne Ereignisse), Uhr und Takt setzt `step`
    /// (Go: `voySim`).
    pub fn with_script(mut self, frames: Vec<Frame>) -> FakeSim {
        self.script = frames;
        self
    }

    fn world(&self) -> World {
        let xs = self.probe.xs.get();
        let player = |i: usize| Player {
            index: i64::try_from(i).unwrap_or(0),
            x: xs[i],
            ..Player::default()
        };
        World {
            players: vec![player(0), player(1)],
            ..World::default()
        }
    }
}

impl Sim for FakeSim {
    fn command(&mut self, player: i64, c: Command) -> Result<(), Fehler> {
        self.probe.cmds.borrow_mut().push((player, c));
        Ok(())
    }

    fn dev_session(&mut self) -> Option<&mut dyn DevSession> {
        self.dev.as_mut().map(|d| d as &mut dyn DevSession)
    }

    #[allow(clippy::cast_precision_loss)]
    fn step(&mut self) -> Frame {
        self.tick += 1;
        self.probe.steps.set(self.tick);
        if !self.script.is_empty() {
            let i = usize::try_from(self.tick).unwrap_or(usize::MAX).min(self.script.len()) - 1;
            let mut f = self.script[i].clone();
            if self.tick > self.script.len() as u64 {
                f.events.clear();
            }
            f.tick = self.tick;
            f.clock = self.tick as f64 / 30.0;
            return f;
        }
        Frame {
            tick: self.tick,
            clock: self.tick as f64 / 30.0,
            seats: self.seats.clone(),
            outcome: self.probe.outcome.get(),
            stages: vec![Stage {
                index: 0,
                world: Some(Rc::new(self.world())),
                ..Stage::default()
            }],
            ..Frame::default()
        }
    }
}

impl Session for FakeSim {
    fn new_game(&mut self, g: NewGame) -> Result<(), Fehler> {
        self.probe.games.borrow_mut().push(g);
        self.probe.active.set(true);
        Ok(())
    }

    fn continue_game(&mut self, _devices: &[DeviceID]) -> Result<(), Fehler> {
        self.probe.active.set(true);
        Ok(())
    }

    fn load(&mut self, _name: &str, _devices: &[DeviceID]) -> Result<(), Fehler> {
        self.probe.active.set(true);
        Ok(())
    }

    fn list(&mut self) -> Result<Vec<SaveInfo>, Fehler> {
        Ok(self.probe.saves.borrow().clone())
    }

    fn delete(&mut self, name: &str) -> Result<(), Fehler> {
        if self.probe.active.get() && *self.probe.running.borrow() == name {
            return Err(Fehler::Session("läuft".to_string()));
        }
        self.probe.deleted.borrow_mut().push(name.to_string());
        Ok(())
    }

    fn save(&mut self) -> Result<(), Fehler> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), Fehler> {
        self.probe.closed.set(true);
        self.probe.active.set(false);
        Ok(())
    }

    fn active(&self) -> bool {
        self.probe.active.get()
    }

    fn pause(&mut self, on: bool) {
        self.probe.paused.set(on);
    }

    fn join(&mut self, d: DeviceID) -> Result<i64, Fehler> {
        self.probe.joins.borrow_mut().push(d);
        let player = i64::try_from(self.seats.len()).unwrap_or(0);
        if self.seats.len() < 2 {
            self.seats.push(Seat {
                player,
                monarch: player,
                device: d,
                ..Seat::default()
            });
        }
        Ok(player)
    }

    fn leave(&mut self, _player: i64) -> Result<(), Fehler> {
        Ok(())
    }

    fn rebind(&mut self, player: i64, d: DeviceID) -> Result<(), Fehler> {
        self.probe.rebinds.borrow_mut().push((player, d));
        Ok(())
    }

    fn learn(&mut self, _player: i64, skill: &str) -> Result<(), Fehler> {
        self.probe.learned.borrow_mut().push(skill.to_string());
        Ok(())
    }

    fn respec(&mut self, _player: i64) -> Result<(), Fehler> {
        Ok(())
    }

    fn choose_island(&mut self, _player: i64, _island: &str) -> Result<(), Fehler> {
        Ok(())
    }

    fn set_settings(&mut self, s: Settings) -> Result<(), Fehler> {
        self.probe.set.borrow_mut().push(s);
        Ok(())
    }

    fn settings(&self) -> Settings {
        Settings::default()
    }

    fn seats(&self) -> Vec<Seat> {
        self.seats.clone()
    }

    fn world_list(&self) -> Vec<WorldInfo> {
        Vec::new()
    }

    fn world(&self) -> String {
        String::new()
    }
}

/// Eingabe-Quelle ohne Eingaben.
pub struct NoInput;

impl Source for NoInput {
    fn poll(&mut self, _raw: &InputState) -> Input {
        Input::default()
    }
}

/// Drawer, der Ebene und Zelle ins Protokoll schreibt; mit `panics` stürzt er nach dem Eintrag ab.
pub struct LogDrawer {
    pub layer: Layer,
    pub name: &'static str,
    pub log: Log,
    pub panics: bool,
    /// Uhr des Frames, wie der Drawer sie sah
    pub clock: Rc<Counter<f64>>,
}

impl LogDrawer {
    pub fn new(layer: Layer, name: &'static str, log: &Log) -> LogDrawer {
        LogDrawer {
            layer,
            name,
            log: Rc::clone(log),
            panics: false,
            clock: Rc::default(),
        }
    }
}

impl Drawer for LogDrawer {
    fn layer(&self) -> Layer {
        self.layer
    }

    fn draw(&self, _dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        self.clock.set(f.clock);
        let idx = if c.index >= 0 {
            c.index.to_string()
        } else {
            String::new()
        };
        self.log.borrow_mut().push(format!("{}{idx}", self.name));
        assert!(!self.panics, "Drawer {} stürzt ab", self.name);
    }
}

/// Screen, der bleibt (`Stay`) oder die Schleife mit `kind` weiterschaltet; zeichnet "screen" ins Protokoll.
pub struct TestScreen {
    pub kind: StepKind,
    pub log: Log,
}

impl Screen for TestScreen {
    fn update(&mut self, _input: &Input, _s: &mut dyn Session) -> Step {
        Step {
            kind: self.kind,
            ..Step::default()
        }
    }

    fn draw(&self, _dst: &mut dyn Canvas, _f: &Frame, _c: &Cell) {
        self.log.borrow_mut().push("screen".to_string());
    }
}

/// Menüs aus `TestScreen`; `kind` bestimmt, was der Hauptmenü-Screen im Update tut.
pub struct TestMenus {
    pub kind: StepKind,
    pub log: Log,
}

impl Menus for TestMenus {
    fn main(&mut self) -> Box<dyn Screen> {
        Box::new(TestScreen {
            kind: self.kind,
            log: Rc::clone(&self.log),
        })
    }

    fn pause(&mut self) -> Box<dyn Screen> {
        self.main()
    }
}

/// Teile für die Schleife: Eingabe ohne Tasten, Test-Menüs und die gegebenen Drawer.
pub fn parts(kind: StepKind, log: &Log, drawers: Vec<Box<dyn Drawer>>) -> Parts {
    let part = super::Part {
        drawers,
        source: Some(Box::new(NoInput)),
        menus: Some(Box::new(TestMenus {
            kind,
            log: Rc::clone(log),
        })),
        ..super::Part::named("test")
    };
    // `assemble` meldet nur fehlende Pflicht-Rollen; hier stehen beide (kein Fehlerfall).
    assemble(vec![part]).unwrap_or_else(|e| unreachable!("{e}"))
}
