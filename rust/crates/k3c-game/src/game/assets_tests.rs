//! Test der Grafik-Ordner (Go: `TestAssetDirs` aus `modes_test.go`).

use super::*;
use crate::game::{Libs, assemble, packages};

/// --assets nennt je Level die Grafik aller Asset-Quellen: Wald-Natur, Höhlen-Natur, Gebäude und Kulissen.
#[test]
fn asset_dirs_nennt_alle_quellen() {
    let lv = levels().unwrap();
    assert!(
        lv.iter().any(|l| l == "forest") && lv.iter().any(|l| l == "cave"),
        "Level {lv:?}"
    );
    let p = assemble(packages(&Libs::default(), None)).unwrap();
    let dirs = asset_dirs(&p.assets).unwrap();
    for want in ["objekte/tanne", "objekte/wurzel"] {
        assert!(dirs.iter().any(|d| d == want), "{want} fehlt in {dirs:?}");
    }
    assert!(dirs.is_sorted(), "nicht sortiert: {dirs:?}");
    let mut uniq = dirs.clone();
    uniq.dedup();
    assert_eq!(uniq.len(), dirs.len(), "doppelt: {dirs:?}");
}

/// Der Ordner der Daten und die eingebetteten Dateien nennen dieselben Level.
#[test]
fn levels_ordner_und_eingebettet_gleich() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data");
    let from_dir = levels_in(&Source::Dir(dir)).unwrap();
    assert_eq!(from_dir, levels_in(&Source::Embedded).unwrap());
}
