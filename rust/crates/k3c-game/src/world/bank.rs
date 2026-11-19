//! Ufer im Look-Profil `k2c` (RU55.1, `docs/standards/bildaufbau.md` § 1): ein dunkles Band mit Charakter unter der
//! Gehlinie, das Welt und Fluss trennt. Erde mit Rissen und Wurzeln in einem dunklen Ton der Boden-Farbe des Bioms,
//! darüber Grasbüschel an der Kante. Alles hängt an der Welt (Boden-Ebene, Faktor 1) und folgt der Kamera; Lage und Form
//! der Einzelteile folgen aus dem Index, nie aus Zufall oder Uhr. Der Aufruf legt nichts an (Formen gehen direkt an den
//! Canvas).

use k3c_platform::Canvas;

use super::World;
use super::backdrop::Rgba;
use crate::view::cell::Cell;

/// Abstand der Stellen (Risse, Wurzeln, Büschel) in Units.
pub const SPOT_UNITS: f64 = 1.1;
/// Helligkeit der Erde gegen die Boden-Farbe des Bioms (dunkler Ton).
pub const EARTH_BRIGHTNESS: f64 = 0.55;

/// Mischt eine Ganzzahl und ein Salz zu einer Zahl 0 … 1 (stabil, ohne Zufallszahlen: nur für Optik).
pub fn unit_hash(i: i64, salt: u64) -> f64 {
    #[allow(clippy::cast_sign_loss)]
    let mut h = (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ salt.wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 29;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 32;
    #[allow(clippy::cast_precision_loss)]
    {
        (h & 0xFF_FFFF) as f64 / f64::from(0x100_0000_u32)
    }
}

/// Farbe eines Grasbüschels: die Boden-Farbe mit Grün-Stich, heller als die Erde.
pub fn tuft_color(ground: Rgba) -> [f32; 4] {
    let ch = |v: u8| f32::from(v) / 255.0;
    [
        ch(ground[0]) * 0.8,
        (ch(ground[1]) * 1.15 + 0.08).min(1.0),
        ch(ground[2]) * 0.7,
        1.0,
    ]
}

/// Dunkler Ton der Erde und der Risse (Faktor `k` auf die Boden-Farbe).
pub fn earth_color(ground: Rgba, k: f64) -> [f32; 4] {
    #[allow(clippy::cast_possible_truncation)]
    let ch = |v: u8| (f64::from(v) / 255.0 * k) as f32;
    [ch(ground[0]), ch(ground[1]), ch(ground[2]), 1.0]
}

/// Erste und letzte Stelle (Index) im sichtbaren Bereich der Zelle samt einem Rand von einer Stelle.
#[allow(clippy::cast_possible_truncation)]
pub fn spots_in_view(c: &Cell) -> (i64, i64) {
    let (l, r) = c.camera.span();
    ((l / SPOT_UNITS).floor() as i64 - 1, (r / SPOT_UNITS).ceil() as i64 + 1)
}

impl World {
    /// DrawBank malt Risse, Wurzeln und Grasbüschel auf den Uferstreifen (Höhe `h` ab der Gehlinie) der Zelle `c`;
    /// `ground` ist die Boden-Farbe des Bioms.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn draw_bank(&self, dst: &mut dyn Canvas, c: &Cell, ground: Rgba, h: f64) {
        if h < 2.0 {
            return;
        }
        let (k, top) = (&c.camera, c.ground_y() as f32);
        let (crack, root, tuft) = (earth_color(ground, 0.3), earth_color(ground, 0.8), tuft_color(ground));
        let (first, last) = spots_in_view(c);
        let hf = h as f32;
        let w = (0.07 * k.scale).max(1.0) as f32;
        for i in first..=last {
            let sx = k.screen_x((i as f64 + unit_hash(i, 1) * 0.8) * SPOT_UNITS) as f32;
            // Riss: schräg durch die Erde
            let len = (0.4 + 0.5 * unit_hash(i, 2)) as f32 * hf;
            let lean = (unit_hash(i, 3) - 0.5) as f32 * hf * 0.8;
            dst.stroke_line(sx, top, sx + lean, top + len, w, crack);
            // Wurzel: kurzer hellerer Strich von der Kante nach unten, jede zweite Stelle
            if unit_hash(i, 4) < 0.5 {
                let rx = sx + (0.5 * k.scale) as f32;
                dst.stroke_line(rx, top + 0.1 * hf, rx + (0.3 * k.scale) as f32, top + 0.6 * hf, w, root);
            }
            // Grasbüschel an der Kante: drei Halme, nach oben
            if unit_hash(i, 5) < 0.8 {
                let bh = ((0.3 + 0.3 * unit_hash(i, 6)) * k.scale) as f32;
                let bw = (0.09 * k.scale).max(1.0) as f32;
                for (j, lean) in [-1.0_f32, 0.0, 1.0].into_iter().enumerate() {
                    let bx = sx + (j as f32 - 1.0) * bw * 1.2;
                    let tip = bh * (1.0 - 0.25 * lean.abs());
                    let tri = [
                        [bx - bw / 2.0, top + 1.0],
                        [bx + bw / 2.0, top + 1.0],
                        [bx + lean * bw, top - tip],
                    ];
                    dst.fill_path(&tri, tuft);
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "bank_tests.rs"]
mod bank_tests;
