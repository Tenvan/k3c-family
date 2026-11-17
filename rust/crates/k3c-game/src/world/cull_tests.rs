//! Port von `native/k3c-game/world/cull_test.go`.

use super::*;
use crate::view::cell::{new_camera, rect};

/// cam ist eine Kamera mit 1 px je Unit über 2000 px: Ausschnitt X ± 1000 Units.
fn cam(x: f64) -> Camera {
    new_camera(rect(0, 0, 2000, 100), x, 1.0)
}

/// Fälle aus cameraCull.test.ts (PF1.2): Ausschnitte 1000 … 3000 und 8000 … 10000, Rand CULL_MARGIN.
#[test]
fn cull_mask_faelle() {
    let views = [cam(2000.0), cam(9000.0)];
    let cases: [(&str, f64, u64); 5] = [
        ("im Ausschnitt der ersten", 2000.0, 2),
        ("rechter Rand der ersten", 3000.0 + CULL_MARGIN, 2),
        ("linker Rand der ersten", 1000.0 - CULL_MARGIN, 2),
        ("nur in der zweiten", 9000.0, 1),
        ("außerhalb beider", 5500.0, 3),
    ];
    for (name, x, want) in cases {
        let got = cull_mask(x, &views, CULL_MARGIN);
        assert_eq!(got, want, "{name}: Maske {got:b}, erwartet {want:b}");
    }
    assert_eq!(cull_mask(5500.0, &[], CULL_MARGIN), 0, "ohne Kamera");
}

/// Ausschnitt je Ebene: fern (Faktor 0) hängt nicht an der Kamera, mitte und nah verschieben sich um X · Faktor.
#[test]
fn span_parallax() {
    for x in [0.0, 5000.0] {
        let (l, r) = span(&cam(x), 0.0);
        assert!(l == -1000.0 && r == 1000.0, "fern bei X={x}: {l} … {r}");
    }
    let (l, r) = span(&cam(4000.0), 0.5);
    assert!(l == 1000.0 && r == 3000.0, "nah: {l} … {r}");
    let (l, r) = span(
        &Camera {
            x: 3.0,
            ..Camera::default()
        },
        1.0,
    );
    assert!(l > r, "ohne Maßstab nicht leer: {l} … {r}");
}

/// Kacheln im Ausschnitt: genau die, die hineinragen; ohne Maßstab keine.
#[test]
fn tile_range_faelle() {
    let k = new_camera(rect(0, 0, 480, 100), 30.0, 10.0); // 6 … 54 Units
    assert_eq!(tile_range(&k, 1.0, 24.0), (0, 2), "Boden");
    assert_eq!(tile_range(&k, 0.0, 24.0), (-1, 1), "fern");
    let (a, b) = tile_range(&Camera::default(), 1.0, 24.0);
    assert!(a > b, "ohne Maßstab: {a} … {b}");
}
