//! Port von `TestUIMethodsWieWailsTS`: die Aufruf-Tabelle hat genau die Methoden von `interface GoApp`
//! (`tests/data/ui-methoden.txt`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::PathBuf;

use k3c_workbench::rpc::METHODS;

fn file(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn wanted() -> BTreeSet<String> {
    let text = std::fs::read_to_string(file("tests/data/ui-methoden.txt")).unwrap();
    text.lines().map(str::to_owned).collect()
}

#[test]
fn tabelle_ist_go_app() {
    let table: BTreeSet<String> = METHODS.iter().map(|(name, _)| (*name).to_owned()).collect();
    assert_eq!(table.len(), METHODS.len(), "Name doppelt in METHODS");
    assert_eq!(table, wanted());
}

#[test]
fn liste_wie_wails_ts() {
    let path = file("../../../../tools/k3c-workbench/frontend/src/api/wails.ts");
    let Ok(source) = std::fs::read_to_string(&path) else {
        eprintln!("🚫 übersprungen: {} fehlt", path.display());
        return;
    };
    let source = source.replace("\r\n", "\n");
    // Eine Zeile von `GoApp` beginnt mit der Methode; `ts_methods` liest genau diese.
    let names = k3c_kit::uikit::ts_methods(&source).unwrap();
    assert_eq!(names.into_iter().collect::<BTreeSet<_>>(), wanted());
}
