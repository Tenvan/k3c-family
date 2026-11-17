//! Port von native/k3c-rig/paint/painter_test.go (plus `parse_color`).

use super::assets::ImageRef;
use super::painter::{Painter, parse_color};
use k3c_platform::TexId;

fn white() -> ImageRef {
    ImageRef {
        tex: TexId(0),
        w: 3,
        h: 3,
    }
}

/// Die Deckkraft des Painters gilt begrenzt auf 0..1; NaN zeichnet deckend (Standard von `new` ist 1).
#[test]
fn painter_alpha() {
    let cases: [(f64, f32); 6] = [
        (1.0, 1.0),
        (0.35, 0.35),
        (0.0, 0.0),
        (-2.0, 0.0),
        (3.0, 1.0),
        (f64::NAN, 1.0),
    ];
    for (input, want) in cases {
        let mut p = Painter::new(white());
        p.alpha = input;
        assert_eq!(p.alpha(), want, "Alpha {input}");
    }
    assert!((Painter::new(white()).alpha - 1.0).abs() < f64::EPSILON, "new: Alpha 1");
}

/// `parse_color` liest #rrggbb; alles andere ergibt Schwarz.
#[test]
fn parse_color_wie_go() {
    assert_eq!(parse_color("#ff8000"), [255, 128, 0, 255]);
    assert_eq!(parse_color("#ABCDEF"), [0xab, 0xcd, 0xef, 255]);
    assert_eq!(parse_color("ff8000"), [0, 0, 0, 255]);
    assert_eq!(parse_color("#fff"), [0, 0, 0, 255]);
    assert_eq!(parse_color("#gg0000"), [0, 0, 0, 255]);
}
