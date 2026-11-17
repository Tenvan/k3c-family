//! Port von `native/k3c-game/world/lightdraw.go`.
//!
//! Licht zeichnen (B-795/AC-03, Vorbild src/scenes/render/lightView.ts): eine Lichtkarte multipliziert die ganze Zelle
//! (Welt und Fluss, über Gebäuden und Figuren, unter den Effekten), gerechnet in `LIGHT_COLUMNS` Spalten und dazwischen
//! linear verlaufen; darüber die Fackeln (Halter immer, Flamme mit ihrem Schein) und der Übergang nach einem
//! Stufenwechsel (Abblenden aus Schwarz). Am vollen Tag ohne Übergang zeichnet es nichts.
//!
//! Abweichung (RU13.2): `k3c-platform::Canvas` kennt keinen Multiply-Blend (`ebiten.Blend` mit Ziel × Quelle). Die
//! Lichtkarte wird deshalb gerechnet (`LightMesh::fill`, getestet), aber nicht gezeichnet; Ticket „Canvas: Multiply-Blend
//! für die Lichtkarte“.

use k3c_platform::{Canvas, Vertex};
use k3c_sim::state::World as SimWorld;

use super::World;
use super::backdrop::Rgba;
use super::cull::visible;
use super::layers::stage_of;
use super::light::{LightData, daylight, glow_alpha, torch_side};
use crate::view::cell::Cell;
use crate::view::frame::Frame;

/// LIGHT_COLUMNS ist die Zahl der Spalten der Lichtkarte je Zelle.
pub const LIGHT_COLUMNS: usize = 64;
// Fackel (lightView.ts, in Units): Flammen-Mitte über dem Boden, Halter, Flammen-Radius.
pub const TORCH_LIFT: f64 = 40.0 / 32.0;
pub const TORCH_STICK_W: f64 = 4.0 / 32.0;
pub const TORCH_STICK_H: f64 = 16.0 / 32.0;
pub const TORCH_FLAME_R: f64 = 5.0 / 32.0;

pub const TORCH_STICK: Rgba = [0x5a, 0x3a, 0x22, 255];
pub const BLACK: Rgba = [0, 0, 0, 255];

/// LightMesh hält die Puffer der Lichtkarte (einmal angelegt, je Zelle neu gefüllt).
#[derive(Clone, Debug, Default)]
pub struct LightMesh {
    pub verts: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl LightMesh {
    /// Ensure legt die Puffer beim ersten Gebrauch an: Spalte i hat Ecken 2i (oben) und 2i+1 (unten).
    #[allow(clippy::cast_possible_truncation)]
    pub fn ensure(&mut self) {
        if !self.verts.is_empty() {
            return;
        }
        self.verts = vec![Vertex::default(); 2 * (LIGHT_COLUMNS + 1)];
        self.indices = Vec::with_capacity(6 * LIGHT_COLUMNS);
        for i in 0..LIGHT_COLUMNS as u16 {
            let a = 2 * i;
            self.indices.extend_from_slice(&[a, a + 1, a + 2, a + 1, a + 3, a + 2]);
        }
    }

    /// Fill füllt die Ecken mit der Lichtfarbe je Spalte der Zelle (Go: Schleife in `drawLightMap`).
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn fill(&mut self, c: &Cell, light: &LightData, wd: &SimWorld) {
        self.ensure();
        let (top, bottom) = (c.rect.min.y as f32, c.rect.max.y as f32);
        for i in 0..=LIGHT_COLUMNS {
            let sx = column_x(c, i);
            let (r, g, b) = light.color_at(c.camera.world_x(sx), wd);
            for (j, y) in [top, bottom].into_iter().enumerate() {
                self.verts[2 * i + j] = Vertex {
                    pos: [sx as f32, y],
                    uv: [0.5, 0.5],
                    color: [r as f32, g as f32, b as f32, 1.0],
                };
            }
        }
    }
}

/// ColumnX ist das Fenster-x der Spalten-Kante i (0 … LIGHT_COLUMNS) der Zelle.
#[allow(clippy::cast_precision_loss)]
pub fn column_x(c: &Cell, i: usize) -> f64 {
    c.rect.min.x as f64 + c.rect.dx() as f64 * i as f64 / LIGHT_COLUMNS as f64
}

impl World {
    /// DrawLight zeichnet Lichtkarte, Fackeln und Übergang der Zelle.
    #[allow(clippy::cast_precision_loss)]
    pub fn draw_light(&mut self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(st) = stage_of(f, c) else {
            return;
        };
        let Some(wd) = st.world.as_deref() else {
            return;
        };
        if daylight(&wd.cycle) < 1.0 {
            self.draw_light_map(c, wd);
        }
        self.draw_torches(dst, c, wd);
        let a = self.cams.fade(c.player, f.clock);
        if a > 0.0 {
            self.fill(
                dst,
                c.rect.min.x as f64,
                c.rect.min.y as f64,
                c.rect.dx() as f64,
                c.rect.dy() as f64,
                BLACK,
                1.0,
                a,
            );
        }
    }

    /// DrawLightMap multipliziert die Zelle mit der Lichtfarbe je Spalte: hier nur gerechnet (siehe Kopf), der
    /// Multiply-Aufruf fehlt im Canvas.
    pub fn draw_light_map(&mut self, c: &Cell, wd: &SimWorld) {
        self.mesh.fill(c, &self.light, wd);
    }

    /// DrawTorches zeichnet die sichtbaren Fackeln: Halter aus Holz, darüber die Flamme in glowColor mit der Deckkraft
    /// ihres Scheins (am Tag aus).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn draw_torches(&self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld) {
        let offs = &self.light.torches;
        if offs.is_empty() {
            return;
        }
        let k = &c.camera;
        let a = glow_alpha(self.light.torch.strength, &wd.cycle);
        let g = self.light.glow;
        let flame: Rgba = [(g[0] * 255.0) as u8, (g[1] * 255.0) as u8, (g[2] * 255.0) as u8, 255];
        let top = c.screen_y(TORCH_LIFT);
        for s in &wd.sites {
            if s.state != "built" {
                continue;
            }
            let side = torch_side(s.x, wd.hub_x);
            for o in offs.get(&s.kind).into_iter().flatten() {
                let x = s.x + side * o;
                if !visible(k, x, TORCH_FLAME_R) {
                    continue;
                }
                let sx = k.screen_x(x);
                self.fill(
                    dst,
                    sx - TORCH_STICK_W * k.scale / 2.0,
                    top,
                    TORCH_STICK_W * k.scale,
                    TORCH_STICK_H * k.scale,
                    TORCH_STICK,
                    1.0,
                    1.0,
                );
                let r = TORCH_FLAME_R * k.scale;
                self.fill(dst, sx - r, top - r / 2.0 - r, 2.0 * r, 2.0 * r, flame, 1.0, a);
            }
        }
    }
}

#[cfg(test)]
#[path = "lightdraw_tests.rs"]
mod lightdraw_tests;
