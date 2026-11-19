//! RU55.2 (B-1045): Maßstab in Monarchenhöhen M im Look-Profil `k2c`; Profil `go` unverändert.

use k3c_data::look::{MONARCH_UNITS, with_profile};

use super::layout::{cell_rect, scale_n};
use crate::view::cell::{Cell, new_camera, rect};
use crate::world::register_look;

fn cell_of(r: crate::view::cell::Rectangle, n: usize) -> Cell {
    let mut c = Cell {
        rect: r,
        ..Cell::default()
    };
    c.camera = new_camera(r, 0.0, scale_n(&c, n));
    c
}

/// AC-02: M (Monarch, 3,797 Units) ist ≈ 12 % der Ansicht bei einer, ≈ 16 % der Hälfte bei zwei Ansichten; die Hälfte
/// zeigt M 0,7-fach der vollen Ansicht in px. Profil `go`: 420/32 Units über der Gehlinie in beiden Fällen.
#[test]
#[allow(clippy::cast_precision_loss)]
fn masstab_in_monarchenhoehen() {
    register_look();
    let m_share = |name: &str, n: usize| {
        with_profile(name, || {
            let r = cell_rect(rect(0, 0, 1920, 1080), n, 0);
            MONARCH_UNITS * scale_n(&cell_of(r, n), n) / r.dy() as f64
        })
    };
    let (one, two) = (m_share("k2c", 1), m_share("k2c", 2));
    assert!((one - 0.12).abs() < 0.01, "M bei einer Ansicht: {one}");
    assert!((two - 0.16).abs() < 0.01, "M je Hälfte: {two}");
    assert!((two / one * 0.5 - 0.7).abs() < 0.01, "Hälfte: {two} gegen {one}");
    for n in [1_usize, 2] {
        let units = with_profile("go", || {
            let r = cell_rect(rect(0, 0, 1920, 1080), n, 0);
            let c = cell_of(r, n);
            c.world_rect().dy() as f64 / scale_n(&c, n)
        });
        assert!((units - 420.0 / 32.0).abs() < 1e-9, "go: {units} Units bei {n}");
    }
}

/// Mindestgröße des Monarchen: auch ein kleines Fenster zeigt M mit mindestens `min_figure_px` (k2c: 40 px).
#[test]
fn mindestgroesse_des_monarchen() {
    register_look();
    let px = with_profile("k2c", || {
        let r = cell_rect(rect(0, 0, 320, 180), 2, 0);
        MONARCH_UNITS * scale_n(&cell_of(r, 2), 2)
    });
    assert!(px >= 40.0 - 1e-9, "M {px} px");
}
