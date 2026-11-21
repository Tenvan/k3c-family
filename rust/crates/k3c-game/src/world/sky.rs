//! LA1.5 (B-1164): Himmel nach dem Abgleich mit K2C (`data/look.json` › sky, Standard Bildaufbau §§ 5 und 6).
//!
//! Über der flachen Himmelsfarbe liegen ein Verlauf (oben tiefer, zum Horizont heller), ein warmes Abendrot am
//! Horizont in Dämmerung und Morgengrauen und nachts Sterne. Alles ist Dekoration der Zelle: feste Lage aus einem
//! Zahlenstrom ohne Zufall und ohne Zustand, je Bild entsteht nichts Neues. Biome ohne Himmel bekommen nichts.

use k3c_data::look::current;
use k3c_data::look_sky::Sky;
use k3c_platform::Canvas;
use k3c_sim::state::{CycleInfo, World as SimWorld};

use super::World;
use super::backdrop::Rgba;
use super::light::daylight;
use super::tint::has_sky;
use crate::view::cell::Cell;

/// BANDS ist die Zahl der waagerechten Streifen für Verlauf und Abendrot.
const BANDS: usize = 24;
/// Anteil der Himmelshöhe (von oben), in dem Sterne stehen.
const STAR_AREA: f64 = 0.4;

/// Glow_weight ist das Dämmerlicht am Horizont: 0 am Tag und in tiefer Nacht, 1 in der Mitte von Dämmerung und
/// Morgengrauen.
pub fn dusk_weight(c: &CycleInfo) -> f64 {
    match c.phase.as_str() {
        "dusk" | "dawn" => 1.0 - (c.progress.clamp(0.0, 1.0) * 2.0 - 1.0).abs(),
        _ => 0.0,
    }
}

/// Rgba_of macht aus einer RGB-Angabe 0 … 255 die Byte-Farbe.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn rgba_of(v: [f64; 3]) -> Rgba {
    let c = |i: usize| v[i].clamp(0.0, 255.0) as u8;
    [c(0), c(1), c(2), 255]
}

/// Star_at liefert Lage (0 … 1, 0 … 1) und Helligkeit (0,5 … 1) des Sterns `i` aus einem festen Zahlenstrom.
#[allow(clippy::cast_precision_loss)]
pub fn star_at(i: usize) -> (f64, f64, f64) {
    let h = |k: u64| {
        let mut x = (i as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ k.wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x ^= x >> 30;
        x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^= x >> 27;
        (x % 10_000) as f64 / 10_000.0
    };
    (h(1), h(2) * STAR_AREA, 0.5 + 0.5 * h(3))
}

impl World {
    /// DrawSky legt Verlauf, Abendrot und Sterne über den Himmel der Zelle (nach der Himmelsfarbe, vor der Schicht
    /// `fern`); `back` ist die Nacht-Helligkeit des Hintergrunds.
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_sky(&self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, level: &str, back: f64) {
        let s: Sky = current().sky;
        if !s.enabled || !has_sky(level) || !super::fx::on("sky") {
            return;
        }
        let r = c.world_rect();
        let (x, y, w, h) = (r.min.x as f64, r.min.y as f64, r.dx() as f64, r.dy() as f64);
        let band = h / BANDS as f64;
        let dusk = dusk_weight(&wd.cycle) * s.glow_horizon;
        let top = rgba_of(s.top);
        let warm = rgba_of(s.horizon);
        for i in 0..BANDS {
            let t = i as f64 / (BANDS - 1) as f64; // 0 oben, 1 am Horizont
            let by = (y + band * i as f64).round();
            let bh = (y + band * (i + 1) as f64).round() - by;
            let a = s.gradient * (1.0 - t).powf(1.4);
            self.fill(dst, x, by, w, bh, top, back, a);
            let g = dusk * t.powi(3);
            self.fill(dst, x, by, w, bh, warm, back.max(0.5), g);
        }
    }

    /// DrawStars zeichnet die Sterne nach der Lichtkarte (sonst würde sie die Nacht verschlucken): nur nachts, kleine
    /// helle Punkte mit fester Lage im oberen Himmel der Zelle.
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_stars(&self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, level: &str) {
        let s: Sky = current().sky;
        if !s.enabled || !has_sky(level) || !super::fx::on("stars") {
            return;
        }
        let r = c.world_rect();
        let (x, y, w, h) = (r.min.x as f64, r.min.y as f64, r.dx() as f64, r.dy() as f64);
        let night = 1.0 - daylight(&wd.cycle);
        let a = s.star_alpha * night * night;
        if a <= 0.0 {
            return;
        }
        let size = (h / 300.0).round().max(1.0);
        for i in 0..s.stars {
            let (fx, fy, b) = star_at(i);
            self.fill(
                dst,
                x + fx * w,
                y + fy * h,
                size,
                size,
                [235, 240, 255, 255],
                1.0,
                a * b,
            );
        }
    }
}

#[cfg(test)]
#[path = "sky_tests.rs"]
mod sky_tests;
