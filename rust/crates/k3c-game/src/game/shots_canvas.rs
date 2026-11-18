//! Die Zeichenfläche des Aufnahme-Modus: Go liest die Pixel des Bildschirms nach `Draw` (`screen.ReadPixels`), die
//! Plattform kann in Rust kein Bild auslesen. Darum zeichnet der Aufnahme-Modus gegen den Software-Rasterer
//! (`still::raster::SoftCanvas`). Der Rasterer rechnet ein Bild und verbraucht dabei seine Texturen; das Spiel lädt
//! sie aber einmal (Grafik der Level) und hält ihre Handles über viele Bilder. `ShotCanvas` merkt darum jeden Upload
//! (Textur und Schrift), vergibt eigene Handles und lädt sie in den Rasterer des nächsten Bilds neu hoch (`next_image`).

use k3c_platform::{Affine, Blend, Canvas, Filter, FontId, Rect, RiverUniforms, TexId, Vertex};

use crate::still::Image;
use crate::still::raster::SoftCanvas;

/// Eine hochgeladene Textur (vormultiplizierte RGBA-Bytes, wie das Spiel sie liefert).
struct Upload {
    rgba: Vec<u8>,
    w: u32,
    h: u32,
    filter: Filter,
}

/// Handle, das nirgends gilt (freigegebene Textur): der Rasterer überspringt solche Aufrufe.
const GONE: TexId = TexId(u32::MAX);

/// Ein `Canvas`, der mehrere Bilder nacheinander rastert und die Uploads zwischen den Bildern behält.
pub struct ShotCanvas {
    inner: SoftCanvas,
    w: u32,
    h: u32,
    /// Uploads nach eigenem Handle (`None` = freigegeben)
    texs: Vec<Option<Upload>>,
    /// eigenes Handle → Handle im Rasterer des laufenden Bilds
    tex_ids: Vec<TexId>,
    /// Schriften (TTF-Bytes) nach eigenem Handle
    fonts: Vec<Vec<u8>>,
    font_ids: Vec<FontId>,
}

impl ShotCanvas {
    pub fn new(w: u32, h: u32) -> ShotCanvas {
        let (w, h) = (w.max(1), h.max(1));
        ShotCanvas {
            inner: SoftCanvas::new(w, h),
            w,
            h,
            texs: Vec::new(),
            tex_ids: Vec::new(),
            fonts: Vec::new(),
            font_ids: Vec::new(),
        }
    }

    /// Rastert das laufende Bild und beginnt ein neues mit denselben Texturen und Schriften.
    pub fn next_image(&mut self) -> Image {
        let done = std::mem::replace(&mut self.inner, SoftCanvas::new(self.w, self.h));
        let img = done.finish();
        self.tex_ids.clear();
        for t in &self.texs {
            let id = match t {
                Some(u) => self.inner.texture_with(&u.rgba, u.w, u.h, u.filter),
                None => GONE,
            };
            self.tex_ids.push(id);
        }
        self.font_ids.clear();
        for ttf in &self.fonts {
            // Die Bytes haben schon einmal geladen; scheitert es jetzt, bleibt das Handle leer (Text fehlt).
            self.font_ids.push(self.inner.font(ttf).unwrap_or(FontId(u32::MAX)));
        }
        img
    }

    fn tex(&self, t: TexId) -> TexId {
        self.tex_ids.get(t.0 as usize).copied().unwrap_or(GONE)
    }

    fn font_of(&self, f: FontId) -> FontId {
        self.font_ids.get(f.0 as usize).copied().unwrap_or(FontId(u32::MAX))
    }
}

impl Canvas for ShotCanvas {
    fn size(&self) -> (f32, f32) {
        self.inner.size()
    }
    fn layer(&mut self, z: u32) {
        self.inner.layer(z);
    }
    fn blend(&mut self, mode: Blend) {
        self.inner.blend(mode);
    }
    fn clip(&mut self, area: Option<Rect>) {
        self.inner.clip(area);
    }
    fn texture_with(&mut self, rgba: &[u8], w: u32, h: u32, f: Filter) -> TexId {
        let inner = self.inner.texture_with(rgba, w, h, f);
        let own = TexId(u32::try_from(self.texs.len()).unwrap_or(u32::MAX));
        self.texs.push(Some(Upload {
            rgba: rgba.to_vec(),
            w,
            h,
            filter: f,
        }));
        self.tex_ids.push(inner);
        own
    }
    fn free_texture(&mut self, t: TexId) {
        if let Some(u) = self.texs.get_mut(t.0 as usize) {
            *u = None; // gibt die Bytes frei; die nächsten Bilder laden sie nicht mehr hoch
        }
        let inner = self.tex(t);
        self.inner.free_texture(inner);
        if let Some(id) = self.tex_ids.get_mut(t.0 as usize) {
            *id = GONE;
        }
    }
    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]) {
        let t = self.tex(t);
        self.inner.sprite(t, src, dst, tint);
    }
    fn sprite_tf(&mut self, t: TexId, src: Rect, tf: Affine, tint: [f32; 4]) {
        let t = self.tex(t);
        self.inner.sprite_tf(t, src, tf, tint);
    }
    fn mesh(&mut self, t: TexId, verts: &[Vertex], idx: &[u16]) {
        let t = self.tex(t);
        self.inner.mesh(t, verts, idx);
    }
    fn rect(&mut self, dst: Rect, color: [f32; 4]) {
        self.inner.rect(dst, color);
    }
    fn stroke_rect(&mut self, r: Rect, width: f32, color: [f32; 4]) {
        self.inner.stroke_rect(r, width, color);
    }
    fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]) {
        self.inner.fill_circle(cx, cy, r, color);
    }
    fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, width: f32, color: [f32; 4]) {
        self.inner.stroke_circle(cx, cy, r, width, color);
    }
    fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: [f32; 4]) {
        self.inner.stroke_line(x0, y0, x1, y1, width, color);
    }
    fn fill_path(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        self.inner.fill_path(points, color);
    }
    fn river(&mut self, dst: Rect, u: &RiverUniforms) -> bool {
        self.inner.river(dst, u)
    }
    fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        self.inner.text(s, x, y, px, color);
    }
    fn font(&mut self, ttf: &[u8]) -> Result<FontId, String> {
        let inner = self.inner.font(ttf)?;
        let own = FontId(u32::try_from(self.fonts.len()).unwrap_or(u32::MAX));
        self.fonts.push(ttf.to_vec());
        self.font_ids.push(inner);
        Ok(own)
    }
    fn text_font(&mut self, f: FontId, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        let f = self.font_of(f);
        self.inner.text_font(f, s, x, y, px, color);
    }
    fn text_width(&self, f: FontId, s: &str, px: f32) -> f32 {
        self.inner.text_width(self.font_of(f), s, px)
    }
    fn text_height(&self, f: FontId, px: f32) -> f32 {
        self.inner.text_height(self.font_of(f), px)
    }
}
