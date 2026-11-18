//! Port von `native/k3c-game/hud/drawsea.go`: Zeichnen der Seekarte am Hafen (`seekarte.rs`) aus gezeichneten Formen:
//! dunkle Tafel, je Etappe eine Zeile mit Beschriftung links und den Inseln als Knoten (Marke: Haken = abgeschlossen,
//! Ring = Ziel, Schloss = gesperrt). Gelber Rahmen: Cursor dieses Spielers; weißer Rahmen und hellere Fläche: gewähltes
//! Ziel (gilt für alle).
//!
//! Abweichung von Go: Alle Flächen liegen auf der Ebene des Inhalts (`fill_top`), die Karte liegt über den Elementen
//! (Go kennt keine Ebenen). Die Sheets des Dioramas liest `draw_cell` vor dem Ausleihen der Bibliothek (`SeaArt`).

use k3c_platform::Canvas;

use super::Hud;
use super::draw::rect_of;
use super::drawsea_paper::SeaArt;
use super::fonts::{WHITE, YELLOW, rgb};
use super::layout::Rect;
use super::paint::{Z_CONTENT, straight};
use super::seekarte::{SeaKind, SeaNode, SeaView};
use crate::art::Font;
use crate::view::cell::Cell;

pub const CLR_SEA_BACK: u32 = 0x0c1024;
pub const CLR_SEA_DONE: u32 = 0x1f3d2b;
pub const CLR_SEA_FREE: u32 = 0x2b3a67;
pub const CLR_SEA_PICKED: u32 = 0x4a63b8;
pub const CLR_SEA_OFF: u32 = 0x2a2a2a;
pub const CLR_SEA_GREEN: u32 = 0x4caf50;
pub const CLR_SEA_GREY: u32 = 0x8a8a8a;

/// sea_fill ist die Flächenfarbe eines Knotens.
pub fn sea_fill(n: &SeaNode) -> u32 {
    if n.chosen {
        return CLR_SEA_PICKED;
    }
    match n.kind {
        SeaKind::Done => CLR_SEA_DONE,
        SeaKind::Target => CLR_SEA_FREE,
        SeaKind::Locked => CLR_SEA_OFF,
    }
}

/// SeaSizes sind die bei Schriftgröße b gemessenen Maße der Tafel.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeaSizes {
    pub lab_w: f64,
    pub name_w: f64,
    pub title_w: f64,
    pub hint_w: f64,
    pub max_nodes: usize,
    /// `hint_w` gilt für den Hinweis mit Tasten-Icons
    pub hint_icons: bool,
}

impl Hud {
    /// measure_sea misst Beschriftungen, Namen, Titel und Hinweis bei Schriftgröße b.
    pub fn measure_sea(&self, v: &mut SeaView, f: Option<&Font>, b: f64) -> SeaSizes {
        let mut m = SeaSizes::default();
        let Some(font) = f else { return m };
        m.title_w = font.measure(&v.title, b).0;
        (m.hint_w, m.hint_icons) = self.line_width(f, &mut v.line, b * 0.8);
        if !m.hint_icons {
            m.hint_w = font.measure(&v.hint, b * 0.8).0;
        }
        for r in &v.rows {
            m.lab_w = m.lab_w.max(font.measure(&r.title, b).0);
            m.max_nodes = m.max_nodes.max(r.nodes.len());
            for n in &r.nodes {
                m.name_w = m.name_w.max(font.measure(&n.name, b).0);
            }
        }
        m
    }

    /// draw_sea zeichnet die Tafel mittig und etwas hoch in die Zelle c; passt sie nicht, schrumpft sie samt Schrift (nie
    /// über die Zelle hinaus). Sind alle Assets der Seekarte geladen (`art`), zeichnet es das Diorama
    /// (`drawsea_paper.rs`), sonst die Formen (B-964/R5).
    pub fn draw_sea(
        &mut self,
        dst: &mut dyn Canvas,
        v: &mut SeaView,
        c: &Cell,
        k: f64,
        f: Option<&mut Font>,
        art: Option<SeaArt>,
    ) {
        self.draw_sea_with(dst, v, c, k, f, art.as_ref());
        if let Some(a) = art {
            self.sea_isl = a.isl; // Puffer der Insel-Sheets zurück, je Bild wiederverwendet
        }
    }

    fn draw_sea_with(
        &mut self,
        dst: &mut dyn Canvas,
        v: &mut SeaView,
        c: &Cell,
        k: f64,
        mut f: Option<&mut Font>,
        art: Option<&SeaArt>,
    ) {
        if f.is_none() || !v.open || v.rows.is_empty() {
            return;
        }
        let b = 28.0 * k;
        if v.sizes_k != k || v.sizes.hint_icons != self.line_icons(&v.line) {
            v.sizes = self.measure_sea(v, f.as_deref(), b); // nur bei neuer Karte, Skalierung oder anderen Tasten-Icons
            v.sizes_k = k;
        }
        if let Some(a) = art {
            self.draw_sea_paper(dst, f, v, a, rect_of(c.rect), k);
            return;
        }
        let m = v.sizes.clone();
        let (mut pad, mut gap, mut node_h) = (0.6 * b, 0.4 * b, 1.6 * b);
        let mut node_w = m.name_w + node_h + pad;
        let n = v.rows.len() as f64;
        let mut w = (2.0 * pad + m.lab_w + gap + m.max_nodes as f64 * (node_w + gap))
            .max(2.0 * pad + m.title_w)
            .max(2.0 * pad + m.hint_w);
        let mut ht = 2.0 * pad + 1.4 * b + n * (node_h + gap) + 1.1 * b;
        let cell = rect_of(c.rect);
        let s = (cell.w * 0.96 / w).min(cell.h * 0.9 / ht).min(1.0);
        let b = b * s;
        (pad, gap, node_h, node_w, w, ht) = (pad * s, gap * s, node_h * s, node_w * s, w * s, ht * s);
        let bx = cell.x + (cell.w - w) / 2.0;
        let by = cell.y.max(cell.y + (cell.h - ht) / 2.0 - 40.0 * k);
        self.paint
            .fill_top(dst, Rect::new(bx, by, w, ht), rgb(CLR_SEA_BACK, 0.85));
        let (x, mut y) = (bx + pad, by + pad);
        self.paint.text(dst, f.as_deref_mut(), &v.title, x, y, b, YELLOW);
        y += 1.4 * b;
        for r in &v.rows {
            let ty = y + (node_h - b * 1.2) / 2.0;
            self.paint.text(dst, f.as_deref_mut(), &r.title, x, ty, b, WHITE);
            let mut nx = x + m.lab_w * s + gap;
            for nd in &r.nodes {
                self.draw_sea_node(dst, f.as_deref_mut(), nd, Rect::new(nx, y, node_w, node_h), b);
                nx += node_w + gap;
            }
            y += node_h + gap;
        }
        self.draw_hint(dst, f, &mut v.line, &v.hint, x, y, b * 0.8);
    }

    /// draw_sea_node zeichnet den Knoten nd in r: Fläche, Rahmen, Marke und Name.
    fn draw_sea_node(&mut self, dst: &mut dyn Canvas, f: Option<&mut Font>, nd: &SeaNode, r: Rect, b: f64) {
        self.paint.fill_top(dst, r, rgb(sea_fill(nd), 1.0));
        if nd.chosen {
            self.paint.frame(dst, r, (b * 0.08).max(2.0), WHITE);
        }
        if nd.cursor {
            self.paint.frame(dst, r, (b * 0.12).max(3.0), YELLOW);
        }
        let mr = r.h * 0.3;
        self.draw_sea_mark(dst, nd.kind, r.x + r.h * 0.55, r.y + r.h / 2.0, mr);
        let clr = if nd.kind == SeaKind::Locked {
            rgb(CLR_SEA_GREY, 1.0)
        } else {
            WHITE
        };
        self.paint
            .text(dst, f, &nd.name, r.x + r.h, r.y + (r.h - b * 1.2) / 2.0, b, clr);
    }

    /// draw_sea_mark zeichnet die Marke eines Knotens um (cx, cy) mit Radius r.
    fn draw_sea_mark(&self, dst: &mut dyn Canvas, kind: SeaKind, cx: f64, cy: f64, r: f64) {
        let pt = &self.paint;
        match kind {
            SeaKind::Done => {
                pt.circle(dst, cx, cy, r, rgb(CLR_SEA_GREEN, 1.0));
                let lw = (r * 0.22).max(2.0) as f32;
                let white = straight(WHITE);
                dst.layer(Z_CONTENT);
                let p = |x: f64, y: f64| (x as f32, y as f32);
                let ((x0, y0), (x1, y1), (x2, y2)) = (
                    p(cx - r * 0.5, cy),
                    p(cx - r * 0.1, cy + r * 0.4),
                    p(cx + r * 0.55, cy - r * 0.4),
                );
                dst.stroke_line(x0, y0, x1, y1, lw, white);
                dst.stroke_line(x1, y1, x2, y2, lw, white);
            }
            SeaKind::Target => pt.ring_at(dst, cx, cy, r, WHITE),
            SeaKind::Locked => {
                // Schloss aus Bügel und Körper
                let grey = rgb(CLR_SEA_GREY, 1.0);
                pt.ring_at(dst, cx, cy - r * 0.25, r * 0.5, grey);
                pt.fill_top(dst, Rect::new(cx - r * 0.7, cy - r * 0.1, r * 1.4, r * 0.95), grey);
            }
        }
    }
}
