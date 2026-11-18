//! Die Wiedergabe als `App` der Plattform-Schicht: je Takt ein Frame wie im Go-Spiel (`game.go` › Update: Zellen,
//! Kameras, Listener), je Bild die Zeichner des Spiels in zwei Zellen übereinander (zwei Spieler), mit Zwischenbildern.

use std::rc::Rc;

use k3c_data::biome::Biome;
use k3c_platform::{App, Canvas, InputState, LOGICAL};
use k3c_sim::state::World;

use super::blend::{Blend, MAX_CAMS};
use super::layout::{self, Cameras};
use super::player::Playback;
use super::scene::Scene;
use super::trace::Trace;
use crate::art::Config;
use crate::view::cell::{Cell, Rectangle, rect};
use crate::view::frame::{Event, Frame, Seat, Stage};
use crate::view::input::{DeviceID, DeviceKind};

/// Hub-Stufe zu Beginn (Go: `HubLevel` 1); die Aufzeichnung trägt sie nicht (`json:"-"`).
const START_HUB_LEVEL: i64 = 1;

pub struct ReplayApp {
    play: Playback,
    blend: Blend,
    frame: Frame,
    cells: Vec<Cell>,
    cams: Cameras,
    scene: Option<Scene>,
    /// Grafik (`None`: ohne Grafik, nur Tests)
    cfg: Option<Config>,
    /// Level der Stufe (= Biom-ID; Schlüssel der Grafik)
    level: String,
    /// Frame seit dem letzten Update der Zellen und Listener neu
    fresh: bool,
    win: Rectangle,
    /// Drawer-Aufrufe des letzten Bilds.
    pub last_rects: u32,
    /// Gezeichnete Bilder.
    pub frames: u64,
}

/// Biom `id` aus den Spiel-Daten (`K3C_DATA_DIR` oder eingebettet); unbekannt: leer.
fn load_biome(id: &str) -> Biome {
    let src = k3c_data::Source::from_env();
    k3c_data::load_all(&src)
        .ok()
        .and_then(|d| d.biomes.into_iter().find(|b| b.id == id))
        .unwrap_or_default()
}

/// Geräte wie die Szene des Go-Spiels (`Scene.Devices`): Spieler 1 Tastatur, Spieler 2 Pad 0.
fn seats(w: &World, n: u32) -> Vec<Seat> {
    (0..i64::from(n))
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

impl ReplayApp {
    /// Wiedergabe ohne Grafik (Tests, Prüfwert).
    pub fn new(t: Trace) -> Self {
        Self::with_config(t, None)
    }

    /// Wiedergabe mit der Grafik aus `cfg`.
    pub fn with_config(t: Trace, cfg: Option<Config>) -> Self {
        let play = Playback::new(t);
        let mut w = play.world().cloned().unwrap_or_default();
        w.level = play.head.level.clone();
        w.biome = load_biome(&w.level.biome_id);
        w.hub_level = START_HUB_LEVEL;
        let level = w.level.biome_id.clone();
        let frame = Frame {
            seats: seats(&w, play.head.spieler),
            stages: vec![Stage {
                level: level.clone(),
                ..Stage::default()
            }],
            ..Frame::default()
        };
        let mut a = Self {
            play,
            blend: Blend::new(0),
            frame,
            cells: Vec::new(),
            cams: Cameras::default(),
            scene: None,
            cfg,
            level,
            fresh: true,
            win: window(LOGICAL),
            last_rects: 0,
            frames: 0,
        };
        a.install(w);
        a
    }

    pub fn playback(&self) -> &Playback {
        &self.play
    }

    /// Zustand des aktuellen Takts.
    pub fn world(&self) -> Option<&World> {
        self.frame.stages.first()?.world.as_deref()
    }

    /// Die Zeichner (nach dem ersten Bild).
    pub fn scene(&self) -> Option<&Scene> {
        self.scene.as_ref()
    }

    /// Frame und Zellen des aktuellen Takts.
    pub fn frame(&self) -> (&Frame, &[Cell]) {
        (&self.frame, &self.cells)
    }

    /// Legt `w` als Zustand des aktuellen Takts in den Frame, mit seinen Ereignissen.
    fn install(&mut self, w: World) {
        self.frame.tick = self.play.pos() as u64;
        self.frame.clock = w.time;
        self.frame.events.clear();
        let tick = self.frame.tick;
        let evs = self.play.events().iter().map(|e| Event {
            stage: 0,
            tick,
            data: e.clone(),
        });
        self.frame.events.extend(evs);
        self.frame.stages[0].world = Some(Rc::new(w));
    }

    /// Nächster Zustand als Kopie (im Takt, nicht im Bild; die Aufzeichnung bleibt unverändert): übernimmt Level, Biom
    /// und Hub-Stufe (nicht in der Aufzeichnung) vom Takt davor.
    fn next_world(&mut self) -> World {
        let prev = self.frame.stages[0].world.take();
        let mut prev = prev
            .map(|r| Rc::try_unwrap(r).unwrap_or_else(|r| (*r).clone()))
            .unwrap_or_default();
        let mut w = self.play.world().cloned().unwrap_or_default();
        w.level = std::mem::take(&mut prev.level);
        w.biome = std::mem::take(&mut prev.biome);
        w.hub_level = prev.hub_level;
        w
    }

    /// Update wie `game.go` nach dem Takt: Zellen, Kameras, Listener.
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

    fn cam_xs(&self) -> [f64; MAX_CAMS] {
        std::array::from_fn(|i| self.cells.get(i).map_or(0.0, |c| c.camera.x))
    }

    fn set_cam_xs(&mut self, xs: &[f64; MAX_CAMS]) {
        for (c, &x) in self.cells.iter_mut().zip(xs) {
            c.camera.x = x;
        }
        self.frame.cells.clone_from(&self.cells);
    }
}

impl App for ReplayApp {
    // PERF(B-1077): rund 780 Allokationen je Takt, gemessen je Teil (Mittel): `next_world` 316 (`World::clone`: Strings und
    // Vecs der Welt allokieren einzeln, `clone_from` gibt es in k3c-sim nicht; die Aufzeichnung muss unverändert bleiben,
    // `playback().world()` liest sie), `update` 155 (Layout, Kameras, Listener von hud und props), `install` 9 (Ereignisse).
    // Abbau braucht `clone_from` oder geteilte Teile der Welt in k3c-sim, nicht in dieser Datei.
    fn step(&mut self, _input: &InputState, _dt: f64) {
        let cams = self.cam_xs();
        let n = self.cells.len().min(MAX_CAMS);
        if let Some(w) = self.frame.stages[0].world.as_deref() {
            self.blend.capture(w, &cams[..n]);
        }
        if !self.play.advance() {
            return;
        }
        let w = self.next_world();
        self.install(w);
        self.update();
    }

    // PERF(B-1078): bis zu 29 Allokationen je Bild in den Zeichnern
    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.frames += 1;
        self.last_rects = 0;
        if self.scene.is_none() {
            self.win = window(c.size());
            let w = self.world().cloned().unwrap_or_default();
            self.scene = Some(Scene::build(self.cfg.as_ref(), &self.level, &w, c));
            for n in &self.scene.as_ref().map(|s| s.notes.clone()).unwrap_or_default() {
                eprintln!("⚠ replay: {n}");
            }
        }
        if self.fresh {
            self.update();
        }
        let mut cams = self.cam_xs();
        let n = self.cells.len().min(MAX_CAMS);
        // Nur der Frame hält die Welt; hielte ein Modul eine zweite `Rc`, zeichnet das Bild ohne Zwischenbild (Meldung)
        if let Some(w) = self.frame.stages[0].world.as_mut().and_then(Rc::get_mut) {
            self.blend.apply(w, &mut cams[..n], alpha);
            self.frame.clock = w.time;
        } else {
            debug_assert!(false, "Welt im Frame geteilt: kein Zwischenbild");
            eprintln!("⚠ replay: Welt im Frame geteilt, Bild ohne Zwischenbild");
        }
        self.set_cam_xs(&cams);
        if let Some(s) = &self.scene {
            self.last_rects = s.draw(c, &self.frame, &self.cells);
        }
        if let Some(w) = self.frame.stages[0].world.as_mut().and_then(Rc::get_mut) {
            self.blend.restore(w, &mut cams[..n]);
            self.frame.clock = w.time;
        }
        self.set_cam_xs(&cams);
    }

    fn done(&self) -> bool {
        self.play.finished()
    }
}
