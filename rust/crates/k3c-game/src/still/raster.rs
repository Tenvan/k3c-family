//! Software-Rasterer für das Standbild: eine `Canvas`, die in ein Bild im Speicher zeichnet.
//!
//! Aufbau: Die Aufrufe sammelt der `Frame` der Plattform (`k3c_platform::Frame`), derselbe Weg wie vor dem GPU-Backend:
//! Rechteck-Instanzen (`sprite`, `sprite_tf`, `rect`, Schrift-Glyphen), Dreiecke (`mesh`, Formen) und Texturen samt
//! Glyphen-Atlas. Dieser Rasterer rechnet die Befehle wie `shader.wgsl` nach: Pixelmitte, Abtasten linear oder nächster
//! Nachbar mit Rand-Klemmung, Tönung (vormultipliziert) mal Textur, Mischen "vormultipliziert über" in 8 Bit.
//!
//! Abweichung von der Plattform: Der `Frame` ordnet je (Ebene, Ausschnitt) nach Aufruf (B-1058). Das Standbild
//! zeichnet zusätzlich jeden Aufruf auf eine eigene Ebene (`layer` des Aufrufers wird ignoriert), damit Ausschnitte
//! die Reihenfolge gegenüber Go nicht verändern.

use k3c_platform::{Affine, Canvas, Filter, FontId, Frame, Kind, Rect, TexId, Vertex};

/// Bild im Speicher: RGBA8, gerade Farben, Zeilen oben nach unten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pix: Vec<u8>,
}

impl Image {
    /// Opakes Schwarz wie `screen.Fill(clearColor)` in Go.
    pub fn black(width: u32, height: u32) -> Self {
        let mut pix = vec![0u8; width as usize * height as usize * 4];
        pix.as_chunks_mut::<4>().0.iter_mut().for_each(|p| p[3] = 255);
        Self { width, height, pix }
    }
}

/// Eine hochgeladene Textur: RGBA8 vormultipliziert.
struct Tex {
    w: u32,
    h: u32,
    filter: Filter,
    rgba: Vec<u8>,
}

/// SoftCanvas sammelt die Aufrufe; `finish` rastert sie.
pub struct SoftCanvas {
    frame: Frame,
    seq: u32,
    w: u32,
    h: u32,
}

impl SoftCanvas {
    pub fn new(w: u32, h: u32) -> Self {
        let mut frame = Frame::new();
        frame.begin();
        Self { frame, seq: 0, w, h }
    }

    /// Jeder zeichnende Aufruf bekommt die nächste Ebene (Aufrufreihenfolge).
    fn next(&mut self) {
        self.frame.layer(self.seq);
        self.seq += 1;
    }

    /// Rastert alle gesammelten Aufrufe in ein schwarzes Bild.
    pub fn finish(mut self) -> Image {
        self.frame.finish();
        let mut texs: Vec<Option<Tex>> = Vec::new();
        for p in self.frame.take_pending() {
            let i = p.id.0 as usize;
            if texs.len() <= i {
                texs.resize_with(i + 1, || None);
            }
            texs[i] = Some(Tex {
                w: p.w,
                h: p.h,
                filter: p.filter,
                rgba: p.rgba,
            });
        }
        if let Some((id, ups)) = self.frame.take_atlas_updates() {
            for u in ups {
                if let Some(Some(t)) = texs
                    .get_mut(id.0 as usize)
                    .filter(|t| t.as_ref().is_some_and(|t| u.x + u.w <= t.w && u.y + u.h <= t.h))
                {
                    for row in 0..u.h {
                        let d = ((u.y + row) * t.w + u.x) as usize * 4;
                        let s = (row * u.w) as usize * 4;
                        t.rgba[d..d + u.w as usize * 4].copy_from_slice(&u.rgba[s..s + u.w as usize * 4]);
                    }
                }
            }
        }
        let mut img = Image::black(self.w, self.h);
        let quads: Vec<_> = self.frame.instances().copied().collect();
        for run in self.frame.runs() {
            let clip = clip_box(run.clip, self.w, self.h);
            let tex = run.tex.and_then(|t| texs.get(t.0 as usize)).and_then(Option::as_ref);
            if run.tex.is_some() && tex.is_none() {
                continue;
            }
            let (first, count) = (run.first as usize, run.count as usize);
            match run.kind {
                Kind::Quads => {
                    for q in &quads[first..first + count] {
                        draw_quad(&mut img, clip, tex, q);
                    }
                }
                Kind::Tris => {
                    let idx = &self.frame.indices()[first..first + count];
                    for t in idx.as_chunks::<3>().0 {
                        let v = self.frame.vertices();
                        draw_tri(
                            &mut img,
                            clip,
                            tex,
                            [v[t[0] as usize], v[t[1] as usize], v[t[2] as usize]],
                        );
                    }
                }
            }
        }
        img
    }
}

/// Ausschnitt als Pixelgrenzen (x0, y0, x1, y1) im Bild.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn clip_box(clip: Option<Rect>, w: u32, h: u32) -> (i64, i64, i64, i64) {
    let (mut x0, mut y0, mut x1, mut y1) = (0, 0, i64::from(w), i64::from(h));
    if let Some(r) = clip {
        // Scissor: ganze Pixel, Mitte im Rechteck zählt (wie die Rasterung)
        x0 = x0.max((r.x - 0.5).ceil() as i64);
        y0 = y0.max((r.y - 0.5).ceil() as i64);
        x1 = x1.min((r.x + r.w - 0.5).ceil() as i64);
        y1 = y1.min((r.y + r.h - 0.5).ceil() as i64);
    }
    (x0, y0, x1, y1)
}

/// Tastet die Textur bei (u, v) als Anteil ab; Ergebnis vormultipliziert, 0 … 1.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn sample(t: &Tex, u: f32, v: f32) -> [f32; 4] {
    let (w, h) = (t.w as i64, t.h as i64);
    let texel = |x: i64, y: i64| -> [f32; 4] {
        let i = ((y.clamp(0, h - 1) * w + x.clamp(0, w - 1)) * 4) as usize;
        [0, 1, 2, 3].map(|k| f32::from(t.rgba[i + k]) / 255.0)
    };
    let (fx, fy) = (u * t.w as f32, v * t.h as f32);
    if t.filter == Filter::Nearest {
        return texel(fx.floor() as i64, fy.floor() as i64);
    }
    let (fx, fy) = (fx - 0.5, fy - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ax, ay) = (fx - x0, fy - y0);
    let (x0, y0) = (x0 as i64, y0 as i64);
    let (p00, p10, p01, p11) = (
        texel(x0, y0),
        texel(x0 + 1, y0),
        texel(x0, y0 + 1),
        texel(x0 + 1, y0 + 1),
    );
    [0, 1, 2, 3].map(|k| {
        let top = p00[k] + (p10[k] - p00[k]) * ax;
        let bot = p01[k] + (p11[k] - p01[k]) * ax;
        top + (bot - top) * ay
    })
}

/// Mischt die vormultiplizierte Quelle über das Ziel (`PREMULTIPLIED_ALPHA_BLENDING`, 8 Bit).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn blend(img: &mut Image, x: i64, y: i64, s: [f32; 4]) {
    let i = (y as usize * img.width as usize + x as usize) * 4;
    for k in 0..4 {
        let d = f32::from(img.pix[i + k]);
        img.pix[i + k] = (s[k] * 255.0 + d * (1.0 - s[3])).round().clamp(0.0, 255.0) as u8;
    }
}

fn mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

fn premul_tint(c: [f32; 4]) -> [f32; 4] {
    [c[0] * c[3], c[1] * c[3], c[2] * c[3], c[3]]
}

/// Rechteck-Instanz: Einheitsquadrat q → (m·q + t); Vollfarbe, wenn die Quelle (uv.zw) leer ist.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn draw_quad(img: &mut Image, clip: (i64, i64, i64, i64), tex: Option<&Tex>, q: &k3c_platform::Inst) {
    let (a, b, c, d) = (q.m[0], q.m[1], q.m[2], q.m[3]);
    let det = a * d - b * c;
    if det == 0.0 || !det.is_finite() {
        return;
    }
    let xs = [0.0, a, b, a + b].map(|v| v + q.t[0]);
    let ys = [0.0, c, d, c + d].map(|v| v + q.t[1]);
    let lo = |v: &[f32; 4]| v.iter().copied().fold(f32::INFINITY, f32::min);
    let hi = |v: &[f32; 4]| v.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let x0 = clip.0.max((lo(&xs) - 0.5).ceil() as i64);
    let x1 = clip.2.min((hi(&xs) - 0.5).ceil() as i64 + 1);
    let y0 = clip.1.max((lo(&ys) - 0.5).ceil() as i64);
    let y1 = clip.3.min((hi(&ys) - 0.5).ceil() as i64 + 1);
    let solid = q.uv[2] == 0.0 && q.uv[3] == 0.0;
    let tint = premul_tint(q.c);
    for y in y0..y1 {
        for x in x0..x1 {
            let (dx, dy) = (x as f32 + 0.5 - q.t[0], y as f32 + 0.5 - q.t[1]);
            let (qx, qy) = ((d * dx - b * dy) / det, (a * dy - c * dx) / det);
            if !(0.0..1.0).contains(&qx) || !(0.0..1.0).contains(&qy) {
                continue;
            }
            let s = if solid {
                tint
            } else if let Some(t) = tex {
                mul(sample(t, q.uv[0] + qx * q.uv[2], q.uv[1] + qy * q.uv[3]), tint)
            } else {
                continue;
            };
            blend(img, x, y, s);
        }
    }
}

/// Dreieck mit Ecken (Ziel in Pixeln, Quelle als Anteil, Farbe als Faktor); ohne Textur gilt Weiß (Formen).
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn draw_tri(img: &mut Image, clip: (i64, i64, i64, i64), tex: Option<&Tex>, v: [Vertex; 3]) {
    let [p0, p1, p2] = v.map(|v| v.pos);
    let area = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]);
    if area == 0.0 || !area.is_finite() {
        return;
    }
    let lo = |f: fn(&[f32; 2]) -> f32| [p0, p1, p2].iter().map(f).fold(f32::INFINITY, f32::min);
    let hi = |f: fn(&[f32; 2]) -> f32| [p0, p1, p2].iter().map(f).fold(f32::NEG_INFINITY, f32::max);
    let x0 = clip.0.max((lo(|p| p[0]) - 0.5).ceil() as i64);
    let x1 = clip.2.min((hi(|p| p[0]) - 0.5).ceil() as i64 + 1);
    let y0 = clip.1.max((lo(|p| p[1]) - 0.5).ceil() as i64);
    let y1 = clip.3.min((hi(|p| p[1]) - 0.5).ceil() as i64 + 1);
    for y in y0..y1 {
        for x in x0..x1 {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = ((p1[0] - px) * (p2[1] - py) - (p2[0] - px) * (p1[1] - py)) / area;
            let w1 = ((p2[0] - px) * (p0[1] - py) - (p0[0] - px) * (p2[1] - py)) / area;
            let w2 = 1.0 - w0 - w1;
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }
            let lerp = |f: &dyn Fn(&Vertex) -> f32| w0 * f(&v[0]) + w1 * f(&v[1]) + w2 * f(&v[2]);
            let col = [0, 1, 2, 3].map(|k| lerp(&|v: &Vertex| v.color[k]));
            let base = match tex {
                Some(t) => sample(t, lerp(&|v: &Vertex| v.uv[0]), lerp(&|v: &Vertex| v.uv[1])),
                None => [1.0; 4],
            };
            blend(img, x, y, mul(base, col));
        }
    }
}

impl Canvas for SoftCanvas {
    fn size(&self) -> (f32, f32) {
        (self.w as f32, self.h as f32)
    }
    fn layer(&mut self, _z: u32) {}
    fn clip(&mut self, area: Option<Rect>) {
        self.frame.clip(area);
    }
    fn texture_with(&mut self, rgba: &[u8], w: u32, h: u32, f: Filter) -> TexId {
        self.frame.texture_with(rgba, w, h, f)
    }
    /// Alle Texturen bleiben bis zum Ende des Standbilds liegen.
    fn free_texture(&mut self, _t: TexId) {}
    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]) {
        self.next();
        self.frame.sprite(t, src, dst, tint);
    }
    fn sprite_tf(&mut self, t: TexId, src: Rect, tf: Affine, tint: [f32; 4]) {
        self.next();
        self.frame.sprite_tf(t, src, tf, tint);
    }
    fn mesh(&mut self, t: TexId, verts: &[Vertex], idx: &[u16]) {
        self.next();
        self.frame.mesh(t, verts, idx);
    }
    fn rect(&mut self, dst: Rect, color: [f32; 4]) {
        self.next();
        self.frame.rect(dst, color);
    }
    fn stroke_rect(&mut self, r: Rect, width: f32, color: [f32; 4]) {
        self.next();
        self.frame.stroke_rect(r, width, color);
    }
    fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]) {
        self.next();
        self.frame.fill_circle(cx, cy, r, color);
    }
    fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, width: f32, color: [f32; 4]) {
        self.next();
        self.frame.stroke_circle(cx, cy, r, width, color);
    }
    fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: [f32; 4]) {
        self.next();
        self.frame.stroke_line(x0, y0, x1, y1, width, color);
    }
    fn fill_path(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        self.next();
        self.frame.fill_path(points, color);
    }
    fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        self.next();
        self.frame.text(s, x, y, px, color);
    }
    fn font(&mut self, ttf: &[u8]) -> Result<FontId, String> {
        self.frame.font(ttf)
    }
    fn text_font(&mut self, f: FontId, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        self.next();
        self.frame.text_font(f, s, x, y, px, color);
    }
    fn text_width(&self, f: FontId, s: &str, px: f32) -> f32 {
        self.frame.text_width(f, s, px)
    }
}
