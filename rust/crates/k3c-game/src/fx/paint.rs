//! Port von `native/k3c-game/fx/paint.go`.
//!
//! Zeichnen: Scheibe, Ring, Dreieck und Bildpunkt als weiße Vorlagen, eingefärbt über die Tönung (je Bild entsteht
//! nichts Neues). Orte in Welt-px des Browsers (32 px je Unit) werden mit dem Maßstab der Kamera umgerechnet, Texte
//! erscheinen in fester Größe (Fensterhöhe / 1080).
//!
//! Abweichung von Go: Die Vorlagen sind 64 × 64 große Texturen der Zeichenfläche (`Canvas::texture`), `stamp` ist
//! `Canvas::sprite`; der Bildpunkt ist eine ganz weiße Vorlage, aus der `stamp` nur das Innere ohne Rand abtastet (Go
//! nimmt den mittleren Punkt eines 3 × 3-Bildes). `rgba` liefert wie Go vormultiplizierte Bytes, `tint` macht daraus
//! die gerade Farbe, die `Canvas` erwartet. Die Münze nimmt `Canvas::sprite_tf` mit Deckkraft statt `Sheet::draw`
//! (das zeichnet ungetönt).

use std::collections::HashMap;

use k3c_platform::{Affine, Canvas, Rect, TexId};
use k3c_sim::state::World;

use super::effects::{Effect, Kind, UNIT_PX};
use super::looks::piece_at;
use super::status::urgency;
use crate::art::{Font, OUTLINE4, OutlineCache, Sheet};
use crate::view::cell::Cell;

/// Kantenlänge der Vorlagen in Bildpunkten
pub const STENCIL_PX: u32 = 64;

/// MAX_WIDTHS: so viele gemessene Texte bleiben gemerkt, danach beginnt die Liste neu.
pub const MAX_WIDTHS: usize = 256;

/// Farben der Geschosse (worldRenderer.ts: eigene 0xffffff, Gegner 0xff4d6d); die eigenen bleiben der helle Strich von NG1.
pub const OWN_PROJECTILE: u32 = 0xfafaf0;
pub const ENEMY_PROJECTILE: u32 = 0xff4d6d;

/// Farbe des Warnkreises vor dem Flächenschlag
const WARN_COLOR: u32 = 0xe63946;

/// `Stencils` sind die weißen Vorlagen auf der Zeichenfläche (Go: `pixel`, `disc`, `ring`, `tri`).
#[derive(Clone, Copy, Debug)]
pub struct Stencils {
    pub pixel: TexId,
    pub disc: TexId,
    pub ring: TexId,
    pub tri: TexId,
}

/// `Painter` hält Vorlagen und gemessene Texte (begrenzt; Schadenszahlen wiederholen sich).
#[derive(Debug, Default)]
pub struct Painter {
    pub st: Option<Stencils>,
    widths: HashMap<(String, u64), (f64, f64)>,
    /// aufsteigende Texte mit Kontur; Sweep je Takt in Listen
    pub outline: OutlineCache,
}

/// `make` legt eine weiße, vormultiplizierte Vorlage an; `inside` sagt für einen Punkt (Bildpunkte), ob er zur Form
/// gehört (4 × 4 Unterabtastung für den glatten Rand).
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn make(canvas: &mut dyn Canvas, inside: impl Fn(f64, f64) -> bool) -> TexId {
    let n = STENCIL_PX as usize;
    let mut rgba = vec![0u8; n * n * 4];
    for y in 0..n {
        for x in 0..n {
            let mut hit = 0u32;
            for sy in 0..4 {
                for sx in 0..4 {
                    let px = x as f64 + (f64::from(sx) + 0.5) / 4.0;
                    let py = y as f64 + (f64::from(sy) + 0.5) / 4.0;
                    if inside(px, py) {
                        hit += 1;
                    }
                }
            }
            let a = (hit * 255 / 16) as u8;
            let i = (y * n + x) * 4;
            rgba[i..i + 4].copy_from_slice(&[a, a, a, a]);
        }
    }
    canvas.texture(&rgba, STENCIL_PX, STENCIL_PX)
}

/// `in_triangle`: Punkt (x, y) im Dreieck (Spitze oben, Grundlinie unten) der Vorlage.
fn in_triangle(x: f64, y: f64) -> bool {
    let r = f64::from(STENCIL_PX) / 2.0;
    let edge = |ax: f64, ay: f64, bx: f64, by: f64| (bx - ax) * (y - ay) - (by - ay) * (x - ax);
    let s = f64::from(STENCIL_PX);
    let (d1, d2, d3) = (edge(r, 0.0, s, s), edge(s, s, 0.0, s), edge(0.0, s, r, 0.0));
    !((d1 < 0.0 || d2 < 0.0 || d3 < 0.0) && (d1 > 0.0 || d2 > 0.0 || d3 > 0.0))
}

impl Painter {
    /// `stencils` legt die Vorlagen beim ersten Aufruf an und liefert sie.
    pub fn stencils(&mut self, canvas: &mut dyn Canvas) -> Stencils {
        if let Some(s) = self.st {
            return s;
        }
        let r = f64::from(STENCIL_PX) / 2.0;
        let s = Stencils {
            pixel: make(canvas, |_, _| true),
            disc: make(canvas, |x, y| (x - r) * (x - r) + (y - r) * (y - r) <= r * r),
            ring: make(canvas, |x, y| {
                let d = (x - r) * (x - r) + (y - r) * (y - r);
                d <= r * r && d >= (r - 6.0) * (r - 6.0)
            }),
            tri: make(canvas, in_triangle),
        };
        self.st = Some(s);
        s
    }
}

/// `rgba` ist 0xRRGGBB mit Deckkraft a (0..1), vormultipliziert (Bytes wie Go: abgeschnitten).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn rgba(hex: u32, a: f64) -> [u8; 4] {
    let a = k3c_det::fmax(0.0, k3c_det::fmin(1.0, a));
    let ch = |shift: u32| (f64::from(hex >> shift & 0xff) * a) as u8;
    [ch(16), ch(8), ch(0), (255.0 * a) as u8]
}

/// `tint` macht aus vormultiplizierten Bytes die gerade Farbe der `Canvas`.
pub fn tint(c: [u8; 4]) -> [f32; 4] {
    if c[3] == 0 {
        return [0.0; 4];
    }
    let d = f32::from(c[3]);
    [f32::from(c[0]) / d, f32::from(c[1]) / d, f32::from(c[2]) / d, d / 255.0]
}

#[allow(clippy::cast_possible_truncation)]
fn area(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect::new(x as f32, y as f32, w as f32, h as f32)
}

/// Quelle einer Vorlage: ganz, der Bildpunkt ohne den Rand (kein Ausbluten beim Filtern).
fn full() -> Rect {
    Rect::new(0.0, 0.0, STENCIL_PX as f32, STENCIL_PX as f32)
}

fn inner() -> Rect {
    Rect::new(1.0, 1.0, STENCIL_PX as f32 - 2.0, STENCIL_PX as f32 - 2.0)
}

impl Stencils {
    /// `stamp` zeichnet eine Vorlage in das Rechteck (x, y, w, h) mit der Farbe clr.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn stamp(&self, dst: &mut dyn Canvas, tex: TexId, x: f64, y: f64, w: f64, h: f64, clr: [u8; 4]) {
        if w <= 0.0 || h <= 0.0 || clr[3] == 0 {
            return;
        }
        let src = if tex == self.pixel { inner() } else { full() };
        dst.sprite(tex, src, area(x, y, w, h), tint(clr));
    }

    /// `pixel_at` füllt ein Rechteck mit der Farbe clr (Go: `stamp(pixel, …)`).
    pub fn pixel_at(&self, dst: &mut dyn Canvas, x: f64, y: f64, w: f64, h: f64, clr: [u8; 4]) {
        self.stamp(dst, self.pixel, x, y, w, h, clr);
    }

    pub fn circle(&self, dst: &mut dyn Canvas, x: f64, y: f64, r: f64, clr: [u8; 4]) {
        self.stamp(dst, self.disc, x - r, y - r, 2.0 * r, 2.0 * r, clr);
    }

    pub fn ring_at(&self, dst: &mut dyn Canvas, x: f64, y: f64, r: f64, clr: [u8; 4]) {
        self.stamp(dst, self.ring, x - r, y - r, 2.0 * r, 2.0 * r, clr);
    }

    /// `line`: gedrehter Bildpunkt von (x0, y0) nach (x1, y1) mit Breite lw.
    #[allow(clippy::cast_possible_truncation, clippy::too_many_arguments)]
    pub fn line(&self, dst: &mut dyn Canvas, x0: f64, y0: f64, x1: f64, y1: f64, lw: f64, clr: [u8; 4]) {
        let (dx, dy) = (x1 - x0, y1 - y0);
        let n = inner().w;
        let mut g = Affine::IDENTITY;
        g.scale(1.0 / n, 1.0 / n)
            .translate(0.0, -0.5)
            .scale(dx.hypot(dy) as f32, lw as f32)
            .rotate(dy.atan2(dx) as f32)
            .translate(x0 as f32, y0 as f32);
        dst.sprite_tf(self.pixel, inner(), g, tint(clr));
    }
}

/// `at` ist der Fenster-Punkt bei Welt-x x (Units) und Höhe y (Welt-px über dem Boden); s rechnet Welt-px in Fenster-px.
pub fn at(c: &Cell, x: f64, y: f64) -> (f64, f64, f64) {
    let s = c.camera.scale / UNIT_PX;
    (c.camera.screen_x(x), c.screen_y(y / UNIT_PX), s)
}

/// `projectile_color` liefert die Farbe eines Geschosses nach Seite: Gegner rot, alles andere unverändert.
pub fn projectile_color(team: &str) -> u32 {
    if team == "enemy" {
        return ENEMY_PROJECTILE;
    }
    OWN_PROJECTILE
}

impl Painter {
    /// `effects` zeichnet Ring und Partikel bzw. aufsteigenden Text jedes Effekts zur Zeit now.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn effects(
        &mut self,
        dst: &mut dyn Canvas,
        list: &[Effect],
        mut font: Option<&mut Font>,
        coin: Option<&Sheet>,
        c: &Cell,
        now: f64,
        k: f64,
    ) {
        let Some(st) = self.st else { return };
        for e in list {
            if !c.camera.visible(e.x, 4.0) {
                continue;
            }
            let p = (now - e.start) / e.look.duration;
            if !e.text.is_empty() {
                self.rising(dst, e, font.as_deref_mut(), c, p, k);
                continue;
            }
            let (x, y, s) = at(c, e.x, e.y);
            for j in 0..=e.look.particles {
                let q = piece_at(e.look, j, p);
                if e.kind == Kind::CoinLoss
                    && j > 0
                    && let Some(sh) = coin
                {
                    // Münzschauer: Münz-Sprite statt Scheibe (B-940)
                    coin_at(dst, sh, j, x + q.dx * s, y + q.dy * s, q.radius * s, q.alpha);
                    continue;
                }
                st.circle(
                    dst,
                    x + q.dx * s,
                    y + q.dy * s,
                    q.radius * s,
                    rgba(e.look.color, q.alpha),
                );
            }
        }
    }

    /// `rising` zeichnet den Text eines Effekts mittig, steigt auf und blendet aus (noTarget 24 px, Schadenszahl 28 px).
    fn rising(&mut self, dst: &mut dyn Canvas, e: &Effect, font: Option<&mut Font>, c: &Cell, p: f64, k: f64) {
        let Some(f) = font else { return };
        let (mut size, mut rise, mut clr) = (24.0 * k, 24.0, 0xffffff_u32);
        if e.kind == Kind::Damage {
            (size, rise, clr) = (28.0 * k, 40.0, 0xfbf236);
            if e.target {
                clr = 0xff4d4d;
            }
        }
        let (w, h) = self.measure(f, &e.text, size);
        let (x, y, s) = at(c, e.x, e.y);
        let p = p.clamp(0.0, 1.0);
        let (tx, ty) = (x - w / 2.0, y - h / 2.0 - rise * s * p);
        // ausblenden: ganzes Bild
        self.outline
            .draw(dst, Some(f), &e.text, tx, ty, size, rgba(clr, 1.0), &OUTLINE4, 1.0 - p);
    }

    /// `measure` misst einen Text, gemerkt je Text und Größe.
    pub fn measure(&mut self, f: &Font, s: &str, size: f64) -> (f64, f64) {
        let key = (s.to_string(), size.to_bits());
        if let Some(m) = self.widths.get(&key) {
            return *m;
        }
        if self.widths.len() >= MAX_WIDTHS {
            self.widths.clear();
        }
        let m = f.measure(s, size);
        self.widths.insert(key, m);
        m
    }

    /// Zahl der gemerkten Textbreiten (Tests).
    pub fn measured(&self) -> usize {
        self.widths.len()
    }

    /// `warn` zeichnet die Warnkreise vor dem Flächenschlag um den Boden (kampfView.ts › drawWarn).
    pub fn warn(&self, dst: &mut dyn Canvas, w: &World, c: &Cell) {
        let Some(st) = self.st else { return };
        for e in &w.enemies {
            let Some(z) = &e.warn else { continue };
            if !c.camera.visible(z.x, z.r) {
                continue;
            }
            let (x, y, s) = at(c, z.x, 0.0);
            let (r, u) = (z.r * UNIT_PX * s, urgency(z.r#in));
            st.circle(dst, x, y, r, rgba(WARN_COLOR, 0.15 + 0.35 * u));
            st.ring_at(dst, x, y, r, rgba(WARN_COLOR, 0.6 + 0.4 * u));
        }
    }

    /// `projectiles`: Pfeile und Geschosse als kurze Striche, Gegner rot (Platzhalter-Form wie NG1).
    pub fn projectiles(&self, dst: &mut dyn Canvas, w: &World, c: &Cell) {
        let Some(st) = self.st else { return };
        let k = &c.camera;
        for p in &w.projectiles {
            if k.visible(p.x, 0.3) {
                st.pixel_at(
                    dst,
                    k.screen_x(p.x - 0.3),
                    c.screen_y(1.2),
                    0.6 * k.scale,
                    0.1 * k.scale,
                    rgba(projectile_color(&p.team), 1.0),
                );
            }
        }
    }
}

/// `coin_at` zeichnet Bild i des Münz-Sheets als Kreis um (x, y) mit Radius r (Fenster-px) und Deckkraft alpha.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn coin_at(dst: &mut dyn Canvas, sh: &Sheet, i: i64, x: f64, y: f64, r: f64, alpha: f64) {
    let Some(fr) = sh.frame(i) else { return };
    if sh.fw <= 0 || r <= 0.0 {
        return;
    }
    let k = 2.0 * r / sh.fw as f64;
    let mut g = Affine::IDENTITY;
    g.scale(k as f32, k as f32)
        .translate((x - r) as f32, (y - sh.fh as f64 * k / 2.0) as f32);
    dst.sprite_tf(fr.tex, fr.src, g, [1.0, 1.0, 1.0, alpha.clamp(0.0, 1.0) as f32]);
}
