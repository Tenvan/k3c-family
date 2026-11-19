//! Port von `native/k3c-game/world/ground.go`.
//!
//! Boden der Stufe (B-795/AC-02): ein Uferstreifen in der Boden-Farbe des Bioms unter der Boden-Linie, oben im
//! Fluss-Drittel der Zelle (NV3: native/k3c-ebiten/river.go › bankH); darunter fließt der Fluss. Lava-Streifen
//! (engine/sim/world_lava.go) liegen glühend im Uferstreifen.

use k3c_det::fmin;
use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;

use super::World;
use super::backdrop::Rgba;
use super::cull::visible;
use super::layers::stage_of;
use crate::view::cell::Cell;
use crate::view::frame::Frame;

/// LAVA_COLOR ist die Farbe eines Lava-Streifens (nur Darstellung).
pub const LAVA_COLOR: Rgba = [232, 96, 32, 255];

/// BankHeight ist die Höhe des Uferstreifens der Zelle in px. Profil `k2c`: `bank_share` der Zellenhöhe (7 %), höchstens
/// ein Drittel des Flusses; Profil `go`: `bank_px` (20 px bei 32 px je Unit, wie NV3) mal Maßstab, höchstens der Fluss.
#[allow(clippy::cast_precision_loss)]
pub fn bank_height(c: &Cell) -> f64 {
    let l = k3c_data::look::current();
    let river = c.river_rect().dy() as f64;
    if l.bank_share > 0.0 {
        fmin(l.bank_share * c.rect.dy() as f64, river / 3.0)
    } else {
        fmin(l.bank_px / 32.0 * c.camera.scale, river)
    }
}

/// LavaSpan liefert links und rechts (Units) des Lava-Streifens im Chunk i der Welt; `None` ohne Lava.
pub fn lava_span(w: &SimWorld, i: usize) -> Option<(f64, f64)> {
    let ch = &w.level.chunks[i];
    let half = w.biome.lava.width_units / 2.0;
    if ch.kind != "lava" || half <= 0.0 {
        return None;
    }
    let mid = ch.start_units + w.level.chunk_width_units / 2.0;
    Some((mid - half, mid + half))
}

impl World {
    /// DrawGround zeichnet den Uferstreifen und die sichtbaren Lava-Streifen.
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_ground(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(st) = stage_of(f, c) else {
            return;
        };
        let Some(wd) = st.world.as_deref() else {
            return;
        };
        let k = &c.camera;
        let (top, h) = (c.ground_y() as f64, bank_height(c));
        let ground = self.looks.of(&st.level).ground;
        let k2c = k3c_data::look::current().bank_share > 0.0;
        // k2c: dunkle Erde mit Rissen, Wurzeln und Grasbüscheln (bank.rs); go: Streifen in der Boden-Farbe
        let earth = if k2c { super::bank::EARTH_BRIGHTNESS } else { 1.0 };
        self.fill(dst, c.rect.min.x as f64, top, c.rect.dx() as f64, h, ground, earth, 1.0);
        if k2c {
            self.draw_bank(dst, c, ground, h);
        }
        for i in 0..wd.level.chunks.len() {
            let Some((l, r)) = lava_span(wd, i) else {
                continue;
            };
            if !visible(k, (l + r) / 2.0, (r - l) / 2.0) {
                continue;
            }
            let (x0, x1) = (k.screen_x(l), k.screen_x(r));
            self.fill(dst, x0, top, x1 - x0, h, LAVA_COLOR, 1.0, 1.0);
        }
    }
}
