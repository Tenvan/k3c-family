//! Hilfen der Tests: ein `Canvas`, der Bild- und Netz-Aufrufe in Reihenfolge festhält.

use k3c_platform::{Affine, Canvas, Filter, FontId, NullCanvas, Rect, TexId, Vertex};

/// Ein festgehaltener Zeichen-Aufruf.
#[derive(Debug, Clone)]
pub enum Call {
    Sprite {
        tex: TexId,
        src: Rect,
        tf: Affine,
        tint: [f32; 4],
    },
    Mesh {
        tex: TexId,
        verts: Vec<Vertex>,
        idx: Vec<u16>,
    },
}

/// RecCanvas hält `sprite_tf` und `mesh` fest; alles andere geht an einen `NullCanvas` (zählt nur).
#[derive(Default)]
pub struct RecCanvas {
    pub null: NullCanvas,
    pub calls: Vec<Call>,
}

impl Canvas for RecCanvas {
    fn size(&self) -> (f32, f32) {
        self.null.size()
    }

    fn texture_with(&mut self, rgba: &[u8], w: u32, h: u32, f: Filter) -> TexId {
        self.null.texture_with(rgba, w, h, f)
    }

    fn free_texture(&mut self, t: TexId) {
        self.null.free_texture(t);
    }

    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]) {
        self.null.sprite(t, src, dst, tint);
    }

    fn sprite_tf(&mut self, tex: TexId, src: Rect, tf: Affine, tint: [f32; 4]) {
        self.calls.push(Call::Sprite { tex, src, tf, tint });
    }

    fn mesh(&mut self, tex: TexId, verts: &[Vertex], idx: &[u16]) {
        self.calls.push(Call::Mesh {
            tex,
            verts: verts.to_vec(),
            idx: idx.to_vec(),
        });
    }

    fn rect(&mut self, dst: Rect, color: [f32; 4]) {
        self.null.rect(dst, color);
    }

    fn stroke_rect(&mut self, r: Rect, width: f32, color: [f32; 4]) {
        self.null.stroke_rect(r, width, color);
    }

    fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]) {
        self.null.fill_circle(cx, cy, r, color);
    }

    fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, width: f32, color: [f32; 4]) {
        self.null.stroke_circle(cx, cy, r, width, color);
    }

    fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: [f32; 4]) {
        self.null.stroke_line(x0, y0, x1, y1, width, color);
    }

    fn fill_path(&mut self, points: &[[f32; 2]], color: [f32; 4]) {
        self.null.fill_path(points, color);
    }

    fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        self.null.text(s, x, y, px, color);
    }

    fn font(&mut self, ttf: &[u8]) -> Result<FontId, String> {
        self.null.font(ttf)
    }

    fn text_font(&mut self, font: FontId, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]) {
        self.null.text_font(font, s, x, y, px, color);
    }

    fn text_width(&self, font: FontId, s: &str, px: f32) -> f32 {
        self.null.text_width(font, s, px)
    }
}
