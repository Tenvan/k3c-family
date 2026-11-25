//! `sim balance --window` als `App` der Plattform-Schicht (RU40.2): Die Läufe (Szenarien einer Matrix oder eines Ziel-Modus,
//! eine Aufnahme) rechnen nacheinander im Fenster, gezeichnet mit den Zeichnern des Spiels (Welt, Bauten, Figuren, Effekte;
//! die Kamera folgt dem Monarchen, ab zwei Monarchen zwei Zellen übereinander). Die Logik hängt nie am Zeichnen: Die
//! Ergebnisse sind bei jedem Zeitraffer und ohne Fenster dieselben wie headless (`balance_run.rs`).

use std::collections::VecDeque;
use std::rc::Rc;

use k3c_balance::parallel::ProgressFn;
use k3c_balance::replay::Replay;
use k3c_balance::run::Scenario;
use k3c_platform::{App, Canvas, InputState, LOGICAL};
use k3c_sim::island::Island;
use k3c_sim::state::World;

use super::balance_run::{Active, Done, ReplayRun, ScenarioRun};
use super::stop::Stop;
use super::window::{HOLD_FRAMES, seats};
use crate::art::Config as ArtConfig;
use crate::replay::layout::{self, Cameras};
use crate::replay::scene::Scene;
use crate::view::cell::{Cell, Rectangle, rect};
use crate::view::frame::{Event, Frame, Stage};

/// Ein Lauf der Warteschlange.
pub enum Job {
    /// Ein Szenario mit der Gold-Schwelle des Berichts.
    Scenario(Scenario, i64),
    Replay(Box<Replay>),
}

impl Job {
    fn start(&self) -> Active {
        match self {
            Job::Scenario(sc, threshold) => Active::Scenario(Box::new(ScenarioRun::start(sc, *threshold))),
            Job::Replay(r) => Active::Replay(Box::new(ReplayRun::start(r))),
        }
    }
}

/// Wie die Läufe endeten.
pub enum Outcome {
    /// Alle Läufe fertig, in der Reihenfolge der Warteschlange.
    Finished(Vec<Done>),
    /// Abbruch (`--stop-stdin`) oder Fenster vor dem Ende geschlossen.
    Aborted,
}

/// Die Stufe, die das Fenster zeigt: die erste mit einem Monarchen (Matrix mit Welt: dort, wo der Bot gerade ist).
fn shown_stage(isl: &Island) -> &World {
    isl.stages
        .iter()
        .find(|w| !w.players.is_empty())
        .unwrap_or(&isl.stages[0])
}

#[allow(clippy::cast_possible_truncation)]
fn window(size: (f32, f32)) -> Rectangle {
    rect(0, 0, size.0 as i64, size.1 as i64)
}

/// Bild, Zellen und Kameras des laufenden Laufs.
struct Viewer {
    cfg: Option<ArtConfig>,
    level: String,
    frame: Frame,
    cells: Vec<Cell>,
    cams: Cameras,
    scene: Option<Scene>,
    win: Rectangle,
    fresh: bool,
    ticks: u64,
    players: i64,
}

impl Viewer {
    fn new(cfg: Option<ArtConfig>) -> Viewer {
        Viewer {
            cfg,
            level: String::new(),
            frame: Frame::default(),
            cells: Vec::new(),
            cams: Cameras::default(),
            scene: None,
            win: window(LOGICAL),
            fresh: true,
            ticks: 0,
            players: 0,
        }
    }

    /// Ein neuer Lauf: Szene und Zähler von vorn.
    fn reset(&mut self) {
        self.scene = None;
        self.ticks = 0;
        self.players = 0;
    }

    /// Die Ereignisse eines Takts der gezeigten Stufe.
    fn tick(&mut self, isl: &Island, events: &mut Vec<Event>) {
        self.ticks += 1;
        let tick = self.ticks;
        events.extend(shown_stage(isl).events.iter().map(|e| Event {
            stage: 0,
            tick,
            data: e.clone(),
        }));
    }

    /// Legt die gezeigte Stufe mit den Ereignissen seit dem letzten Bild in den Frame.
    fn show(&mut self, isl: &Island, events: Vec<Event>) {
        let w = shown_stage(isl);
        let n = w.players.len().max(1) as i64;
        if self.level != w.biome.id || self.players != n {
            if self.level != w.biome.id {
                self.scene = None;
            }
            self.level.clone_from(&w.biome.id);
            self.players = n;
            self.frame.seats = seats(n, w);
            self.frame.stages = vec![Stage {
                level: self.level.clone(),
                ..Stage::default()
            }];
        }
        self.frame.tick = self.ticks;
        self.frame.clock = w.time;
        self.frame.events = events;
        self.frame.stages[0].world = Some(Rc::new(w.clone()));
        self.fresh = true;
    }

    /// Zellen, Kameras und Listener einmal je Bild.
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

    fn draw(&mut self, c: &mut dyn Canvas) {
        let Some(world) = self.frame.stages.first().and_then(|s| s.world.clone()) else {
            return;
        };
        if self.scene.is_none() {
            self.win = window(c.size());
            let scene = Scene::build(self.cfg.as_ref(), &self.level, &world, c);
            for n in &scene.notes {
                eprintln!("⚠ sim balance: {n}");
            }
            self.scene = Some(scene);
            self.fresh = true;
        }
        if self.fresh {
            self.update();
        }
        if let Some(s) = &self.scene {
            s.draw(c, &self.frame, &self.cells);
        }
    }
}

/// Die Läufe als `App`.
pub struct BalanceApp<'a> {
    pending: VecDeque<Job>,
    active: Option<Active>,
    done: Vec<Done>,
    total: usize,
    stop: Stop,
    speed: i64,
    hold: u32,
    held: u32,
    aborted: bool,
    view: Option<Viewer>,
    progress: Option<ProgressFn<'a>>,
    /// Gezeichnete Bilder.
    pub frames: u64,
}

impl<'a> BalanceApp<'a> {
    /// Ohne Fenster und ohne Zeichnen: rechnet `speed` Takte je Schritt.
    pub fn headless(jobs: Vec<Job>, stop: Stop, speed: i64, progress: Option<ProgressFn<'a>>) -> BalanceApp<'a> {
        BalanceApp::build(jobs, stop, speed, progress, None)
    }

    /// Mit Fenster: `speed` Takte je Schritt (1 = Echtzeit, 30 Takte je Sekunde), danach `HOLD_FRAMES` Bilder stehen.
    /// `cfg` `None` zeichnet ohne Grafik (Tests).
    pub fn windowed(
        jobs: Vec<Job>,
        stop: Stop,
        speed: i64,
        progress: Option<ProgressFn<'a>>,
        cfg: Option<ArtConfig>,
    ) -> BalanceApp<'a> {
        BalanceApp::build(jobs, stop, speed, progress, Some(Viewer::new(cfg)))
    }

    fn build(
        jobs: Vec<Job>,
        stop: Stop,
        speed: i64,
        progress: Option<ProgressFn<'a>>,
        view: Option<Viewer>,
    ) -> BalanceApp<'a> {
        let mut a = BalanceApp {
            total: jobs.len(),
            pending: jobs.into(),
            active: None,
            done: Vec::new(),
            stop,
            speed: speed.max(1),
            hold: if view.is_some() { HOLD_FRAMES } else { 0 },
            held: 0,
            aborted: false,
            view,
            progress,
            frames: 0,
        };
        a.next_job();
        a
    }

    fn next_job(&mut self) {
        self.active = self.pending.pop_front().map(|j| j.start());
        if let Some(v) = self.view.as_mut() {
            v.reset();
        }
    }

    fn all_done(&self) -> bool {
        self.active.is_none()
    }

    /// Die Ergebnisse, wenn alle Läufe fertig sind; vor dem Ende oder nach Abbruch `Aborted`.
    pub fn outcome(self) -> Outcome {
        if self.aborted || !self.all_done() {
            Outcome::Aborted
        } else {
            Outcome::Finished(self.done)
        }
    }

    /// Ein Takt des laufenden Laufs; `true`: der Lauf ist damit zu Ende.
    fn step_once(&mut self, events: &mut Vec<Event>) -> bool {
        let Some(a) = self.active.as_mut() else { return true };
        a.step();
        if let (Some(v), Some(isl)) = (self.view.as_mut(), a.island()) {
            v.tick(isl, events);
        }
        if !a.finished() {
            return false;
        }
        if let Some(a) = self.active.take() {
            self.done.push(a.into_done());
        }
        if let Some(p) = self.progress.as_mut() {
            p(self.done.len(), self.total);
        }
        self.next_job();
        true
    }
}

impl App for BalanceApp<'_> {
    fn step(&mut self, _input: &InputState, _dt: f64) {
        if self.all_done() {
            // alle Läufe sind gerechnet: ein Stop beendet nur noch das Stehenbleiben, die Ergebnisse gelten
            self.held = if self.stop.is_set() { self.hold } else { self.held + 1 };
            return;
        }
        if self.stop.is_set() {
            self.aborted = true;
            return;
        }
        let mut events = Vec::new();
        for _ in 0..self.speed {
            if self.step_once(&mut events) {
                break;
            }
        }
        if let (Some(v), Some(isl)) = (self.view.as_mut(), self.active.as_ref().and_then(Active::island)) {
            v.show(isl, events);
            v.update();
        }
    }

    fn draw(&mut self, c: &mut dyn Canvas, _alpha: f64) {
        if let Some(v) = self.view.as_mut() {
            self.frames += 1;
            v.draw(c);
        }
    }

    fn done(&self) -> bool {
        self.aborted || (self.all_done() && self.held >= self.hold)
    }
}
