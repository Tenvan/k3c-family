//! Die Wiedergabe als `App` der Plattform-Schicht: zwei Bildhälften (wie zwei Spieler), 30-Hz-Takt, Zwischenbilder.

use k3c_platform::{App, Canvas, InputState, Rect};

use super::blend::Blend;
use super::pending::{CELL, camera_x, draw_cell};
use super::player::Playback;
use super::trace::Trace;

/// Zahl der Bildhälften.
pub const CELLS: usize = 2;

pub struct ReplayApp {
    play: Playback,
    blend: Blend,
    /// Zeichenbefehle (Rechtecke) des letzten Bilds.
    pub last_rects: u32,
    /// Gezeichnete Bilder.
    pub frames: u64,
}

impl ReplayApp {
    pub fn new(t: Trace) -> Self {
        let mut a = Self {
            play: Playback::new(t),
            blend: Blend::new(0),
            last_rects: 0,
            frames: 0,
        };
        a.capture();
        a
    }

    pub fn playback(&self) -> &Playback {
        &self.play
    }

    fn cams(&self) -> [f64; CELLS] {
        let w = self.play.world();
        std::array::from_fn(|i| w.map_or(0.0, |w| camera_x(w, i as i64)))
    }

    /// Merkt die Lage des aktuellen Takts als „Takt davor“ für den nächsten.
    fn capture(&mut self) {
        let cams = self.cams();
        if let Some(w) = self.play.world() {
            self.blend.capture(w, &cams);
        }
    }
}

impl App for ReplayApp {
    fn step(&mut self, _input: &InputState, _dt: f64) {
        self.capture();
        self.play.advance();
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.frames += 1;
        self.last_rects = 0;
        let mut cams = self.cams();
        let Some(w) = self.play.world_mut() else { return };
        self.blend.apply(w, &mut cams, alpha);
        for (i, &cam) in cams.iter().enumerate() {
            let left = CELL.0 * i as f32;
            c.clip(Some(Rect::new(left, 0.0, CELL.0, CELL.1)));
            self.last_rects += draw_cell(c, w, cam, left);
        }
        c.clip(None);
        self.blend.restore(w, &mut cams);
    }

    fn done(&self) -> bool {
        self.play.finished()
    }
}
