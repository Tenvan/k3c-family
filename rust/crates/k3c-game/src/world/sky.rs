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

/// CLOUD_AREA ist der Anteil der Himmelshöhe (von oben), in dem Wolken ziehen.
const CLOUD_AREA: f64 = 0.45;

/// Unit liefert aus Wolken- oder Sternnummer `i` und Salz `k` eine feste Zahl in 0 … 1 (ohne Zufall).
#[allow(clippy::cast_precision_loss)]
fn unit(i: usize, k: u64) -> f64 {
    let mut x = (i as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ k.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 30;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 27;
    (x % 10_000) as f64 / 10_000.0
}

/// Cloud_boxes liefert die drei gestuften Ballen der Wolke `i` zur Zeit `clock` in einer Himmelsfläche `w × h` (Lage
/// relativ zur Fläche, waagerecht auf sie begrenzt, damit nichts in die Nachbarzelle ragt): breite Basis, mittlerer
/// und oberer Ballen. Die Wolke zieht mit `speed` Flächenbreiten je Sekunde nach rechts, größere Wolken langsamer.
pub fn cloud_boxes(i: usize, clock: f64, w: f64, h: f64, speed: f64) -> Vec<[f64; 4]> {
    let size = 0.6 + 0.8 * unit(i, 4);
    let (cw, ch) = (w * 0.16 * size, w * 0.16 * size * 0.28);
    let y = unit(i, 5) * CLOUD_AREA * h;
    let pos = (unit(i, 6) + clock * speed / size).rem_euclid(1.0);
    let x = pos * (w + 2.0 * cw) - cw;
    [
        [x, y + ch * 0.5, cw, ch * 0.5],
        [x + cw * 0.15, y + ch * 0.15, cw * 0.6, ch * 0.5],
        [x + cw * 0.35, y, cw * 0.3, ch * 0.45],
    ]
    .into_iter()
    .filter_map(|[bx, by, bw, bh]| {
        let (x0, x1) = (bx.max(0.0), (bx + bw).min(w));
        (x1 > x0).then_some([x0, by, x1 - x0, bh])
    })
    .collect()
}

impl World {
    /// DrawSky legt Verlauf, Abendrot, Wolken und Sterne über den Himmel der Zelle (nach der Himmelsfarbe, vor der
    /// Schicht `fern`); `back` ist die Nacht-Helligkeit des Hintergrunds, `clock` die Spielzeit der Wolken.
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_sky(&self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, level: &str, back: f64, clock: f64) {
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
        self.draw_clouds(dst, (x, y, w, h), wd, (&s, back, clock));
    }

    /// DrawClouds lässt die Wolken ziehen: Weiß, am Abend ins Abendrot getönt, nachts auf `cloudNight` der Deckkraft
    /// und mit dem dunklen Hintergrund abgedunkelt.
    #[allow(clippy::cast_precision_loss)]
    fn draw_clouds(&self, dst: &mut dyn Canvas, area: (f64, f64, f64, f64), wd: &SimWorld, p: (&Sky, f64, f64)) {
        let (s, back, clock) = p;
        let (x, y, w, h) = area;
        let day = daylight(&wd.cycle);
        let a = s.cloud_alpha * (s.cloud_night + (1.0 - s.cloud_night) * day);
        if s.clouds == 0 || a <= 0.0 {
            return;
        }
        let k = (dusk_weight(&wd.cycle) * 0.6).clamp(0.0, 1.0);
        let (white, warm) = ([255.0; 3], s.horizon);
        let tint = rgba_of([0, 1, 2].map(|i| white[i] + (warm[i] - white[i]) * k));
        for i in 0..s.clouds {
            for [bx, by, bw, bh] in cloud_boxes(i, clock, w, h, s.cloud_speed) {
                self.fill(dst, x + bx, y + by, bw, bh, tint, back, a);
            }
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
