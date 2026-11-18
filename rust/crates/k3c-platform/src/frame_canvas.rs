//! `impl Canvas for Frame`: Aufrufe → Instanzen und Dreiecke (siehe `batch.rs`).

use crate::batch::{Inst, Pending, SOLID, Vertex};
use crate::ttf::{ATLAS, quant_px};
use crate::{Affine, Blend, Canvas, Filter, FontId, Frame, LOGICAL, Rect, RiverUniforms, TexId, font};

impl Canvas for Frame {
    fn size(&self) -> (f32, f32) {
        LOGICAL
    }

    fn layer(&mut self, z: u32) {
        self.layer = z;
    }

    fn blend(&mut self, mode: Blend) {
        self.blend = mode;
    }

    fn clip(&mut self, area: Option<Rect>) {
        let Some(r) = area else {
            self.clip = 0;
            return;
        };
        // Gleicher Ausschnitt wie ein früherer: denselben Index nehmen, damit die Bündel zusammenfallen.
        let i = match self.clips.iter().position(|c| *c == r) {
            Some(i) => i,
            None => {
                self.clips.push(r);
                self.clips.len() - 1
            }
        };
        self.clip = i as u32 + 1;
    }

    fn texture_with(&mut self, rgba: &[u8], w: u32, h: u32, filter: Filter) -> TexId {
        assert_eq!(rgba.len(), w as usize * h as usize * 4, "RGBA8 erwartet");
        let id = TexId(self.sizes.len() as u32);
        self.sizes.push((w.max(1), h.max(1)));
        self.pending.push(Pending {
            id,
            w,
            h,
            filter,
            rgba: rgba.to_vec(),
        });
        id
    }

    fn free_texture(&mut self, t: TexId) {
        self.free(t);
    }

    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]) {
        let Some((w, h)) = self.tex_size(t) else {
            return;
        };
        let uv = [src.x / w, src.y / h, src.w / w, src.h / h];
        self.push_quad(t.0, Inst::rect(dst, tint, uv));
    }

    fn sprite_tf(&mut self, t: TexId, src: Rect, tf: Affine, tint: [f32; 4]) {
        let Some((w, h)) = self.tex_size(t) else {
            return;
        };
        // Einheitsquadrat → Quellgröße → GeoM (wie DrawImage auf ein SubImage).
        let inst = Inst {
            m: [tf.a * src.w, tf.b * src.h, tf.c * src.w, tf.d * src.h],
            t: [tf.tx, tf.ty, 0.0, 0.0],
            c: tint,
            uv: [src.x / w, src.y / h, src.w / w, src.h / h],
        };
        self.push_quad(t.0, inst);
    }

    fn river(&mut self, dst: Rect, u: &RiverUniforms) -> bool {
        if dst.w > 0.0 && dst.h > 0.0 {
            self.push_river(dst, u);
        }
        true
    }

    fn mesh(&mut self, t: TexId, verts: &[Vertex], idx: &[u16]) {
        let Some(size) = self.tex_size(t) else {
            return;
        };
        if verts.is_empty() || idx.len() < 3 {
            return;
        }
        self.push_mesh(t.0, size, verts, idx);
    }

    fn rect(&mut self, dst: Rect, color: [f32; 4]) {
        self.push_quad(SOLID, Inst::rect(dst, color, [0.0; 4]));
    }

    fn stroke_rect(&mut self, r: Rect, width: f32, color: [f32; 4]) {
        let h = width / 2.0;
        let (x0, y0) = (r.x - h, r.y - h);
        self.rect(Rect::new(x0, y0, r.w + width, width), color);
        self.rect(Rect::new(x0, r.y + r.h - h, r.w + width, width), color);
        if r.h > width {
            let side = r.h - width;
            self.rect(Rect::new(x0, r.y + h, width, side), color);
            self.rect(Rect::new(r.x + r.w - h, r.y + h, width, side), color);
        }
    }

    fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]) {
        self.tris.circle(cx, cy, r);
        self.push_shape(color);
    }

    fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, width: f32, color: [f32; 4]) {
        self.tris.ring(cx, cy, r, width);
        self.push_shape(color);
    }

    fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: [f32; 4]) {
        let (dx, dy) = (x1 - x0, y1 - y0);
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 || !len.is_finite() {
            return;
        }
        // Quadrat: x läuft die Linie entlang, y quer mit Breite `width`, mittig.
        let (nx, ny) = (-dy / len * width, dx / len * width);
        let inst = Inst {
            m: [dx, nx, dy, ny],
            t: [x0 - nx / 2.0, y0 - ny / 2.0, 0.0, 0.0],
            c: color,
            uv: [0.0; 4],
        };
        self.push_quad(SOLID, inst);
    }

    fn fill_path(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        self.tris.polygon(points);
        self.push_shape(color);
    }

    fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        let k = px / font::GLYPH_H as f32;
        let mut cx = x;
        for ch in s.chars() {
            font::runs(ch, |gx, gy, n| {
                let r = Rect::new(cx + gx as f32 * k, y + gy as f32 * k, n as f32 * k, k);
                self.rect(r, color);
            });
            cx += font::ADVANCE as f32 * k;
        }
    }

    fn font(&mut self, ttf: &[u8]) -> Result<FontId, String> {
        self.fonts.load(ttf)
    }

    fn text_font(&mut self, f: FontId, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        let tex = self.atlas_texture();
        self.atlas
            .layout(&self.fonts, (f, quant_px(px)), s, (x, y), &mut self.atlas_updates);
        let k = 1.0 / ATLAS as f32;
        for i in 0..self.atlas.quads.len() {
            let (src, dst) = self.atlas.quads[i];
            let uv = [src.x * k, src.y * k, src.w * k, src.h * k];
            self.push_quad(tex.0, Inst::rect(dst, color, uv));
        }
    }

    fn text_width(&self, f: FontId, s: &str, px: f32) -> f32 {
        self.fonts.width(f, s, px)
    }

    fn text_height(&self, f: FontId, px: f32) -> f32 {
        self.fonts.height(f, px)
    }
}

impl Frame {
    /// Legt die Atlas-Textur beim ersten Bedarf an (leer; Glyphen kommen als Teil-Updates).
    fn atlas_texture(&mut self) -> TexId {
        if let Some(t) = self.atlas_tex {
            return t;
        }
        let id = TexId(self.sizes.len() as u32);
        self.sizes.push((ATLAS, ATLAS));
        self.pending.push(Pending {
            id,
            w: ATLAS,
            h: ATLAS,
            filter: Filter::Linear,
            rgba: vec![0; (ATLAS * ATLAS * 4) as usize],
        });
        self.atlas_tex = Some(id);
        id
    }
}
