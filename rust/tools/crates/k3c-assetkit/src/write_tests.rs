//! Port von `write_test.go`.

use std::fs;

use crate::register::write_atomic;
use crate::testutil::tempdir;

/// WriteAtomic (WZ9.3): Inhalt vollständig, Rechte einer vorhandenen Datei bleiben, keine Temp-Datei bleibt liegen.
#[test]
fn write_atomic_behaelt_rechte() {
    let dir = tempdir();
    let path = dir.path().join("hub.json");
    fs::write(&path, "alt").unwrap();
    #[cfg(unix)]
    fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o640)).unwrap();
    let before = fs::metadata(&path).unwrap().permissions();
    write_atomic(&path, b"neu").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "neu");
    assert_eq!(fs::metadata(&path).unwrap().permissions(), before);
    assert_eq!(
        fs::read_dir(dir.path()).unwrap().count(),
        1,
        "erwartet 1 Datei im Ordner"
    );
}

#[test]
fn write_atomic_neue_datei() {
    let dir = tempdir();
    let path = dir.path().join("neu.json");
    write_atomic(&path, b"{}").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "{}");
}

/// Eigener Test: scheitert das Umbenennen, bleibt keine Temp-Datei liegen.
#[test]
fn write_atomic_raeumt_bei_fehler_auf() {
    let dir = tempdir();
    let path = dir.path().join("ordner");
    fs::create_dir(&path).unwrap();
    fs::write(path.join("inhalt"), "x").unwrap();
    assert!(write_atomic(&path, b"{}").is_err());
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1, "Temp-Datei blieb liegen");
}
