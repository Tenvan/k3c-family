//! Port von `native/k3c-game/menu/backdrop.go`: Papier-Hintergrund der Menüs (B-812, Vorbild
//! `src/scenes/paperBackdrop.ts` ohne Sterne und Wald): Nachthimmel, zwei geschnittene Papierbahnen mit heller Kante und
//! ein Papiermond mit Schlagschatten. Die Lage rechnen `Backdrop::of` und `shapes` rein, der Painter zeichnet die Formen
//! (erst Rechtecke, dann Scheiben).
//!
//! Abweichung: Scheiben zeichnet `Canvas::fill_circle` (Go: 64-px-Scheibenbild mit Linear-Filter); der Rand kann im
//! Bildvergleich (RU9.6) abweichen.

use k3c_platform::{Canvas, Filter};

use super::draw::{Painter, tint};
use super::layout::{COL_NIGHT, Geom, MAT_LINEN, Rgba, SHADOW_LAYERS, fade, rgb};
use crate::view::cell::Rectangle;

/// Farbe des Mond-Schattens (aus `papier.lage.farbe` von `public/grafik/stil.paper.json`).
pub const COL_SHADOW: Rgba = rgb(0x001a_0f14);
const COL_BAND1: Rgba = rgb(0x001b_2550);
const COL_BAND2: Rgba = rgb(0x003b_3a6b);

/// Der Schnitt der Oberkante einer Bahn: Versatz je Abschnitt in px bei 1080 (die zweite Bahn liest die Folge versetzt,
/// damit die Kanten nicht parallel laufen).
pub const BAND_STEPS: [f64; 12] = [0.0, -6.0, 4.0, -3.0, 7.0, -5.0, 2.0, -8.0, 5.0, -2.0, 6.0, -4.0];

/// Zahl der Formen: Himmel, je Bahn-Abschnitt Fläche und Kante, Mondschatten, Mond, Ausschnitt.
pub const BACKDROP_SHAPES: usize = 1 + 2 * 2 * BAND_STEPS.len() + SHADOW_LAYERS.len() + 2;

/// Die helle Kante der Bahnen.
fn col_edge() -> Rgba {
    fade([255, 255, 255, 255], 0.14)
}

/// Band ist eine Papierbahn: Oberkante (ohne Schnitt) in px und Farbe; sie reicht bis zum unteren Rand.
#[derive(Clone, Copy, Debug)]
pub struct Band {
    pub top: f64,
    pub clr: Rgba,
    /// Versatz in `BAND_STEPS`
    pub shift: usize,
}

/// Backdrop ist die Lage des Hintergrunds in einem Fenster.
#[derive(Clone, Copy, Debug)]
pub struct Backdrop {
    pub win: Rectangle,
    pub k: f64,
    pub bands: [Band; 2],
    pub moon_x: f64,
    pub moon_y: f64,
    pub moon_r: f64,
}

/// Shape ist eine Form des Hintergrunds: Rechteck (Ecke x, y, Größe w × h) oder Scheibe (Mittelpunkt x, y, Radius w).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Shape {
    pub disc: bool,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub clr: Rgba,
}

impl Shape {
    /// Der Filter der Form: Scheiben glatt (Linear), Rechtecke scharf (Nearest).
    pub fn filter(&self) -> Filter {
        if self.disc { Filter::Linear } else { Filter::Nearest }
    }
}

impl Backdrop {
    /// Rechnet die Lage für das Fenster der Seite `g` (Browser: 1920 × 1080, Bahnen ab 430 und 690 px, Mond bei 1660/150
    /// mit Radius 56).
    #[allow(clippy::cast_precision_loss)]
    pub fn of(g: &Geom) -> Backdrop {
        let (w, top) = (g.win.dx() as f64, g.win.min.y as f64);
        Backdrop {
            win: g.win,
            k: g.k,
            bands: [
                Band {
                    top: top + 430.0 * g.k,
                    clr: COL_BAND1,
                    shift: 0,
                },
                Band {
                    top: top + 690.0 * g.k,
                    clr: COL_BAND2,
                    shift: 5,
                },
            ],
            moon_x: g.win.min.x as f64 + w * 1660.0 / 1920.0,
            moon_y: top + 150.0 * g.k,
            moon_r: 56.0 * g.k,
        }
    }

    /// Liefert Abschnitt `j` einer Bahn: linke Kante, Breite und Oberkante `(x, w, y)`.
    #[allow(clippy::cast_precision_loss)]
    pub fn segment(&self, bd: &Band, j: usize) -> (f64, f64, f64) {
        let w = self.win.dx() as f64 / BAND_STEPS.len() as f64;
        (
            self.win.min.x as f64 + j as f64 * w,
            w,
            bd.top + BAND_STEPS[(j + bd.shift) % BAND_STEPS.len()] * self.k,
        )
    }

    /// Füllt die Formen in Zeichenreihenfolge: erst alle Rechtecke, dann alle Scheiben. So wechselt der Filter je Bild
    /// nur einmal (B-820); Mond und Bahnen überlappen nicht.
    #[allow(clippy::cast_precision_loss)]
    pub fn shapes(&self) -> [Shape; BACKDROP_SHAPES] {
        let mut out = [Shape::default(); BACKDROP_SHAPES];
        let r = self.win;
        out[0] = Shape {
            x: r.min.x as f64,
            y: r.min.y as f64,
            w: r.dx() as f64,
            h: r.dy() as f64,
            clr: COL_NIGHT,
            ..Shape::default()
        };
        let mut n = 1;
        for bd in &self.bands {
            for j in 0..BAND_STEPS.len() {
                let (x, w, y) = self.segment(bd, j);
                out[n] = Shape {
                    x,
                    y,
                    w: w + 1.0,
                    h: r.max.y as f64 - y,
                    clr: bd.clr,
                    ..Shape::default()
                };
                out[n + 1] = Shape {
                    x,
                    y,
                    w: w + 1.0,
                    h: 3.0 * self.k,
                    clr: col_edge(),
                    ..Shape::default()
                };
                n += 2;
            }
        }
        self.moon_shapes(&mut out, n);
        out
    }

    /// Mondschatten, Mond und Ausschnitt (alles Scheiben) ab Index `n`.
    fn moon_shapes(&self, out: &mut [Shape; BACKDROP_SHAPES], mut n: usize) {
        for (off, alpha) in SHADOW_LAYERS {
            let d = off * self.k;
            out[n] = Shape {
                disc: true,
                x: self.moon_x + d * 0.6,
                y: self.moon_y + d,
                w: self.moon_r,
                clr: fade(COL_SHADOW, alpha),
                ..Shape::default()
            };
            n += 1;
        }
        out[n] = Shape {
            disc: true,
            x: self.moon_x,
            y: self.moon_y,
            w: self.moon_r,
            clr: MAT_LINEN.licht,
            ..Shape::default()
        };
        out[n + 1] = Shape {
            disc: true,
            x: self.moon_x - 22.0 * self.k,
            y: self.moon_y - 12.0 * self.k,
            w: 50.0 * self.k,
            clr: COL_NIGHT,
            ..Shape::default()
        };
    }
}

impl Painter {
    /// Zeichnet den Hintergrund ins Fenster `g`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn backdrop(&mut self, dst: &mut dyn Canvas, g: &Geom) {
        for s in &Backdrop::of(g).shapes() {
            if s.disc {
                dst.fill_circle(s.x as f32, s.y as f32, s.w as f32, tint(s.clr));
            } else {
                self.rect(dst, s.x, s.y, s.w, s.h, s.clr);
            }
        }
    }
}
