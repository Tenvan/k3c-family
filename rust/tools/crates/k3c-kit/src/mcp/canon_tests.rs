//! Port von `mcpkit/canon_windows_test.go`.

use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

use super::same_path;
use super::workspace::canon;
use crate::testutil::tmp_dir;

/// Kurzname (8.3) über `cmd`, ohne `unsafe`.
fn short_name(long: &Path) -> Option<String> {
    let out = Command::new("cmd")
        .raw_arg(format!("/d /c for %I in (\"{}\") do @echo %~sI", long.display()))
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

// TestSamePathKurznameOhneVorhandenesEnde (B-628): Ein Kurzname (wie RUNNER~1) gleicht dem Langnamen, auch wenn das
// Ende des Pfads noch fehlt.
#[test]
fn same_path_kurzname_ohne_vorhandenes_ende() {
    let tmp = tmp_dir();
    let long = canon(tmp.path()).join("ein langer Ordnername");
    std::fs::create_dir(&long).unwrap();
    let Some(short) = short_name(&long).filter(|s| s.contains('~')) else {
        eprintln!("kein Kurzname mit ~ (8.3 auf dem Laufwerk aus), übersprungen");
        return;
    };
    let short = Path::new(&short);
    assert!(same_path(
        short.join("fehlt").join("noch"),
        long.join("fehlt").join("noch")
    ));
    assert!(!same_path(short.join("a"), long.join("b")));
}
