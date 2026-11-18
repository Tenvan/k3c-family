//! Tests zu font.rs; Vorlage: `TestFontMissing`, `TestFontLoad` (`art_test.go`), `TestDefaultConfigFont`
//! (`config_test.go`). Go testet mit goregular; Rust mit der eingecheckten Schrift `public/schrift/k3c.ttf`.

use std::path::{Path, PathBuf};

use k3c_platform::NullCanvas;

use super::testutil::temp_dir;
use super::*;

fn repo_font() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(DEFAULT_FONT_PATH)
}

#[test]
fn font_missing() {
    let path = temp_dir("font-missing").join("fehlt.ttf");
    let err = load_font(&path, true).unwrap_err();
    assert!(
        err.is_missing() && err.to_string().contains(&path.display().to_string()),
        "strenger Modus: {err}"
    );
    let f = load_font(&path, false).unwrap();
    assert!(f.fallback, "Ersatz-Schrift: {f:?}");
    assert_eq!(f.measure("abc", 20.0), (18.0, 16.0), "Debug-Maße");
    assert_eq!(f.measure("ab\nabcd", 20.0), (24.0, 32.0), "Debug-Maße mit Umbruch");
}

/// B-1065: Die Höhe einer Zeile ist Ober- plus Unterlänge wie in Go (`text.Measure`: 27,738 bei Größe 24), nicht die
/// Schriftgröße; mehrere Zeilen im Abstand `LINE_SPACING` × Größe; leerer Text misst 0.
#[test]
fn font_measure_hoehe_wie_go() {
    let f = load_font(&repo_font(), true).unwrap();
    let (w, h) = f.measure("Hallo Welt", 24.0);
    assert!((h - 27.738_28).abs() < 0.01, "Höhe {h}");
    assert!((w - 112.734_375).abs() < 0.1, "Breite {w}");
    let (_, h2) = f.measure(
        "Hallo
Welt",
        24.0,
    );
    assert!((h2 - (1.25 * 24.0 + 27.738_28)).abs() < 0.01, "zwei Zeilen {h2}");
    assert_eq!(f.measure("", 24.0), (0.0, 0.0));
}

#[test]
fn font_load() {
    let f = load_font(&repo_font(), true).unwrap();
    assert!(!f.fallback);
    let (w, h) = f.measure("Hallo", 20.0);
    assert!(w > 0.0 && h > 0.0, "Maße {w}×{h}");
    let (w2, h2) = f.measure("Hallo\nHallo", 20.0);
    assert!(
        w2 == w && (h2 - h - LINE_SPACING * 20.0).abs() <= 1e-9,
        "Umbruch: {w2}×{h2}, erwartet {w}×{}",
        h + LINE_SPACING * 20.0
    );
    let bad = temp_dir("font-bad").join("kaputt.ttf");
    std::fs::write(&bad, "kein Font").unwrap();
    let err = load_font(&bad, false).unwrap_err();
    assert!(!err.is_missing(), "kaputte Schrift: {err}");
}

#[test]
fn font_draw_meldet_schrift_einmal_an() {
    let mut f = load_font(&repo_font(), true).unwrap();
    let mut c = NullCanvas::default();
    f.draw(&mut c, "Hallo\nWelt", 10.0, 20.0, 24.0, [255; 4]);
    f.draw(&mut c, "Ä", 10.0, 20.0, 24.0, [255; 4]);
    assert_eq!(c.texts, 3, "eine Zeile je text_font");
}

#[test]
fn default_config_font() {
    let cfg = default_config();
    assert!(cfg.strict_font, "StrictFont ist aus");
    let f = load_font(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join(&cfg.font_path),
        true,
    );
    assert!(
        f.as_ref().is_ok_and(|f| !f.fallback),
        "Schrift {:?}: {f:?}",
        cfg.font_path
    );
}
