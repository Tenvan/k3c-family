//! Port von `native/k3c-game/game/devshot.go`: Screenshot der Entwickler-Taste F12 (B-888): das nächste Bild als PNG
//! nach `reports/` (Arbeitsordner, wie `task native:game`), Name nach Datum und Uhrzeit. Ein Fehler steht im Log, das
//! Spiel läuft weiter.
//!
//! Abweichung von Go (mit Default entschieden): Go liest die Pixel des Bildschirms; die Plattform kann in Rust kein
//! Bild auslesen. F12 zeichnet das Bild darum noch einmal gegen den Software-Rasterer (`ShotCanvas`) in 1920 × 1080 und
//! schreibt es. Das Bild kann sich vom Fensterbild unterscheiden: Der Rasterer kennt die Texturen der Fensterfläche
//! nicht, Sprites fehlen, Flächen, Linien und Text stehen darin.

use std::path::{Path, PathBuf};

use k3c_platform::log;

use super::shots_canvas::ShotCanvas;
use super::{Game, layout};
use crate::devtools::report::Zeit;
use crate::still::Image;
use crate::still::pngio::write_png;
use crate::view::roles::REPORTS_DIR;

/// Logische Größe des Bildes.
const SHOT_W: u32 = 1920;
const SHOT_H: u32 = 1080;

impl Game {
    /// Schreibt das Bild, wenn F12 es verlangt hat.
    pub(super) fn dev_shot(&mut self, alpha: f64) {
        if !self.shot_next {
            return;
        }
        self.shot_next = false;
        let win = self.win;
        let mut canvas = ShotCanvas::new(SHOT_W, SHOT_H);
        self.draw_game(&mut canvas, alpha);
        let img = canvas.next_image();
        self.win = win; // das Fenster zeichnet das nächste Bild wieder mit seiner Größe
        layout::place(&mut self.frame.cells, win);
        match save_shot(Path::new(REPORTS_DIR), &Zeit::now(), &img) {
            Ok(path) => log::info("📄 Screenshot geschrieben", &[("datei", &path.display().to_string())]),
            Err(e) => log::warn("💥 Screenshot nicht geschrieben", &[("err", &e)]),
        }
    }
}

/// Schreibt `img` als `shot-<Datum>-<Zeit>.png` in `dir` (legt `dir` an) und liefert den Pfad.
pub fn save_shot(dir: &Path, now: &Zeit, img: &Image) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let name = format!(
        "shot-{:04}{:02}{:02}-{:02}{:02}{:02}.{:03}.png",
        now.year,
        now.month,
        now.day,
        now.hour,
        now.minute,
        now.second,
        now.nanos / 1_000_000
    );
    let path = dir.join(name);
    write_png(&path, img)?;
    Ok(path)
}

#[cfg(test)]
#[path = "devshot_tests.rs"]
mod devshot_tests;
