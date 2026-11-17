//! Port von `native/k3c-game/fx/paintmarks.go`.
//!
//! Zeichnen der Status-Markierungen (statusLooks.ts › drawMark): Bodenflächen (Aura, Sturm) zuerst, Symbole danach;
//! Drehung, Puls und Blitze allein aus der Zeit.

use std::f64::consts::PI;

use k3c_platform::{Affine, Canvas};

use super::effects::UNIT_PX;
use super::paint::{Stencils, at, rgba, tint};
use super::status::{Mark, SHIELD_Y, STORM_COLOR, STORM_FILL, StatusKind};
use crate::view::cell::Cell;

// Takte der Status-Looks in Sekunden.
pub const STUN_SPIN: f64 = 0.9;
pub const STORM_FLASH: f64 = 0.22;
pub const LAST_STAND_PULSE: f64 = 0.6;
pub const STORM_BOLT_PX: f64 = 70.0;
pub const STORM_BOLTS: i64 = 3;

impl Stencils {
    /// `marks` zeichnet die Bodenflächen (`ground`) bzw. die übrigen Markierungen der Zelle zur Zeit now.
    pub fn marks(&self, dst: &mut dyn Canvas, list: &[Mark], c: &Cell, now: f64, ground: bool) {
        for m in list {
            let on_ground = m.kind == StatusKind::Aura || m.kind == StatusKind::Storm;
            if on_ground != ground || !c.camera.visible(m.x, m.radius / UNIT_PX + 1.0) {
                continue;
            }
            let (x, y, s) = at(c, m.x, m.y);
            self.mark(dst, m, x, y, s, now);
        }
    }

    /// `mark` zeichnet eine Markierung mit Mitte (x, y) in Fenster-px; s rechnet Welt-px in Fenster-px.
    #[allow(clippy::cast_precision_loss)]
    pub fn mark(&self, dst: &mut dyn Canvas, m: &Mark, x: f64, y: f64, s: f64, now: f64) {
        let r = m.radius * s;
        match m.kind {
            StatusKind::Aura => {
                self.circle(dst, x, y, r, rgba(m.color, 0.14));
                self.ring_at(dst, x, y, r, rgba(m.color, 0.5));
            }
            StatusKind::Storm => self.storm(dst, x, y, r, s, now),
            StatusKind::Shield => {
                self.circle(dst, x, y, r, rgba(m.color, 0.28));
                self.ring_at(dst, x, y, r, rgba(m.color, 0.8));
                let (bx, by) = (x - 22.0 * s, y - (104.0 - SHIELD_Y) * s);
                self.pixel_at(dst, bx, by, 44.0 * s, 5.0 * s, rgba(0, 0.6));
                self.pixel_at(dst, bx, by, 44.0 * s * m.share, 5.0 * s, rgba(m.color, 1.0));
            }
            StatusKind::LastStand => {
                let pulse = 0.5 + 0.5 * (now / LAST_STAND_PULSE * PI * 2.0).sin();
                self.ring_at(dst, x, y, r, rgba(m.color, 0.4 + 0.5 * pulse));
            }
            StatusKind::Slow => {
                for i in 0..3 {
                    let a = f64::from(i) / 3.0 * PI;
                    let (dx, dy) = (a.cos() * r, a.sin() * r);
                    self.line(
                        dst,
                        x - dx,
                        y - dy,
                        x + dx,
                        y + dy,
                        (3.0 * s).max(1.0),
                        rgba(m.color, 1.0),
                    );
                }
            }
            StatusKind::Stun => {
                for i in 0..3 {
                    let a = now / STUN_SPIN * PI * 2.0 + f64::from(i) / 3.0 * PI * 2.0;
                    self.star(
                        dst,
                        x + a.cos() * r,
                        y + a.sin() * r * 0.35,
                        7.0 * s,
                        rgba(m.color, 1.0),
                    );
                }
            }
            StatusKind::Taunt => {
                self.pixel_at(dst, x - 2.5 * s, y - r, 5.0 * s, r * 0.7, rgba(m.color, 1.0));
                self.circle(dst, x, y + 3.0 * s, 3.0 * s, rgba(m.color, 1.0));
            }
        }
    }

    /// `star`: kleiner Stern aus zwei Dreiecken (eines gespiegelt).
    #[allow(clippy::cast_possible_truncation)]
    pub fn star(&self, dst: &mut dyn Canvas, x: f64, y: f64, size: f64, clr: [u8; 4]) {
        self.stamp(dst, self.tri, x - size, y - size, 2.0 * size, 1.6 * size, clr);
        if size <= 0.0 || clr[3] == 0 {
            return;
        }
        let n = f64::from(super::paint::STENCIL_PX);
        let mut g = Affine::IDENTITY;
        g.scale((2.0 * size / n) as f32, (-1.6 * size / n) as f32)
            .translate((x - size) as f32, (y + size) as f32);
        let src = k3c_platform::Rect::new(0.0, 0.0, n as f32, n as f32);
        dst.sprite_tf(self.tri, src, g, tint(clr));
    }

    /// `storm`: Bodenfläche mit Blitzen, die je `STORM_FLASH` Sekunden an andere Stellen springen (aus der Zeit, kein
    /// Zufall).
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn storm(&self, dst: &mut dyn Canvas, x: f64, y: f64, r: f64, s: f64, now: f64) {
        self.circle(dst, x, y, r, rgba(STORM_FILL, 0.2));
        let step = (now / STORM_FLASH).floor() as i64;
        let clr = rgba(STORM_COLOR, 0.9);
        let lw = (3.0 * s).max(1.0);
        for i in 0..STORM_BOLTS {
            let bx = x + (((step * 7 + i * 13) % 17) as f64 / 8.0 - 1.0) * r * 0.8;
            let top = y - STORM_BOLT_PX * s;
            self.line(dst, bx, top, bx + 8.0 * s, top + STORM_BOLT_PX * s * 0.5, lw, clr);
            self.line(
                dst,
                bx + 8.0 * s,
                top + STORM_BOLT_PX * s * 0.5,
                bx - 4.0 * s,
                y,
                lw,
                clr,
            );
        }
    }
}
