//! Sammlung je Bild: Rechteck-Instanzen (mit Matrix) und Dreiecke, sortiert nach Ebene, Ausschnitt, Textur und
//! Aufruf; ein Draw-Call je Bündel.
//!
//! Reine CPU-Seite ohne Grafikabhängigkeit; das Backend liest [`Frame::runs`], [`Frame::write_instances`],
//! [`Frame::write_vertices`] und [`Frame::indices`]. Vollfarbe (Rechtecke, Formen, Pixel-Schrift) trägt den
//! Schlüssel [`SOLID`] und landet innerhalb von Ebene und Ausschnitt nach allen Texturen (also obenauf).
//! `impl Canvas for Frame` steht in `frame_canvas.rs`.

use crate::shape::Tris;
use crate::ttf::{Atlas, AtlasUpdate, Fonts};
use crate::{Filter, Rect, TexId};

/// Größe einer Instanz in Byte (16 × f32).
pub const INST_BYTES: usize = 64;
/// Größe einer Ecke in Byte (8 × f32).
pub const VERT_BYTES: usize = 32;
/// Sortierschlüssel der Vollfarbe: hinter allen Texturen.
pub const SOLID: u32 = u32::MAX;

/// Eine Rechteck-Instanz: Einheitsquadrat → `m` (a, b, c, d wie GeoM) plus Verschiebung `t.xy`, Farbe `c`,
/// Quelle als Anteil der Textur `uv` (u, v, w, h). `uv.z == 0` und `uv.w == 0` bedeutet Vollfarbe.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Inst {
    pub m: [f32; 4],
    pub t: [f32; 4],
    pub c: [f32; 4],
    pub uv: [f32; 4],
}

impl Inst {
    /// Achsenparalleles Rechteck `dst`.
    pub fn rect(dst: Rect, c: [f32; 4], uv: [f32; 4]) -> Self {
        Self {
            m: [dst.w, 0.0, 0.0, dst.h],
            t: [dst.x, dst.y, 0.0, 0.0],
            c,
            uv,
        }
    }
}

/// Ecke eines Dreiecksnetzes (wie `ebiten.Vertex`): Ziel in logischen Einheiten, Quelle in Textur-Pixeln,
/// Farbe als Faktor auf die Textur.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

/// Art eines Draw-Calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// Instanzen `first..first+count` aus [`Frame::instances`].
    Quads,
    /// Indizes `first..first+count` aus [`Frame::indices`].
    Tris,
}

/// Ein Draw-Call: Ebene, Ausschnitt, Textur (`None` = Vollfarbe) und Bereich.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Run {
    pub layer: u32,
    /// Ausschnitt (Scissor) in logischen Einheiten; `None` = ganzes Bild.
    pub clip: Option<Rect>,
    pub tex: Option<TexId>,
    pub kind: Kind,
    pub first: u32,
    pub count: u32,
}

/// Noch nicht hochgeladene Textur.
pub struct Pending {
    pub id: TexId,
    pub w: u32,
    pub h: u32,
    pub filter: Filter,
    pub rgba: Vec<u8>,
}

#[derive(Clone, Copy)]
enum Body {
    Quad(Inst),
    /// Bereich in `raw_idx` (Indizes schon auf `verts` bezogen).
    Tris(u32, u32),
}

#[derive(Clone, Copy)]
struct Item {
    layer: u32,
    clip: u32,
    tex: u32,
    seq: u32,
    body: Body,
}

/// Sammelt die Zeichenaufrufe eines Bildes; implementiert [`crate::Canvas`].
#[derive(Default)]
pub struct Frame {
    items: Vec<Item>,
    runs: Vec<Run>,
    quads: Vec<Inst>,
    verts: Vec<Vertex>,
    raw_idx: Vec<u32>,
    idx: Vec<u32>,
    pub(crate) sizes: Vec<(u32, u32)>,
    pub(crate) pending: Vec<Pending>,
    freed: Vec<TexId>,
    /// Ausschnitte dieses Bildes; `Item::clip` = Index + 1, 0 = ohne.
    pub(crate) clips: Vec<Rect>,
    pub(crate) layer: u32,
    pub(crate) clip: u32,
    pub(crate) tris: Tris,
    pub(crate) fonts: Fonts,
    pub(crate) atlas: Atlas,
    /// Textur des Glyphen-Atlas (beim ersten TTF-Text angelegt).
    pub(crate) atlas_tex: Option<TexId>,
    pub(crate) atlas_updates: Vec<AtlasUpdate>,
}

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }

    /// Beginnt ein Bild; behält die Puffer, damit im Leerlauf nichts alloziert wird.
    pub fn begin(&mut self) {
        self.items.clear();
        self.runs.clear();
        self.verts.clear();
        self.raw_idx.clear();
        self.clips.clear();
        self.layer = 0;
        self.clip = 0;
        self.atlas.begin_frame();
    }

    /// Sortiert nach Ebene, Ausschnitt, Textur, Aufruf und bildet die Draw-Calls; Instanzen und Indizes liegen
    /// danach je Bündel zusammenhängend.
    pub fn finish(&mut self) {
        self.items.sort_unstable_by_key(|i| (i.layer, i.clip, i.tex, i.seq));
        self.quads.clear();
        self.idx.clear();
        for it in &self.items {
            let (kind, first, count) = match it.body {
                Body::Quad(inst) => {
                    self.quads.push(inst);
                    (Kind::Quads, self.quads.len() as u32 - 1, 1)
                }
                Body::Tris(a, n) => {
                    let first = self.idx.len() as u32;
                    self.idx.extend_from_slice(&self.raw_idx[a as usize..(a + n) as usize]);
                    (Kind::Tris, first, n)
                }
            };
            let clip = it.clip.checked_sub(1).map(|c| self.clips[c as usize]);
            let tex = (it.tex != SOLID).then_some(TexId(it.tex));
            match self.runs.last_mut() {
                Some(r) if (r.layer, r.clip, r.tex, r.kind) == (it.layer, clip, tex, kind) => r.count += count,
                _ => self.runs.push(Run {
                    layer: it.layer,
                    clip,
                    tex,
                    kind,
                    first,
                    count,
                }),
            }
        }
    }

    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    pub fn instance_count(&self) -> usize {
        self.quads.len()
    }

    /// Instanzen in Zeichenreihenfolge (nach `finish`).
    pub fn instances(&self) -> impl Iterator<Item = &Inst> {
        self.quads.iter()
    }

    /// Ecken der Dreiecke (Quelle schon als Anteil der Textur).
    pub fn vertices(&self) -> &[Vertex] {
        &self.verts
    }

    /// Indizes der Dreiecke in Zeichenreihenfolge (nach `finish`).
    pub fn indices(&self) -> &[u32] {
        &self.idx
    }

    /// Schreibt die Instanzen als Bytes (`INST_BYTES` je Stück) in `out`.
    pub fn write_instances(&self, out: &mut [u8]) {
        let (chunks, _) = out.as_chunks_mut::<INST_BYTES>();
        for (chunk, i) in chunks.iter_mut().zip(&self.quads) {
            put_floats(chunk, i.m.iter().chain(&i.t).chain(&i.c).chain(&i.uv));
        }
    }

    /// Schreibt die Ecken als Bytes (`VERT_BYTES` je Stück) in `out`.
    pub fn write_vertices(&self, out: &mut [u8]) {
        let (chunks, _) = out.as_chunks_mut::<VERT_BYTES>();
        for (chunk, v) in chunks.iter_mut().zip(&self.verts) {
            put_floats(chunk, v.pos.iter().chain(&v.uv).chain(&v.color));
        }
    }

    /// Texturen, die das Backend noch hochladen muss.
    pub fn take_pending(&mut self) -> std::vec::Drain<'_, Pending> {
        self.pending.drain(..)
    }

    /// Texturen, die das Backend am Bildende freigeben soll.
    pub fn take_freed(&mut self) -> std::vec::Drain<'_, TexId> {
        self.freed.drain(..)
    }

    /// Neue Glyphen für die Atlas-Textur (nach dem Hochladen der Textur einzuspielen).
    pub fn take_atlas_updates(&mut self) -> Option<(TexId, std::vec::Drain<'_, AtlasUpdate>)> {
        Some((self.atlas_tex?, self.atlas_updates.drain(..)))
    }

    /// Größe der Textur; `None`, wenn unbekannt oder freigegeben.
    pub(crate) fn tex_size(&self, t: TexId) -> Option<(f32, f32)> {
        match self.sizes.get(t.0 as usize) {
            Some(&(w, h)) if w > 0 => Some((w as f32, h as f32)),
            _ => None,
        }
    }

    pub(crate) fn free(&mut self, t: TexId) {
        if self.tex_size(t).is_some() {
            self.sizes[t.0 as usize] = (0, 0);
            self.pending.retain(|p| p.id != t);
            self.freed.push(t);
        }
    }

    pub(crate) fn push_quad(&mut self, tex: u32, inst: Inst) {
        self.push(tex, Body::Quad(inst));
    }

    /// Dreiecke aus `verts` (Quelle in Textur-Pixeln, wird hier auf Anteile umgerechnet) und `idx`.
    pub(crate) fn push_mesh(&mut self, tex: u32, size: (f32, f32), verts: &[Vertex], idx: &[u16]) {
        let base = self.verts.len() as u32;
        self.verts.extend(verts.iter().map(|v| Vertex {
            uv: [v.uv[0] / size.0, v.uv[1] / size.1],
            color: premul(v.color),
            ..*v
        }));
        let first = self.raw_idx.len() as u32;
        let n = verts.len() as u32;
        self.raw_idx
            .extend(idx.iter().map(|&i| base + u32::from(i).min(n.saturating_sub(1))));
        self.push(tex, Body::Tris(first, idx.len() as u32 / 3 * 3));
    }

    /// Dreiecke aus dem Form-Puffer `tris` in Vollfarbe (vormultipliziert); leert den Puffer.
    pub(crate) fn push_shape(&mut self, color: [f32; 4]) {
        let base = self.verts.len() as u32;
        let color = premul(color);
        self.verts.extend(self.tris.pos.iter().map(|&pos| Vertex {
            pos,
            uv: [0.0; 2],
            color,
        }));
        let first = self.raw_idx.len() as u32;
        self.raw_idx.extend(self.tris.idx.iter().map(|&i| base + i));
        let n = self.tris.idx.len() as u32;
        self.tris.clear();
        if n > 0 {
            self.push(SOLID, Body::Tris(first, n));
        }
    }

    fn push(&mut self, tex: u32, body: Body) {
        let seq = self.items.len() as u32;
        self.items.push(Item {
            layer: self.layer,
            clip: self.clip,
            tex,
            seq,
            body,
        });
    }
}

/// Gerade Farbe → vormultipliziert (wie `ColorScaleModeStraightAlpha` bei Ebitengine).
fn premul(c: [f32; 4]) -> [f32; 4] {
    [c[0] * c[3], c[1] * c[3], c[2] * c[3], c[3]]
}

fn put_floats<'a, const N: usize>(chunk: &mut [u8; N], floats: impl Iterator<Item = &'a f32>) {
    let (cells, _) = chunk.as_chunks_mut::<4>();
    for (cell, v) in cells.iter_mut().zip(floats) {
        *cell = v.to_ne_bytes();
    }
}

#[cfg(test)]
#[path = "batch_tests.rs"]
mod tests;
