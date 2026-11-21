//! Port von `native/k3c-game/world/lightdraw.go`.
//!
//! Licht zeichnen (B-795/AC-03, Vorbild src/scenes/render/lightView.ts): eine Lichtkarte multipliziert die ganze Zelle
//! (Welt und Fluss, über Gebäuden und Figuren, unter den Effekten), gerechnet in `LIGHT_COLUMNS` Spalten und dazwischen
//! linear verlaufen; darüber die Fackeln (Halter immer, Flamme mit ihrem Schein) und der Übergang nach einem
//! Stufenwechsel (Abblenden aus Schwarz). Am vollen Tag ohne Übergang zeichnet es nichts.
//!
//! Die Lichtkarte zeichnet `Canvas::blend(Blend::Multiply)` mit `mesh` auf einem weißen 1×1-Bild (B-1056, Go:
//! `ebiten.Blend` mit Ziel × Quelle, Deckkraft des Ziels bleibt).

use k3c_data::look::current;
use k3c_platform::{Blend, Canvas, Vertex};
use k3c_sim::state::World as SimWorld;

use super::World;
use super::backdrop::Rgba;
use super::cull::visible;
use super::forest::forest_tint;
use super::layers::stage_of;
use super::light::{LightData, daylight, glow_alpha, torch_flicker};
use super::tint::{NEUTRAL, is_neutral, tint_of};
use crate::view::cell::Cell;
use crate::view::frame::Frame;

/// LIGHT_COLUMNS ist die Zahl der Spalten der Lichtkarte je Zelle.
pub const LIGHT_COLUMNS: usize = 64;
// Fackel (lightView.ts, in Units): Flammen-Mitte über dem Boden, Halter, Flammen-Radius.
pub const TORCH_LIFT: f64 = 40.0 / 32.0;
pub const TORCH_STICK_W: f64 = 4.0 / 32.0;
pub const TORCH_STICK_H: f64 = 16.0 / 32.0;
pub const TORCH_FLAME_R: f64 = 12.0 / 32.0;

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
        self.fill_tinted(c, light, wd, NEUTRAL);
    }

    /// FillTinted wie `fill`, die Lichtfarbe je Kanal mit der Tönung `tint` (RU55.4) multipliziert.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn fill_tinted(&mut self, c: &Cell, light: &LightData, wd: &SimWorld, tint: [f64; 3]) {
        self.ensure();
        let (top, bottom) = (c.rect.min.y as f32, c.rect.max.y as f32);
        for i in 0..=LIGHT_COLUMNS {
            let sx = column_x(c, i);
            let (r, g, b) = light.color_at(c.camera.world_x(sx), wd);
            for (j, y) in [top, bottom].into_iter().enumerate() {
                self.verts[2 * i + j] = Vertex {
                    pos: [sx as f32, y],
                    uv: [0.5, 0.5],
                    color: [(r * tint[0]) as f32, (g * tint[1]) as f32, (b * tint[2]) as f32, 1.0],
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
        let d = self.density(c, wd, f.clock, &st.level);
        let tint = tint_of(&st.level, wd);
        let tint = if d > 0.0 {
            let ft = forest_tint(d);
            [0, 1, 2].map(|i| tint[i] * ft[i])
        } else {
            tint
        };
        self.light.flicker_t = f.clock;
        if daylight(&wd.cycle) < 1.0 || !is_neutral(tint) {
            self.draw_light_map(dst, c, wd, tint);
        }
        self.draw_stars(dst, c, wd, &st.level);
        self.draw_halos(dst, c, wd, &st.level, 1.0 + current().forest.fire * d);
        if self.mirror_wanted(f, dst) {
            self.draw_mirror(dst, c, f.clock, wd);
        }
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

    /// DrawLightMap multipliziert die Zelle mit der Lichtfarbe je Spalte und der Tönung `tint`.
    pub fn draw_light_map(&mut self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, tint: [f64; 3]) {
        self.mesh.fill_tinted(c, &self.light, wd, tint);
        let pixel = *self.light_pixel.get_or_insert_with(|| dst.texture(&[255; 4], 1, 1));
        dst.blend(Blend::Multiply);
        dst.mesh(pixel, &self.mesh.verts, &self.mesh.indices);
        dst.blend(Blend::Over);
    }

    /// DrawTorchPart zeichnet die Halter der Fackeln (Ebene der Figuren: Gebäude sind darunter, Figuren davor, die
    /// Lichtkarte dunkelt sie nachts mit ab).
    pub fn draw_torch_part(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(wd) = stage_of(f, c).and_then(|st| st.world.as_deref()) else {
            return;
        };
        let (k, top) = (&c.camera, c.screen_y(TORCH_LIFT));
        let (sw, sh) = (TORCH_STICK_W * k.scale, TORCH_STICK_H * k.scale);
        let wid = world_of(f);
        self.light.for_each_torch_at(wd, |n, x, kind| {
            if !visible(k, x, TORCH_FLAME_R) {
                return;
            }
            if self.torch_sprite(dst, c, wid, kind, x, f.clock + n as f64 * 0.3) {
                return;
            }
            self.fill(dst, k.screen_x(x) - sw / 2.0, top, sw, sh, TORCH_STICK, 1.0, 1.0);
        });
        self.draw_flames(dst, c, wd, f.clock, wid);
    }

    /// TorchSprite zeichnet das Licht-Asset (LA2.3) der Welt `wid` für die Art `kind` mit animierter Flamme an den
    /// Anker x; `false` ohne geladenes Sheet (dann zeichnet der Aufrufer Stab und Flamme prozedural).
    #[allow(clippy::cast_possible_truncation)]
    fn torch_sprite(&self, dst: &mut dyn Canvas, c: &Cell, wid: &str, kind: &str, x: f64, t: f64) -> bool {
        let (asset, w, lift) = light_asset(wid, kind);
        let Some(sh) = self.lib.as_ref().and_then(|l| l.sheet(asset, "idle")) else {
            return false;
        };
        let Some(fr) = sh.frame_at(t) else {
            return false;
        };
        if sh.fw <= 0 {
            return false;
        }
        let (k, wpx) = (&c.camera, w * c.camera.scale);
        let s = wpx / sh.fw as f64;
        let mut g = k3c_platform::Affine::IDENTITY;
        g.scale(s as f32, s as f32).translate(
            (k.screen_x(x) - wpx / 2.0) as f32,
            (c.screen_y(lift) - sh.fh as f64 * s) as f32,
        );
        dst.sprite_tf(fr.tex, fr.src, g, [1.0, 1.0, 1.0, 1.0]);
        true
    }

    /// DrawFlames zeichnet die Flammen mit den Haltern (unter den Figuren, von der Lichtkarte nur wenig gedämpft): drei Schichten (außen orange, Mitte
    /// glowColor, Kern hell), nachts mit der Deckkraft ihres Scheins, flackernd mit der Spielzeit (am Tag aus).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn draw_flames(&self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, clock: f64, wid: &str) {
        let a = glow_alpha(self.light.torch.strength, &wd.cycle);
        if a <= 0.0 {
            return;
        }
        let (k, top) = (&c.camera, c.screen_y(TORCH_LIFT));
        let g = self.light.glow;
        let mid: Rgba = [(g[0] * 255.0) as u8, (g[1] * 255.0) as u8, (g[2] * 255.0) as u8, 255];
        self.light.for_each_torch_at(wd, |n, x, kind| {
            let size = TORCH_FLAME_R * flicker_scale(clock, n);
            // mit Licht-Asset trägt dessen Flamme das Bild; der Schein kommt aus den Höfen
            if !visible(k, x, size) || self.has_light_asset(wid, kind) {
                return;
            }
            let (sx, al, r) = (k.screen_x(x), a * flicker(clock, n), size * k.scale);
            // Schichten: (Breite, Höhe, Farbe, Deckkraft) – breit unten, schmal oben, wie eine Flamme
            let layers: [(f64, f64, Rgba, f64); 3] = [
                (1.0, 2.6, [255, 120, 30, 255], 0.55),
                (0.7, 2.0, mid, 0.85),
                (0.35, 1.2, [255, 240, 200, 255], 1.0),
            ];
            for (w, h, col, ca) in layers {
                self.fill(dst, sx - r * w, top - r * h, 2.0 * r * w, r * h, col, 1.0, al * ca);
            }
        });
    }
}

/// LIGHT_ASSETS sind die Licht-Assets (LA2.3) je Welt (Standard, Sengoku, Nil, Britannien) und Art (Fackel, Laterne,
/// Feuerschale).
const LIGHT_ASSETS: [[&str; 3]; 4] = [
    [
        "objekte/licht-fackel-standard",
        "objekte/licht-laterne-standard",
        "objekte/licht-schale-standard",
    ],
    [
        "objekte/licht-fackel-sengoku",
        "objekte/licht-laterne-sengoku",
        "objekte/licht-schale-sengoku",
    ],
    [
        "objekte/licht-fackel-nil",
        "objekte/licht-laterne-nil",
        "objekte/licht-schale-nil",
    ],
    [
        "objekte/licht-fackel-britannien",
        "objekte/licht-laterne-britannien",
        "objekte/licht-schale-britannien",
    ],
];

/// light_asset wählt Asset, Breite (Units) und Höhe der Unterkante über dem Boden (Units): die Burg trägt
/// Feuerschalen am Boden, Taverne und Heilplatz Laternen am Boden, alles andere Fackeln an der Wand; unbekannte
/// Welten zeigen die der Standard-Welt.
fn light_asset(wid: &str, kind: &str) -> (&'static str, f64, f64) {
    let w = match wid {
        "sengoku" => 1,
        "nil" => 2,
        "britannien" => 3,
        _ => 0,
    };
    match kind {
        "castle" => (LIGHT_ASSETS[w][2], 0.95, 0.0),
        "tavern" | "healer" => (LIGHT_ASSETS[w][1], 0.7, 0.0),
        _ => (LIGHT_ASSETS[w][0], 0.6, TORCH_LIFT - TORCH_STICK_H),
    }
}

/// world_of ist die Welt-ID der Insel im Bild (leer = Standard).
pub(super) fn world_of(f: &Frame) -> &str {
    f.route
        .iter()
        .find(|r| r.id == f.campaign.island)
        .map_or("", |r| r.world.as_str())
}

impl World {
    /// has_light_asset meldet, ob das Sheet des Licht-Assets geladen ist.
    fn has_light_asset(&self, wid: &str, kind: &str) -> bool {
        self.lib
            .as_ref()
            .is_some_and(|l| l.sheet(light_asset(wid, kind).0, "idle").is_some())
    }
}

/// Flicker ist die Deckkraft-Schwankung (0,7 … 1) der Flamme `n` zur Spielzeit `t`.
fn flicker(t: f64, n: usize) -> f64 {
    0.7 + 0.3 * torch_flicker(t, n)
}

/// Flicker_scale ist die Größen-Schwankung (0,88 … 1,12) der Flamme, aus demselben Muster.
fn flicker_scale(t: f64, n: usize) -> f64 {
    0.88 + 0.24 * torch_flicker(t, n)
}

#[cfg(test)]
#[path = "lightdraw_tests.rs"]
mod lightdraw_tests;
