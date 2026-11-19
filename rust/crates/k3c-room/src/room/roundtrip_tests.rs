//! Format-Prüfung des Sprints RU24 (B-1014, AC-03): `spielstand_rundlauf`. Für jede Fixture `testdata/saves/v1` bis `v9`
//! wird ein Raum angelegt (`Manager::create` mit dem Stand), ein paar Takte gerechnet, gespeichert und neu geladen; der
//! JSON-Baum aller Stufen ist danach gleich und die neu geschriebene Datei hat die heutige Version 9. Es gibt keine neuen
//! Fixtures. Der Vergleich ist der eigene JSON-Diff aus `save_tests.rs` (`k3c-room` hängt nicht an `k3c-det`).
use serde_json::Value;

use super::Options;
use super::save_tests::diff;
use super::save_testutil::create_with;
use super::testutil::{manager, ticks};

/// Die Fixture `testdata/saves/v<version>/familie.json`.
fn fixture(version: u32) -> Vec<u8> {
    let path = format!(
        "{}/../../../testdata/saves/v{version}/familie.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// Legt einen Raum aus `data` an, rechnet `n` Takte (0: keine) und speichert; die Antwort ist die neue Datei.
fn run_and_save(data: &[u8], n: usize, what: &str) -> Vec<u8> {
    let (mut m, _) = manager();
    m.store.data.insert("familie".into(), data.to_vec());
    let (code, _xi) = create_with(&mut m, "xbox", 1, "familie", false, Options::default());
    ticks(&mut m, &code, n);
    assert_eq!(
        m.with_room(&code, |r, env| r.save_now(env).is_ok()),
        Some(true),
        "{what}: Speichern"
    );
    m.store.data["familie"].clone()
}

/// Der JSON-Baum aller Stufen des Stands `data`.
fn stages_of(data: &[u8], what: &str) -> Value {
    let (m, _) = manager();
    serde_json::to_value(&m.load_in(data).expect(what).stages).expect("Stufen")
}

// Jede Version laden und speichern: gleicher Zustands-Baum, die Datei hat Version 9. Dazu 20 Takte gerechnet: Speichern
// und Laden ist ein Fixpunkt (zweites Speichern ergibt dieselben Bytes und denselben Baum). Der Zustand einer laufenden
// Insel gleicht dem geladenen nicht in jedem Feld (z. B. `payCooldown` ist Laufzeit, kein Teil des Stands), darum
// vergleicht die Prüfung nach dem Rechnen die zwei Speicherungen miteinander.
#[test]
fn spielstand_rundlauf() {
    for version in 1..=9 {
        let raw = fixture(version);
        let what = format!("v{version}");
        let saved = run_and_save(&raw, 0, &what);
        let out: Value = serde_json::from_slice(&saved).expect("JSON");
        assert_eq!(out["version"], 11, "{what}: Version der neuen Datei");
        let (before, after) = (stages_of(&raw, &what), stages_of(&saved, &what));
        assert_eq!(
            diff("stages", &before, &after),
            "",
            "{what}: Zustand nach Speichern und Laden"
        );

        let first = run_and_save(&raw, 20, &what);
        let second = run_and_save(&first, 0, &what);
        assert_eq!(first, second, "{what}: zweites Speichern weicht ab");
        let (a, b) = (stages_of(&first, &what), stages_of(&second, &what));
        assert_eq!(
            diff("stages", &a, &b),
            "",
            "{what}: Zustand nach 20 Takten, Speichern und Laden"
        );
    }
}
