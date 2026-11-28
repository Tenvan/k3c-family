//! Die Aufruf-Tabelle hat genau die Methoden der Liste `tests/data/ui-methoden.txt`.
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
