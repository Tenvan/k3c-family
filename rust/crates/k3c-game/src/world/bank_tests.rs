//! RU55.1: Ufer im Profil `k2c` (Risse, Wurzeln, Grasbüschel) ist stabil und zeichnet nur im sichtbaren Bereich.

use super::*;
use k3c_platform::Frame;

use crate::view::cell::{Cell, new_camera, rect};

fn cell(x: f64) -> Cell {
    let r = rect(0, 0, 1920, 1080);
    Cell {
        rect: r,
        camera: new_camera(r, x, 33.0),
        ..Cell::default()
    }
}

#[test]
fn hash_ist_stabil_und_in_0_bis_1() {
    for i in -50..50 {
        let (a, b) = (unit_hash(i, 3), unit_hash(i, 3));
        assert_eq!(a, b);
        assert!((0.0..1.0).contains(&a), "{a}");
    }
    assert_ne!(unit_hash(1, 1), unit_hash(2, 1));
}

#[test]
fn stellen_folgen_der_kamera_und_decken_die_zelle() {
    let c = cell(100.0);
    let (first, last) = spots_in_view(&c);
    let (l, r) = c.camera.span();
    #[allow(clippy::cast_precision_loss)]
    {
        assert!(first as f64 * SPOT_UNITS <= l && last as f64 * SPOT_UNITS >= r);
    }
    assert!(last - first < 100, "wenige Stellen je Zelle: {}", last - first);
}

#[test]
fn ufer_zeichnet_gleich_je_aufruf_und_zu_flaches_nichts() {
    let w = World::new(None);
    let c = cell(0.0);
    let count = |h: f64| {
        let mut f = Frame::new();
        f.begin();
        w.draw_bank(&mut f, &c, [90, 140, 60, 255], h);
        f.finish();
        (f.instance_count(), f.vertices().len())
    };
    let a = count(75.0);
    assert!(a.0 > 0 && a.1 > 0, "Risse als Linien, Büschel als Dreiecke: {a:?}");
    assert_eq!(a, count(75.0));
    assert_eq!(count(1.0), (0, 0));
}
