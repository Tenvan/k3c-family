//! Port von `find_test.go`.

use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use super::{BUILD_COMMAND, find, locate};
use crate::test_helpers::tmp_dir;

const MINUTE: Duration = Duration::from_secs(60);
const HOUR: Duration = Duration::from_secs(3600);

fn write(root: &Path, rel: &str, mtime: SystemTime) -> PathBuf {
    let mut path = root.to_path_buf();
    path.extend(rel.split('/'));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "x").unwrap();
    File::options()
        .write(true)
        .open(&path)
        .unwrap()
        .set_modified(mtime)
        .unwrap();
    path
}

const BUILD: &str = "rust/target/release/k3c-game.exe";
const PACKAGE: &str = "dist/k3c-game/k3c-game.exe";

#[test]
fn suchliste_reihenfolge() {
    let tmp = tmp_dir();
    let root = tmp.path();
    let now = SystemTime::now();
    let error = locate(root).unwrap_err();
    assert!(error.contains(BUILD_COMMAND), "ohne EXE: {error}");
    assert_eq!(find(root).unwrap_err(), error);
    let package = write(root, PACKAGE, now);
    let (exe, dir) = locate(root).unwrap();
    assert_eq!((&exe, dir.as_path()), (&package, package.parent().unwrap()));
    let build = write(root, BUILD, now);
    let (exe, dir) = locate(root).unwrap();
    assert_eq!(exe, build, "Build hat Vorrang");
    // Ohne „.“ am Ende, sonst steht es im Arbeitsordner.
    assert_eq!(dir.as_os_str(), root.as_os_str());
}

#[test]
fn frische() {
    let tmp = tmp_dir();
    let root = tmp.path();
    let built = SystemTime::now() - HOUR;
    write(root, BUILD, built);
    write(root, "rust/crates/a/src/lib.rs", built - MINUTE);
    write(root, "rust/crates/a/Cargo.toml", built - MINUTE);
    write(root, "rust/Cargo.lock", built - MINUTE);
    // Zählt nicht: README, Ordner target, die Werkzeuge unter rust/tools.
    write(root, "rust/crates/a/README.md", built + HOUR);
    write(root, "rust/crates/a/target/gen.rs", built + HOUR);
    write(root, "rust/tools/crates/b/src/lib.rs", built + HOUR);
    find(root).expect("frische EXE");
    for file in [
        "rust/crates/a/src/lib.rs",
        "rust/crates/a/Cargo.toml",
        "rust/Cargo.lock",
    ] {
        write(root, file, built + MINUTE);
        let error = find(root).unwrap_err();
        assert!(
            error.contains(file) && error.contains(BUILD_COMMAND),
            "{file} jünger: {error}"
        );
        write(root, file, built - MINUTE);
    }
}

#[test]
fn find_nimmt_frischen_kandidaten() {
    let tmp = tmp_dir();
    let root = tmp.path();
    let now = SystemTime::now();
    write(root, BUILD, now - HOUR);
    let package = write(root, PACKAGE, now);
    write(root, "rust/crates/a/lib.rs", now - MINUTE);
    assert_eq!(find(root).expect("frisches Paket").0, package);
}
