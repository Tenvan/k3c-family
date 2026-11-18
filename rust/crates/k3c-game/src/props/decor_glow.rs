//! Port von `native/k3c-game/props/decor_glow.go`: Darstellung der Natur-Eigenschaften, Teil 2 (B-904 bis B-906):
//! Stützbalken und Verfall der Mine, Glut des Eisenstollens, Schein der Kristall-Adern. Regeln wie `decor.rs` (Wert je
//! Stufe, Orte aus der Position).

use k3c_sim::state::World;

use super::decor::{cell_hash, span, trait_of};
use super::nature::mix;
use super::plan::{Item, Planner, Shape, visible};
use crate::view::cell::Camera;

// Maße der Darstellung in Units.
/// Abstand der Stützbalken-Rahmen
pub const BEAM_STEP: f64 = 8.0;
/// um die Burg kein Rahmen
pub const BEAM_HUB: f64 = 10.0;
/// Höhe des Querbalkens
pub const BEAM_HEIGHT: f64 = 4.0;
/// halber Abstand der Pfosten
pub const BEAM_HALF: f64 = 1.3;
/// Breite der Balken
pub const BEAM_WIDTH: f64 = 0.25;
/// Raster der Glutpunkte
pub const EMBER_STEP: f64 = 1.5;
/// Höhe des Hitze-Schleiers
pub const HAZE_HEIGHT: f64 = 1.2;
/// Mitte des Scheins über dem Boden (halbe Höhe der Ader)
pub const VEIN_GLOW_MID: f64 = 1.0;

pub const BEAM_WOOD: [u8; 4] = [107, 74, 47, 255];
pub const RUBBLE_STONE: [u8; 4] = [96, 80, 66, 255];
pub const EMBER_COLOR: [u8; 4] = [255, 122, 40, 255];
pub const HAZE_COLOR: [u8; 4] = [200, 52, 20, 255];

/// `vein_glow` ist die Farbe des Scheins je Kristallfarbe (Basis-Ton der Materialien kristall, purpur, smaragd).
pub fn vein_glow(variant: &str) -> Option<[u8; 4]> {
    match variant {
        "blau" => Some([95, 205, 228, 255]),
        "violett" => Some([168, 101, 184, 255]),
        "gruen" => Some([95, 209, 138, 255]),
        _ => None,
    }
}

impl Planner {
    /// `decor_glow` plant Balken, Glut und Schein (Aufruf aus `decor`).
    pub fn decor_glow(&mut self, w: &World, k: &Camera, clock: f64) {
        self.mine_beams(w, k);
        self.embers(w, k, clock);
        self.crystal_glow(w, k, clock);
    }

    /// `mine_beams`: alle `BEAM_STEP` Units ein Stützbalken-Rahmen; verfallen, wenn `mix % 4 < Wert` (B-904).
    pub fn mine_beams(&mut self, w: &World, k: &Camera) {
        let Some(v) = trait_of(w, "decay") else {
            return;
        };
        let Some((from, to)) = span(w, k, BEAM_STEP, BEAM_HALF + 2.0) else {
            return;
        };
        let mut x = from;
        while x < to {
            if (x - w.hub_x).abs() >= BEAM_HUB {
                if i64::from(cell_hash(x, BEAM_STEP, 307) % 4) < v {
                    self.broken_beam(x);
                } else {
                    self.beam(x - BEAM_HALF, 0.0, x - BEAM_HALF, BEAM_HEIGHT);
                    self.beam(x + BEAM_HALF, 0.0, x + BEAM_HALF, BEAM_HEIGHT);
                    self.beam(x - BEAM_HALF - 0.3, BEAM_HEIGHT, x + BEAM_HALF + 0.3, BEAM_HEIGHT);
                }
            }
            x += BEAM_STEP;
        }
    }

    /// `broken_beam`: ein Pfosten schief, der Querbalken hängt schräg herab, darunter drei Brocken Geröll.
    pub fn broken_beam(&mut self, x: f64) {
        self.beam(x - BEAM_HALF, 0.0, x - BEAM_HALF + 0.7, BEAM_HEIGHT - 0.4);
        self.beam(x + BEAM_HALF, 0.0, x + BEAM_HALF, BEAM_HEIGHT);
        self.beam(x + BEAM_HALF + 0.3, BEAM_HEIGHT, x - BEAM_HALF + 0.2, 1.4);
        for (i, dx) in [-0.6, -0.1, 0.35].into_iter().enumerate() {
            let s = 0.22 + 0.08 * (i % 2) as f64;
            Planner::rect(&mut self.body, x + dx, 0.0, s, s * 0.8, RUBBLE_STONE);
        }
    }

    /// `beam` legt eine Balken-Linie an.
    pub fn beam(&mut self, x: f64, y: f64, x2: f64, y2: f64) {
        self.body.push(Item {
            shape: Shape::Line,
            x,
            y,
            x2,
            y2,
            size: BEAM_WIDTH,
            clr: BEAM_WOOD,
            ..Default::default()
        });
    }

    /// `embers`: Hitze-Schleier am Boden und Glutpunkte (`mix % 8 < 2 · Wert`), Deckkraft pulsiert mit der Uhr (B-905).
    pub fn embers(&mut self, w: &World, k: &Camera, clock: f64) {
        let Some(v) = trait_of(w, "glow").filter(|&v| v > 0) else {
            return;
        };
        let Some((from, to)) = span(w, k, EMBER_STEP, 1.0) else {
            return;
        };
        self.body.push(Item {
            shape: Shape::Rect,
            x: (from + to) / 2.0,
            w: to - from,
            h: HAZE_HEIGHT,
            clr: HAZE_COLOR,
            alpha: 0.06 * v as f32,
            ..Default::default()
        });
        let mut x = from;
        while x < to {
            let h = cell_hash(x, EMBER_STEP, 401);
            if i64::from(h % 8) < 2 * v {
                let r = 0.08 + f64::from((h >> 4) % 11) / 100.0;
                let y = 0.05 + f64::from((h >> 8) % 36) / 100.0;
                let phase = f64::from((h >> 12) % 628) / 100.0;
                let a = (0.55 + 0.4 * (clock * 2.3 + phase).sin()) as f32;
                self.body.push(Item {
                    shape: Shape::Circle,
                    x: x + f64::from((h >> 16) % 10) / 10.0,
                    y,
                    w: r,
                    clr: EMBER_COLOR,
                    alpha: a,
                    ..Default::default()
                });
            }
            x += EMBER_STEP;
        }
    }

    /// `crystal_glow`: Schein in der Farbe jeder sichtbaren Kristall-Ader, Radius und Deckkraft nach Wert (B-906); er
    /// liegt vor der Ader in der Zeichenliste, also hinter ihr.
    pub fn crystal_glow(&mut self, w: &World, k: &Camera, clock: f64) {
        let Some(v) = trait_of(w, "brightness") else {
            return;
        };
        let r = 0.8 + 0.6 * v as f64;
        for n in &w.nodes {
            if n.kind != "crystalVein" || !visible(k, n.x, r) {
                continue;
            }
            let clr = vein_glow(&n.variant).or_else(|| vein_glow("blau")).unwrap_or_default();
            let pulse = 1.0 + 0.1 * (clock * 1.5 + f64::from(mix(n.x) % 628) / 100.0).sin();
            let a = (0.12 * v as f64 * pulse) as f32;
            for radius in [r, r * 0.55] {
                self.body.push(Item {
                    shape: Shape::Circle,
                    x: n.x,
                    y: VEIN_GLOW_MID,
                    w: radius,
                    clr,
                    alpha: a,
                    ..Default::default()
                });
            }
        }
    }
}
