//! Port von `native/k3c-game/menu/draw.go`: Seiten, Zeilen und der Zeichner (Karten, Papier, Rückfrage).
//!
//! Abweichungen: Go zeichnet Rechtecke über ein 1×1-Bild und die Debug-Schrift über ein Zwischenbild; hier zeichnet
//! `Canvas::rect` und `Font::draw` selbst (auch die Pixel-Schrift im Rückfall). `text_scale` bleibt nur für `measure`.
//! Die Reihenfolge der Zeichenbefehle (Schatten, Rahmen, Papier, Kante, Schild, Text) ist die von Go.

use std::cell::RefCell;
use std::rc::Rc;

use k3c_platform::{Canvas, Rect};

use super::backdrop::COL_SHADOW;
use super::layout::{
    COL_BAD, COL_DIM, COL_LINE, COL_LINE_OFF, COL_PICKED, COL_TEXT, COL_TITLE, Geom, MAT_BLUE, MAT_CLOTH, MAT_GOLD,
    MAT_LINEN, MAT_RED, MAX_ROWS, Material, Rgba, SHADOW_LAYERS, fade, geom_of, text_scale,
};
use super::saves_preview::Preview;
use crate::art::{Font, Library};
use crate::view::cell::Cell;

/// Page ist, was eine Menü-Seite zeigt. Die Screens füllen sie in `draw` nur bei neuer Version oder Sprache neu
/// (Zwischenspeicher der Texte); der Painter zeichnet sie ohne weitere Rechnung.
#[derive(Clone, Debug, Default)]
pub struct Page {
    pub title: String,
    pub sub: String,
    pub rows: [Row; MAX_ROWS],
    pub n: usize,
    /// sichtbare Zeile mit Auswahl, -1 = keine
    pub sel: i64,
    pub note: String,
    pub bad: bool,
    pub hint: String,
    /// Hintergrund abgedunkelt (Optionen aus der Pause: ruhiger Grund)
    pub dim: bool,
    pub ask: String,
    pub ask_rows: [String; 2],
    pub ask_sel: i64,
    /// großes Weltbild mit Name (Weltauswahl); leer = keins
    pub hero: Hero,
    /// Vorschau des markierten Spielstands (Spielstand-Auswahl, GS3.8); `on` false = keine
    pub preview: Preview,

    /// Version des Screens, für die die Texte gelten
    pub ver: i64,
    /// Sprache der Texte
    pub lang: String,
    pub ok: bool,
}

/// Hero ist das große Bild der Weltauswahl: Illustration (leer = nur Name) mit dem Namen darüber und Blätter-Pfeilen.
#[derive(Clone, Debug, Default)]
pub struct Hero {
    pub art: String,
    pub name: String,
    pub on: bool,
    /// links gibt es weitere Welten
    pub left: bool,
    pub right: bool,
    /// Welt aus dem Repo (`WorldInfo::repo`, B-1210): kleines Dev-Symbol oben links im Bild
    pub dev: bool,
}

/// Row ist eine Karte: Beschriftung links, Wert rechts, Papier-Farbe; `off` = ausgegraut bzw. als Problem markiert.
#[derive(Clone, Debug, Default)]
pub struct Row {
    pub label: String,
    pub value: String,
    pub mat: Material,
    pub off: bool,
    /// Illustration `gruppe/id` (B-818), leer = nur Text
    pub art: String,
    /// Siegel `gruppe/id` rechts neben dem Wert (ECO1.2), leer = keins
    pub badge: &'static str,
}

impl Page {
    /// Meldet, ob die Texte für Version `ver` und Sprache `lang` neu gebaut werden müssen, und merkt sich beides.
    pub fn stale(&mut self, ver: i64, lang: &str) -> bool {
        if self.ok && self.ver == ver && self.lang == lang {
            return false;
        }
        self.ver = ver;
        self.lang = lang.to_string();
        self.ok = true;
        true
    }

    /// Leert die Seite vor dem Neubau.
    pub fn reset(&mut self, title: &str) {
        *self = Page {
            title: title.to_string(),
            ver: self.ver,
            lang: std::mem::take(&mut self.lang),
            ok: self.ok,
            ..Page::default()
        };
    }

    /// Hängt eine Zeile an (mehr als `MAX_ROWS` werden verworfen).
    pub fn add(&mut self, r: Row) {
        if self.n < MAX_ROWS {
            self.rows[self.n] = r;
            self.n += 1;
        }
    }
}

/// Painter zeichnet Seiten: Rechtecke über `Canvas::rect`, Text über die Schrift (die der Library, sonst eine eigene).
pub struct Painter {
    /// eigene Schrift, wenn es keine Library gibt
    pub font: Option<Font>,
    /// Illustrationen der Karten (B-818); `None` = nur Text
    pub lib: Option<Rc<RefCell<Library>>>,
}

/// Der Farbton für die Zeichenfläche: aus vormultiplizierten Bytes (Go `color.RGBA`) die gerade Farbe, denn die
/// `Canvas` multipliziert selbst vor (wie `fx::paint::tint`).
pub fn tint(c: Rgba) -> [f32; 4] {
    if c[3] == 0 {
        return [0.0; 4];
    }
    let d = f32::from(c[3]);
    [f32::from(c[0]) / d, f32::from(c[1]) / d, f32::from(c[2]) / d, d / 255.0]
}

impl Painter {
    /// Legt den Zeichner an; ohne Library hält er `font` selbst.
    pub fn new(font: Option<Font>, lib: Option<Rc<RefCell<Library>>>) -> Painter {
        Painter { font, lib }
    }

    /// Füllt ein Rechteck mit `clr` (vormultipliziert).
    #[allow(clippy::cast_possible_truncation)]
    pub fn rect(&self, dst: &mut dyn Canvas, x: f64, y: f64, w: f64, h: f64, clr: Rgba) {
        dst.rect(Rect::new(x as f32, y as f32, w as f32, h as f32), tint(clr));
    }

    fn fallback(&self) -> bool {
        match &self.lib {
            Some(l) => l.borrow().font.fallback,
            None => self.font.as_ref().is_none_or(|f| f.fallback),
        }
    }

    fn raw_measure(&self, s: &str, size: f64) -> (f64, f64) {
        match (&self.lib, &self.font) {
            (Some(l), _) => l.borrow().font.measure(s, size),
            (None, Some(f)) => f.measure(s, size),
            (None, None) => (0.0, 0.0),
        }
    }

    /// Liefert Breite und Höhe von `s` bei Größe `size`; die Pixel-Schrift zählt `text_scale`-fach.
    pub fn measure(&self, s: &str, size: f64) -> (f64, f64) {
        let (w, h) = self.raw_measure(s, size);
        if self.fallback() {
            let k = text_scale(size);
            return (w * k, h * k);
        }
        (w, h)
    }

    /// Zeichnet `s` mit der linken oberen Ecke bei `(x, y)`.
    pub fn text(&mut self, dst: &mut dyn Canvas, s: &str, x: f64, y: f64, size: f64, clr: Rgba) {
        if s.is_empty() {
            return;
        }
        if self.fallback() {
            // wie Go: Pixel-Schrift `text_scale`-fach vergrößert und mit `clr` eingefärbt (Messen und Zeichnen stimmen)
            let k = text_scale(size);
            #[allow(clippy::cast_possible_truncation)]
            for (i, line) in s.split('\n').enumerate() {
                let ly = y + i as f64 * 16.0 * k;
                dst.text(line, x.trunc() as f32, ly.trunc() as f32, (12.0 * k) as f32, tint(clr));
            }
            return;
        }
        match (&self.lib, &mut self.font) {
            (Some(l), _) => l.borrow_mut().font.draw(dst, s, x, y, size, clr),
            (None, Some(f)) => f.draw(dst, s, x, y, size, clr),
            (None, None) => {}
        }
    }

    /// Breite eines Zeichens bei Größe `size` (Lage der Auswahl-Marke).
    pub fn char_w(&self, size: f64) -> f64 {
        self.measure(">", size).0
    }

    /// Zeichnet `s` waagerecht mittig um `cx`, oben bei `y`.
    pub fn centered(&mut self, dst: &mut dyn Canvas, s: &str, cx: f64, y: f64, size: f64, clr: Rgba) {
        let (w, _) = self.measure(s, size);
        self.text(dst, s, cx - w / 2.0, y, size, clr);
    }

    /// Zeichnet die Seite `pg` ins Fenster der Zelle `c`.
    #[allow(clippy::cast_precision_loss)]
    pub fn page(&mut self, dst: &mut dyn Canvas, c: &Cell, pg: &Page) {
        let g = geom_of(c.rect);
        let r = c.rect;
        self.backdrop(dst, &g);
        if pg.dim {
            self.rect(
                dst,
                r.min.x as f64,
                r.min.y as f64,
                r.dx() as f64,
                r.dy() as f64,
                COL_DIM,
            );
        }
        let cx = r.min.x as f64 + r.dx() as f64 / 2.0;
        self.centered(dst, &pg.title, cx, g.title_y, g.title_px, COL_TITLE);
        self.centered(dst, &pg.sub, cx, g.sub_y, g.small_px, COL_TEXT);
        for i in 0..pg.n {
            self.card(dst, &g, i as i64, &pg.rows[i], i as i64 == pg.sel);
        }
        if pg.hero.on {
            self.hero(dst, &g, &pg.hero);
        }
        if pg.preview.on {
            self.preview(dst, &g, &pg.preview);
        }
        let note_clr = if pg.bad { COL_BAD } else { COL_TEXT };
        self.centered(dst, &pg.note, cx, g.note_y, g.small_px, note_clr);
        self.centered(dst, &pg.hint, cx, g.hint_y, g.small_px, COL_TEXT);
        if !pg.ask.is_empty() {
            self.ask_box(dst, &g, pg);
        }
    }

    /// Zeichnet die Karte der sichtbaren Zeile `i`: Schatten, Papier, Auswahl-Rahmen, Leinen-Schild mit Text.
    pub fn card(&mut self, dst: &mut dyn Canvas, g: &Geom, i: i64, rw: &Row, selected: bool) {
        let (x, y, w, h) = g.row_rect(i);
        self.paper(dst, g, (x, y, w, h), (rw.mat, rw.off), selected);
        let inset = g.strip_inset();
        if selected {
            let mx = x - 4.0 * inset - self.char_w(g.row_px);
            self.text(dst, ">", mx, y + (h - g.row_px) / 2.0, g.row_px, COL_TITLE);
        }
        let strip = style_of(rw.mat, rw.off, selected).strip;
        self.rect(dst, x + inset, y + inset, w - 2.0 * inset, h - 2.0 * inset, strip);
        let clr = if rw.off { COL_LINE_OFF } else { COL_LINE };
        let mut lx = x + 3.0 * inset;
        let ah = h - 2.0 * inset;
        if self.art_box(dst, &rw.art, x + 2.0 * inset, y + inset, 1.4 * ah, ah) {
            lx += 1.4 * ah + inset; // Text rechts neben der Illustration
        }
        let (_, th) = self.measure(&rw.label, g.row_px);
        let ty = y + (h - th) / 2.0;
        self.text(dst, &rw.label, lx, ty, g.row_px, clr);
        let (vw, _) = self.measure(&rw.value, g.row_px);
        let mut vx = x + w - 3.0 * inset - vw;
        if self.art_box(dst, rw.badge, x + w - 3.0 * inset - ah, y + inset, ah, ah) {
            vx -= ah + inset; // Wert links neben dem Siegel
        }
        self.text(dst, &rw.value, vx, ty, g.row_px, clr);
    }

    /// Zeichnet ein Papier-Rechteck `r = (x, y, w, h)` mit Schlagschatten; `look = (Material, ausgegraut)`, gewählt
    /// heller mit goldenem Rahmen.
    pub fn paper(
        &mut self,
        dst: &mut dyn Canvas,
        g: &Geom,
        r: (f64, f64, f64, f64),
        look: (Material, bool),
        selected: bool,
    ) {
        let (x, y, w, h) = r;
        for (d, alpha) in SHADOW_LAYERS {
            let d = d * g.k;
            self.rect(dst, x + d, y + d, w, h, fade(COL_SHADOW, alpha));
        }
        let st = style_of(look.0, look.1, selected);
        if st.frame {
            let b = g.frame_width();
            self.rect(dst, x - b, y - b, w + 2.0 * b, h + 2.0 * b, MAT_GOLD.licht);
            self.rect(dst, x - b / 2.0, y - b / 2.0, w + b, h + b, COL_LINE);
        }
        self.rect(dst, x, y, w, h, st.fill);
        self.rect(dst, x, y + h - 3.0 * g.k, w, 3.0 * g.k, st.edge);
    }

    /// Zeichnet die Rückfrage: Kasten mit Frage und zwei Knöpfen nebeneinander.
    #[allow(clippy::cast_precision_loss)]
    pub fn ask_box(&mut self, dst: &mut dyn Canvas, g: &Geom, pg: &Page) {
        let (x, y, w, h) = g.ask_rect();
        let win = g.win;
        self.rect(
            dst,
            win.min.x as f64,
            win.min.y as f64,
            win.dx() as f64,
            win.dy() as f64,
            COL_DIM,
        );
        self.paper(dst, g, (x, y, w, h), (MAT_LINEN, false), false);
        self.centered(dst, &pg.ask, x + w / 2.0, y + 0.6 * g.row_h, g.small_px, COL_LINE);
        let bw = (w - 3.0 * g.row_gap * 2.0) / 2.0;
        for (i, label) in pg.ask_rows.iter().enumerate() {
            let bx = x + 2.0 * g.row_gap + i as f64 * (bw + 2.0 * g.row_gap);
            let by = y + h - 1.6 * g.row_h;
            let m = if i == 1 { MAT_RED } else { MAT_BLUE };
            self.paper(dst, g, (bx, by, bw, g.row_h), (m, false), i as i64 == pg.ask_sel);
            let inset = g.strip_inset();
            self.rect(
                dst,
                bx + inset,
                by + inset,
                bw - 2.0 * inset,
                g.row_h - 2.0 * inset,
                MAT_LINEN.licht,
            );
            let (_, th) = self.measure(label, g.row_px);
            self.centered(dst, label, bx + bw / 2.0, by + (g.row_h - th) / 2.0, g.row_px, COL_LINE);
        }
    }
}

/// CardStyle ist das Aussehen einer Karte (wie `paperCard.ts`): Papierton, Kante unten, Leinen-Schild und ob der
/// goldene Auswahl-Rahmen sie umgibt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CardStyle {
    pub fill: Rgba,
    pub edge: Rgba,
    pub strip: Rgba,
    pub frame: bool,
}

/// Liefert das Aussehen einer Karte aus Material `m`: ausgegraut aus Stoff, gewählt heller mit Rahmen und goldhellem
/// Schild (rein, getestet).
pub fn style_of(m: Material, off: bool, selected: bool) -> CardStyle {
    let m = if off { MAT_CLOTH } else { m };
    let mut st = CardStyle {
        fill: m.basis,
        edge: m.schatten,
        strip: MAT_LINEN.basis,
        frame: false,
    };
    if selected {
        (st.fill, st.strip, st.frame) = (m.licht, COL_PICKED, true);
    }
    st
}
