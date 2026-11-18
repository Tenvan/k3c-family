//! Port von `native/k3c-game/hud/drawmenu.go`: Zeichnen des Skill-Menüs (`skillMenuView.ts`) mittig in der Zelle, etwas
//! hoch, mit der Glyph der Slot-Taste vor belegten Skills (`drawglyph.rs`) und dem Bedienhinweis mit Tasten-Icons
//! (`keyline.rs`). Schrift 36 px in breiten, 32 px in schmalen Zellen, kleiner bis das Menü passt, nie unter 28 px; mit
//! der Debug-Schrift fest.

use k3c_platform::Canvas;

use super::Hud;
use super::draw::rect_of;
use super::fonts::{ROLE_BOSS, WHITE, YELLOW, rgb};
use super::layout::Rect;
use super::paint::TextSlot;
use super::skilltext::MENU_WINDOW;
use super::state::SeatState;
use crate::art::Font;
use crate::view::cell::Cell;

pub const MENU_PX_FULL: f64 = 36.0;
pub const MENU_PX_HALF: f64 = 32.0;
pub const MENU_PX_MIN: f64 = 28.0;
pub const MENU_PAD_X: f64 = 24.0;
pub const MENU_PAD_Y: f64 = 18.0;
pub const MENU_BACK: u32 = 0x0c1024;

/// Abstände im Skill-Menü je px Schriftgröße: unter der Kopfzeile und zwischen zwei Zeilen. Ohne sie berühren sich die
/// Konturen der Texte (`OUTLINE8`) und die Unterlängen von Go Regular (B-813).
pub const MENU_HEAD_GAP: f64 = 0.35;
pub const MENU_ROW_GAP: f64 = 0.15;

/// glyph_px ist die Glyph-Höhe zur Schriftgröße (glyphs.ts: 28 px → 36 px).
pub fn glyph_px(font_px: f64) -> f64 {
    (font_px * 1.3).ceil()
}

/// menu_size wählt die Schriftgröße: Grundgröße `base`, verkleinert im Verhältnis, bis Breite `w` und Höhe `h` (gemessen
/// bei `base`) in `max_w` × `max_h` passen, nie unter `min_px`.
pub fn menu_size(base: f64, w: f64, h: f64, max_w: f64, max_h: f64, min_px: f64) -> f64 {
    if w <= 0.0 || h <= 0.0 {
        return base;
    }
    let px = base * 1.0_f64.min(max_w / w).min(max_h / h);
    min_px.max((px / 2.0).floor() * 2.0)
}

/// MenuBox ist die Lage des Skill-Menüs: Kasten, Kopfzeile, Zeilen und Hinweis untereinander, dazu Schriftgröße `size`
/// und der Maßstab `s` der bei Grundgröße gemessenen Maße.
#[derive(Clone, Copy, Debug, Default)]
pub struct MenuBox {
    pub box_: Rect,
    pub title: Rect,
    pub footer: Rect,
    pub rows: [Rect; MENU_WINDOW],
    pub size: f64,
    pub s: f64,
}

/// MenuSizes sind Breite und Höhe von Kopfzeile (Index 0), n Zeilen (1 … n) und Hinweis (n+1) bei Grundgröße.
#[derive(Clone, Copy, Debug, Default)]
pub struct MenuSizes {
    pub w: [f64; MENU_WINDOW + 2],
    pub h: [f64; MENU_WINDOW + 2],
    pub n: usize,
}

/// layout_menu legt das Menü mittig und etwas hoch in die Zelle `cell` (rein): Größe wie `menu_size`, mit der
/// Debug-Schrift (`fixed`) fest; Kopfzeile, Zeilen und Hinweis überlappen nicht.
pub fn layout_menu(cell: Rect, k: f64, base: f64, m: &MenuSizes, fixed: bool) -> MenuBox {
    let (mut w, mut ht) = (0.0_f64, 0.0_f64);
    for i in 0..m.n + 2 {
        w = w.max(m.w[i]);
        ht += m.h[i];
    }
    ht += (2.0 * MENU_HEAD_GAP + m.n.saturating_sub(1) as f64 * MENU_ROW_GAP) * base;
    let mut out = MenuBox {
        size: base,
        s: 1.0,
        ..MenuBox::default()
    };
    if !fixed {
        out.size = menu_size(
            base,
            w + 2.0 * MENU_PAD_X * k,
            ht + 2.0 * MENU_PAD_Y * k,
            cell.w * 0.96,
            cell.h * 0.9,
            MENU_PX_MIN * k,
        );
        out.s = out.size / base;
    }
    let s = out.s;
    out.box_ = Rect::new(0.0, 0.0, w * s + 2.0 * MENU_PAD_X * k, ht * s + 2.0 * MENU_PAD_Y * k);
    out.box_.x = cell.x + (cell.w - out.box_.w) / 2.0;
    out.box_.y = cell.y.max(cell.y + (cell.h - out.box_.h) / 2.0 - 40.0 * k);
    let (x, mut y) = (out.box_.x + MENU_PAD_X * k, out.box_.y + MENU_PAD_Y * k);
    out.title = Rect::new(x, y, m.w[0] * s, m.h[0] * s);
    y += out.title.h + MENU_HEAD_GAP * base * s;
    for i in 0..m.n {
        out.rows[i] = Rect::new(x, y, m.w[i + 1] * s, m.h[i + 1] * s);
        y += out.rows[i].h + MENU_ROW_GAP * base * s;
    }
    if m.n > 0 {
        y += (MENU_HEAD_GAP - MENU_ROW_GAP) * base * s;
    }
    out.footer = Rect::new(x, y, m.w[m.n + 1] * s, m.h[m.n + 1] * s);
    out
}

impl Hud {
    /// draw_menu misst das Menü bei Grundgröße, legt es ab (`layout_menu`) und zeichnet Titel, Zeilen (Cursor hell) und
    /// Hinweis. Ohne Schrift nichts.
    pub fn draw_menu(&mut self, dst: &mut dyn Canvas, st: &mut SeatState, c: &Cell, k: f64, mut f: Option<&mut Font>) {
        let p = c.player;
        let Some(fixed) = f.as_deref().map(|f| f.fallback) else {
            return;
        };
        let mut base = MENU_PX_HALF * k;
        if c.rect.dx() as f64 / k >= 1400.0 {
            base = MENU_PX_FULL * k;
        }
        let v = &mut st.menu_v;
        let mut m = MenuSizes {
            n: v.n,
            ..MenuSizes::default()
        };
        (m.w[0], m.h[0]) = self
            .paint
            .measure_text(f.as_deref(), p, TextSlot::MENU_TITLE, &v.title, base);
        for i in 0..v.n {
            let slot = TextSlot::menu_row(i);
            (m.w[i + 1], m.h[i + 1]) = self.paint.measure_text(f.as_deref(), p, slot, &v.rows[i].text, base);
            let g = &v.rows[i].glyph;
            if !g.key.is_empty() {
                m.w[i + 1] += self.glyph_width(f.as_deref(), p, TextSlot::menu_cap(i), g, glyph_px(base)) + 8.0 * k;
                m.h[i + 1] = m.h[i + 1].max(glyph_px(base));
            }
        }
        let n = v.n;
        (m.w[n + 1], m.h[n + 1]) = self
            .paint
            .measure_text(f.as_deref(), p, TextSlot::MENU_FOOTER, &v.footer, base);
        if let (w, true) = self.line_width(f.as_deref(), &mut v.line, base * 0.8) {
            m.w[n + 1] = w; // Hinweis mit Tasten-Icons
        }
        let b = layout_menu(rect_of(c.rect), k, base, &m, fixed);
        // Ebene des Inhalts (nicht `fill`): Go kennt keine Ebenen, das Menü liegt über den Elementen und verdeckt sie
        self.paint.fill_top(dst, b.box_, rgb(MENU_BACK, 0.8));
        self.paint.text(
            dst,
            f.as_deref_mut(),
            &v.title,
            b.title.x,
            b.title.y,
            b.size,
            ROLE_BOSS.clr,
        );
        for i in 0..n {
            let r = b.rows[i];
            self.menu_row_at(dst, f.as_deref_mut(), st, c, i, r, b.size, k);
        }
        let v = &mut st.menu_v;
        self.draw_hint(dst, f, &mut v.line, &v.footer, b.footer.x, b.footer.y, b.size * 0.8);
    }

    /// menu_row_at zeichnet Zeile `i` im Rechteck `r`: Glyph (wenn belegt) und Text; die Zeile unter dem Cursor gelb.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    fn menu_row_at(
        &mut self,
        dst: &mut dyn Canvas,
        mut f: Option<&mut Font>,
        st: &SeatState,
        c: &Cell,
        i: usize,
        r: Rect,
        size: f64,
        k: f64,
    ) {
        let row = &st.menu_v.rows[i];
        let clr = if row.selected { YELLOW } else { WHITE };
        let mut x = r.x;
        if !row.glyph.key.is_empty() {
            let gh = glyph_px(size);
            x += self.draw_glyph(
                dst,
                f.as_deref_mut(),
                c.player,
                TextSlot::menu_glyph(i),
                &row.glyph,
                x,
                r.y + r.h / 2.0,
                gh,
            ) + 8.0 * k;
        }
        self.paint
            .text(dst, f, &row.text, x, r.y + (r.h - size * 1.2) / 2.0, size, clr);
    }
}
