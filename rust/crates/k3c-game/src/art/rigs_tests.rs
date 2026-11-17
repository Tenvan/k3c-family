//! Tests zu rigs.rs; Vorlage `native/k3c-game/art/rigs_test.go` (`TestRigView`, `TestActorFrame`) plus
//! Laden aus `native/k3c-rig/assets`.

use std::path::{Path, PathBuf};

use k3c_platform::NullCanvas;
use k3c_rig::rig::{Rig, matrix::apply};

use super::rigs::frame_of;
use super::testutil::{temp_dir, test_grafik};
use super::*;
use crate::view::cell::{new_camera, rect};

fn rig_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../native/k3c-rig/assets")
}

fn rig_with_frame(frame: [f64; 4]) -> Rig {
    let mut r = Rig::default();
    r.frame = frame;
    r
}

// RigView: Fußpunkt an Welt-x auf der Boden-Linie, Maßstab aus Kamera und Größe; gespiegelt bleibt der Rahmen an
// derselben Stelle im Fenster (wie native/k3c-ebiten/rigs.go).
#[test]
fn rig_view_test() {
    let k = new_camera(rect(0, 0, 1000, 600), 50.0, 20.0); // Mitte bei x=500, 20 px je Unit
    let frame = [-30.0, -60.0, 80.0, 70.0]; // Rahmen nicht mittig um den Fußpunkt
    let m = rig_view(&k, 52.0, 400.0, 0.05, frame, false); // 0,05 Units je Einheit: 1 px je Einheit
    assert_eq!(m, [1.0, 0.0, 0.0, 1.0, 540.0, 400.0], "rechts: {m:?}");
    let l = rig_view(&k, 52.0, 400.0, 0.05, frame, true);
    let xs = |v: &M| [apply(v, [frame[0], 0.0])[0], apply(v, [frame[0] + frame[2], 0.0])[0]];
    let left = |v: &M| xs(v)[0].min(xs(v)[1]);
    let right = |v: &M| xs(v)[0].max(xs(v)[1]);
    assert!(
        l[0] == -1.0 && l[3] == 1.0 && left(&l) == left(&m) && right(&l) == right(&m),
        "links: {l:?}, Rahmen {}…{} statt {}…{}",
        left(&l),
        right(&l),
        left(&m),
        right(&m)
    );
}

#[test]
fn actor_frame_test() {
    let rider = rig_with_frame([1.0, 2.0, 3.0, 4.0]);
    let mount = rig_with_frame([5.0, 6.0, 7.0, 8.0]);
    let cases = [
        (Some(&rider), None, rider.frame),
        (Some(&rider), Some(&mount), mount.frame),
        (None, Some(&mount), mount.frame),
        (None, None, [0.0; 4]),
    ];
    for (i, (r, m, want)) in cases.into_iter().enumerate() {
        assert_eq!(frame_of(r, m), want, "Fall {i}");
    }
}

#[test]
fn library_loads_rig_assets() {
    let mut l = Library::new(Config {
        grafik_dir: test_grafik(),
        rig_dir: rig_dir(),
        font_path: temp_dir("rigs-font").join("fehlt.ttf"),
        ..Config::default()
    })
    .unwrap();
    assert!(l.new_painter().unwrap_err().is_missing(), "vor load_rigs");
    let mut c = NullCanvas::default();
    l.load_rigs(&mut c).unwrap();
    assert!(l.rig_assets().is_some() && l.new_painter().is_ok());
    let a = l.new_actor(&mut c, "figuren/monarch", "reittiere/pferd", "").unwrap();
    assert!(actor_frame(&a)[3] > 0.0);
    assert!(l.rig_mib() > 0.0);
}

#[test]
fn missing_rig_dir_is_missing() {
    let err = Library::new(Config {
        rig_dir: temp_dir("rigs-leer"),
        font_path: temp_dir("rigs-font2").join("fehlt.ttf"),
        ..Config::default()
    })
    .err()
    .expect("Fehler");
    assert!(err.is_missing() && err.to_string().contains("Rig-Bibliothek"), "{err}");
}
