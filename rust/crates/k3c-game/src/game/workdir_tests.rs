//! Tests zu `workdir.rs` (RU57.3, B-1140).

use std::path::Path;

use super::workdir::anchor_target;

fn strings(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

/// Ohne Schalter aus fremdem Ordner: Wechsel in den EXE-Ordner, auch bei `play`.
#[test]
fn plain_start_moves_to_exe_dir() {
    let exe = Path::new("D:/k3c");
    for args in [strings(&[]), strings(&["play"])] {
        assert_eq!(
            anchor_target(&args, false, Some(exe), true),
            Some(exe.to_path_buf()),
            "{args:?}"
        );
    }
}

/// Kein Wechsel: Spielordner schon im Arbeitsordner (Repo, Paket), kein Spielordner neben der EXE, Schalter, andere Befehle.
#[test]
fn no_move_when_not_needed() {
    let exe = Path::new("D:/k3c");
    assert_eq!(anchor_target(&[], true, Some(exe), true), None, "Arbeitsordner hat ihn");
    assert_eq!(
        anchor_target(&[], false, Some(exe), false),
        None,
        "EXE-Ordner hat ihn nicht"
    );
    assert_eq!(anchor_target(&[], false, None, true), None, "ohne EXE-Pfad");
    assert_eq!(
        anchor_target(&strings(&["--saves", "s"]), false, Some(exe), true),
        None,
        "Schalter"
    );
    assert_eq!(
        anchor_target(&strings(&["replay", "x.jsonl.gz"]), false, Some(exe), true),
        None,
        "anderer Befehl"
    );
}
