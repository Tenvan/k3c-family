//! Port von `Game.Draw` und `drawGame` aus `native/k3c-game/game/game.go`: ein Bild.

use std::mem::take;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use k3c_platform::{Canvas, Rect};

use super::{CLEAR_COLOR, Game, layout};
use crate::view::cell::{rect, window_cell};

impl Game {
    /// Zeichnet den Fehler, die Schiffsfahrt bzw. die Ladeanzeige, sonst das Spiel (`draw_game`).
    pub(super) fn draw_frame(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.run_pending_load(c);
        if let Some(f) = self.fault.as_ref() {
            f.draw(c);
        } else if self.voy.on {
            self.draw_voyage(c, alpha);
        } else if self.loading {
            self.draw_loading(c);
        } else {
            self.draw_game(c, alpha);
            self.dev_shot(alpha);
            return;
        }
        if let Some(sh) = self.shots.as_mut() {
            sh.capture(c);
        }
    }

    /// Zeichnet je Zelle die Zellen-Ebenen, dann das Overlay, den obersten Screen und zuletzt die Anzeige oben. Die
    /// Zwischenbilder (`Blend`) schieben den Frame für das Zeichnen; `restore` läuft auch nach einem Panic eines
    /// Drawers (Go: `defer`), danach geht der Panic weiter.
    pub(super) fn draw_game(&mut self, c: &mut dyn Canvas, alpha: f64) {
        let t0 = self.meter.is_some().then(super::clock::clock);
        let s0 = self.stats.timing(&self.frame).then(|| (self.stats.now)());
        let (w, h) = c.size();
        c.rect(Rect::new(0.0, 0.0, w, h), CLEAR_COLOR);
        #[allow(clippy::cast_possible_truncation)]
        {
            self.win = rect(0, 0, w as i64, h as i64);
        }
        layout::place(&mut self.frame.cells, self.win);
        let mut cells = take(&mut self.frame.cells);
        self.blend.apply(&mut self.frame, &mut cells, alpha);
        self.frame.cells = cells;
        let drawn = catch_unwind(AssertUnwindSafe(|| self.draw_layers(c, s0)));
        let mut cells = take(&mut self.frame.cells);
        self.blend.restore(&mut self.frame, &mut cells);
        self.frame.cells = cells;
        if let Err(p) = drawn {
            resume_unwind(p);
        }
        if let (Some(t0), Some(mut m)) = (t0, self.meter.take()) {
            // Der Meter liegt außerhalb von `self`, solange der Bericht den Stand des Spiels liest (Go: `g.report(m)`)
            let now = super::clock::clock();
            if m.frame(now, now - t0, alpha, self.frame.tick) {
                let r = self.report(&m);
                m.finish(r);
            }
            self.meter = Some(m);
        }
    }

    /// Zellen-Ebenen je Zelle (beschnitten), Overlay-Ebene, oberster Screen, Leistungsanzeige, Aufnahme.
    fn draw_layers(&mut self, c: &mut dyn Canvas, s0: Option<f64>) {
        for i in 0..self.frame.cells.len() {
            let cell = self.frame.cells[i];
            c.clip(Some(cell.rect.to_rect()));
            for d in &self.parts.drawers {
                if d.layer().per_cell() {
                    d.draw(c, &self.frame, &cell);
                }
            }
        }
        c.clip(None);
        let wc = window_cell(self.win);
        for d in &self.parts.drawers {
            if d.layer() == crate::view::layer::Layer::OVERLAY {
                d.draw(c, &self.frame, &wc);
            }
        }
        if let Some(top) = self.stack.top() {
            top.draw(c, &self.frame, &wc);
        }
        if let Some(s0) = s0 {
            let dt = (self.stats.now)() - s0;
            self.stats.drw.add(dt);
        }
        self.measure_fps();
        self.stats.draw(c, &self.frame, self.fps, None);
        if let Some(sh) = self.shots.as_mut() {
            sh.capture(c);
        }
    }

    /// Bildrate für die Leistungsanzeige aus der Wanduhr (Go: `ebiten.ActualFPS`); nur mit Leistungsanzeige.
    fn measure_fps(&mut self) {
        if !self.stats.timing(&self.frame) {
            return;
        }
        let now = (self.stats.now)();
        let d = now - self.last_draw;
        if self.last_draw > 0.0 && d > 0.0 {
            self.fps = if self.fps > 0.0 {
                0.9 * self.fps + 0.1 / d
            } else {
                1.0 / d
            };
        }
        self.last_draw = now;
    }
}
