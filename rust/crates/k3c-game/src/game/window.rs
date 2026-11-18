//! Port von `native/k3c-game/game/window.go`: Anzeige-Modus (B-826): Fenster mit Rahmen, rahmenloses Fenster in Größe
//! des Monitors oder Vollbild. Die Optionen und F bzw. der rechte Stick ändern `Settings::window` über die Session;
//! `apply_window` merkt es im nächsten Schritt.
//!
//! Abweichung: Rust setzt nur Vollbild (`App::fullscreen`, die Plattform schaltet randloses Vollbild). Größe und Lage
//! des Fensters (`window_rect`) und den Rahmen setzt die Plattform noch nicht (Ticket B-1094, Fenster-API);
//! `window_rect` bleibt als reine Funktion für diese API erhalten.

use super::Game;
use crate::view::input::{Action, Input};
use crate::view::settings::{WINDOW_BORDERLESS, WINDOW_FRAMED, WINDOW_FULLSCREEN};
use k3c_platform::log;

/// Der Modus nach F bzw. rechtem Stick: Vollbild wird Fenster, alles andere Vollbild (Go: `toggledWindow`).
pub fn toggled_window(cur: &str) -> &'static str {
    if cur == WINDOW_FULLSCREEN {
        WINDOW_FRAMED
    } else {
        WINDOW_FULLSCREEN
    }
}

/// Lage und Größe `(x, y, breite, höhe)` des Fensters auf einem Monitor `mw` × `mh`: rahmenlos füllt den Monitor, das
/// Fenster hat die Startgröße `w` × `h` (höchstens 90 % des Monitors, damit die Titelleiste sichtbar bleibt) und steht
/// mittig (Go: `windowRect`).
pub fn window_rect(mode: &str, mw: i64, mh: i64, w: i64, h: i64) -> (i64, i64, i64, i64) {
    if mode == WINDOW_BORDERLESS {
        return (0, 0, mw, mh);
    }
    let (ww, wh) = (w.min(mw * 9 / 10), h.min(mh * 9 / 10));
    ((mw - ww) / 2, (mh - wh) / 2, ww, wh)
}

impl Game {
    /// Schaltet bei `Action::Fullscreen` eines Geräts (Menü und Spiel) zwischen Vollbild und Fenster um
    /// (`Session::set_settings`). Nicht im Aufnahme-Modus und nicht vor dem ersten Frame (dessen Einstellungen sind
    /// noch leer, sonst gingen Lautstärken und Sprache verloren).
    pub(super) fn toggle_window(&mut self, input: &Input) {
        if !self.args.shots.is_empty() || self.frame.settings.language.is_empty() {
            return;
        }
        if !input.has(Action::Fullscreen).1 {
            return;
        }
        let mut s = self.frame.settings.clone();
        s.window = toggled_window(&s.window).to_string();
        if let Err(e) = self.sim.set_settings(s) {
            log::warn("💥 Einstellungen nicht gespeichert", &[("err", &e)]);
        }
    }

    /// Merkt den Anzeige-Modus der Einstellungen, sobald er sich ändert; nicht im Aufnahme-Modus. Den Zustand selbst
    /// liefert `fullscreen_wish` an die Plattform.
    pub(super) fn apply_window(&mut self) {
        let w = &self.frame.settings.window;
        if self.shots.is_some() || !self.args.shots.is_empty() || *w == self.window {
            return;
        }
        let modus = if w.is_empty() { "fenster" } else { w.as_str() };
        log::info("🚦 Anzeige", &[("modus", &modus)]);
        self.window.clone_from(w);
    }

    /// Gewünschter Vollbild-Zustand für `App::fullscreen`: Vollbild und rahmenlos ergeben `Some(true)`, alles andere
    /// `Some(false)`; `None` im Aufnahme-Modus und vor dem ersten Frame.
    pub(super) fn fullscreen_wish(&self) -> Option<bool> {
        if !self.args.shots.is_empty() || self.frame.settings.language.is_empty() {
            return None;
        }
        let w = self.frame.settings.window.as_str();
        Some(w == WINDOW_FULLSCREEN || w == WINDOW_BORDERLESS)
    }
}
