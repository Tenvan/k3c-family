//! Tests zu outline.rs; Vorlage `native/k3c-game/art/outline_test.go`.
//! `TestOutlineSweep` entfällt: ohne Offscreen-Ziel gibt es keine Zwischenbilder zum Freigeben (RU7-README).

use std::path::Path;

use k3c_platform::NullCanvas;

use super::*;

/// Ohne Schrift, Text oder Deckkraft zeichnet draw nichts und legt nichts an.
#[test]
fn outline_draw_skips() {
    let mut c = OutlineCache::default();
    let mut dst = NullCanvas::default();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(DEFAULT_FONT_PATH);
    let mut f = load_font(&path, true).unwrap();
    c.draw(&mut dst, None, "x", 0.0, 0.0, 20.0, [255; 4], &OUTLINE8, 1.0);
    c.draw(&mut dst, Some(&mut f), "", 0.0, 0.0, 20.0, [0; 4], &OUTLINE8, 1.0);
    c.draw(&mut dst, Some(&mut f), "x", 0.0, 0.0, 20.0, [0; 4], &OUTLINE8, 0.0);
    assert_eq!((c.len(), dst.texts), (0, 0));
    assert_eq!(scale_alpha([200, 100, 50, 255], 0.5), [100, 50, 25, 127], "Deckkraft");
    assert!(
        outline_stroke(7.0) == 1.0 && outline_stroke(28.0) == 2.0 && outline_pad(28.0) == 2.0 + 9.0,
        "Kontur {}, Rand {}",
        outline_stroke(28.0),
        outline_pad(28.0)
    );
}

/// Kontur direkt: je Versatz ein schwarzer Text, dann der Text selbst.
#[test]
fn outline_draw_direkt() {
    let mut c = OutlineCache::default();
    let mut dst = NullCanvas::default();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(DEFAULT_FONT_PATH);
    let mut f = load_font(&path, true).unwrap();
    c.draw(
        &mut dst,
        Some(&mut f),
        "Krone",
        10.4,
        20.6,
        28.0,
        [255; 4],
        &OUTLINE4,
        1.0,
    );
    assert_eq!(dst.texts, 5);
    c.sweep();
    assert!(c.is_empty());
}
