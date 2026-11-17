//! Hilfen der world-Tests (Leitfaden § 6): `layerCell` (layers_test.go) und `near` (camera_test.go).

use k3c_sim::state::CycleInfo;

use crate::view::cell::{Cell, new_camera, rect};

/// layerCell ist eine Zelle 1920 × 540 px, Maßstab 32 px je Unit, Kamera bei x.
pub fn layer_cell(x: f64) -> Cell {
    let r = rect(0, 540, 1920, 1080);
    Cell {
        rect: r,
        camera: new_camera(r, x, 32.0),
        ..Cell::default()
    }
}

pub fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

pub fn cycle(phase: &str, p: f64) -> CycleInfo {
    CycleInfo {
        phase: phase.into(),
        progress: p,
        ..CycleInfo::default()
    }
}
