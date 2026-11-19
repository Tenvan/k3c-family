//! Port von `native/k3c-game/world/river.go`.
//!
//! Fluss im unteren Drittel jeder Zelle (B-795/AC-02, B-789; aus native/k3c-ebiten/river.go übernommen): unter dem
//! Uferstreifen sechs Wellen-Reihen in einer Farbe. Der Fluss ist eine Ebene, die auf den Betrachter zuläuft: Jede
//! Reihe weiter vorn ist um denselben Faktor höher, ihre Wellen sind breiter, sie fließt schneller nach links, und
//! eine Kamerafahrt verschiebt sie mit demselben Faktor (die Reihe am Ufer wie die Welt). Die Wellen-Kachel entsteht
//! beim ersten Zeichnen im Code, ein Bild für alle Reihen.
//!
//! Abweichung: Der Shader steht in der Plattform (WGSL, `Canvas::river`, siehe `water.rs`); `draw_river` fällt auf die
//! Kacheln zurück, wenn der Canvas ihn nicht kennt.

use k3c_platform::{Canvas, Rect, TexId};

use super::World;
use super::backdrop::Rgba;
use super::ground::bank_height;
use super::layers::stage_of;
use super::water::{RiverMode, river_mode_for};
use crate::view::cell::Cell;
use crate::view::frame::Frame;

pub const RIVER_ROWS: usize = 6;
/// RIVER_GROW ist der Zuwachs je Reihe nach vorn (Maßstab der Reihe i = 1 + i·riverGrow).
pub const RIVER_GROW: f64 = 0.3;
/// RIVER_TILE_UNITS ist die Breite einer Wellen-Kachel der Reihe am Ufer in Units (150 px wie NV3).
pub const RIVER_TILE_UNITS: f64 = 150.0 / 32.0;
/// RIVER_FLOW_UNITS ist das Tempo der Reihe am Ufer nach links in Units je Sekunde (28 px/s wie NV3).
pub const RIVER_FLOW_UNITS: f64 = 28.0 / 32.0;
/// Größe der Wellen-Kachel im Quell-Bild (Bildpunkte).
pub const RIVER_SRC_W: usize = 256;
pub const RIVER_SRC_H: usize = 64;

pub const RIVER_BASE: Rgba = [58, 118, 168, 255];
pub const RIVER_WAVE: Rgba = [84, 142, 188, 255];

/// RiverRow ist die Lage einer Wellen-Reihe in Fenster-px: oben y, Höhe h, Kachelbreite tile, erste Kachel bei x0
/// (≤ linker Rand der Zelle).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RiverRow {
    pub y: f64,
    pub h: f64,
    pub tile: f64,
    pub x0: f64,
}

/// riverTotal ist die Summe der Maßstäbe aller Reihen.
#[allow(clippy::cast_precision_loss)]
pub const RIVER_TOTAL: f64 = {
    let mut t = 0.0;
    let mut i = 0;
    while i < RIVER_ROWS {
        t += 1.0 + (i as f64) * RIVER_GROW;
        i += 1;
    }
    t
};

/// RiverLayout teilt den Fluss der Zelle (unter dem Uferstreifen bis zum unteren Rand) in die sechs Reihen und legt
/// ihre Kacheln zur Zeit t (Sekunden) fest. Die Kachel-Lage hängt an der Welt: Welt-x (t·Tempo + Kamera-x) liegt in
/// jeder Reihe auf einer Kachelkante, die Reihe i bewegt sich mit Maßstab 1 + i·riverGrow.
#[allow(clippy::cast_precision_loss)]
pub fn river_layout(c: &Cell, t: f64) -> [RiverRow; RIVER_ROWS] {
    let mut rows = [RiverRow::default(); RIVER_ROWS];
    let k = &c.camera;
    let top = c.ground_y() as f64 + bank_height(c);
    let height = c.rect.max.y as f64 - top;
    let mut y = top;
    for (i, row) in rows.iter_mut().enumerate() {
        let g = 1.0 + (i as f64) * RIVER_GROW;
        let h = height * g / RIVER_TOTAL;
        let tile = RIVER_TILE_UNITS * g * k.scale;
        let mut x0 = c.rect.min.x as f64;
        if tile > 0.0 {
            // Kachelkante liegt an der Zellenmitte minus Weg; nach links fließend, Kamerafahrt nach rechts schiebt mit.
            let mut off = (k.center - c.rect.min.x as f64 - (t * RIVER_FLOW_UNITS + k.x) * g * k.scale) % tile;
            if off > 0.0 {
                off -= tile;
            }
            x0 += off;
        }
        *row = RiverRow { y, h, tile, x0 };
        y += h;
    }
    rows
}

/// RiverTile ist das Quell-Bild der Wellen (lazy).
#[derive(Clone, Copy, Debug, Default)]
pub struct RiverTile {
    tex: Option<TexId>,
}

impl RiverTile {
    /// Image liefert das Wellen-Bild, beim ersten Aufruf gemalt und hochgeladen.
    #[allow(clippy::cast_possible_truncation)]
    pub fn image(&mut self, dst: &mut dyn Canvas) -> TexId {
        if let Some(t) = self.tex {
            return t;
        }
        let mut src = vec![0u8; RIVER_SRC_W * RIVER_SRC_H * 4];
        paint_waves(&mut src);
        let t = dst.texture(&src, RIVER_SRC_W as u32, RIVER_SRC_H as u32);
        self.tex = Some(t);
        t
    }
}

/// PaintWaves malt eine Wellen-Reihe: ein gewelltes Band auf der Grundfarbe, in x periodisch (ganze Wellenzahl) und
/// oben wie unten Grundfarbe, damit die Kachel in beide Richtungen nahtlos ist. `dst` ist RGBA, Zeile für Zeile.
#[allow(clippy::cast_precision_loss)]
pub fn paint_waves(dst: &mut [u8]) {
    for y in 0..RIVER_SRC_H {
        for x in 0..RIVER_SRC_W {
            let mut c = RIVER_BASE;
            // Go faltet 2·π·2 als Konstante: 4·π ist als Gleitkommazahl exakt gleich (Faktor 4 ist exakt).
            let wave = (4.0 * std::f64::consts::PI * x as f64 / RIVER_SRC_W as f64).sin();
            let line = RIVER_SRC_H as f64 * 0.5 + 7.0 * wave;
            let d = (y as f64 - line).abs();
            if d < 4.0 {
                c = mix(c, RIVER_WAVE, 1.0 - d / 4.0);
            }
            let i = 4 * (y * RIVER_SRC_W + x);
            dst[i..i + 4].copy_from_slice(&c);
        }
    }
}

/// Mix mischt a nach b um den Anteil t (Kanäle abgeschnitten wie `uint8(…)` in Go), Alpha 255.
pub fn mix(a: Rgba, b: Rgba, t: f64) -> Rgba {
    let f = |x: u8, y: u8| (f64::from(x) * (1.0 - t) + f64::from(y) * t) as u8;
    [f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2]), 255]
}

impl World {
    /// DrawRiver zeichnet die Wellen-Reihen der Zelle zur Zeit Frame.Clock: mit dem Shader, sonst als Kacheln. Die
    /// Fassung folgt `Frame::settings.river` (F3), solange nichts gewählt ist dem Startwert aus K3C_RIVER; kennt der
    /// Canvas den Shader nicht, kommen die Kacheln.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn draw_river(&mut self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(st) = stage_of(f, c) else {
            return;
        };
        // k2c: Der Spiegel (mirror.rs, nach dem Licht) füllt den Fluss, die Wellen entfallen.
        if st.world.is_some() && self.mirror_wanted(f, dst) {
            return;
        }
        if river_mode_for(&f.settings.river, self.start) == RiverMode::Shader
            && self.water.ready()
            && self.water.draw(dst, c, f.clock)
        {
            return;
        }
        let img = self.river.image(dst);
        let right = c.rect.max.x as f64;
        let src = Rect::new(0.0, 0.0, RIVER_SRC_W as f32, RIVER_SRC_H as f32);
        for row in river_layout(c, f.clock) {
            if row.tile <= 0.0 || row.h <= 0.0 {
                continue;
            }
            let mut x = row.x0;
            while x < right {
                // 1 px Überlappung gegen Nähte
                let r = Rect::new(x as f32, row.y as f32, (row.tile + 1.0) as f32, (row.h + 1.0) as f32);
                dst.sprite(img, src, r, [1.0; 4]);
                x += row.tile;
            }
        }
    }
}

#[cfg(test)]
#[path = "river_tests.rs"]
mod river_tests;
