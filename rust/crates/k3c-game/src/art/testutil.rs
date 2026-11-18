//! Hilfen der art-Tests (Leitfaden § 6).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

/// Testgrafik des Go-Pakets (`native/k3c-game/art/testdata/grafik`), direkt gelesen.
pub fn test_grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../native/k3c-game/art/testdata/grafik")
}

/// Leerer, eigener Ordner je Aufruf wie `t.TempDir()` (wird vor dem Anlegen geleert).
pub fn temp_dir(name: &str) -> PathBuf {
    static N: AtomicU32 = AtomicU32::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/tmp").join(format!("k3c-art-{}-{name}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir); // Reste eines alten Laufs, Fehlen ist kein Fehler
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Kopiert einen Ordner rekursiv wie `os.CopyFS`.
pub fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// Das echte `data/` des Repos (Tests der Zuordnungen lesen es direkt, nicht über den Arbeitsordner).
pub fn real_data() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data")
}
