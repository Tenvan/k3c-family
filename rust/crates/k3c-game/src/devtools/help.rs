//! Port von `native/k3c-game/devtools/help.go`.
//!
//! Tastenübersicht (F1): eine Zeile je Eintrag von `DEV_KEYS`, Taste als Tastenkappe (UI-Tasten, unabhängig vom
//! Eingabe-Profil, B-894), Wirkung und Gamepad-Kombi, mittig im Fenster.

use std::sync::OnceLock;

use k3c_platform::Canvas;

use super::{DIM_COLOR, Rgba, TEXT_COLOR, boxed, fill};
use crate::art::{Font, LINE_SPACING};
use crate::view::cell::Cell;
use crate::view::devkeys::DEV_KEYS;

/// HELP_TITLE ist die Überschrift der Übersicht.
const HELP_TITLE: &str = "Entwickler-Tasten";

/// HelpRow ist eine Zeile der Übersicht.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpRow {
    pub key: &'static str,
    pub text: &'static str,
    pub pad: &'static str,
}

/// help_rows liefert die Zeilen der Übersicht aus der Tabelle `DEV_KEYS` (Reihenfolge der Tabelle); die Tabelle
/// ändert sich zur Laufzeit nicht, die Zeilen entstehen einmal.
pub fn help_rows() -> &'static [HelpRow] {
    static ROWS: OnceLock<Vec<HelpRow>> = OnceLock::new();
    ROWS.get_or_init(|| {
        DEV_KEYS
            .iter()
            .map(|k| HelpRow {
                key: k.label,
                text: k.text,
                pad: k.pad,
            })
            .collect()
    })
}

// Farben der Tastenkappe (wie die Kappen im HUD, hud/glyphs.go › glyphColors["key"]).
const CAP_FILL: Rgba = [242, 242, 242, 255];
const CAP_TEXT: Rgba = [17, 17, 17, 255];
const CAP_EDGE: Rgba = [0, 0, 0, 255];

/// draw_help zeichnet die Übersicht mittig in die Zelle `c` (Fenster); Schriftgröße nach der Fensterhöhe.
#[allow(clippy::cast_precision_loss)]
pub fn draw_help(dst: &mut dyn Canvas, font: Option<&mut Font>, c: &Cell) {
    let r = c.rect;
    let px = (r.dy() as f64 / 36.0).max(14.0);
    let line_h = px * LINE_SPACING;
    let rows = help_rows();
    let (key_w, text_w, pad_w) = match font.as_deref() {
        Some(f) => columns(f, px),
        None => (px * 3.0, px * 18.0, px * 7.0),
    };
    let gap = px;
    let w = key_w + text_w + pad_w + 4.0 * gap;
    let h = line_h * (rows.len() + 2) as f64 + 2.0 * gap;
    let x = r.min.x as f64 + (r.dx() as f64 - w) / 2.0;
    let y = r.min.y as f64 + (r.dy() as f64 - h) / 2.0;
    boxed(dst, x, y, w, h);
    let Some(font) = font else { return };
    font.draw(dst, HELP_TITLE, x + gap, y + gap, px, TEXT_COLOR);
    for (i, row) in rows.iter().enumerate() {
        let ry = y + gap + line_h * (i + 2) as f64;
        key_cap(dst, font, row.key, x + gap, ry, px);
        font.draw(dst, row.text, x + 2.0 * gap + key_w, ry, px, TEXT_COLOR);
        if !row.pad.is_empty() {
            font.draw(dst, row.pad, x + 3.0 * gap + key_w + text_w, ry, px, DIM_COLOR);
        }
    }
}

/// key_cap zeichnet die Taste `label` als helle Kappe mit dunkler Schrift, oben links an `x`, `y` (Zeilenhöhe `px`).
fn key_cap(dst: &mut dyn Canvas, font: &mut Font, label: &str, x: f64, y: f64, px: f64) {
    let (w, _) = font.measure(label, px);
    let pad = px * 0.3;
    fill(
        dst,
        x - pad - 1.0,
        y - 1.0,
        w + 2.0 * pad + 2.0,
        px * 1.2 + 2.0,
        CAP_EDGE,
    );
    fill(dst, x - pad, y, w + 2.0 * pad, px * 1.2, CAP_FILL);
    font.draw(dst, label, x, y, px, CAP_TEXT);
}

/// columns misst die Breiten der drei Spalten.
fn columns(font: &Font, px: f64) -> (f64, f64, f64) {
    let (mut key_w, mut text_w, mut pad_w) = (0.0f64, 0.0f64, 0.0f64);
    for row in help_rows() {
        key_w = key_w.max(font.measure(row.key, px).0);
        text_w = text_w.max(font.measure(row.text, px).0);
        pad_w = pad_w.max(font.measure(row.pad, px).0);
    }
    let title_w = font.measure(HELP_TITLE, px).0;
    (key_w, text_w.max(title_w), pad_w)
}
