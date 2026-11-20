//! Port von `native/k3c-game/props/paint.go`: Zeichnen der Zeichenliste. Formen kommen aus wenigen weißen Vorlagen
//! (Bildpunkt, Scheibe, Ring, Dreieck), eingefärbt über die Tönung: so entsteht je Bild nichts Neues und die Plattform
//! stapelt die Aufrufe.
//!
//! Abweichung von Go: Statt `*ebiten.Image` zeichnet `Painter` über `&mut dyn Canvas`; die Vorlagen entstehen beim
//! ersten Zeichnen als RGBA-Pixel (vormultipliziert, geglättet) und werden hochgeladen. `Rect` und `Frame` zeichnen
//! mit `Canvas::rect` (ein einfarbiges Rechteck sieht wie der gestreckte Bildpunkt aus), alle übrigen Formen mit den
//! Vorlagen, damit Kreis, Ring, Dreieck und Linie (stumpfe Enden) bildgleich zu Go bleiben.

use std::collections::BTreeMap;

use k3c_platform::{Affine, Canvas, Rect, TexId};

use super::plan::{Item, Shape};
use crate::actors::catalog::UNIT_PX;
use crate::art::{Font, Library};
use crate::view::cell::Cell;

/// `STENCIL_PX`: Kantenlänge der Vorlagen für Scheibe, Ring und Dreieck.
pub const STENCIL_PX: u32 = 64;

/// `TEXT_OUTLINE`: Versätze der schwarzen Kontur um den Text (siteView.ts › TEXT: stroke 4).
pub const TEXT_OUTLINE: [[f64; 2]; 4] = [[-1.0, 0.0], [1.0, 0.0], [0.0, -1.0], [0.0, 1.0]];

/// `Stencils` sind die hochgeladenen weißen Vorlagen.
#[derive(Debug, Clone, Copy)]
struct Stencils {
    pixel: TexId,
    disc: TexId,
    ring: TexId,
    tri: TexId,
}

/// `Painter` hält die Vorlagen (beim ersten Zeichnen angelegt) und die gemessenen Textbreiten.
#[derive(Debug, Default)]
pub struct Painter {
    st: Option<Stencils>,
    widths: BTreeMap<String, Vec<(u64, f64)>>,
}

/// `coverage` macht aus einem Abstand zur Kante (Pixel, innen positiv) die Deckung eines Pixels (geglättet).
fn coverage(d: f64) -> f64 {
    (d + 0.5).clamp(0.0, 1.0)
}

/// `stencil_pixels` rechnet eine weiße Vorlage (vormultipliziert) aus der Deckung `cov(x, y)` je Pixelmitte.
fn stencil_pixels(cov: impl Fn(f64, f64) -> f64) -> Vec<u8> {
    let n = STENCIL_PX as usize;
    let mut out = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let a = (cov(x as f64 + 0.5, y as f64 + 0.5) * 255.0).round() as u8;
            out.extend_from_slice(&[a, a, a, a]);
        }
    }
    out
}

/// `disc_cov`: Scheibe mit Radius 32 um die Mitte; Ring mit Breite Radius/4 innen am Rand (`inner` = 24).
fn disc_cov(inner: f64) -> impl Fn(f64, f64) -> f64 {
    move |x, y| {
        let d = (x - 32.0).hypot(y - 32.0);
        coverage(32.0 - d).min(coverage(d - inner))
    }
}

/// `tri_cov`: Dreieck mit der Spitze oben mittig und der Basis unten über die ganze Breite.
fn tri_cov(x: f64, y: f64) -> f64 {
    let n = f64::from(STENCIL_PX);
    let slope = (n / 2.0).hypot(n); // Länge der Schräge
    let d = ((x - n / 2.0).abs() * n - (n / 2.0) * y) / slope; // Abstand zur Schräge (außen positiv)
    coverage(-d).min(coverage(n - y))
}

impl Painter {
    /// `stencils` legt die Vorlagen einmal an.
    fn stencils(&mut self, dst: &mut dyn Canvas) -> Stencils {
        if let Some(s) = self.st {
            return s;
        }
        let s = Stencils {
            pixel: dst.texture(&[255; 4], 1, 1),
            disc: dst.texture(&stencil_pixels(disc_cov(-1.0e9)), STENCIL_PX, STENCIL_PX),
            ring: dst.texture(&stencil_pixels(disc_cov(24.0)), STENCIL_PX, STENCIL_PX),
            tri: dst.texture(&stencil_pixels(tri_cov), STENCIL_PX, STENCIL_PX),
        };
        self.st = Some(s);
        s
    }

    /// `paint` zeichnet die Einträge in Reihenfolge in `dst` (Zelle `c`); ohne `lib` zeichnen Bilder und Texte nichts.
    pub fn paint(&mut self, dst: &mut dyn Canvas, items: &[Item], mut lib: Option<&mut Library>, c: &Cell, clock: f64) {
        let st = self.stencils(dst);
        for it in items {
            match it.shape {
                Shape::Sprite => sprite(dst, it, lib.as_deref(), c, clock),
                Shape::Tiles => tiles(dst, it, lib.as_deref(), c),
                Shape::Text => {
                    if let Some(l) = lib.as_deref_mut() {
                        self.text(dst, it, &mut l.font, c);
                    }
                }
                _ => form(dst, &st, it, c),
            }
        }
    }

    // PERF(B-1078): Ein Textwert mit neuer Größe misst und merkt sich die Breite (1 Allokation, Schlüssel und Vec); die
    // Tabelle wächst nur mit neuen Texten und Größen, nicht je Bild.
    /// `text` zeichnet den Text mittig über y (Unterkante), mit schwarzer Kontur; Breiten werden je Text einmal
    /// gemessen.
    fn text(&mut self, dst: &mut dyn Canvas, it: &Item, font: &mut Font, c: &Cell) {
        let size = (it.size * c.camera.scale).max(8.0);
        let bits = size.to_bits();
        let cached = self
            .widths
            .get(it.text.as_str())
            .and_then(|v| v.iter().find(|e| e.0 == bits))
            .map(|e| e.1);
        let w = cached.unwrap_or_else(|| {
            let w = font.measure(&it.text, size).0;
            self.widths.entry(it.text.clone()).or_default().push((bits, w));
            w
        });
        let x = c.camera.screen_x(it.x) - w / 2.0;
        let y = c.screen_y(it.y) - size * 1.2;
        let d = (size / 16.0).max(1.0);
        for o in TEXT_OUTLINE {
            font.draw(dst, &it.text, x + o[0] * d, y + o[1] * d, size, [0, 0, 0, 255]);
        }
        font.draw(dst, &it.text, x, y, size, it.clr);
    }
}

/// `straight` rechnet die vormultiplizierte Farbe `clr` (Go: `color.RGBA`) mit Deckkraft `alpha` (0 = voll) in die
/// gerade Farbe, die `Canvas` für Rechtecke und Tönungen erwartet (Go: `ColorScale.ScaleWithColor` und `ScaleAlpha`).
fn straight(clr: [u8; 4], alpha: f32) -> [f32; 4] {
    let a0 = f32::from(clr[3]) / 255.0;
    if a0 <= 0.0 {
        return [0.0; 4];
    }
    let a = if alpha > 0.0 { alpha } else { 1.0 };
    let [r, g, b, _] = clr.map(|v| (f32::from(v) / 255.0 / a0).min(1.0));
    [r, g, b, a0 * a]
}

/// `sprite` zeichnet ein Bild mit Fuß bei (x, y), Startframe `phase` und Abdunklung.
#[allow(clippy::cast_possible_truncation)]
fn sprite(dst: &mut dyn Canvas, it: &Item, lib: Option<&Library>, c: &Cell, clock: f64) {
    let Some(s) = lib.and_then(|l| l.sheet(it.look.asset, it.look.anim)) else {
        return;
    };
    let Some(f) = s.frame(it.phase + (clock * s.fps) as i64) else {
        return;
    };
    let k = s.units_per_px / UNIT_PX * c.camera.scale;
    let ky = if it.sy > 0.0 { k * it.sy } else { k };
    let x0 = c.camera.screen_x(it.x) - s.origin_x * s.fw as f64 * k;
    let y0 = c.screen_y(it.y) - s.origin_y * s.fh as f64 * ky;
    let mut g = Affine::IDENTITY;
    g.scale(k as f32, ky as f32).translate(x0 as f32, y0 as f32);
    let sh = if it.shade > 0.0 { 1.0 - it.shade } else { 1.0 };
    let a = if it.alpha > 0.0 { it.alpha } else { 1.0 };
    dst.sprite_tf(f.tex, f.src, g, [sh, sh, sh, a]); // gerade Tönung (Go: ColorScale (sh·a, …, a))
}

/// `tiles` kachelt das Bild von x bis x2 (Hintergrundmauer); die letzte Kachel liegt bündig am rechten Ende.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn tiles(dst: &mut dyn Canvas, it: &Item, lib: Option<&Library>, c: &Cell) {
    let Some(s) = lib.and_then(|l| l.sheet(it.look.asset, it.look.anim)) else {
        return;
    };
    if s.frame(0).is_none() || s.fw == 0 {
        return;
    }
    let u = s.units_per_px / UNIT_PX;
    // Mit `kachelung` setzt die Kachelung die sichtbaren Streifen aneinander (ohne den transparenten Rand des Rahmens),
    // `lead` ist der Rand links davor (B-947); sonst ganze Rahmen.
    let (lead, tw) = match s.tile {
        Some((a, b)) => (a * u, (b - a) * u),
        None => (0.0, s.fw as f64 * u),
    };
    let k = u * c.camera.scale;
    let y0 = c.screen_y(it.y) - s.origin_y * s.fh as f64 * k;
    let n = ((it.x2 - it.x) / tw).ceil() as i64;
    for i in 0..n {
        let tx = (it.x + i as f64 * tw).min(it.x2 - tw);
        if !c.camera.visible(tx + tw / 2.0, tw / 2.0) {
            continue;
        }
        let mut g = Affine::IDENTITY;
        g.scale(k as f32, k as f32)
            .translate(c.camera.screen_x(tx - lead) as f32, y0 as f32);
        s.draw(dst, 0, g);
    }
}

/// `stamp` zeichnet die Vorlage `tex` gestreckt auf das Rechteck (x, y, w, h) in Fenster-Pixeln; ein negatives `h`
/// (Zacken, Tropfsteine) spiegelt sie wie `GeoM.Scale` in Go.
fn stamp(dst: &mut dyn Canvas, tex: TexId, size: u32, r: [f64; 4], tint: [f32; 4]) {
    let n = size as f32;
    let mut g = Affine::IDENTITY;
    g.scale(r[2] as f32 / n, r[3] as f32 / n)
        .translate(r[0] as f32, r[1] as f32);
    dst.sprite_tf(tex, Rect::new(0.0, 0.0, n, n), g, tint);
}

/// `solid` zeichnet ein einfarbiges Rechteck (x, y, w, h); negative Maße wie bei `stamp` gespiegelt, also normiert.
fn solid(dst: &mut dyn Canvas, r: [f64; 4], color: [f32; 4]) {
    let (x, w) = if r[2] < 0.0 { (r[0] + r[2], -r[2]) } else { (r[0], r[2]) };
    let (y, h) = if r[3] < 0.0 { (r[1] + r[3], -r[3]) } else { (r[1], r[3]) };
    dst.rect(Rect::new(x as f32, y as f32, w as f32, h as f32), color);
}

/// `form` zeichnet Rechteck, Rahmen, Kreis, Ring, Linie oder Dreieck.
fn form(dst: &mut dyn Canvas, st: &Stencils, it: &Item, c: &Cell) {
    let s = c.camera.scale;
    let (x, y) = (c.camera.screen_x(it.x), c.screen_y(it.y));
    let (w, h, r) = (it.w * s, it.h * s, it.w * s);
    let tint = straight(it.clr, it.alpha);
    match it.shape {
        Shape::Rect => solid(dst, [x - w / 2.0, y - h, w, h], straight(it.clr, it.alpha)),
        Shape::Frame => frame(
            dst,
            [x - w / 2.0, y - h, w, h],
            (it.size * s).max(1.0),
            straight(it.clr, it.alpha),
        ),
        Shape::Circle => stamp(dst, st.disc, STENCIL_PX, [x - r, y - r, 2.0 * r, 2.0 * r], tint),
        Shape::Ring => stamp(dst, st.ring, STENCIL_PX, [x - r, y - r, 2.0 * r, 2.0 * r], tint),
        Shape::Tri => stamp(dst, st.tri, STENCIL_PX, [x - w / 2.0, y - h, w, h], tint),
        Shape::Line => {
            let (x1, y1) = (c.camera.screen_x(it.x2), c.screen_y(it.y2));
            line(dst, st.pixel, [x, y, x1, y1], (it.size * s).max(1.0), tint);
        }
        Shape::Sprite | Shape::Tiles | Shape::Text => {}
    }
}

/// `frame`: vier Kanten der Breite `lw` innen am Rechteck (x, y, w, h).
fn frame(dst: &mut dyn Canvas, r: [f64; 4], lw: f64, color: [f32; 4]) {
    let [x, y, w, h] = r;
    solid(dst, [x, y, w, lw], color);
    solid(dst, [x, y + h - lw, w, lw], color);
    solid(dst, [x, y, lw, h], color);
    solid(dst, [x + w - lw, y, lw, h], color);
}

/// `line`: gedrehter Bildpunkt von (x0, y0) nach (x1, y1) mit Breite `lw`, stumpfe Enden.
fn line(dst: &mut dyn Canvas, pixel: TexId, p: [f64; 4], lw: f64, tint: [f32; 4]) {
    let (dx, dy) = (p[2] - p[0], p[3] - p[1]);
    let mut g = Affine::IDENTITY;
    g.translate(0.0, -0.5)
        .scale(dx.hypot(dy) as f32, lw as f32)
        .rotate(dy.atan2(dx) as f32)
        .translate(p[0] as f32, p[1] as f32);
    dst.sprite_tf(pixel, Rect::new(0.0, 0.0, 1.0, 1.0), g, tint);
}
