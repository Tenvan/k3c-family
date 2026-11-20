//! B-1149: Wachstumsstufen der Plantage.

use std::path::Path;

use super::nature::{plant_tree_look, young_tree_look};
use super::plan_objects::young_stage;

#[test]
fn jungbaum_in_der_zweiten_haelfte() {
    assert!(!young_stage(0.0, 60.0), "Platz wächst nicht");
    assert!(!young_stage(40.0, 60.0), "erste Hälfte: Setzling");
    assert!(
        young_stage(30.0, 60.0) && young_stage(1.0, 60.0),
        "zweite Hälfte: Jungbaum"
    );
    assert!(!young_stage(10.0, 0.0), "ohne Wachszeit nie");
}

/// Höhe der Rahmen in Spiel-px laut Manifest (Rahmenhöhe, nicht sichtbar; die Stufen sind weit genug auseinander).
fn frame_px(asset: &str) -> f64 {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../public/grafik/objekte")
        .join(asset)
        .join("manifest.json");
    let v: serde_json::Value = serde_json::from_slice(&std::fs::read(p).expect("erst task gen:paper")).unwrap();
    v["rahmen"][3].as_f64().unwrap()
}

#[test]
fn drei_stufen_mindestens_halbe_monarchenhoehe_auseinander() {
    k3c_data::look::with_profile(k3c_data::look::K2C, || {
        let young = young_tree_look().expect("Jungbaum im Profil k2c");
        let old = plant_tree_look();
        assert_eq!((young.asset, old.asset), ("objekte/pappel-2", "objekte/pappel"));
        let sizes = k3c_data::sizes::from_source(&k3c_data::Source::Embedded).unwrap();
        let (seed, youngpx) = (frame_px("setzling") * 0.9, frame_px("pappel-2") * 0.9);
        let oldpx = sizes.target_px("objekte/pappel").unwrap();
        let half = 0.5 * sizes.reference_px;
        assert!(
            youngpx - seed >= half && oldpx - youngpx >= half,
            "Setzling {seed} Jung {youngpx} Alt {oldpx}"
        );
    });
    k3c_data::look::with_profile(k3c_data::look::GO, || {
        assert!(young_tree_look().is_none());
        assert_eq!(plant_tree_look().asset, "objekte/pappel-niedrig");
    });
}
