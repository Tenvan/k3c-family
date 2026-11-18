//! Port von `native/k3c-game/hud/drawsea_paper.go` (Teil 1 von 2): Seekarte als Papierschnitt-Diorama (B-964): Meer aus
//! Papierschichten, die Etappen als Spalten von links nach rechts, je Insel eine Miniatur ihrer Welt. Hier die Sheets
//! (`SeaArt`), die Lage in der Zelle (`SeaPlan`) und die Kacheln; die Szene (Inseln, Rahmen, Texte) steht in
//! `drawsea_scene.rs`. Maße in Asset-Pixeln, `u` rechnet sie in Bildpunkte um. Fehlt ein Asset, zeichnet `draw_sea` die
//! Formen (`drawsea.rs`).
//!
//! Abweichung von Go: Die Sheets liest `draw_cell` vor dem Ausleihen der Bibliothek (die Schrift hält sie geliehen) und
//! reicht sie als Werte (`SeaSheet`, `SeaArt`) herein; der Puffer der Insel-Sheets (`Hud::sea_isl`) wird je Bild
//! wiederverwendet.

use k3c_platform::Canvas;

use super::Hud;
use super::drawsea::SeaSizes;
use super::fonts::{WHITE, rgb};
use super::icons::ICON_ANIM;
use super::layout::Rect;
use super::seekarte::SeaView;
use crate::art::{Font, Library, SheetFrame};

/// SEA_ANIM ist die Animation von Meer und Miniaturen; Schiff, Aufsätze und Zeichen haben `ICON_ANIM`.
pub const SEA_ANIM: &str = "stand";
/// Breite einer Etappen-Spalte
pub const SEA_COL_W: f64 = 340.0;
/// Abstand der Wasserlinien zweier Reihen
pub const SEA_LANE_H: f64 = 90.0;
/// Platz über der Wasserlinie der hintersten Reihe (höchste Miniatur)
pub const SEA_TOP_H: f64 = 172.0;
/// Oberkante einer Welle über der Wasserlinie ihrer Reihe
pub const SEA_WAVE_UP: f64 = 30.0;
/// Oberkante der vordersten Welle unter der letzten Wasserlinie (ihr freier Rand liegt auf der Welle davor)
pub const SEA_FRONT_DROP: f64 = 36.0;
/// Höhe einer Papierschicht des Meers
pub const SEA_LAYER_H: f64 = 96.0;
/// seitlicher Versatz der Inseln einer Etappe von Reihe zu Reihe (im Wechsel links, rechts)
pub const SEA_STAGGER: f64 = 150.0;
/// Höhe einer Schicht unter der Insel
pub const SEA_SLAB_H: f64 = 7.0;
pub const CLR_SEA_SKY: u32 = 0xdfeaf0;
pub const CLR_SEA_SLAB_A: u32 = 0xb08a5e;
pub const CLR_SEA_SLAB_B: u32 = 0x8f6d4a;

/// SeaSheet ist das erste Bild eines Sheets mit seinen Maßen (Go: `*art.Sheet`, hier als Wert).
#[derive(Clone, Copy, Debug)]
pub struct SeaSheet {
    pub frame: SheetFrame,
    pub fw: f64,
    pub fh: f64,
    pub origin_x: f64,
    pub origin_y: f64,
}

/// sea_sheet liefert das Sheet des Assets (None ohne Asset, ungeladen oder leer).
pub fn sea_sheet(lib: &Library, asset: &str) -> Option<SeaSheet> {
    if asset.is_empty() {
        return None;
    }
    let s = lib.sheet(asset, SEA_ANIM).or_else(|| lib.sheet(asset, ICON_ANIM))?;
    if s.fw <= 0 || s.fh <= 0 {
        return None;
    }
    Some(SeaSheet {
        frame: *s.frame(0)?,
        fw: s.fw as f64,
        fh: s.fh as f64,
        origin_x: s.origin_x,
        origin_y: s.origin_y,
    })
}

/// SeaArt sind die Sheets der Karte: Meer, Schiff, Zeichen und je Insel Miniatur und Aufsatz in Reihenfolge der Knoten
/// (Zeile für Zeile).
#[derive(Clone, Debug, Default)]
pub struct SeaArt {
    pub back: Option<SeaSheet>,
    pub mid: Option<SeaSheet>,
    pub front: Option<SeaSheet>,
    pub ship: Option<SeaSheet>,
    pub flag: Option<SeaSheet>,
    pub fog: Option<SeaSheet>,
    /// `[Miniatur, Aufsatz]` je Knoten
    pub isl: Vec<[SeaSheet; 2]>,
}

impl SeaArt {
    /// island liefert Miniatur und Aufsatz des Knotens j der Zeile i von v.
    pub fn island(&self, v: &SeaView, i: usize, j: usize) -> Option<&[SeaSheet; 2]> {
        let before: usize = v.rows.iter().take(i).map(|r| r.nodes.len()).sum();
        self.isl.get(before + j)
    }
}

impl Hud {
    /// sea_art_of liefert die Sheets der Karte v; None, wenn nicht alle da sind, auch Miniatur und Aufsatz jeder Insel
    /// (R5: sonst Formen). Der Puffer `isl` kommt aus `Hud::sea_isl` und geht mit `draw_sea` zurück.
    pub fn sea_art_of(&mut self, v: &SeaView) -> Option<SeaArt> {
        let m = self.sea_map.as_ref()?;
        let lib = self.lib.as_ref()?.try_borrow().ok()?;
        let one = |a: &str| sea_sheet(&lib, a);
        let (back, mid, front) = (one(&m.sea.back)?, one(&m.sea.mid)?, one(&m.sea.front)?);
        let (ship, flag, fog) = (one(&m.ship)?, one(&m.marks.done)?, one(&m.marks.locked)?);
        let mut isl = std::mem::take(&mut self.sea_isl);
        isl.clear();
        for n in v.rows.iter().flat_map(|r| &r.nodes) {
            let l = &n.look;
            match (one(m.island(&l.world)), one(m.topper(l.seed))) {
                (Some(a), Some(b)) => isl.push([a, b]),
                _ => {
                    self.sea_isl = isl;
                    return None;
                }
            }
        }
        Some(SeaArt {
            back: Some(back),
            mid: Some(mid),
            front: Some(front),
            ship: Some(ship),
            flag: Some(flag),
            fog: Some(fog),
            isl,
        })
    }
}

/// SeaPlan ist die Lage des Dioramas in der Zelle: Tafel, Bildpunkte je Asset-Pixel `u`, Schriftgröße `b`, Rand `pad`,
/// linke Kante der ersten Spalte `x0`, Wasserlinie der hintersten Reihe `water`, Oberkante der vordersten Welle `front`,
/// Zahl der Reihen `lanes`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SeaPlan {
    pub box_: Rect,
    pub u: f64,
    pub b: f64,
    pub pad: f64,
    pub x0: f64,
    pub water: f64,
    pub front: f64,
    pub lanes: usize,
}

/// sea_paper_plan legt das Diorama mittig und etwas hoch in die Zelle; passt es nicht, schrumpft es samt Schrift (nie
/// über die Zelle hinaus). `m` sind die bei Schriftgröße 28·k gemessenen Texte.
pub fn sea_paper_plan(m: &SeaSizes, cols: usize, cell: Rect, k: f64) -> SeaPlan {
    let mut p = SeaPlan {
        u: k,
        b: 28.0 * k,
        lanes: m.max_nodes.max(1),
        ..SeaPlan::default()
    };
    p.pad = 0.6 * p.b;
    let mut head = p.pad + 2.5 * p.b; // Titel und Etappen-Zeile
    let mut cols_w = cols as f64 * SEA_COL_W * p.u;
    if p.lanes > 1 {
        cols_w += SEA_STAGGER * p.u; // Versatz der Reihen
    }
    let mut w = cols_w
        .max(2.0 * p.pad + m.title_w)
        .max(2.0 * p.pad + m.hint_w)
        .max(2.0 * p.pad + m.name_w);
    let sea = SEA_TOP_H + (p.lanes - 1) as f64 * SEA_LANE_H + SEA_FRONT_DROP + SEA_LAYER_H;
    let mut ht = head + sea * p.u;
    let s = (cell.w * 0.96 / w).min(cell.h * 0.9 / ht).min(1.0);
    (p.u, p.b, p.pad, head, w, ht) = (p.u * s, p.b * s, p.pad * s, head * s, w * s, ht * s);
    p.box_ = Rect::new(
        cell.x + (cell.w - w) / 2.0,
        cell.y.max(cell.y + (cell.h - ht) / 2.0 - 40.0 * k),
        w,
        ht,
    );
    p.x0 = p.box_.x + (w - cols as f64 * SEA_COL_W * p.u) / 2.0; // Spalten mittig, der Versatz liegt je zur Hälfte links und rechts
    p.water = p.box_.y + head + SEA_TOP_H * p.u;
    p.front = p.water + ((p.lanes - 1) as f64 * SEA_LANE_H + SEA_FRONT_DROP) * p.u;
    p
}

impl SeaPlan {
    /// water_of ist die Wasserlinie der Reihe lane (0 = hinten).
    pub fn water_of(&self, lane: usize) -> f64 {
        self.water + lane as f64 * SEA_LANE_H * self.u
    }

    /// node_x ist die Mitte der Insel in Spalte col und Reihe lane von n Inseln der Etappe: eine allein mittig, mehrere
    /// im Wechsel links und rechts der Mitte, damit die vordere die hintere nicht verdeckt.
    pub fn node_x(&self, col: usize, lane: usize, n: usize) -> f64 {
        let mut x = self.x0 + (col as f64 + 0.5) * SEA_COL_W * self.u;
        if n > 1 {
            x += ((lane % 2) as f64 - 0.5) * SEA_STAGGER * self.u;
        }
        x
    }
}

impl Hud {
    /// draw_sea_paper zeichnet das Diorama der Karte v mit den Sheets a in die Zelle cell.
    pub fn draw_sea_paper(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        v: &mut SeaView,
        a: &SeaArt,
        cell: Rect,
        k: f64,
    ) {
        let p = sea_paper_plan(&v.sizes, v.rows.len(), cell, k);
        self.draw_sea_scene(dst, f, v, a, &p, &SeaExtra::default());
    }

    /// draw_sea_strip kachelt die Papierschicht s ab (x, y) über die Breite w: so viele ganze Kacheln, wie am besten
    /// passen, leicht gestreckt statt beschnitten, je 1 Bildpunkt überlappt (keine Naht).
    pub fn draw_sea_strip(&self, dst: &mut dyn Canvas, s: &SeaSheet, x: f64, y: f64, w: f64, u: f64) {
        let n = ((w / (s.fw * u)).round() as i64).max(1);
        let tw = w / n as f64;
        for i in 0..n {
            let over = if i == n - 1 { 0.0 } else { 1.0 };
            let r = Rect::new(x + i as f64 * tw, y, tw + over, s.fh * u);
            self.paint.stamp(dst, s.frame.tex, s.frame.src, r, WHITE);
        }
    }

    /// draw_sea_sprite zeichnet Bild 0 von s im Maßstab sc mit dem Fußpunkt bei (x, y), eingefärbt mit clr.
    pub fn draw_sea_sprite(&self, dst: &mut dyn Canvas, s: &SeaSheet, x: f64, y: f64, sc: f64, clr: [u8; 4]) {
        let (w, ht) = (s.fw * sc, s.fh * sc);
        let r = Rect::new(x - s.origin_x * w, y - s.origin_y * ht, w, ht);
        self.paint.stamp(dst, s.frame.tex, s.frame.src, r, clr);
    }
}

/// SeaExtra sind die Zusätze der Schiffsfahrt im Diorama (`drawvoyage.rs`, RU23.5): das fahrende Schiff mit Fußpunkt
/// (x, y), gezeichnet nach den Inseln der Reihe `lane` (es liegt vor ihnen), und die bewegte vordere Welle (um `shift`
/// Bildpunkte versetzt, beschnitten auf den Ausschnitt `front`). Der Nullwert zeichnet die Karte am Hafen.
#[derive(Clone, Copy, Debug, Default)]
pub struct SeaExtra {
    pub ship: bool,
    pub lane: usize,
    pub x: f64,
    pub y: f64,
    pub moving: bool,
    pub shift: f64,
}

/// tint ist die weiße Tönung mit Deckung alpha.
pub fn tint(alpha: f64) -> [u8; 4] {
    rgb(0xffffff, alpha)
}
