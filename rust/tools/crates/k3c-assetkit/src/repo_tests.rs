//! Port von `repo_test.go`: das echte Register wird nur gelesen, gespeichert wird in eine Temp-Kopie.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::register::{CATEGORIES, REGISTER_PATH, check_choice, load};
use crate::testutil::tempdir;

/// Das echte Slot-Register des Repos: jede Zuordnung gültig, Kategorien bekannt, IDs eindeutig, und Speichern ändert
/// kein Byte.
#[test]
fn echtes_register() {
    let root = repo_root();
    let reg = load(&root).unwrap();
    let mut seen = HashSet::new();
    let mut problems = Vec::new();
    for s in &reg.slots {
        if !seen.insert(s.id.as_str()) {
            problems.push(format!("{} doppelt", s.id));
        }
        if !CATEGORIES
            .iter()
            .any(|(kind, list)| *kind == s.kind && list.contains(&s.category.as_str()))
        {
            problems.push(format!(
                "{}: Kategorie {:?} passt nicht zur Art {:?}",
                s.id, s.category, s.kind
            ));
        }
        for (i, c) in s.choices().iter().enumerate() {
            if let Err(e) = check_choice(&root, &s.kind, c) {
                problems.push(format!("{} Präferenz {}: {e}", s.id, i + 1));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
    let before = fs::read(root.join(REGISTER_PATH)).unwrap();
    let tmp = tempdir();
    fs::create_dir_all(tmp.path().join("data")).unwrap();
    reg.save(tmp.path()).unwrap();
    let after = fs::read(tmp.path().join(REGISTER_PATH)).unwrap();
    assert!(
        normalize(&before) == after,
        "Speichern ändert data/sounds.json (Format weicht ab)"
    );
}

/// Macht CRLF (Checkout unter Windows) zu LF.
fn normalize(b: &[u8]) -> Vec<u8> {
    b.iter()
        .enumerate()
        .filter(|&(i, &c)| !(c == b'\r' && b.get(i + 1) == Some(&b'\n')))
        .map(|(_, &c)| c)
        .collect()
}

/// Das erste Verzeichnis über dem Crate mit `data/sounds.json` (kein fester Relativpfad).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|dir| dir.join(REGISTER_PATH).exists())
        .expect("keine Repo-Wurzel mit data/sounds.json über dem Crate")
        .to_path_buf()
}
