//! RU55.1/RU55.2: Look-Profile `k2c` und `go` im Layout der Zellen (Gehlinie, Ufer, Maßstab).

use k3c_data::look::with_profile;

use super::layout::{cell_rect, scale};
use crate::view::cell::{Cell, Rectangle, new_camera, rect};
use crate::world::{ground::bank_height, register_look};

fn cell_of(r: Rectangle) -> Cell {
    let mut c = Cell {
        rect: r,
        ..Cell::default()
    };
    c.camera = new_camera(r, 0.0, scale(&c));
    c
}

/// Gehlinie (px) und Ufer (px) einer Zelle im Profil `name`.
#[allow(clippy::cast_precision_loss)]
fn lines(name: &str, r: Rectangle) -> (f64, f64, f64, f64) {
    register_look();
    with_profile(name, || {
        let c = cell_of(r);
        (
            c.world_rect().dy() as f64,
            bank_height(&c),
            c.river_rect().dy() as f64,
            c.camera.scale,
        )
    })
}

/// RU55.1 AC-01: Profil `k2c`: Gehlinie bei 58 % der Zelle (± 1 px) und Ufer 7 % der Zelle, in einer und zwei Ansichten;
/// Profil `go`: zwei Drittel und 20 px je Unit mal Maßstab.
#[test]
#[allow(clippy::cast_precision_loss)]
fn look_k2c_baender_58_7_35() {
    for (n, h) in [(1_usize, 1080_i64), (2, 1080), (1, 720), (2, 720)] {
        let win = rect(0, 0, 1920, h);
        for i in 0..n {
            let r = cell_rect(win, n, i);
            let dy = r.dy() as f64;
            let (world, bank, river, _) = lines("k2c", r);
            assert!((world - 0.58 * dy).abs() <= 1.0, "Gehlinie {n}x{h}: {world}");
            assert!((bank - 0.07 * dy).abs() <= 1.0, "Ufer {bank} bei {dy}");
            assert!(bank <= river / 3.0, "Ufer höchstens ein Drittel des Flusses");
            let (world_go, bank_go, _, scale_go) = lines("go", r);
            assert!((world_go - dy * 2.0 / 3.0).abs() <= 1.0, "go: Welt {world_go}");
            assert!((bank_go - 20.0 / 32.0 * scale_go).abs() < 1e-9, "go: Ufer {bank_go}");
        }
    }
}
