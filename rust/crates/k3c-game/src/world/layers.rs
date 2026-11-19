//! Port von `native/k3c-game/world/layers.go`.
//!
//! Zeichnen der Kulisse und des Bodens: dünn, Lage aus den reinen Funktionen (cull, backdrop, ground).

use k3c_platform::{Affine, Canvas};
use k3c_sim::state::World as SimWorld;

use super::World;
use super::backdrop::{FRAME_PER_UNIT, Look, Rgba, TILE_UNITS, variant};
use super::cull::tile_range;
use super::tint::haze_of;
use crate::view::cell::Cell;
use crate::view::frame::{Frame, Stage};

/// ANIM_STAND ist die Animation der Kulissen-Kacheln.
pub const ANIM_STAND: &str = "stand";

impl World {
    /// DrawFar füllt den Welt-Teil der Zelle mit dem Himmel des Bioms und zeichnet die feste Schicht `fern`; beide
    /// sind nachts dunkler als der Vordergrund (backgroundBrightness).
    pub fn draw_far(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(st) = stage_of(f, c) else {
            return;
        };
        let Some(wd) = st.world.as_deref() else {
            return;
        };
        let lk = self.looks.of(&st.level);
        let back = self.light.backdrop(&wd.cycle);
        self.fill_rect(dst, c.world_rect(), lk.sky, back);
        self.draw_layer(dst, c, wd, lk, 0, back);
        self.draw_haze(dst, c, &st.level, lk.sky, back, 0);
    }

    /// DrawHaze legt Dunst in Himmelsfarbe über die Welt der Zelle nach der Schicht `i` (Luftperspektive, RU55.4).
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_haze(&self, dst: &mut dyn Canvas, c: &Cell, level: &str, sky: Rgba, back: f64, i: usize) {
        let a = haze_of(level, i);
        if a > 0.0 {
            let r = c.world_rect();
            self.fill(
                dst,
                r.min.x as f64,
                r.min.y as f64,
                r.dx() as f64,
                r.dy() as f64,
                sky,
                back,
                a,
            );
        }
    }

    pub fn draw_mid(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        self.draw_scrolling(dst, f, c, 1);
    }

    pub fn draw_near(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        self.draw_scrolling(dst, f, c, 2);
    }

    /// DrawScrolling zeichnet die scrollende Schicht i (1 = mitte, 2 = nah).
    pub fn draw_scrolling(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell, i: usize) {
        let Some(st) = stage_of(f, c) else {
            return;
        };
        let Some(wd) = st.world.as_deref() else {
            return;
        };
        let (lk, back) = (self.looks.of(&st.level), self.light.backdrop(&wd.cycle));
        self.draw_layer(dst, c, wd, lk, i, back);
        self.draw_haze(dst, c, &st.level, lk.sky, back, i);
    }

    /// DrawLayer zeichnet die sichtbaren Kacheln der Schicht i, Fußlinie auf der Boden-Linie, getönt mit Helligkeit
    /// back.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn draw_layer(&self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, lk: &Look, i: usize, back: f64) {
        let names = &lk.assets[i];
        let Some(lib) = self.lib.as_ref() else {
            return;
        };
        if names.is_empty() {
            return;
        }
        let factor = lk.scroll[i];
        let (first, last) = tile_range(&c.camera, factor, TILE_UNITS);
        for k in first..=last {
            let v = variant(&wd.level.backdrops, k, names.len() as i64);
            let Some(sh) = lib.sheet(&names[(v - 1) as usize], ANIM_STAND) else {
                continue;
            };
            if sh.fw == 0 || sh.fh == 0 {
                continue;
            }
            let r = tile_rect(c, factor, k, sh.fw, sh.fh, sh.origin_x, sh.origin_y, sh.units_per_px);
            let Some(fr) = sh.frame(0) else {
                continue;
            };
            let mut g = Affine::IDENTITY;
            g.scale((r.w / sh.fw as f64) as f32, (r.h / sh.fh as f64) as f32);
            g.translate(r.x as f32, r.y as f32);
            let b = back as f32;
            dst.sprite_tf(fr.tex, fr.src, g, [b, b, b, 1.0]);
        }
    }
}

/// RectF ist ein Rechteck in Fenster-px.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// TILE_OVERLAP ist die Überlappung einer Kachel mit ihrem rechten Nachbarn in Bildpunkten (B-947): Rundung und
/// Filterung des Rands lassen so nie eine Lücke zwischen zwei Kacheln.
pub const TILE_OVERLAP: f64 = 1.0;

/// TileRect liefert die Lage der Kachel k einer Schicht mit Scroll-Faktor factor: links bei Welt-x k·TILE_UNITS auf
/// der Ebene, Fußpunkt (Anteil origin_x, origin_y des Frames) auf der Boden-Linie. Die Kanten liegen auf ganzen
/// Bildpunkten, die rechte reicht `TILE_OVERLAP` Bildpunkte unter den Nachbarn: keine Lücke bei jedem Maßstab.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
pub fn tile_rect(
    c: &Cell,
    factor: f64,
    k: i64,
    fw: i64,
    fh: i64,
    origin_x: f64,
    origin_y: f64,
    units_per_px: f64,
) -> RectF {
    let s = c.camera.scale * units_per_px / FRAME_PER_UNIT;
    let (w, h) = (fw as f64 * s, fh as f64 * s);
    let x0 = c.camera.parallax_x(k as f64 * TILE_UNITS, factor) - origin_x * w;
    let x1 = c.camera.parallax_x((k + 1) as f64 * TILE_UNITS, factor) - origin_x * w;
    let (left, mut right) = (x0.round(), (x0 + w).round());
    if (x1 - (x0 + w)).abs() < 1.0 {
        // Kachel so breit wie der Abstand: an die gerundete Nachbarkante legen
        right = x1.round();
    }
    RectF {
        x: left,
        y: c.ground_y() as f64 - origin_y * h,
        w: right - left + TILE_OVERLAP,
        h,
    }
}

/// StageOf liefert die Stufe der Zelle mit Welt, sonst `None`.
pub fn stage_of<'a>(f: &'a Frame, c: &Cell) -> Option<&'a Stage> {
    let seat = f.seat(c.player)?;
    let st = f.stage(seat.stage)?;
    st.world.as_ref()?;
    Some(st)
}

#[cfg(test)]
#[path = "layers_tests.rs"]
mod layers_tests;
