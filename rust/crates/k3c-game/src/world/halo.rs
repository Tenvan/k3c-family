//! RU55.5 (B-1046): Lichthöfe im Profil `k2c` (`data/look.json` › glow, Standard Bildaufbau §§ 5 und 6).
//!
//! Lagerfeuer, Fackeln (Mauer, Turm und die Fackeln der Burg) und der Mond haben nachts einen weichen Hof in
//! `glowColor`: ein Verlauf (Kern, Mitte, Rand) als Dreiecksnetz aus einem festen Puffer, Deckkraft nach `1 − daylight()`.
//! Den Hof zeichnet die Welt nach der Lichtkarte und vor dem Spiegel, damit der Fluss die Lichter mit zeigt. Gerechnet
//! wird nur, was im Ausschnitt liegt, höchstens `glow.max` Höfe je Zelle (ohne den Mond); je Bild entsteht nichts Neues.

use k3c_data::look::{Glow, current};
use k3c_platform::{Canvas, Vertex};
use k3c_sim::state::World as SimWorld;

use super::World;
use super::cull::visible;
use super::light::{LightData, glow_alpha, has_campfire, torch_side};
use super::lightdraw::TORCH_LIFT;
use super::tint::has_sky;
use crate::view::cell::Cell;

/// SEGMENTS ist die Zahl der Segmente eines Hofs, RINGS die Zahl der Ringe nach der Mitte (Kern, Mitte, Rand).
pub const SEGMENTS: usize = 16;
const RINGS: usize = 3;
const VERTS: usize = 1 + RINGS * SEGMENTS;
/// Höhe des Hofs im Verhältnis zur Breite (ein Hof am Boden ist flacher als breit).
const SQUASH: f64 = 0.7;
const MOON_COLOR: [f32; 3] = [0.86, 0.9, 1.0];
const BLOOD_COLOR: [f32; 3] = [1.0, 0.5, 0.42];

/// Kind unterscheidet die Art des Lichts (für Farbe und Tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Campfire,
    Torch,
    /// Folgendes Licht am Monarchen (LA1.8)
    Monarch,
    Moon,
    MoonDisc,
}

/// Halo ist ein Hof in Fenster-px: Mitte, Halbmesser, Deckkraft in der Mitte, Anteil des Kerns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Halo {
    pub kind: Kind,
    pub x: f64,
    pub y: f64,
    pub rx: f64,
    pub ry: f64,
    pub alpha: f64,
    pub core: f64,
    pub color: [f32; 3],
}

/// HaloMesh hält Ecken und Indizes eines Hofs (einmal angelegt, je Hof neu gefüllt).
#[derive(Clone, Debug, Default)]
pub struct HaloMesh {
    verts: Vec<Vertex>,
    indices: Vec<u16>,
    dirs: Vec<[f32; 2]>,
}

impl HaloMesh {
    /// Ensure legt die Puffer beim ersten Gebrauch an: Ecke 0 ist die Mitte, Ring r Segment i liegt bei `1 + r·SEGMENTS + i`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn ensure(&mut self) {
        if !self.verts.is_empty() {
            return;
        }
        self.verts = vec![Vertex::default(); VERTS];
        self.dirs = (0..SEGMENTS)
            .map(|i| {
                let a = std::f64::consts::TAU * i as f64 / SEGMENTS as f64;
                [a.cos() as f32, a.sin() as f32]
            })
            .collect();
        let at = |r: usize, i: usize| (1 + r * SEGMENTS + i % SEGMENTS) as u16;
        for i in 0..SEGMENTS {
            self.indices.extend_from_slice(&[0, at(0, i), at(0, i + 1)]);
            for r in 0..RINGS - 1 {
                let (a, b, c, d) = (at(r, i), at(r, i + 1), at(r + 1, i), at(r + 1, i + 1));
                self.indices.extend_from_slice(&[a, c, b, b, c, d]);
            }
        }
    }

    /// Fill füllt die Ecken für `h`: Kern bei Anteil `core` voll, auf halbem Weg `falloff` der Mitte, am Rand 0.
    #[allow(clippy::cast_possible_truncation)]
    pub fn fill(&mut self, h: &Halo, falloff: f64) {
        self.ensure();
        let [r, g, b] = h.color;
        let core = h.core.clamp(0.0, 0.98);
        let stops = [
            (core.max(0.02), h.alpha),
            (core + (1.0 - core) * 0.5, h.alpha * falloff.clamp(0.0, 1.0)),
            (1.0, 0.0),
        ];
        self.verts[0] = Vertex {
            pos: [h.x as f32, h.y as f32],
            uv: [0.5, 0.5],
            color: [r, g, b, h.alpha as f32],
        };
        for (ring, (f, a)) in stops.into_iter().enumerate() {
            for (i, d) in self.dirs.iter().enumerate() {
                self.verts[1 + ring * SEGMENTS + i] = Vertex {
                    pos: [
                        (h.x + f64::from(d[0]) * h.rx * f) as f32,
                        (h.y + f64::from(d[1]) * h.ry * f) as f32,
                    ],
                    uv: [0.5, 0.5],
                    color: [r, g, b, a as f32],
                };
            }
        }
    }
}

/// Collect sammelt die Höfe der Zelle in `out` (leert ihn zuerst): Lagerfeuer, Fackeln, dann der Mond mit Scheibe.
/// Nur Lichter im Ausschnitt, nur nachts (Deckkraft > 0), höchstens `g.max` am Boden.
pub fn collect(light: &LightData, g: &Glow, c: &Cell, wd: &SimWorld, (level, boost): (&str, f64), out: &mut Vec<Halo>) {
    out.clear();
    if !g.enabled {
        return;
    }
    let (k, color) = (&c.camera, light.glow.map(|v| v as f32));
    let make = |kind, x: f64, h: f64, radius: f64, alpha: f64| Halo {
        kind,
        x: k.screen_x(x),
        y: c.screen_y(h),
        rx: radius * g.radius * k.scale,
        ry: radius * g.radius * k.scale * SQUASH,
        alpha,
        core: g.core,
        color,
    };
    let a = (glow_alpha(light.campfire.strength, &wd.cycle) * g.strength * boost).min(1.0);
    if g.sources
        && light.campfire.valid()
        && has_campfire(wd)
        && a > 0.0
        && visible(k, wd.castle.x, light.campfire.radius * g.radius)
    {
        out.push(make(Kind::Campfire, wd.castle.x, 1.0, light.campfire.radius, a));
    }
    let a = (glow_alpha(light.torch.strength, &wd.cycle) * g.strength * boost).min(1.0);
    if g.sources && light.torch.valid() && a > 0.0 {
        for s in wd.sites.iter().filter(|s| s.state == "built") {
            let side = torch_side(s.x, wd.hub_x);
            for o in light.torches.get(&s.kind).into_iter().flatten() {
                let x = s.x + side * o;
                if out.len() < g.max && visible(k, x, light.torch.radius * g.radius) {
                    out.push(make(Kind::Torch, x, TORCH_LIFT + 0.2, light.torch.radius, a));
                }
            }
        }
    }
    if g.sources {
        collect_monarchs(g, c, wd, boost, out);
    }
    if has_sky(level) {
        collect_moon(g, c, wd, out);
    }
}

/// Folgendes Licht (LA1.8, B-1161): jeder Monarch trägt nachts einen kleinen warmen Hof über dem Kopf; er zählt nicht
/// zur Obergrenze `glow.max`, weil höchstens vier Spieler Licht tragen.
fn collect_monarchs(g: &Glow, c: &Cell, wd: &SimWorld, boost: f64, out: &mut Vec<Halo>) {
    let (m, k) = (&g.monarch, &c.camera);
    let a = (glow_alpha(m.strength, &wd.cycle) * boost).min(1.0);
    if a <= 0.0 || m.radius <= 0.0 {
        return;
    }
    let color = [1.0, 0.8, 0.5];
    for p in wd.players.iter().filter(|p| visible(k, p.x, m.radius)) {
        let r = m.radius * k.scale;
        out.push(Halo {
            kind: Kind::Monarch,
            x: k.screen_x(p.x),
            y: c.screen_y(m.height),
            rx: r,
            ry: r,
            alpha: a,
            core: g.core,
            color,
        });
    }
}

/// Mond mit weitem Hof und Scheibe am Himmel der Zelle; Voll- und Blutmond größer, Blutmond rot.
#[allow(clippy::cast_precision_loss)]
fn collect_moon(g: &Glow, c: &Cell, wd: &SimWorld, out: &mut Vec<Halo>) {
    let m = &g.moon;
    let a = m.strength * (1.0 - super::light::daylight(&wd.cycle));
    if a <= 0.0 {
        return;
    }
    let has = |id: &str| wd.night_events.iter().any(|e| e.id == id);
    let blood = has("bloodMoon");
    let big = if blood || has("fullMoon") { 1.3 } else { 1.0 };
    let (x, y) = (
        c.rect.min.x as f64 + c.rect.dx() as f64 * m.x,
        c.rect.min.y as f64 + c.world_rect().dy() as f64 * m.y,
    );
    let r = m.radius * c.camera.scale * big;
    let color = if blood { BLOOD_COLOR } else { MOON_COLOR };
    let halo = |kind, r: f64, alpha: f64, core: f64| Halo {
        kind,
        x,
        y,
        rx: r,
        ry: r,
        alpha,
        core,
        color,
    };
    out.push(halo(Kind::Moon, r, a * 0.6, 0.0));
    out.push(halo(Kind::MoonDisc, r * m.disc, (a * 1.6).min(1.0), 0.9));
}

impl World {
    /// DrawHalos zeichnet die Höfe der Zelle (Profil `k2c`; sonst nichts).
    pub fn draw_halos(&mut self, dst: &mut dyn Canvas, c: &Cell, wd: &SimWorld, level: &str, boost: f64) {
        let g = current().glow;
        if !super::fx::on("halos") {
            return;
        }
        collect(&self.light, &g, c, wd, (level, boost), &mut self.halos);
        if self.halos.is_empty() {
            return;
        }
        let pixel = *self.light_pixel.get_or_insert_with(|| dst.texture(&[255; 4], 1, 1));
        for i in 0..self.halos.len() {
            let h = self.halos[i];
            self.halo_mesh.fill(&h, g.falloff);
            dst.mesh(pixel, &self.halo_mesh.verts, &self.halo_mesh.indices);
        }
    }
}

#[cfg(test)]
#[path = "halo_tests.rs"]
mod halo_tests;
