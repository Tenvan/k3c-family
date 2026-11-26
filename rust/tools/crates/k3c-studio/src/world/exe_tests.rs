use super::exe;
use crate::gameexe::BUILD_COMMAND;
use crate::test_helpers::tmp_dir;

// RU34.3/AC-03: `exe` rechnet über die Spiel-EXE des Checkouts; ohne EXE nennt der Fehler den Bau-Befehl.
#[test]
fn exe_ohne_spiel_exe() {
    let root = tmp_dir();
    let error = exe(root.path())(br#"{"op":"check"}"#.to_vec()).unwrap_err();
    assert!(error.contains(BUILD_COMMAND), "{error}");
}
