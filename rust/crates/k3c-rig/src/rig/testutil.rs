//! Hilfen der Tests (Go: `expected_test.go`): Rig-Ordner finden, Rigs laden.

use std::path::{Path, PathBuf};

use super::load::Rig;

/// `assets` ist `native/k3c-rig/assets`.
pub fn assets() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/rig")
}

/// `testdata` ist `testdata/rust/rig`.
pub fn testdata() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../testdata/rust/rig")
}

fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .expect("Ordner lesbar")
        .map(|e| e.expect("Eintrag").path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, root, out);
        } else if p.file_name().is_some_and(|n| n == "rig.json") {
            let rel = p.parent().expect("Ordner").strip_prefix(root).expect("unter assets");
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// `rig_dirs` nennt die Ordner aller Rigs (relativ zu assets/, mit /, sortiert); ohne Rigs schlägt der Test fehl.
pub fn rig_dirs() -> Vec<String> {
    let root = assets();
    let mut dirs = Vec::new();
    walk(&root, &root, &mut dirs);
    assert!(
        !dirs.is_empty(),
        "keine Rigs unter {} (erst `task gen:rig`)",
        root.display()
    );
    dirs.sort();
    dirs
}

/// `is_boss`: Rigs unter `bosse/` sind nach MG10 (B-885, RU61.6) auf Spec-`version` 3 neu exportiert. Die Go-Referenzwerte
/// in `testdata/rust/rig/` stammen vom alten Export (Go ist eingefroren, Tag `go-referenz-final`); Port-Orakel ist das
/// Verhalten (Entscheidung 012), die Spec ist die Wahrheit. Die `*_gegen_go`-Tests lassen Bosse darum aus, `boss_tests`
/// prüft sie stattdessen Rust-eigen. Die übrigen Figuren bleiben mit unveränderter Toleranz am Go-Orakel.
pub fn is_boss(dir: &str) -> bool {
    dir.starts_with("bosse/")
}

/// `load_rig` lädt ein Rig aus `assets/<dir>`.
pub fn load_rig(dir: &str) -> Rig {
    Rig::load(&assets().join(dir)).unwrap_or_else(|e| panic!("{e}"))
}

/// `animation_names`: Animationen des Rigs, sortiert (stabile Reihenfolge der Subtests).
pub fn animation_names(r: &Rig) -> Vec<String> {
    r.animations.keys().cloned().collect()
}
