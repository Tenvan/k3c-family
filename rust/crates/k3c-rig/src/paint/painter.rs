//! Port von native/k3c-rig/paint/painter.go: Zeichnen über `platform::Canvas`: Teile als Bild mit Matrix, Bänder
//! (Umhang, Beine) als Dreiecksnetz (`Canvas::mesh`, Go: `DrawTriangles`), Zügel als Kurvenband.

use k3c_platform::{Affine, Canvas, Rect, Vertex};

use super::assets::ImageRef;
use crate::rig::matrix::{apply, mul, scale, translate};
use crate::rig::{Band, IDENTITY, M, Point, Vertex as StripVertex};

/// Painter zeichnet in einen `Canvas` (wird je Aufruf übergeben); `view` bildet Welt-Einheiten auf Bildpunkte ab.
/// `alpha` ist die Deckkraft 0..1 der ganzen Figur (Teile, Bänder, Zügel; `new` setzt 1). Jedes Teil wird einzeln mit
/// Alpha gezeichnet: Wo sich Teile überlappen, scheint das hintere durch (anders als ein ganzes Sheet mit Alpha). Die
/// Puffer werden wiederverwendet.
#[derive(Debug, Clone)]
pub struct Painter {
    pub view: M,
    pub alpha: f64,
    white: ImageRef,
    verts: Vec<Vertex>,
    idx: Vec<u16>,
    strip: Vec<StripVertex>,
    pts: Vec<Point>,
}

/// Stützstellen der Zügel-Kurve.
const CURVE_STEPS: i64 = 12;

impl Painter {
    pub fn new(white: ImageRef) -> Painter {
        Painter {
            view: IDENTITY,
            alpha: 1.0,
            white,
            verts: Vec::new(),
            idx: Vec::new(),
            strip: Vec::new(),
            pts: Vec::new(),
        }
    }

    /// `image` zeichnet ein Teil-Bild: Pixel → Einheiten (1/density, Versatz), dann die Matrix des Teils, dann die Sicht.
    pub fn image(&self, c: &mut dyn Canvas, img: ImageRef, m: M, offset: Point, density: f64) {
        let f = mul(
            self.view,
            mul(m, mul(translate(offset[0], offset[1]), scale(1.0 / density))),
        );
        let tf = Affine {
            a: f[0] as f32,
            b: f[2] as f32,
            c: f[1] as f32,
            d: f[3] as f32,
            tx: f[4] as f32,
            ty: f[5] as f32,
        };
        let src = Rect::new(0.0, 0.0, img.w as f32, img.h as f32);
        c.sprite_tf(img.tex, src, tf, [1.0, 1.0, 1.0, self.alpha()]);
    }

    /// `band` biegt das Bild eines Bands entlang der Stützpunkte; m ist die Matrix des Rahmens, in dem die Punkte liegen.
    pub fn band(&mut self, c: &mut dyn Canvas, img: ImageRef, b: &Band, pts: &[Point], m: M, density: f64) {
        b.strip(pts, density, &mut self.strip);
        let f = mul(self.view, m);
        self.verts.clear();
        self.idx.clear();
        let al = self.alpha(); // Ecken-Farben nicht vormultipliziert: Alpha allein genügt
        for (i, v) in self.strip.iter().enumerate() {
            let at = apply(&f, [v.x, v.y]);
            self.verts.push(Vertex {
                pos: [at[0] as f32, at[1] as f32],
                uv: [v.u as f32, v.v as f32],
                color: [1.0, 1.0, 1.0, al],
            });
            if i >= 2 && i % 2 == 0 {
                let k = i as u16;
                self.idx.extend([k - 2, k - 1, k, k - 1, k + 1, k]);
            }
        }
        c.mesh(img.tex, &self.verts, &self.idx);
    }

    /// `curve` zeichnet eine Bézier-Kurve zweiten Grades (Welt-Einheiten) als Band der Breite width in einer Farbe.
    pub fn curve(&mut self, c: &mut dyn Canvas, ends: (Point, Point, Point), width: f64, col: [u8; 4], opacity: f64) {
        let (a, ctrl, b) = ends;
        self.pts.clear();
        for i in 0..=CURVE_STEPS {
            let t = i as f64 / CURVE_STEPS as f64;
            let u = 1.0 - t;
            let q = [
                u * u * a[0] + 2.0 * u * t * ctrl[0] + t * t * b[0],
                u * u * a[1] + 2.0 * u * t * ctrl[1] + t * t * b[1],
            ];
            self.pts.push(apply(&self.view, q));
        }
        let half = width / 2.0 * self.view[0].hypot(self.view[1]);
        // Ecken-Farben sind nicht vormultipliziert (Standard von DrawTriangles)
        let (r, g, bl) = (
            f32::from(col[0]) / 255.0,
            f32::from(col[1]) / 255.0,
            f32::from(col[2]) / 255.0,
        );
        let al = opacity as f32 * self.alpha();
        self.verts.clear();
        self.idx.clear();
        let last = self.pts.len() - 1;
        for i in 0..self.pts.len() {
            let (prev, next) = (self.pts[i.saturating_sub(1)], self.pts[(i + 1).min(last)]);
            let (tx, ty) = (next[0] - prev[0], next[1] - prev[1]);
            let mut d = tx.hypot(ty);
            if d == 0.0 {
                d = 1.0;
            }
            let (nx, ny) = (ty / d, -tx / d);
            let q = self.pts[i];
            // vier Ecken je Punkt: außen durchsichtig, innen deckend. Das glättet die Kante ohne Ebitengines AntiAlias,
            // das je Aufruf ein eigenes Zwischenbild braucht und das Bündeln der Zeichenaufrufe bricht (Messung NV2.3).
            let inner = f64::max(half - 0.5, 0.0);
            for (k, off) in [-half - 0.5, -inner, inner, half + 0.5].into_iter().enumerate() {
                let alpha = if k == 0 || k == 3 { 0.0 } else { al };
                self.verts.push(Vertex {
                    pos: [(q[0] + off * nx) as f32, (q[1] + off * ny) as f32],
                    uv: [1.5, 1.5],
                    color: [r, g, bl, alpha],
                });
            }
            if i > 0 {
                for cc in 0..3u16 {
                    let k = 4 * i as u16 + cc;
                    self.idx.extend([k - 4, k - 3, k, k - 3, k + 1, k]);
                }
            }
        }
        c.mesh(self.white.tex, &self.verts, &self.idx);
    }

    /// `alpha` liefert die Deckkraft begrenzt auf 0..1 (NaN = 1).
    pub fn alpha(&self) -> f32 {
        if self.alpha.is_nan() {
            return 1.0;
        }
        self.alpha.clamp(0.0, 1.0) as f32
    }
}

/// `parse_color` liest #rrggbb; alles andere ergibt Schwarz.
pub fn parse_color(hex: &str) -> [u8; 4] {
    let b = hex.as_bytes();
    let mut rgb = [0u8; 3];
    if b.len() == 7 && b[0] == b'#' {
        for (i, v) in rgb.iter_mut().enumerate() {
            *v = hex_byte(b[1 + 2 * i]) << 4 | hex_byte(b[2 + 2 * i]);
        }
    }
    [rgb[0], rgb[1], rgb[2], 255]
}

fn hex_byte(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}
