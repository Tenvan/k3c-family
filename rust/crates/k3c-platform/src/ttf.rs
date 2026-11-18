//! TTF-Schrift (fontdue): Laden, Breite messen, Glyphen rastern und in einen Atlas packen. Reine Rechnung ohne
//! GPU; `Frame` lädt den Atlas als Textur und schreibt neue Glyphen als Teil-Updates nach.
//!
//! Lage wie bei Ebitengine `text/v2`: `(x, y)` ist die linke obere Ecke der Zeile, die Grundlinie liegt bei
//! `y + ascent`, die Zeilenhöhe ist Ober- plus Unterlänge (`HAscent + HDescent`). Pixelgrößen bleiben gebrochen
//! (wie `GoTextFace.Size`) und werden nur auf 1/64 Pixel gequantelt (Cache-Schlüssel und Messung gleich, B-1065).

use std::collections::HashMap;

use crate::Rect;

/// Kantenlänge des Glyphen-Atlas in Pixeln.
pub const ATLAS: u32 = 2048;
/// Höchstens so viele Glyphen im Cache; danach wird der Atlas geleert und neu befüllt.
pub const CACHE: usize = 4096;
/// Abstand zwischen Glyphen im Atlas (gegen Ausbluten beim linearen Filter).
const GAP: u32 = 1;

/// Handle einer geladenen Schrift.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontId(pub u32);

/// Eine Glyphe im Atlas: Quelle (Atlas-Pixel) und Lage relativ zu Stift und Zeilenoberkante.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glyph {
    pub src: Rect,
    pub dx: f32,
    pub dy: f32,
    pub advance: f32,
}

/// Neue Glyphen-Pixel für den Atlas (RGBA8, vormultipliziertes Weiß).
pub struct AtlasUpdate {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

/// Geladene Schriften; misst ohne Atlas (headless gleich wie im Fenster).
#[derive(Default, Clone)]
pub struct Fonts {
    fonts: Vec<fontdue::Font>,
}

impl std::fmt::Debug for Fonts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fonts({})", self.fonts.len())
    }
}

impl Fonts {
    pub fn load(&mut self, ttf: &[u8]) -> Result<FontId, String> {
        let font = fontdue::Font::from_bytes(ttf, fontdue::FontSettings::default()).map_err(|e| e.to_string())?;
        self.fonts.push(font);
        Ok(FontId(self.fonts.len() as u32 - 1))
    }

    pub fn get(&self, id: FontId) -> Option<&fontdue::Font> {
        self.fonts.get(id.0 as usize)
    }

    /// Breite von `s` in Pixelgröße `px` (Vorschub plus Unterschneidung); 0 bei unbekannter Schrift.
    pub fn width(&self, id: FontId, s: &str, px: f32) -> f32 {
        let Some(f) = self.get(id) else {
            return 0.0;
        };
        let px = quant_px(px);
        let mut w = 0.0;
        let mut prev = None;
        for ch in s.chars() {
            w += kern(f, prev, ch, px) + f.metrics(ch, px).advance_width;
            prev = Some(ch);
        }
        w
    }

    /// Oberlänge (Abstand Zeilenoberkante → Grundlinie).
    pub fn ascent(&self, id: FontId, px: f32) -> f32 {
        self.get(id)
            .and_then(|f| f.horizontal_line_metrics(quant_px(px)))
            .map_or(0.0, |m| m.ascent)
    }

    /// Höhe einer Zeile: Ober- plus Unterlänge (Go: `HAscent + HDescent`, die Höhe von `text.Measure` für eine Zeile).
    pub fn height(&self, id: FontId, px: f32) -> f32 {
        self.get(id)
            .and_then(|f| f.horizontal_line_metrics(quant_px(px)))
            .map_or(0.0, |m| m.ascent - m.descent)
    }
}

/// Pixelgröße, auf 1/64 Pixel gequantelt, mindestens 1 (gebrochene Größen bleiben erhalten).
pub fn quant_px(px: f32) -> f32 {
    (px.max(1.0) * 64.0).round() / 64.0
}

fn kern(f: &fontdue::Font, prev: Option<char>, ch: char, px: f32) -> f32 {
    prev.and_then(|p| f.horizontal_kern(p, ch, px)).unwrap_or(0.0)
}

/// Glyphen-Atlas mit Zeilen-Packer und Cache je (Schrift, Zeichen, Pixelgröße).
pub struct Atlas {
    cache: HashMap<(u32, char, u32), Glyph>,
    x: u32,
    y: u32,
    row: u32,
    /// Atlas oder Cache ist in diesem Bild voll gelaufen: neue Glyphen erst nach dem Leeren am nächsten Bildanfang
    /// (sonst würden schon gelegte Quads dieses Bildes überschrieben).
    full: bool,
    /// Glyphen-Quads des letzten `layout` (Quelle im Atlas, Ziel); wiederverwendet.
    pub quads: Vec<(Rect, Rect)>,
}

impl Default for Atlas {
    fn default() -> Self {
        Self {
            cache: HashMap::with_capacity(CACHE),
            x: 0,
            y: 0,
            row: 0,
            full: false,
            quads: Vec::new(),
        }
    }
}

impl Atlas {
    #[cfg(test)]
    pub fn glyphs(&self) -> usize {
        self.cache.len()
    }

    /// Bildanfang: ein voll gelaufener Atlas wird hier geleert (alle Glyphen werden neu gerastert).
    pub fn begin_frame(&mut self) {
        if self.full {
            self.reset();
        }
    }

    /// Legt `s` aus und füllt `quads`; neue Glyphen werden gerastert und als `updates` gemeldet.
    pub fn layout(
        &mut self,
        fonts: &Fonts,
        (id, px): (FontId, f32),
        s: &str,
        (x, y): (f32, f32),
        updates: &mut Vec<AtlasUpdate>,
    ) {
        self.quads.clear();
        let Some(f) = fonts.get(id) else {
            return;
        };
        let px = quant_px(px);
        let base = y + fonts.ascent(id, px);
        let mut pen = x;
        let mut prev = None;
        for ch in s.chars() {
            pen += kern(f, prev, ch, px);
            prev = Some(ch);
            let Some(g) = self.glyph(f, id, ch, px, updates) else {
                continue;
            };
            if g.src.w > 0.0 {
                // Ganze Pixel: der Atlas wird linear gefiltert, Bruchteile machen Glyphen unscharf.
                let dst = Rect::new((pen + g.dx).round(), (base + g.dy).round(), g.src.w, g.src.h);
                self.quads.push((g.src, dst));
            }
            pen += g.advance;
        }
    }

    fn glyph(
        &mut self,
        f: &fontdue::Font,
        id: FontId,
        ch: char,
        px: f32,
        updates: &mut Vec<AtlasUpdate>,
    ) -> Option<Glyph> {
        let key = (id.0, ch, (px * 64.0).round() as u32);
        if let Some(g) = self.cache.get(&key) {
            return Some(*g);
        }
        if self.cache.len() >= CACHE {
            self.full = true;
        }
        let (m, cov) = if self.full {
            (f.metrics(ch, px), Vec::new())
        } else {
            f.rasterize(ch, px)
        };
        let (w, h) = (m.width as u32, m.height as u32);
        let Some((ax, ay)) = self.place(w, h) else {
            // Kein Platz: Vorschub bleibt, gezeichnet wird nichts. Zu große Glyphen merkt sich der Cache.
            let g = Glyph {
                src: Rect::default(),
                dx: 0.0,
                dy: 0.0,
                advance: m.advance_width,
            };
            if !self.full {
                self.cache.insert(key, g);
            }
            return Some(g);
        };
        if w > 0 && h > 0 {
            let rgba = cov.iter().flat_map(|&c| [c, c, c, c]).collect();
            updates.push(AtlasUpdate {
                x: ax,
                y: ay,
                w,
                h,
                rgba,
            });
        }
        let g = Glyph {
            src: Rect::new(ax as f32, ay as f32, w as f32, h as f32),
            dx: m.xmin as f32,
            dy: -(m.ymin as f32 + h as f32),
            advance: m.advance_width,
        };
        self.cache.insert(key, g);
        Some(g)
    }

    /// Platz für `w`×`h`; `None`, wenn die Glyphe zu groß ist oder der Atlas in diesem Bild voll ist (dann wird
    /// er am nächsten Bildanfang geleert).
    fn place(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        if w + GAP > ATLAS || h + GAP > ATLAS || self.full {
            return None;
        }
        if self.x + w + GAP > ATLAS {
            self.x = 0;
            self.y += self.row;
            self.row = 0;
        }
        if self.y + h + GAP > ATLAS {
            self.full = true;
            return None;
        }
        let at = (self.x, self.y);
        self.x += w + GAP;
        self.row = self.row.max(h + GAP);
        Some(at)
    }

    fn reset(&mut self) {
        self.cache.clear();
        self.x = 0;
        self.y = 0;
        self.row = 0;
        self.full = false;
    }
}

#[cfg(test)]
#[path = "ttf_tests.rs"]
mod tests;
