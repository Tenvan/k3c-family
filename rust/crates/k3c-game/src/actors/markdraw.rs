//! Port von `native/k3c-game/actors/markdraw.go`.
//!
//! Zeichnen der Anzeigen (marks.rs): Rechtecke direkt über `Canvas::rect`, Ellipsen als eingefärbte weiße Scheibe
//! (Vorlage); je Bild entsteht nichts Neues. Die Vorlage entsteht beim ersten Zeichnen.
//!
//! Abweichung von Go: Der weiße Bildpunkt von Go (Vorlage für Rechtecke) entfällt, `Canvas::rect` füllt dasselbe;
//! Matrix und Farbskala (`op`) liegen als Werte im Aufruf. `tint` liefert die gerade (nicht vormultiplizierte) Farbe,
//! die `Canvas` erwartet, gerechnet aus den vormultiplizierten Bytes von Go.

use k3c_platform::{Canvas, Rect, TexId};
use k3c_sim::state::{Enemy, Player, Troop};

use super::Actors;
use super::catalog::UNIT_PX;
use super::combatmarks::{BARE_ALPHA, CROWN_ABOVE};
use super::draw::rig_size;
use super::figures::Figure;
use super::marks::{
    BAR_BACK, BAR_BACK_ALPHA, BarLook, BoxPx, EDGE_WIDTH, ENEMY_BAR, LOAD_EDGE, LOAD_FILL, LOOT_ABOVE, LOOT_EDGE,
    LOOT_FILL, LOOT_RADIUS, MARKER_ALPHA, TROOP_BAR, bar_boxes, bar_fraction, bar_visible, carrying, has_loot,
    load_box, loot_center, marker_box, player_color,
};
use crate::view::cell::Cell;

/// Kantenlänge der Scheiben-Vorlage in Bildpunkten
pub const STENCIL_PX: u32 = 64;

/// marksHalf: halbe Breite der Anzeigen in Units (Balken 50 px, Punkt 56 px) für die Sichtprüfung.
pub const MARKS_HALF: f64 = 2.0;

/// ink hält die Vorlage der Anzeigen (weiße Scheibe mit glattem Rand).
#[derive(Clone, Debug, Default)]
pub struct Ink {
    pub disc: Option<TexId>,
}

/// newInk legt die Scheiben-Vorlage an (weiß, vormultipliziert, 4×4 Unterabtastung für den glatten Rand).
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn new_ink(canvas: &mut dyn Canvas) -> Ink {
    let n = STENCIL_PX as usize;
    let r = n as f64 / 2.0;
    let mut rgba = vec![0u8; n * n * 4];
    for y in 0..n {
        for x in 0..n {
            let mut hit = 0;
            for sy in 0..4 {
                for sx in 0..4 {
                    let px = x as f64 + (f64::from(sx) + 0.5) / 4.0 - r;
                    let py = y as f64 + (f64::from(sy) + 0.5) / 4.0 - r;
                    if px * px + py * py <= r * r {
                        hit += 1;
                    }
                }
            }
            let a = (hit * 255 / 16) as u8;
            let i = (y * n + x) * 4;
            rgba[i..i + 4].copy_from_slice(&[a, a, a, a]);
        }
    }
    Ink {
        disc: Some(canvas.texture(&rgba, STENCIL_PX, STENCIL_PX)),
    }
}

/// tint ist 0xRRGGBB mit Deckkraft a (0 … 1) als gerade Farbe; Go rechnet vormultipliziert in Bytes (abgeschnitten),
/// das hier daraus zurück.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn tint(hex: u32, a: f64) -> [f32; 4] {
    let a = k3c_det::fmax(0.0, k3c_det::fmin(1.0, a));
    let ch = |shift: u32| (f64::from(hex >> shift & 0xff) * a) as u8;
    let (r, g, b, a8) = (ch(16), ch(8), ch(0), (255.0 * a) as u8);
    if a8 == 0 {
        return [0.0; 4];
    }
    let d = f32::from(a8);
    [f32::from(r) / d, f32::from(g) / d, f32::from(b) / d, d / 255.0]
}

/// anchor ist der Bezugspunkt der Anzeigen einer Figur: Fußpunkt (sx, sy) in Fenster-px, s Fenster-px je Spiel-Pixel.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Anchor {
    pub sx: f64,
    pub sy: f64,
    pub s: f64,
}

#[allow(clippy::cast_possible_truncation)]
fn area(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect::new(x as f32, y as f32, w as f32, h as f32)
}

impl Ink {
    /// rect füllt b (px über dem Fußpunkt, y nach oben) mit clr; grow vergrößert es rundum um so viele px.
    pub fn rect(&self, dst: &mut dyn Canvas, a: Anchor, b: BoxPx, grow: f64, clr: [f32; 4]) {
        let (w, h) = (b.w + 2.0 * grow, b.h + 2.0 * grow);
        if w <= 0.0 || h <= 0.0 || clr[3] == 0.0 {
            return;
        }
        let (x, y) = (a.sx + (b.cx - w / 2.0) * a.s, a.sy - (b.cy + h / 2.0) * a.s);
        dst.rect(area(x, y, w * a.s, h * a.s), clr);
    }

    /// oval füllt die Ellipse in b.
    #[allow(clippy::cast_precision_loss)]
    pub fn oval(&self, dst: &mut dyn Canvas, a: Anchor, b: BoxPx, clr: [f32; 4]) {
        let Some(disc) = self.disc else {
            return;
        };
        if b.w <= 0.0 || b.h <= 0.0 || clr[3] == 0.0 {
            return;
        }
        let src = Rect::new(0.0, 0.0, STENCIL_PX as f32, STENCIL_PX as f32);
        let (x, y) = (a.sx + (b.cx - b.w / 2.0) * a.s, a.sy - (b.cy + b.h / 2.0) * a.s);
        dst.sprite(disc, src, area(x, y, b.w * a.s, b.h * a.s), clr);
    }

    /// bar zeichnet den Lebensbalken: schwarzer Hintergrund, Füllung in der Farbe des Balkens.
    pub fn bar(&self, dst: &mut dyn Canvas, a: Anchor, l: BarLook, top_px: f64, frac: f64, alpha: f64) {
        let (back, fill) = bar_boxes(l, top_px, frac);
        self.rect(dst, a, back, 0.0, tint(BAR_BACK, BAR_BACK_ALPHA * alpha));
        self.rect(dst, a, fill, 0.0, tint(l.color, alpha));
    }
}

/// marksAnchor liefert den Bezugspunkt der Anzeigen der Figur fg bei Welt-x x, wenn sie in die Zelle ragen.
pub fn marks_anchor(c: &Cell, fg: &Figure, x: f64) -> Option<Anchor> {
    if !c.camera.visible(x, MARKS_HALF) {
        return None;
    }
    Some(Anchor {
        sx: c.camera.screen_x(x),
        sy: c.screen_y(fg.lift),
        s: c.camera.scale / UNIT_PX,
    })
}

impl Actors {
    /// topPx ist die Oberkante der gezeichneten Figur über dem Fußpunkt in Spiel-Pixeln (Rig: Rahmen-Oberkante, beritten
    /// samt Reittier; Sheet: Höhe über dem Fuß-Ursprung).
    pub fn top_px(&self, fg: &Figure) -> f64 {
        if fg.actor.is_some() {
            let (size, frame) = rig_size(fg);
            return -frame[1] * size * UNIT_PX;
        }
        if let Some(lib) = &self.lib
            && let Some(s) = lib.borrow().sheet(&fg.look.asset, &fg.pose.anim)
        {
            return fg.look.height * s.origin_y * UNIT_PX;
        }
        fg.look.height * UNIT_PX
    }

    /// troopMarks zeichnet Last und Lebensbalken einer Truppe.
    pub fn troop_marks(&self, dst: &mut dyn Canvas, c: &Cell, fg: &Figure, t: &Troop) {
        let (Some(ink), Some(a)) = (&self.ink, marks_anchor(c, fg, t.x)) else {
            return;
        };
        let top = self.top_px(fg);
        if carrying(t.job.as_ref()) {
            let b = load_box(top, fg.left);
            ink.rect(dst, a, b, EDGE_WIDTH / 2.0, tint(LOAD_EDGE, fg.ui));
            ink.rect(dst, a, b, -EDGE_WIDTH / 2.0, tint(LOAD_FILL, fg.ui));
        }
        self.leader_marks(dst, a, fg, t, top);
        if bar_visible(t.hp, t.max_hp) {
            ink.bar(dst, a, TROOP_BAR, top, bar_fraction(t.hp, t.max_hp), fg.ui);
        }
    }

    /// enemyMarks zeichnet Goldkreis und Lebensbalken eines Gegners.
    pub fn enemy_marks(&self, dst: &mut dyn Canvas, c: &Cell, fg: &Figure, e: &Enemy) {
        let (Some(ink), Some(a)) = (&self.ink, marks_anchor(c, fg, e.x)) else {
            return;
        };
        let top = self.top_px(fg);
        if has_loot(e.carried_gold) {
            let (cx, cy) = loot_center(top);
            let d = 2.0 * LOOT_RADIUS;
            let oval = |w: f64| BoxPx { cx, cy, w, h: w };
            ink.oval(dst, a, oval(d + EDGE_WIDTH), tint(LOOT_EDGE, fg.ui));
            ink.oval(dst, a, oval(d - EDGE_WIDTH), tint(LOOT_FILL, fg.ui));
        }
        if e.crown.is_some() {
            // Kronenraub: der Gegner trägt die Krone über dem Goldkreis
            self.crown(dst, a, (0.0, top + LOOT_ABOVE + 2.0 * LOOT_RADIUS + CROWN_ABOVE), fg.ui);
        }
        if bar_visible(e.hp, e.max_hp) {
            ink.bar(dst, a, ENEMY_BAR, top, bar_fraction(e.hp, e.max_hp), fg.ui);
        }
    }

    /// playerMarker zeichnet den Punkt in der Spielerfarbe unter den Füßen (vor der Figur).
    pub fn player_marker(&self, dst: &mut dyn Canvas, c: &Cell, fg: &Figure, p: &Player) {
        let (Some(ink), Some(a)) = (&self.ink, marks_anchor(c, fg, p.x)) else {
            return;
        };
        let mut alpha = MARKER_ALPHA;
        if p.crown.is_some() {
            // ohne Krone: Spielerfarbe bleibt, der Punkt ist gedämpft
            alpha *= BARE_ALPHA;
        }
        ink.oval(dst, a, marker_box(), tint(player_color(p.index), alpha * fg.ui));
    }
}
