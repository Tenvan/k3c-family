//! Tests aus `native/k3c-game/menu/keys_test.go`: `TestGeomFitsWindow`.

use super::layout::{MAX_ROWS, geom_of, text_scale};
use crate::view::cell::rect;

#[test]
fn geom_fits_window() {
    for win in [rect(0, 0, 1280, 720), rect(0, 0, 1920, 1080), rect(0, 0, 320, 180)] {
        let g = geom_of(win);
        let (_, y, w, h) = g.row_rect(MAX_ROWS as i64 - 1);
        assert!(
            !(y + h > g.note_y
                || g.row_x < 0.0
                || g.row_x + w > win.max.x as f64
                || g.title_y >= g.sub_y
                || g.sub_y + g.small_px > g.row_top),
            "{win:?}: Zeilen ragen über die Meldung ({:.0} > {:.0}) oder aus dem Fenster",
            y + h,
            g.note_y
        );
        let (ax, ay, aw, ah) = g.ask_rect();
        assert!(
            !(ax < 0.0 || ay < 0.0 || ax + aw > win.max.x as f64 || ay + ah > win.max.y as f64),
            "{win:?}: Rückfrage ragt aus dem Fenster"
        );
    }
    assert!(text_scale(10.0) == 1.0 && text_scale(43.0) == 3.0, "text_scale");
}
