//! Go `tuneparam_test.go`.

use super::super::test_helpers::repo_root;
use super::{locate_number, parse_tune_param, proposal_diff};

/// `locate_number` findet Zahlen in Listen und verschachtelten Objekten; der Diff nennt nur geänderte Zeilen.
#[test]
fn tune_diff_verschachtelt() {
    let raw = "{\n  \"a\": [1, {\"b\": 2.5}],\n  \"c\": {\"d\": 3}\n}\n";
    for (path, want) in [("a.0", "1"), ("a.1.b", "2.5"), ("c.d", "3")] {
        let keys: Vec<&str> = path.split('.').collect();
        let (start, end) = locate_number(raw, &keys).expect(path);
        assert_eq!(&raw[start..end], want, "{path}");
    }
    assert!(locate_number(raw, &["c"]).is_err(), "objekt als Zahl gefunden");
    let root = repo_root();
    let p = parse_tune_param(&root, "economy.json:dawnGoldPerPlayer=1..9").expect("Parameter");
    let same = proposal_diff(&root, std::slice::from_ref(&p), &[p.base]).expect("Diff");
    assert_eq!(same, "", "diff ohne Änderung");
    let diff = proposal_diff(&root, std::slice::from_ref(&p), &[p.base + 1.0]).expect("Diff");
    assert!(
        diff.starts_with("--- data/economy.json\n+++ data/economy.json\n@@ -"),
        "{diff}"
    );
}

/// Fehlertexte wie Go.
#[test]
fn parameter_fehler() {
    let root = repo_root();
    for (spec, want) in [
        ("economy.json", "erwartet datei.json:a.b.c=min..max"),
        ("economy.json:=1..2", "erwartet datei.json:a.b.c=min..max"),
        ("economy.json:dawnGoldPerPlayer=2..2", "min < max"),
        ("economy.json:dawnGoldPerPlayer=a..2", "min < max"),
        ("economy.json:fehlt=1..2", "\"fehlt\" fehlt"),
    ] {
        let err = parse_tune_param(&root, spec).expect_err(spec);
        assert!(err.starts_with("parameter \"") && err.contains(want), "{spec}: {err}");
    }
}
