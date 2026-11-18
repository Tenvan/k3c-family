//! Port von `native/k3c-game/devtools/drawcheats.go`.
//!
//! Zeichnen des Cheat-Menüs in der Zelle des Spielers: Titel, Reiter der Gruppen, sichtbarer Ausschnitt der Einträge um
//! die Auswahl, Ergebnis der letzten Aktion und Bedienhinweis. Schriftgröße nach der Zellenhöhe (TV-tauglich).

use k3c_platform::Canvas;

use super::DevOverlay;
use super::cheatlist::cheat_groups;
use super::cheatmenu::{CheatState, label};
use super::{BAD_COLOR, DIM_COLOR, KEY_COLOR, SEL_COLOR, TEXT_COLOR, boxed, fill};
use crate::art::{Font, LINE_SPACING};
use crate::view::cell::Cell;

/// CHEAT_ROWS ist die Zahl der sichtbaren Einträge.
pub const CHEAT_ROWS: usize = 9;

/// CHEAT_HINT ist der Bedienhinweis unter den Einträgen.
pub const CHEAT_HINT: &str = "links/rechts Gruppe · hoch/runter Eintrag · A/Leertaste auslösen · B/F9 schließen";

/// window liefert den ersten sichtbaren Eintrag, damit `row` bei `n` Einträgen in einem Fenster von `size` Zeilen liegt.
pub fn window(row: usize, n: usize, size: usize) -> usize {
    if n <= size {
        return 0;
    }
    row.saturating_sub(size / 2).min(n - size)
}

impl DevOverlay {
    /// draw_cheats zeichnet das Menü `st` in die Zelle `c`.
    #[allow(clippy::cast_precision_loss)]
    pub(super) fn draw_cheats(&self, dst: &mut dyn Canvas, font: Option<&mut Font>, c: &Cell, st: &CheatState) {
        let r = c.rect;
        let px = (r.dy() as f64 / 24.0).max(14.0);
        let line_h = px * LINE_SPACING;
        let gap = px * 0.6;
        let w = (r.dx() as f64 - 2.0 * gap).min(px * 24.0);
        let h = line_h * (CHEAT_ROWS + 4) as f64 + 2.0 * gap;
        let (x, y) = (r.min.x as f64 + gap, r.min.y as f64 + gap);
        boxed(dst, x, y, w, h);
        let Some(g) = cheat_groups().get(st.group) else {
            return;
        };
        let first = window(st.row, g.entries.len(), CHEAT_ROWS);
        let sel = y + gap + line_h * (2 + st.row.saturating_sub(first)) as f64;
        fill(dst, x, sel, w, line_h, SEL_COLOR);
        let Some(font) = font else { return };
        font.draw(
            dst,
            &format!("Cheats · Spieler {}", c.player + 1),
            x + gap,
            y + gap,
            px,
            KEY_COLOR,
        );
        let mut tx = x + gap;
        for (i, gr) in cheat_groups().iter().enumerate() {
            let clr = if i == st.group { TEXT_COLOR } else { DIM_COLOR };
            font.draw(dst, gr.name, tx, y + gap + line_h, px, clr);
            tx += font.measure(gr.name, px).0 + gap * 1.5;
        }
        for (n, e) in g.entries.iter().skip(first).take(CHEAT_ROWS).enumerate() {
            let ry = y + gap + line_h * (2 + n) as f64;
            font.draw(dst, &label(&self.toggles, e), x + gap * 2.0, ry, px, TEXT_COLOR);
        }
        let small = px * 0.8;
        if !st.status.is_empty() {
            let clr = if st.bad { BAD_COLOR } else { TEXT_COLOR };
            let sy = y + gap + line_h * (CHEAT_ROWS + 2) as f64;
            font.draw(dst, &st.status, x + gap, sy, small, clr);
        }
        font.draw(
            dst,
            CHEAT_HINT,
            x + gap,
            y + gap + line_h * (CHEAT_ROWS + 3) as f64,
            small,
            DIM_COLOR,
        );
    }
}
