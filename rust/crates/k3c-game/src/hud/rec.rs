//! Anzeige des Spieltests (WZ66, B-1220): REC-Punkt einmal je Fenster und „Markiert“ in der Zelle des Spielers.
//! Reine Zeichnung ohne Zustand; die Logik liegt in `playtest`. REC steht unten mittig im Fenster (die HUD-Zeilen sitzen
//! oben in jeder Zelle, die Anzeige der Bildrate oben rechts) und ist am TV gut lesbar.

use k3c_platform::{Canvas, Rect};

use crate::view::cell::Rectangle;

const RED: [f32; 4] = [0.92, 0.12, 0.12, 1.0];
const BACK: [f32; 4] = [0.0, 0.0, 0.0, 0.6];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

/// Zeichnet den REC-Punkt mit „REC“ unten mittig; im Fehlerzustand (`failed`) durchgestrichen und grau.
pub fn draw_rec(c: &mut dyn Canvas, failed: bool) {
    let (w, h) = c.size();
    let (bw, bh) = (150.0, 56.0);
    let (x, y) = ((w - bw) / 2.0, h - bh - 24.0);
    c.rect(Rect::new(x, y, bw, bh), BACK);
    let (dot, text) = if failed {
        ([0.6, 0.6, 0.6, 1.0], [0.7, 0.7, 0.7, 1.0])
    } else {
        (RED, WHITE)
    };
    c.fill_circle(x + 30.0, y + bh / 2.0, 12.0, dot);
    c.text("REC", x + 56.0, y + 12.0, 32.0, text);
    if failed {
        c.stroke_line(x + 12.0, y + bh - 10.0, x + bw - 12.0, y + 10.0, 5.0, RED);
    }
}

/// Zeichnet „Markiert“ mittig im unteren Drittel der Zelle `cell` (Fensterpixel).
pub fn draw_marked(c: &mut dyn Canvas, cell: &Rectangle) {
    let (cx, w) = ((cell.min.x + cell.max.x) as f32 / 2.0, (cell.max.x - cell.min.x) as f32);
    let y = cell.min.y as f32 + (cell.max.y - cell.min.y) as f32 * 0.72;
    let bw = (w * 0.5).clamp(200.0, 320.0);
    c.rect(Rect::new(cx - bw / 2.0, y, bw, 56.0), BACK);
    c.fill_circle(cx - bw / 2.0 + 26.0, y + 28.0, 9.0, RED);
    c.text("MARKIERT", cx - bw / 2.0 + 48.0, y + 12.0, 32.0, WHITE);
}
