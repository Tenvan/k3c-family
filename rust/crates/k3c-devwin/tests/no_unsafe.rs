//! `unsafe` bleibt aus dem Rust-Workspace heraus, außer in `k3c-devwin/src/win32/` (RU56, rust-port.md § 12);
//! dort trägt jeder Block ein `// SAFETY:` (AC-02).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode

use std::fs;
use std::path::{Path, PathBuf};

/// Schon vorhandene Ausnahmen: zählende Allocatoren für Tests (`unsafe impl GlobalAlloc`).
const ALLOC_EXCEPTIONS: [&str; 2] = ["/k3c-alloc-probe/", "/k3c-platform/tests/alloc.rs"];

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n != "target") {
                rs_files(&p, out);
            }
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Zeilen mit dem Schlüsselwort (ohne Zeilenkommentar), als (Zeilennummer, Text).
fn unsafe_lines(src: &str) -> Vec<usize> {
    let kw = ["un", "safe"].concat();
    let mut hits = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let code = line.split("//").next().unwrap_or("");
        let b = code.as_bytes();
        for (at, _) in code.match_indices(&kw) {
            let before = at == 0 || !(b[at - 1].is_ascii_alphanumeric() || b[at - 1] == b'_');
            let end = at + kw.len();
            let after = end >= b.len() || !(b[end].is_ascii_alphanumeric() || b[end] == b'_');
            if before && after {
                hits.push(i);
            }
        }
    }
    hits
}

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn norm(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

#[test]
fn kein_unsafe_ausserhalb_win32() {
    let mut files = Vec::new();
    rs_files(&crates_dir(), &mut files);
    assert!(files.len() > 10, "Crates nicht gefunden");
    let mut bad = Vec::new();
    for f in &files {
        let n = norm(f);
        if n.contains("/k3c-devwin/src/win32/")
            || ALLOC_EXCEPTIONS.iter().any(|x| n.contains(x))
            || n.ends_with("/no_unsafe.rs")
        {
            continue;
        }
        let src = fs::read_to_string(f).unwrap();
        if !unsafe_lines(&src).is_empty() {
            bad.push(n);
        }
    }
    assert!(bad.is_empty(), "unsafe außerhalb von k3c-devwin/src/win32/: {bad:?}");
}

#[test]
fn jeder_unsafe_block_hat_safety() {
    let mut files = Vec::new();
    rs_files(&crates_dir().join("k3c-devwin/src/win32"), &mut files);
    let mut bad = Vec::new();
    for f in &files {
        let src = fs::read_to_string(f).unwrap();
        let lines: Vec<&str> = src.lines().collect();
        for i in unsafe_lines(&src) {
            let from = i.saturating_sub(3);
            if !lines[from..=i].iter().any(|l| l.contains("SAFETY:")) {
                bad.push(format!("{}:{}", norm(f), i + 1));
            }
        }
    }
    assert!(bad.is_empty(), "unsafe ohne // SAFETY: {bad:?}");
}
