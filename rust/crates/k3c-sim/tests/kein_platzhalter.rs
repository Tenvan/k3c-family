//! RU22.5: In `k3c-sim` steht kein Platzhalter mehr: kein `unimplemented!`, kein `todo!`, keine Datei `pending.rs` und kein
//! `panic!` mit Sprint-Marke (`RU…:`). Der Test liest die Quelltexte unter `src/`. Ausgenommen sind Testdateien
//! (`*_tests.rs`, `tests.rs`, `testutil.rs`, `*_testutil.rs`): Dort stehen die offenen Testhelfer von B-1103
//! (`fixture_island` u. a.) als `unimplemented!`; sie laufen nicht im Spielpfad.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::path::{Path, PathBuf};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

fn is_test_file(p: &Path) -> bool {
    let n = p.file_name().unwrap().to_string_lossy();
    n.ends_with("_tests.rs") || n == "tests.rs" || n.contains("testutil")
}

/// `strip_comments` lässt Kommentare weg, damit Erklärungen den Test nicht auslösen.
fn code_of(text: &str) -> String {
    text.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn quelltexte_haben_keinen_platzhalter() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    assert!(files.len() > 100, "nur {} Dateien gefunden", files.len());
    let mut funde = Vec::new();
    for p in &files {
        let rel = p.strip_prefix(&src).unwrap().display().to_string();
        if p.file_name().is_some_and(|n| n == "pending.rs") {
            funde.push(format!("{rel}: Datei pending.rs"));
        }
        if is_test_file(p) {
            continue;
        }
        let code = code_of(&std::fs::read_to_string(p).unwrap());
        for (what, needle) in [
            ("unimplemented!", "unimplemented!"),
            ("todo!", "todo!"),
            ("panic!(\"RU", "panic!(\"RU"),
        ] {
            if code.contains(needle) {
                funde.push(format!("{rel}: {what}"));
            }
        }
    }
    assert!(funde.is_empty(), "Platzhalter in k3c-sim:\n{}", funde.join("\n"));
}
