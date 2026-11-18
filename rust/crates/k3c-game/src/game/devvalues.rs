//! Port von `native/k3c-game/game/devvalues.go`: Messwerte und Auswahl für die Seite Werte des Debug-Overlays (F10,
//! B-889): Takte je Wandsekunde (gemessen nur, solange die Seite offen ist) und die per Maus gewählte Stelle in der
//! Welt einer Zelle. Nicht im Aufnahme-Modus.
//!
//! Abweichung von Go: `InputState` der Plattform kennt keine Maus (Zeiger und Klick fehlen). `Game::dev_values` setzt
//! darum keine Auswahl (`dev.pick` bleibt leer); die Seite Werte nutzt die Stelle des Monarchen (`devtools::pick_x`).
//! `pick_at` steht bereit, sobald die Plattform den Zeiger liefert.

use super::Game;
use super::clock::clock;
use crate::view::cell::{Cell, Point};
use crate::view::devkeys::{DEBUG_VALUES, Pick};

/// Zählt die Takte der Simulation je Wandsekunde.
#[derive(Clone, Copy, Debug, Default)]
pub struct TpsMeter {
    /// Beginn des Messfensters (`clock`)
    since: f64,
    /// `Frame::tick` zu Beginn
    tick: u64,
}

impl TpsMeter {
    /// Misst mit der Uhrzeit `now` und dem Takt `tick`; nach mindestens einer Sekunde liefert es den neuen Wert.
    #[allow(clippy::cast_precision_loss)]
    pub fn update(&mut self, now: f64, tick: u64) -> Option<f64> {
        if self.since == 0.0 || tick < self.tick {
            self.since = now;
            self.tick = tick;
            return None;
        }
        let dt = now - self.since;
        if dt >= 1.0 {
            let tps = (tick - self.tick) as f64 / dt;
            self.since = now;
            self.tick = tick;
            return Some(tps);
        }
        None
    }
}

/// Liefert die Stelle unter dem Punkt `pt` (Fenster-Pixel) in der Zelle, die ihn enthält.
#[allow(clippy::cast_precision_loss)]
pub fn pick_at(cells: &[Cell], pt: Point) -> Option<Pick> {
    cells
        .iter()
        .find(|c| {
            pt.x >= c.rect.min.x && pt.x < c.rect.max.x && pt.y >= c.rect.min.y && pt.y < c.rect.max.y && c.player >= 0
        })
        .map(|c| Pick {
            picked: true,
            player: c.player,
            x: c.camera.world_x(pt.x as f64),
        })
}

impl Game {
    /// Aktualisiert die Takte je Sekunde, solange die Seite Werte offen ist.
    pub(super) fn dev_values(&mut self) {
        if self.dev.debug != DEBUG_VALUES || !self.args.shots.is_empty() {
            self.tps = TpsMeter::default();
            return;
        }
        if let Some(tps) = self.tps.update(clock(), self.frame.tick) {
            self.dev.tps = tps;
        }
    }
}

#[cfg(test)]
#[path = "devvalues_tests.rs"]
mod devvalues_tests;
