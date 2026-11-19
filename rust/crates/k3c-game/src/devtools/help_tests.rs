//! Port von `native/k3c-game/devtools/help_test.go`.

use k3c_platform::NullCanvas;

use super::help::help_rows;
use super::new;
use crate::view::cell::{rect, window_cell};
use crate::view::devkeys::DEV_KEYS;
use crate::view::frame::Frame;

/// AC-01 (B-888): Die Übersicht hat eine Zeile je Taste der Tabelle, mit Taste und Wirkung.
#[test]
fn test_help_rows_cover_table() {
    let rows = help_rows();
    assert_eq!(rows.len(), DEV_KEYS.len(), "Zeilen und Tabelle");
    for (row, k) in rows.iter().zip(DEV_KEYS.iter()) {
        assert!(
            row.key == k.label && !row.text.is_empty(),
            "Zeile {row:?} für {}",
            k.label
        );
    }
}

/// B-889 und RU56: Die F1-Übersicht nennt das Cheat-Fenster (F4) und das Debug-Overlay mit seinen Seiten (F10); die
/// Werte zeigt nur das Overlay, das Fenster schaltet dieselbe Seite (`devwin/view.rs`), es hat keine zweite Anzeige.
#[test]
fn test_help_names_f10_and_window() {
    let rows = help_rows();
    let f10 = rows.iter().find(|r| r.key == "F10").expect("F10 in der Übersicht");
    assert!(f10.text.contains("Werte"), "F10: {f10:?}");
    let f4 = rows.iter().find(|r| r.key == "F4").expect("F4 in der Übersicht");
    assert!(f4.text.contains("Cheat-Fenster"), "F4: {f4:?}");
}

/// Ohne Schrift und ohne F1 zeichnet die Übersicht ohne Absturz bzw. gar nicht.
#[test]
fn test_help_draw_without_font() {
    let parts = new(None);
    let mut dst = NullCanvas::default();
    let c = window_cell(rect(0, 0, 320, 180));
    let mut f = Frame::default();
    for d in &parts.drawers {
        d.draw(&mut dst, &f, &c);
    }
    assert_eq!(dst.rects, 0, "ohne F1 nichts");
    f.dev.help = true;
    for d in &parts.drawers {
        d.draw(&mut dst, &f, &c);
    }
    assert_eq!(dst.rects, 1, "ohne Schrift nur das Feld");
}
