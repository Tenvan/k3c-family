//! Port von `native/k3c-game/game/notice.go`: Meldung im Fenster (B-810): Fehler und Ladeanzeige als große Pixel-Schrift
//! in der Mitte des Fensters. Die Pixel-Schrift (`Canvas::text`) braucht weder die Schrift-Datei noch Grafik, die
//! Meldung erscheint also auch, wenn genau die fehlen; sie kennt nur ASCII, Umlaute und Emojis werden vorher
//! umgeschrieben bzw. weggelassen. Text und Maßstab sind reine Funktionen.
//!
//! Abweichung: Go zeichnet erst in ein Textbild und skaliert es; Rust zeichnet jede Zeile mit `Canvas::text` im Maßstab
//! `notice_scale` an ihre Stelle im Fenster (gleiche Lage und Größe, kein eigenes Bild).

use k3c_platform::{Canvas, Rect};

/// Maße der Debug-Schrift (`ebitenutil.DebugPrintAt`): Zeichenbreite und Zeilenhöhe in px; `NOTICE_PAD` ist der Rand des
/// Bilds.
pub const NOTICE_COLS: usize = 56;
pub const NOTICE_CHAR_W: i64 = 6;
pub const NOTICE_LINE_H: i64 = 16;
pub const NOTICE_PAD: i64 = 8;
pub const NOTICE_MAX_ZOOM: i64 = 6;

/// Zeichenhöhe der Pixel-Schrift in der 16er-Zeile des Textbilds (5×7-Raster, wie `art::Font` im Rückfall).
const NOTICE_PX: f32 = 12.0;

const NOTICE_BACK: [f32; 4] = [16.0 / 255.0, 16.0 / 255.0, 24.0 / 255.0, 1.0];
const NOTICE_TEXT: [f32; 4] = [1.0; 4];

/// ascii_text schreibt Umlaute um (ä → ae, ß → ss) und lässt alle anderen Zeichen außerhalb von ASCII weg (Emojis); eine
/// Lücke vor dem weggelassenen Zeichen bleibt nicht stehen.
pub fn ascii_text(s: &str) -> String {
    let mut b = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'ä' => b.push_str("ae"),
            'ö' => b.push_str("oe"),
            'ü' => b.push_str("ue"),
            'Ä' => b.push_str("Ae"),
            'Ö' => b.push_str("Oe"),
            'Ü' => b.push_str("Ue"),
            'ß' => b.push_str("ss"),
            c if c.is_ascii() => b.push(c),
            _ => {}
        }
    }
    b.trim().to_string()
}

/// Teilt `w` nach höchstens `cols` Bytes an einer Zeichengrenze (Go schneidet Bytes; nach `ascii_text` ist das gleich).
fn split_at_cols(w: &str, cols: usize) -> (&str, &str) {
    let mut i = cols.min(w.len());
    while i > 0 && !w.is_char_boundary(i) {
        i -= 1;
    }
    if i == 0 {
        i = w.chars().next().map_or(0, char::len_utf8);
    }
    w.split_at(i)
}

/// wrap_text bricht `s` in Zeilen von höchstens `cols` Zeichen: an Leerzeichen, ein längeres Wort (Pfad) hart. Leere
/// Zeilen bleiben, damit Absätze erhalten bleiben.
pub fn wrap_text(s: &str, cols: usize) -> Vec<String> {
    if s.is_empty() {
        return vec![String::new()];
    }
    let cols = cols.max(1);
    let mut out = Vec::new();
    let mut line = String::new();
    for mut w in s.split_whitespace() {
        while w.len() > cols {
            if !line.is_empty() {
                out.push(std::mem::take(&mut line));
            }
            let (head, tail) = split_at_cols(w, cols);
            out.push(head.to_string());
            w = tail;
        }
        if line.is_empty() {
            line.push_str(w);
        } else if line.len() + 1 + w.len() <= cols {
            line.push(' ');
            line.push_str(w);
        } else {
            out.push(std::mem::replace(&mut line, w.to_string()));
        }
    }
    if !line.is_empty() {
        out.push(line);
    }
    out
}

/// notice_lines macht aus den Absätzen die Zeilen der Meldung (ASCII, umbrochen).
pub fn notice_lines(paras: &[&str]) -> Vec<String> {
    paras
        .iter()
        .flat_map(|p| wrap_text(&ascii_text(p), NOTICE_COLS))
        .collect()
}

/// cap_lines kürzt die Zeilen auf höchstens `n`; die letzte Zeile ist dann „...“.
pub fn cap_lines(mut lines: Vec<String>, n: usize) -> Vec<String> {
    if lines.len() <= n {
        return lines;
    }
    lines.truncate(n.saturating_sub(1));
    if n > 0 {
        lines.push("...".to_string());
    }
    lines
}

/// notice_scale ist der ganzzahlige Maßstab, mit dem ein Textbild von `iw` × `ih` px in ein Fenster von `w` × `h` px
/// passt (höchstens 90 % des Fensters, mindestens 1, höchstens `NOTICE_MAX_ZOOM`).
pub fn notice_scale(w: i64, h: i64, iw: i64, ih: i64) -> i64 {
    if iw <= 0 || ih <= 0 {
        return 1;
    }
    (w * 9 / 10 / iw).min(h * 9 / 10 / ih).clamp(1, NOTICE_MAX_ZOOM)
}

/// Notice ist eine Meldung: ihre Zeilen.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Notice {
    pub lines: Vec<String>,
}

impl Notice {
    /// Baut die Meldung aus Absätzen (jeder wird umbrochen).
    pub fn new(paras: &[&str]) -> Self {
        Self {
            lines: notice_lines(paras),
        }
    }

    /// Größe des Textbilds in px.
    pub fn size(&self) -> (i64, i64) {
        (
            NOTICE_COLS as i64 * NOTICE_CHAR_W + 2 * NOTICE_PAD,
            self.lines.len() as i64 * NOTICE_LINE_H + 2 * NOTICE_PAD,
        )
    }

    /// Füllt das Fenster dunkel und zeichnet die Meldung vergrößert in die Mitte; legt nichts an.
    pub fn draw(&self, dst: &mut dyn Canvas) {
        let (iw, ih) = self.size();
        let (fw, fh) = dst.size();
        dst.rect(Rect::new(0.0, 0.0, fw, fh), NOTICE_BACK);
        let k = notice_scale(fw as i64, fh as i64, iw, ih);
        let (ox, oy) = ((fw as i64 - iw * k) / 2, (fh as i64 - ih * k) / 2);
        for (i, l) in self.lines.iter().enumerate() {
            let x = (ox + NOTICE_PAD * k) as f32;
            let y = (oy + (NOTICE_PAD + i as i64 * NOTICE_LINE_H) * k) as f32;
            dst.text(l, x, y, NOTICE_PX * k as f32, NOTICE_TEXT);
        }
    }
}

#[cfg(test)]
#[path = "notice_tests.rs"]
mod notice_tests;
