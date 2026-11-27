//! Port von `TestUIMethodsWieWailsTS`: Aufruf-Tabelle und [`AUSSTEHEND_UI`] zusammen sind genau die Methoden von
//! `interface GoApp` (`tests/data/ui-methoden.txt`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::PathBuf;

use k3c_workbench::rpc::METHODS;

/// Methoden, die erst spätere Sessions von RU69 bringen; am Ende leer.
const AUSSTEHEND_UI: &[&str] = &[
    "PlaytestCheck",
    "PlaytestDelete",
    "PlaytestDescribe",
    "PlaytestDescribeCancel",
    "PlaytestDescribeStatus",
    "PlaytestDevices",
    "PlaytestFindingUpdate",
    "PlaytestFindings",
    "PlaytestFrameSkip",
    "PlaytestList",
    "PlaytestModels",
    "PlaytestPlan",
    "PlaytestStart",
    "PlaytestStatus",
    "PlaytestStop",
    "PlaytestTimeline",
    "PlaytestTranscribe",
    "PlaytestTranscribeCancel",
    "PlaytestTranscribeStatus",
    "PlaytestVisionInfo",
    "Resources",
];

fn file(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn wanted() -> BTreeSet<String> {
    let text = std::fs::read_to_string(file("tests/data/ui-methoden.txt")).unwrap();
    text.lines().map(str::to_owned).collect()
}

#[test]
fn tabelle_und_ausstehende_sind_go_app() {
    let table: BTreeSet<String> = METHODS.iter().map(|(name, _)| (*name).to_owned()).collect();
    assert_eq!(table.len(), METHODS.len(), "Name doppelt in METHODS");
    let pending: BTreeSet<String> = AUSSTEHEND_UI.iter().map(|name| (*name).to_owned()).collect();
    assert_eq!(pending.len(), AUSSTEHEND_UI.len(), "Name doppelt in AUSSTEHEND_UI");
    let done: Vec<&String> = table.intersection(&pending).collect();
    assert!(done.is_empty(), "schon vorhanden: {done:?}");
    let both: BTreeSet<String> = table.union(&pending).cloned().collect();
    assert_eq!(both, wanted());
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
