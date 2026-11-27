//! Port von `nowindow_test.go` (B-416): die Workbench läuft ohne Konsole; Kindprozesse nur über `k3c_kit::proc`
//! (ohne Konsolenfenster), nie direkt.

use std::path::Path;

fn scan(dir: &Path, found: &mut Vec<String>, seen: &mut usize) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            scan(&path, found, seen);
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !name.ends_with(".rs") || name.ends_with("_tests.rs") {
            continue;
        }
        *seen += 1;
        if std::fs::read_to_string(&path).unwrap().contains("Command::new") {
            found.push(path.display().to_string());
        }
    }
}

#[test]
fn kein_kindprozess_ohne_kit() {
    let (mut found, mut seen) = (Vec::new(), 0);
    scan(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut found,
        &mut seen,
    );
    assert!(seen > 10, "nur {seen} Dateien gelesen");
    assert!(found.is_empty(), "nutze k3c_kit::proc: {found:?}");
}
