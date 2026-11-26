//! Go `writeguard_test.go` (WZ63.7/AC-09): Regeln und Helfer. Die Quelltext-Prüfung auf Schreibaufrufe am Wächter
//! vorbei (`TestBalancingSchreibtNurUeberWaechter`) folgt, wenn `balance` und `simrun` vollständig sind.

use std::fs;

use super::super::test_helpers::temp_root;
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
