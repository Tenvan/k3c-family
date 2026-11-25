//! `sim run` als `App` der Plattform-Schicht (RU33.3): derselbe `Stepper` wie headless, im Fenster mit den Zeichnern des
//! Spiels (Welt, Bauten, Figuren, Effekte, HUD; Kamera folgt dem Monarchen, ab zwei Monarchen zwei Zellen übereinander).
//! Die Logik hängt nie am Zeichnen: Das Ergebnis ist bei jedem Zeitraffer und ohne Fenster dasselbe.

use std::rc::Rc;

use k3c_platform::{App, Canvas, InputState, LOGICAL};

use super::stepper::Stepper;
use super::stop::Stop;
use crate::art::Config as ArtConfig;
use crate::replay::layout::{self, Cameras};
use crate::replay::scene::Scene;
use crate::view::cell::{Cell, Rectangle, rect};
use crate::view::frame::{Event, Frame, Seat, Stage};
use crate::view::input::{DeviceID, DeviceKind};

/// Bilder, die das Fenster nach dem letzten Takt stehen bleibt (B-1017).
pub const HOLD_FRAMES: u32 = 60;
/// Zeitraffer: kleinster und größter Wert von `--speed`.
pub const SPEED_RANGE: std::ops::RangeInclusive<i64> = 1..=64;
/// Zellen, die das Spiel zeichnet (zwei Spieler übereinander).
const MAX_CELLS: i64 = 2;

/// Wie der Lauf endete.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// Alle Takte gerechnet: der Text der Zeile `result`.
    Finished(String),
    /// Abbruch (`--stop-stdin`) oder Fenster vor dem Ende geschlossen.
    Aborted,
}

/// Der Lauf als `App`.
pub struct RunApp {
    stepper: Stepper,
    stop: Stop,
    speed: i64,
    hold: u32,
    held: u32,
    aborted: bool,
    draw_on: bool,
    cfg: Option<ArtConfig>,
    level: String,
    frame: Frame,
    cells: Vec<Cell>,
    cams: Cameras,
    scene: Option<Scene>,
    win: Rectangle,
    fresh: bool,
    /// Gezeichnete Bilder.
    pub frames: u64,
}

fn seats(n: i64, w: &k3c_sim::state::World) -> Vec<Seat> {
    (0..n.min(MAX_CELLS))
        .map(|i| Seat {
            player: i,
            device: if i == 0 {
                DeviceID::new(DeviceKind::Keyboard, 0)
            } else {
                DeviceID::new(DeviceKind::Gamepad, i - 1)
            },
            monarch: w.players.get(i as usize).map_or(i, |p| p.index),
            ..Seat::default()
        })
        .collect()
}

#[allow(clippy::cast_possible_truncation)]
fn window(size: (f32, f32)) -> Rectangle {
    rect(0, 0, size.0 as i64, size.1 as i64)
}

impl RunApp {
    /// Ohne Fenster und ohne Zeichnen (Standard von `sim run`): rechnet einen Takt je Schritt.
    pub fn headless(stepper: Stepper, stop: Stop) -> RunApp {
        RunApp::build(stepper, stop, 1, 0, false, None)
    }

    /// Mit Fenster: `speed` Sim-Takte je Schritt (1 = Echtzeit, 30 Takte je Sekunde), danach `HOLD_FRAMES` Bilder stehen.
    /// `cfg` `None` zeichnet ohne Grafik (Tests).
    pub fn windowed(stepper: Stepper, stop: Stop, speed: i64, cfg: Option<ArtConfig>) -> RunApp {
        RunApp::build(stepper, stop, speed, HOLD_FRAMES, true, cfg)
    }

    fn build(stepper: Stepper, stop: Stop, speed: i64, hold: u32, draw_on: bool, cfg: Option<ArtConfig>) -> RunApp {
        let level = stepper.world().biome.id.clone();
        let frame = Frame {
            seats: seats(stepper.players(), stepper.world()),
            stages: vec![Stage {
                level: level.clone(),
                ..Stage::default()
            }],
            ..Frame::default()
        };
        let mut a = RunApp {
            stepper,
            stop,
            speed,
            hold,
            held: 0,
            aborted: false,
            draw_on,
            cfg,
            level,
            frame,
            cells: Vec::new(),
            cams: Cameras::default(),
            scene: None,
            win: window(LOGICAL),
            fresh: true,
            frames: 0,
        };
        if draw_on {
            a.install(Vec::new());
        }
        a
    }

    /// Wie der Lauf endete; vor dem Ende der Takte oder nach Abbruch `Aborted`.
    pub fn outcome(&self) -> Outcome {
        if self.aborted || !self.stepper.done() {
            Outcome::Aborted
        } else {
            Outcome::Finished(self.stepper.finish())
        }
    }

    /// Gerechnete Takte.
    pub fn ticks_done(&self) -> i64 {
        self.stepper.tick()
    }

    /// Legt die Welt des aktuellen Takts mit den Ereignissen aller Takte seit dem letzten Schritt in den Frame.
    fn install(&mut self, events: Vec<Event>) {
        let w = self.stepper.world();
        self.frame.tick = self.stepper.tick() as u64;
        self.frame.clock = w.time;
        self.frame.events = events;
        self.frame.stages[0].world = Some(Rc::new(w.clone()));
        self.fresh = true;
    }

    /// Zellen, Kameras und Listener einmal je Takt.
    fn update(&mut self) {
        let Some(scene) = self.scene.as_mut() else {
            self.fresh = true;
            return;
        };
        layout::cells(&mut self.cells, self.win, &self.frame);
        self.cams.follow(&mut scene.camera_rule, &mut self.cells, &self.frame);
        self.frame.cells.clone_from(&self.cells);
        scene.listen(&self.frame);
        self.fresh = false;
    }
}

impl App for RunApp {
    fn step(&mut self, _input: &InputState, _dt: f64) {
        if self.stepper.done() {
            // alle Takte sind gerechnet: ein Stop beendet nur noch das Stehenbleiben, das Ergebnis gilt
            self.held = if self.stop.is_set() { self.hold } else { self.held + 1 };
            return;
        }
        if self.stop.is_set() {
            self.aborted = true;
            return;
        }
        let mut events = Vec::new();
        for _ in 0..self.speed {
            if self.stepper.done() {
                break;
            }
            self.stepper.step();
            let tick = self.stepper.tick() as u64;
            events.extend(self.stepper.world().events.iter().map(|e| Event {
                stage: 0,
                tick,
                data: e.clone(),
            }));
        }
        if self.draw_on {
            self.install(events);
            self.update();
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, _alpha: f64) {
        if !self.draw_on {
            return;
        }
        self.frames += 1;
        if self.scene.is_none() {
            self.win = window(c.size());
            let w = self.stepper.world().clone();
            let scene = Scene::build(self.cfg.as_ref(), &self.level, &w, c);
            for n in &scene.notes {
                eprintln!("⚠ sim run: {n}");
            }
            self.scene = Some(scene);
        }
        if self.fresh {
            self.update();
        }
        if let Some(s) = &self.scene {
            s.draw(c, &self.frame, &self.cells);
        }
    }

    fn done(&self) -> bool {
        self.aborted || (self.stepper.done() && self.held >= self.hold)
    }
}
