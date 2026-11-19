//! Kanonischer Welt-Hash (RU32.1, AC-01): Der Probe-Baum aus `testdata/rust/e2e/flow.json` (von Go geschrieben) ergibt
//! in Rust denselben Hash wie in Go, dazu drei Einzelfälle der kanonischen Form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::canon::{tree_hash, worlds_hash};
use k3c_det::golden;
use serde_json::{Value, json};

#[test]
fn probe_hash_wie_go() {
    let flow = golden::read(golden::testdata().join("rust/e2e/flow.json"));
    let probe = &flow["probe"];
    let want = probe["hash"].as_str().expect("probe.hash");
    assert_eq!(
        tree_hash(&probe["tree"]),
        want,
        "kanonischer Hash des Probe-Baums weicht von Go ab"
    );
    assert_eq!(flow["stations"].as_array().expect("stations").len(), 14);
}

#[test]
fn schluesselreihenfolge_ist_egal() {
    let a: Value = serde_json::from_str(r#"{"a":1,"b":{"x":[1,2],"y":"z"}}"#).unwrap();
    let b: Value = serde_json::from_str(r#"{"b":{"y":"z","x":[1,2]},"a":1}"#).unwrap();
    assert_eq!(tree_hash(&a), tree_hash(&b));
    assert_ne!(
        tree_hash(&json!([1, 2])),
        tree_hash(&json!([2, 1])),
        "Listen behalten ihre Reihenfolge"
    );
}

#[test]
fn null_gleich_leere_liste() {
    assert_eq!(tree_hash(&json!({"a": null})), tree_hash(&json!({"a": []})));
    assert_ne!(tree_hash(&json!({"a": null})), tree_hash(&json!({"a": [0]})));
}

#[test]
fn zahlen_haengen_an_den_bits() {
    assert_ne!(tree_hash(&json!(0.1)), tree_hash(&json!(0.100_000_000_000_000_1)));
    assert_eq!(
        tree_hash(&json!(0.0)),
        tree_hash(&json!(-0.0)),
        "minus null zählt als null"
    );
    assert_eq!(
        tree_hash(&json!(3)),
        tree_hash(&json!(3.0)),
        "ganze Zahl und Gleitkomma gleichen Werts"
    );
    assert_eq!(worlds_hash(&[]), "-");
    let w = [(1, json!({"a": 1})), (0, json!({"a": 2}))];
    let r = [(0, json!({"a": 2})), (1, json!({"a": 1}))];
    assert_eq!(worlds_hash(&w), worlds_hash(&r), "Stufen nach Index");
}
