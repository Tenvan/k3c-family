//! Zeichnet den Fragebogen (WZ66.3): ein Feld je Spieler in seiner Ansicht (bei zwei Spielern oben und unten wie im
//! Split-Screen), große Schrift für den TV. Zeichnet nur, der Zustand liegt in `survey`.

use k3c_platform::{Canvas, Rect};

use super::survey::{OPTIONS, QUESTIONS, Sheet, Survey};

const DIM: [f32; 4] = [0.0, 0.0, 0.0, 0.82];
const PANEL: [f32; 4] = [0.10, 0.10, 0.16, 0.96];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREY: [f32; 4] = [0.55, 0.55, 0.62, 1.0];
const GOLD: [f32; 4] = [1.0, 0.82, 0.2, 1.0];

/// Zeichnet den Bogen über das Spiel.
pub fn draw(c: &mut dyn Canvas, s: &Survey) {
    let (w, h) = c.size();
    c.layer(32);
    c.clip(None);
    c.rect(Rect::new(0.0, 0.0, w, h), DIM);
    let n = s.sheets.len().max(1) as f32;
    for (i, sh) in s.sheets.iter().enumerate() {
        let ph = h / n;
        draw_sheet(c, sh, Rect::new(0.0, ph * i as f32, w, ph));
    }
}

fn draw_sheet(c: &mut dyn Canvas, s: &Sheet, area: Rect) {
    let (pw, ph) = (area.w.min(1100.0), area.h.min(460.0));
    let (x, y) = (area.x + (area.w - pw) / 2.0, area.y + (area.h - ph) / 2.0);
    c.rect(Rect::new(x, y, pw, ph), PANEL);
    c.text(&format!("SPIELER {}", s.player), x + 30.0, y + 20.0, 34.0, GOLD);
    if s.finished() {
        c.text("DANKE!", x + 30.0, y + ph / 2.0 - 20.0, 56.0, WHITE);
        return;
    }
    c.text(
        &format!("FRAGE {} VON {}", s.q + 1, QUESTIONS.len()),
        x + pw - 330.0,
        y + 24.0,
        28.0,
        GREY,
    );
    c.text(QUESTIONS[s.q], x + 30.0, y + 80.0, 56.0, WHITE);
    draw_options(c, s, Rect::new(x + 30.0, y + 180.0, pw - 60.0, 110.0));
    c.text(
        "A OK    B UEBERSPRINGEN    START BOGEN BEENDEN",
        x + 30.0,
        y + ph - 56.0,
        26.0,
        GREY,
    );
}

fn draw_options(c: &mut dyn Canvas, s: &Sheet, r: Rect) {
    let opts = OPTIONS[s.q];
    let gap = 16.0;
    let bw = (r.w - gap * (opts.len() as f32 - 1.0)) / opts.len() as f32;
    for (i, o) in opts.iter().enumerate() {
        let x = r.x + i as f32 * (bw + gap);
        let on = i == s.sel[s.q];
        let fill = if s.q == 0 { i <= s.sel[0] } else { on };
        c.rect(
            Rect::new(x, r.y, bw, r.h),
            if fill {
                [0.3, 0.26, 0.08, 1.0]
            } else {
                [0.16, 0.16, 0.22, 1.0]
            },
        );
        if on {
            c.stroke_rect(Rect::new(x, r.y, bw, r.h), 6.0, GOLD);
        }
        let col = if fill { GOLD } else { GREY };
        if s.q == 0 {
            c.fill_circle(x + bw / 2.0, r.y + r.h / 2.0, 30.0, col);
        } else {
            c.text(o, x + 24.0, r.y + r.h / 2.0 - 20.0, 40.0, col);
        }
    }
}
