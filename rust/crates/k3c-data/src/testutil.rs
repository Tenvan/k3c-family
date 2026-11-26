//! Hilfen der Tests: Daten lesen und mit dem Baum aus Go vergleichen (testdata/rust/data-tree.json.gz).

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use k3c_det::golden;
use serde::Serialize;
use serde_json::Value;

/// `data_dir` ist der Ordner data/ des Repos.
pub(crate) fn data_dir() -> PathBuf {
    golden::testdata().join("../data")
}

/// `raw` liest eine Datei unter data/.
pub(crate) fn raw(name: &str) -> String {
    let p = data_dir().join(name);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

/// `go_tree` ist der Baum aus Go je Datei (einmal gelesen).
pub(crate) fn go_tree() -> &'static BTreeMap<String, Value> {
    static TREE: OnceLock<BTreeMap<String, Value>> = OnceLock::new();
    TREE.get_or_init(|| {
        let v = golden::read(golden::testdata().join("rust/data-tree.json.gz"));
        serde_json::from_value(v).expect("data-tree.json.gz ist ein Objekt")
    })
}

/// `same_as_go` vergleicht den Baum von got mit dem Eintrag name aus Go.
pub(crate) fn same_as_go<T: Serialize + ?Sized>(name: &str, got: &T) {
    let want = go_tree().get(name).unwrap_or_else(|| {
        panic!("{name} fehlt in data-tree.json.gz (eingefroren seit RU41, Go-Schreibweg im Tag go-referenz-final)")
    });
    let d = golden::diff(name, want, &golden::tree(got));
    assert!(d.is_empty(), "Rust weicht von Go ab: {d}");
}

/// `same_as_go_zahlen_aus_daten` wie `same_as_go`, aber alle Zahlen kommen aus den Daten: Der Go-Baum ist
/// eingefroren, die Asset-Sprints (BB1, #772) ändern Maße in data/paper.json weiter. Verglichen bleiben
/// Schlüssel aus Go (neue Schlüssel in den Daten sind erlaubt), Texte, Wahrheitswerte und Listenlängen.
pub(crate) fn same_as_go_zahlen_aus_daten<T: Serialize + ?Sized>(name: &str, got: &T) {
    let mut want = go_tree()
        .get(name)
        .unwrap_or_else(|| panic!("{name} fehlt in data-tree.json.gz"))
        .clone();
    let mut have = golden::tree(got);
    zahlen_uebernehmen(&mut want, &mut have);
    let d = golden::diff(name, &want, &have);
    assert!(d.is_empty(), "Rust weicht von Go ab: {d}");
}

fn zahlen_uebernehmen(want: &mut Value, have: &mut Value) {
    match (want, have) {
        (w @ Value::Number(_), h @ Value::Number(_)) => *w = h.clone(),
        (Value::Object(w), Value::Object(h)) => {
            h.retain(|k, _| w.contains_key(k));
            for (k, wv) in w.iter_mut() {
                if let Some(hv) = h.get_mut(k) {
                    zahlen_uebernehmen(wv, hv);
                }
            }
        }
        (Value::Array(w), Value::Array(h)) => {
            for (wv, hv) in w.iter_mut().zip(h.iter_mut()) {
                zahlen_uebernehmen(wv, hv);
            }
        }
        _ => {}
    }
}
