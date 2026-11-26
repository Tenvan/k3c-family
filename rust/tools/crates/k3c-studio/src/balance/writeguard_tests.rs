//! Go `writeguard_test.go` (WZ63.7/AC-09): Regeln, Helfer und die Quelltext-Prüfung auf Schreibaufrufe am Wächter
//! vorbei.

use std::fs;
use std::path::Path;

use regex::Regex;

use super::super::test_helpers::{repo_root, temp_root};
use super::{guard, mkdir_all, remove, remove_all, rename, write_file};

/// Das Balancing schreibt Quelldateien nur unter `data/worlds/`, Laufausgaben nur unter `reports/`.
#[test]
fn waechter_regeln() {
    let (_dir, root) = temp_root();
    for p in [
        "data/worlds/standard.json",
        "reports/simtest-7/simtest.md",
        "reports/baseline/M1/run.json",
        "reports/ratings.json",
        "reports/r/result.json",
        "reports/r/balance-20261122-100000.html",
    ] {
        assert_eq!(guard().check(&root, p), Ok(()), "{p} abgelehnt");
    }
    let outside = root.parent().expect("Eltern").join("fremd.json");
    for p in [
        "data/troops.json",
        "data/economy.json",
        "data/paper.json",
        "engine/sim/x.go",
        "rust/crates/k3c-sim/src/lib.rs",
        "tools/k3c-studio/app.go",
        "docs/arbeitsweise.md",
        "cmd/k3c-sim/main.go",
        "public/grafik/a.png",
        "saves/a.json",
        "testdata/balance/baseline.json",
        "data/worlds/../troops.json",
        outside.to_str().expect("Text"),
    ] {
        let err = guard().check(&root, p).expect_err(p);
        assert!(
            err.contains("Plan im Chat"),
            "{p}: Meldung ohne Hinweis auf den Plan im Chat: {err}"
        );
    }
}

/// Die Helfer schreiben nichts, wenn der Wächter ablehnt.
#[test]
fn helfer_schreiben_nichts_ausserhalb() {
    let (_dir, root) = temp_root();
    assert!(
        write_file(&root, "data/troops.json", b"{}").is_err(),
        "data/troops.json geschrieben"
    );
    assert!(
        mkdir_all(&root, root.join("engine").join("sim")).is_err(),
        "engine/sim angelegt"
    );
    assert!(!root.join("data").exists(), "data/ entstand trotz Ablehnung");
    mkdir_all(&root, "reports/r").expect("Ordner");
    write_file(&root, "reports/r/a.md", b"x").expect("Datei");
    let r = root.join("reports").join("r");
    rename(&root, r.join("a.md"), r.join("b.md")).expect("umbenennen");
    remove(&root, "reports/r/b.md").expect("Datei löschen");
    remove_all(&root, "reports/r").expect("Ordner löschen");
    assert!(!r.exists());
    remove_all(&root, "reports/r").expect("fehlend ist kein Fehler");
}

/// Löschen folgt keiner Junction nach außen: darin verschwindet nur der Link, als Ziel oder auf dem Weg wird abgelehnt.
#[cfg(windows)]
#[test]
fn loeschen_folgt_keiner_junction() {
    use super::super::test_helpers::junction;

    let (_dir, root) = temp_root();
    let outside = root.parent().expect("Eltern").join("draussen");
    fs::create_dir_all(&outside).expect("draußen");
    fs::write(outside.join("bleibt.txt"), "x").expect("Datei");
    mkdir_all(&root, "reports/r").expect("Ordner");
    assert!(
        junction(&root.join("reports").join("r").join("raus"), &outside),
        "mklink /J"
    );
    assert!(remove_all(&root, "reports/r/raus").is_err(), "Junction als Ziel");
    assert!(
        write_file(&root, "reports/r/raus/neu.txt", b"x").is_err(),
        "über die Junction geschrieben"
    );
    remove_all(&root, "reports/r").expect("Ordner löschen");
    assert!(!root.join("reports").join("r").exists());
    assert!(outside.join("bleibt.txt").is_file() && !outside.join("neu.txt").exists());
}

/// Die Rust-Quellen unter `path` ohne Testdateien (Pfad ab `src/` → Inhalt).
fn sources(src: &Path, path: &Path, out: &mut Vec<(String, String)>) {
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Ordner") {
            sources(src, &entry.expect("Eintrag").path(), out);
        }
        return;
    }
    let rel = path
        .strip_prefix(src)
        .expect("unter src")
        .to_string_lossy()
        .replace('\\', "/");
    let test = rel.ends_with("_tests.rs") || rel.ends_with("test_helpers.rs");
    if rel.ends_with(".rs") && !test {
        out.push((rel, fs::read_to_string(path).expect("Quelle")));
    }
}

/// `balance`, `simrun`, `datadoc`, `gamedata` und die Balance-Bindings schreiben nur über die Helfer mit Wächter.
/// `simrun`, `datadoc` und `app/balance*.rs` zählen mit, sobald es sie gibt (RU70.18 bis RU70.20).
#[test]
fn balancing_schreibt_nur_ueber_waechter() {
    let raw_write = Regex::new(
        r"\b(fs::(write|rename|copy|remove_file|remove_dir|remove_dir_all|create_dir|create_dir_all|hard_link|set_permissions)|File::(create|create_new|options)|OpenOptions::new|write_atomic|symlink_(file|dir))\(",
    )
    .expect("Muster");
    let src = repo_root().join("rust/tools/crates/k3c-studio/src");
    let mut files = Vec::new();
    for dir in ["balance", "simrun", "datadoc", "gamedata"] {
        let path = src.join(dir);
        if path.exists() {
            sources(&src, &path, &mut files);
        }
    }
    for entry in fs::read_dir(src.join("app")).expect("app") {
        let path = entry.expect("Eintrag").path();
        if path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with("balance"))
        {
            sources(&src, &path, &mut files);
        }
    }
    assert!(
        files.len() >= 20,
        "nur {} Dateien gelesen: Pfade stimmen nicht",
        files.len()
    );
    // die Helfer selbst, jeder Aufruf hinter dem Wächter
    let allowed = "balance/writeguard.rs";
    for (rel, text) in &files {
        let found: Vec<&str> = raw_write.find_iter(text).map(|m| m.as_str()).collect();
        if rel == allowed {
            assert!(!found.is_empty(), "{rel} schreibt nicht mehr roh: Ausnahme streichen");
        } else {
            assert!(found.is_empty(), "{rel} schreibt am Wächter vorbei: {found:?}");
        }
    }
    assert!(files.iter().any(|(rel, _)| rel == allowed));
}
