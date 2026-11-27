use std::ffi::OsStr;

use super::datafiles::{DirEntry, data_dir, read, read_dir};
use super::test_helpers::repo_root;
use crate::test_helpers::tmp_dir;

/// `K3C_DATA_DIR` geht vor `<root>/data/`; leer oder fehlend zählt nicht (wie Go `data.Files`). Der Wert wird als
/// Argument geprüft, weil Tests die Umgebung des Prozesses nicht ändern (`unsafe`).
#[test]
fn env_ordner_hat_vorrang() {
    let tmp = tmp_dir();
    let root = tmp.path();
    let other = root.join("anders");
    std::fs::create_dir_all(&other).expect("Ordner");
    assert_eq!(data_dir(root, Some(other.as_os_str())), other);
    assert_eq!(data_dir(root, None), root.join("data"));
    assert_eq!(data_dir(root, Some(OsStr::new(""))), root.join("data"));
    let missing = root.join("fehlt");
    assert_eq!(data_dir(root, Some(missing.as_os_str())), root.join("data"));
}

#[test]
fn liest_dateien_und_ordner_sortiert() {
    let tmp = tmp_dir();
    let worlds = tmp.path().join("data").join("worlds");
    std::fs::create_dir_all(worlds.join("alt")).expect("Ordner");
    std::fs::write(worlds.join("b.json"), "{}").expect("Datei");
    std::fs::write(worlds.join("a.json"), "{}").expect("Datei");
    if std::env::var_os(super::datafiles::DIR_ENV).is_some() {
        eprintln!("übersprungen: K3C_DATA_DIR ist gesetzt");
        return;
    }
    assert_eq!(read(tmp.path(), "worlds/a.json").expect("lesen"), b"{}");
    let entry = |name: &str, is_dir| DirEntry {
        name: name.to_owned(),
        is_dir,
    };
    assert_eq!(
        read_dir(tmp.path(), "worlds").expect("Ordner"),
        [entry("a.json", false), entry("alt", true), entry("b.json", false)]
    );
    assert!(read(tmp.path(), "fehlt.json").is_err());
}

#[test]
fn kein_pfad_verlaesst_data() {
    let root = repo_root();
    for bad in [
        "../Cargo.toml",
        "worlds/../../x",
        "/etc/passwd",
        "C:/x",
        r"worlds\standard.json",
        "",
    ] {
        let err = read(&root, bad).expect_err(bad);
        assert!(err.ends_with("invalid argument"), "{bad}: {err}");
    }
}
