//! Port von `native/k3c-game/hud/keyline.go`.
//!
//! Bedienhinweis mit Tasten-Icons (B-911): eine Zeile aus Text und Glyphen, gebaut aus der Vorlage eines Textes
//! („{move} wählen · {ok} bestätigen“) und den Tasten des Eingabe-Profils. Skill-Menü und Seekarte zeichnen ihren
//! Hinweis damit; sind die Icons der Zeile nicht geladen, steht derselbe Hinweis als reiner Text (plain) wie vor den
//! Icons. Gebaut wird nur bei Änderung, gemessen einmal bei `LINE_REF`; je Bild rechnet die Zeile nur um.
//!
//! Öffentliche Schnittstelle (RU23): `new_key_line(tmpl, &[arg(name, glyphs)])` baut die Zeile, `Hud::draw_hint`
//! zeichnet sie mit linker oberer Ecke des Textes bei `(x, y)` (mit Icons als Zeile, sonst den Text `plain`),
//! `Hud::line_width` liefert die Breite (`false`: ohne Icons, es gilt `plain`).

use super::Hud;
use super::draw::space_w;
use super::drawglyph::{GLYPH_LABEL_K, cap_box, is_fixed, is_key_glyph};
use super::fonts::{Rgba, WHITE};
use super::glyphs::{CAP_WIDE, Glyph};
use crate::art::Font;
use k3c_platform::Canvas;

/// LINE_SEGS: Teile je Zeile höchstens (Text und Glyphen im Wechsel).
pub const LINE_SEGS: usize = 10;
/// LINE_REF: Schriftgröße, bei der die Teile gemessen sind.
pub const LINE_REF: f64 = 64.0;

/// line_glyph_px ist die Glyph-Höhe in einer Zeile der Schriftgröße size: höher als in den Menüzeilen (`glyph_px`), damit
/// die Beschriftung der Kappen neben dem kleineren Hinweistext lesbar bleibt.
pub fn line_glyph_px(size: f64) -> f64 {
    (size * 1.5).ceil()
}

/// LineSeg ist ein Teil der Zeile: `g.key` "" = Text `g.label`, sonst die Glyph; tw und th sind Breite und Höhe von
/// `g.label` bei `LINE_REF`.
#[derive(Clone, Debug, Default)]
pub struct LineSeg {
    pub g: Glyph,
    pub tw: f64,
    pub th: f64,
}

/// KeyLine ist ein Bedienhinweis: n Teile (`segs`) und derselbe Hinweis als Text. n 0 = nur Text.
/// Abweichung von Go: `segs` ist ein `Vec` (höchstens `LINE_SEGS`) statt eines Arrays.
#[derive(Clone, Debug, Default)]
pub struct KeyLine {
    pub segs: Vec<LineSeg>,
    pub n: usize,
    pub plain: String,
    measured: bool,
}

/// LineArg ist der Wert eines Platzhalters der Vorlage: Glyphen und Text (key "") in Reihenfolge.
#[derive(Clone, Debug)]
pub struct LineArg {
    pub name: String,
    pub parts: Vec<Glyph>,
}

/// arg benennt den Platzhalter name mit den Teilen parts.
pub fn arg(name: &str, parts: Vec<Glyph>) -> LineArg {
    LineArg {
        name: name.to_string(),
        parts,
    }
}

/// text_part ist ein Text-Teil ohne Bild.
pub fn text_part(s: &str) -> Glyph {
    Glyph {
        label: s.to_string(),
        ..Glyph::default()
    }
}

/// new_key_line zerlegt die Vorlage tmpl an ihren Platzhaltern `{name}` in Text und die Teile aus args; ein unbekannter
/// Platzhalter bleibt als Text stehen. Mehr als `LINE_SEGS` Teile: nur Text.
pub fn new_key_line(tmpl: &str, args: &[LineArg]) -> KeyLine {
    let mut l = KeyLine::default();
    let mut plain = String::new();
    let mut tmpl = tmpl;
    while !tmpl.is_empty() {
        let (Some(i), Some(j)) = (tmpl.find('{'), tmpl.find('}')) else {
            l.add(&mut plain, text_part(tmpl));
            break;
        };
        if j < i {
            l.add(&mut plain, text_part(tmpl));
            break;
        }
        l.add(&mut plain, text_part(&tmpl[..i]));
        let name = &tmpl[i + 1..j];
        match args.iter().find(|a| a.name == name) {
            None => l.add(&mut plain, text_part(&tmpl[i..=j])),
            Some(a) => {
                for g in &a.parts {
                    l.add(&mut plain, g.clone());
                }
            }
        }
        tmpl = &tmpl[j + 1..];
    }
    l.plain = plain;
    if l.n > LINE_SEGS {
        l.n = 0;
    }
    l
}

impl KeyLine {
    /// add hängt den Teil g an: Text ohne Ränder (den Abstand setzt die Zeile), Text nach Text wächst zusammen.
    fn add(&mut self, plain: &mut String, mut g: Glyph) {
        plain.push_str(&g.label);
        if g.key.is_empty() {
            g.label = g.label.trim().to_string();
            if g.label.is_empty() {
                return;
            }
            if self.n > 0 && self.n <= LINE_SEGS && self.segs[self.n - 1].g.key.is_empty() {
                let last = &mut self.segs[self.n - 1].g.label;
                last.push(' ');
                last.push_str(&g.label);
                return;
            }
        }
        if self.n < LINE_SEGS {
            self.segs.push(LineSeg {
                g,
                ..LineSeg::default()
            });
        }
        self.n += 1;
    }

    /// measure misst die Teile einmal bei `LINE_REF`.
    pub fn measure(&mut self, f: &Font) {
        if self.measured {
            return;
        }
        for s in self.segs.iter_mut().take(self.n) {
            (s.tw, s.th) = f.measure(&s.g.label, LINE_REF);
        }
        self.measured = true;
    }

    /// seg_width ist die Breite von Teil i bei Schriftgröße size und Glyph-Höhe gh; für eine Tastenkappe dazu Breite und
    /// Höhe ihrer Beschriftung bei gh·`GLYPH_LABEL_K`. Die Debug-Schrift hat eine feste Größe, ihre Maße gelten
    /// unverändert.
    pub fn seg_width(&self, i: usize, f: &Font, size: f64, gh: f64) -> (f64, f64, f64) {
        let s = &self.segs[i];
        let mut r = size / LINE_REF;
        if is_key_glyph(&s.g) {
            r = gh * GLYPH_LABEL_K / LINE_REF;
        }
        if f.fallback {
            r = 1.0;
        }
        let (tw, th) = (s.tw * r, s.th * r);
        if s.g.key.is_empty() {
            return (tw, tw, th);
        }
        if is_key_glyph(&s.g) {
            let (w, _) = cap_box(gh, tw, s.g.icon == CAP_WIDE, is_fixed(Some(f)));
            return (w, tw, th);
        }
        (gh, 0.0, 0.0)
    }
}

impl Hud {
    /// line_icons meldet, ob die Zeile mit Icons gezeichnet wird: Sie hat Teile und jede Glyph ein geladenes Icon.
    pub fn line_icons(&self, l: &KeyLine) -> bool {
        for s in l.segs.iter().take(l.n) {
            if !s.g.key.is_empty() && self.glyph_icon(&s.g).is_none() {
                return false;
            }
        }
        l.n > 0
    }

    /// line_width ist die Breite der Zeile l bei Schriftgröße size; false = ohne Icons, es gilt der Text `l.plain`.
    pub fn line_width(&self, f: Option<&Font>, l: &mut KeyLine, size: f64) -> (f64, bool) {
        let Some(f) = f else { return (0.0, false) };
        if !self.line_icons(l) {
            return (0.0, false);
        }
        l.measure(f);
        let (mut w, gh) = (0.0, line_glyph_px(size));
        for i in 0..l.n {
            w += l.seg_width(i, f, size, gh).0;
        }
        (w + (l.n as f64 - 1.0) * space_w(size), true)
    }

    /// draw_line zeichnet die Zeile l mit der linken oberen Ecke des Textes bei (x, y) in Schriftgröße size, Glyphen
    /// mittig zur Textzeile; false = ohne Icons nichts gezeichnet (der Aufrufer zeichnet `l.plain`).
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn draw_line(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        l: &mut KeyLine,
        mut x: f64,
        y: f64,
        size: f64,
        clr: Rgba,
    ) -> bool {
        let Some(f) = f else { return false };
        if !self.line_icons(l) {
            return false;
        }
        l.measure(f);
        let gh = line_glyph_px(size);
        for i in 0..l.n {
            let g = &l.segs[i].g;
            let (w, tw, th) = l.seg_width(i, f, size, gh);
            if g.key.is_empty() {
                self.paint.text(dst, Some(&mut *f), &g.label, x, y, size, clr);
            } else if let Some(ic) = self.glyph_icon(g) {
                self.icon_glyph(dst, Some(&mut *f), &ic, g, x, y + size * 0.6, gh, tw, th);
            }
            x += w + space_w(size);
        }
        true
    }

    /// draw_hint zeichnet den Bedienhinweis: die Zeile l mit Icons, sonst den Text plain.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn draw_hint(
        &mut self,
        dst: &mut dyn Canvas,
        mut f: Option<&mut Font>,
        l: &mut KeyLine,
        plain: &str,
        x: f64,
        y: f64,
        size: f64,
    ) {
        if !self.draw_line(dst, f.as_deref_mut(), l, x, y, size, WHITE) {
            self.paint.text(dst, f, plain, x, y, size, WHITE);
        }
    }
}
