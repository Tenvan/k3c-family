//! Sprite-Sammlung je Bild: nach Textur sortiert, ein Instanz-Puffer, ein Draw-Call je Textur.
//!
//! Reine CPU-Seite ohne Grafikabhängigkeit; das Backend liest [`Frame::runs`] und
//! [`Frame::write_instances`]. Vollfarbflächen und Schrift tragen den Schlüssel [`SOLID`] und
//! landen deshalb nach allen Sprites (also obenauf).

use crate::{Canvas, LOGICAL, Rect, TexId, font};

/// Größe einer Instanz in Byte (12 × f32).
pub const INST_BYTES: usize = 48;
/// Sortierschlüssel der Vollfarbflächen: hinter allen Texturen.
pub const SOLID: u32 = u32::MAX;

/// Eine Instanz: Ziel (x, y, w, h), Farbe, Quelle als Anteil der Textur (u, v, w, h).
/// `uv.z == 0` und `uv.w == 0` bedeutet Vollfarbe.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Inst {
    pub r: [f32; 4],
    pub c: [f32; 4],
    pub uv: [f32; 4],
}

#[derive(Clone, Copy)]
struct Item {
    tex: u32,
    seq: u32,
    inst: Inst,
}

/// Ein Draw-Call: Textur (`None` = Vollfarbe) und Bereich in der sortierten Instanz-Liste.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Run {
    pub tex: Option<TexId>,
    pub first: u32,
    pub count: u32,
}

/// Noch nicht hochgeladene Textur.
pub struct Pending {
    pub id: TexId,
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

/// Sammelt die Zeichenaufrufe eines Bildes; implementiert [`Canvas`].
#[derive(Default)]
pub struct Frame {
    items: Vec<Item>,
    runs: Vec<Run>,
    sizes: Vec<(u32, u32)>,
    pending: Vec<Pending>,
}

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }

    /// Beginnt ein Bild; behält die Puffer, damit im Leerlauf nichts alloziert wird.
    pub fn begin(&mut self) {
        self.items.clear();
        self.runs.clear();
    }

    /// Sortiert nach Textur (Reihenfolge innerhalb einer Textur bleibt) und bildet die Draw-Calls.
    pub fn finish(&mut self) {
        self.items.sort_unstable_by_key(|i| (i.tex, i.seq));
        let mut first = 0;
        for (i, it) in self.items.iter().enumerate() {
            let prev = if i == 0 { it.tex } else { self.items[i - 1].tex };
            if it.tex != prev {
                self.runs.push(run(prev, first, i as u32));
                first = i as u32;
            }
        }
        if !self.items.is_empty() {
            let last = self.items[self.items.len() - 1].tex;
            self.runs.push(run(last, first, self.items.len() as u32));
        }
    }

    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    pub fn instance_count(&self) -> usize {
        self.items.len()
    }

    /// Instanzen in Zeichenreihenfolge (nach `finish`).
    pub fn instances(&self) -> impl Iterator<Item = &Inst> {
        self.items.iter().map(|i| &i.inst)
    }

    /// Schreibt die Instanzen als Bytes (`INST_BYTES` je Stück) in `out`.
    pub fn write_instances(&self, out: &mut [u8]) {
        let (chunks, _) = out.as_chunks_mut::<INST_BYTES>();
        for (chunk, it) in chunks.iter_mut().zip(&self.items) {
            let floats = it.inst.r.iter().chain(&it.inst.c).chain(&it.inst.uv);
            let (cells, _) = chunk.as_chunks_mut::<4>();
            for (cell, v) in cells.iter_mut().zip(floats) {
                *cell = v.to_ne_bytes();
            }
        }
    }

    /// Texturen, die das Backend noch hochladen muss.
    pub fn take_pending(&mut self) -> std::vec::Drain<'_, Pending> {
        self.pending.drain(..)
    }

    fn push(&mut self, tex: u32, inst: Inst) {
        let seq = self.items.len() as u32;
        self.items.push(Item { tex, seq, inst });
    }
}

fn run(tex: u32, first: u32, end: u32) -> Run {
    let tex = (tex != SOLID).then_some(TexId(tex));
    Run {
        tex,
        first,
        count: end - first,
    }
}

impl Canvas for Frame {
    fn size(&self) -> (f32, f32) {
        LOGICAL
    }

    fn texture(&mut self, rgba: &[u8], w: u32, h: u32) -> TexId {
        assert_eq!(rgba.len(), w as usize * h as usize * 4, "RGBA8 erwartet");
        let id = TexId(self.sizes.len() as u32);
        self.sizes.push((w, h));
        self.pending.push(Pending {
            id,
            w,
            h,
            rgba: rgba.to_vec(),
        });
        id
    }

    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]) {
        let Some(&(w, h)) = self.sizes.get(t.0 as usize) else {
            return;
        };
        let (w, h) = (w as f32, h as f32);
        let uv = [src.x / w, src.y / h, src.w / w, src.h / h];
        self.push(
            t.0,
            Inst {
                r: [dst.x, dst.y, dst.w, dst.h],
                c: tint,
                uv,
            },
        );
    }

    fn rect(&mut self, dst: Rect, color: [f32; 4]) {
        self.push(
            SOLID,
            Inst {
                r: [dst.x, dst.y, dst.w, dst.h],
                c: color,
                uv: [0.0; 4],
            },
        );
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
}

#[cfg(test)]
mod tests {
    use super::*;

    const WHITE: [f32; 4] = [1.0; 4];

    fn tex(f: &mut Frame) -> TexId {
        f.texture(&[255; 16], 2, 2)
    }

    #[test]
    fn sortiert_nach_textur_und_behaelt_reihenfolge() {
        let mut f = Frame::new();
        let (a, b) = (tex(&mut f), tex(&mut f));
        f.begin();
        let src = Rect::new(0.0, 0.0, 2.0, 2.0);
        for (i, t) in [b, a, b, a, b].into_iter().enumerate() {
            f.sprite(t, src, Rect::new(i as f32, 0.0, 1.0, 1.0), WHITE);
        }
        f.rect(Rect::new(9.0, 0.0, 1.0, 1.0), WHITE);
        f.finish();
        let xs: Vec<f32> = f.instances().map(|i| i.r[0]).collect();
        assert_eq!(xs, [1.0, 3.0, 0.0, 2.0, 4.0, 9.0]);
        let runs: Vec<_> = f.runs().iter().map(|r| (r.tex, r.first, r.count)).collect();
        assert_eq!(runs, [(Some(a), 0, 2), (Some(b), 2, 3), (None, 5, 1)]);
    }

    #[test]
    fn leeres_bild_hat_keine_draw_calls() {
        let mut f = Frame::new();
        f.begin();
        f.finish();
        assert!(f.runs().is_empty());
    }

    #[test]
    fn quelle_wird_auf_textur_anteile_umgerechnet() {
        let mut f = Frame::new();
        let t = f.texture(&[0; 4 * 8 * 4], 4, 8);
        f.begin();
        f.sprite(t, Rect::new(2.0, 4.0, 2.0, 4.0), Rect::new(0.0, 0.0, 1.0, 1.0), WHITE);
        f.finish();
        assert_eq!(f.instances().next().map(|i| i.uv), Some([0.5, 0.5, 0.5, 0.5]));
    }

    #[test]
    fn text_fasst_pixelstreifen_zu_rechtecken() {
        let mut f = Frame::new();
        f.begin();
        f.text("I", 0.0, 0.0, 7.0, WHITE);
        f.finish();
        // I: oben und unten je ein Streifen (5 breit), dazwischen fünf einzelne Pixel
        assert_eq!(f.instance_count(), 7);
        assert!(f.instances().all(|i| i.uv == [0.0; 4]));
    }

    #[test]
    fn instanzen_belegen_achtundvierzig_byte() {
        let mut f = Frame::new();
        f.begin();
        f.rect(Rect::new(1.0, 2.0, 3.0, 4.0), WHITE);
        f.finish();
        let mut out = [0u8; INST_BYTES];
        f.write_instances(&mut out);
        assert_eq!(f32::from_ne_bytes(out[4..8].try_into().unwrap()), 2.0);
        assert_eq!(f32::from_ne_bytes(out[16..20].try_into().unwrap()), 1.0);
    }
}
