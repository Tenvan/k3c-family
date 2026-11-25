//! Tests von `sim save` (RU40.2, AC-04 Vorarbeit): legt `voll-<seed>` an, prüft den Seed wie Go, kein Fenster.

use k3c_room::store::Stamp;
use serde_json::Value;

use super::save::valid_seed;
use super::testutil::{WorkDir, call};
use super::{EXIT_OK, EXIT_USAGE};

fn input(seed: &str, dir: &WorkDir) -> String {
    serde_json::json!({"seed": seed, "saves": dir.path()}).to_string()
}

// Der Spielstand liegt als `voll-<seed>.json` im Ordner, die Zeile `result` nennt den Namen, er lädt und ist ausgebaut.
#[test]
fn legt_voll_spielstand_an() {
    let dir = WorkDir::new("save");
    let (code, out, err) = call(&input("abc-1", &dir), &["save", "--input", "-"]);
    assert_eq!(code, EXIT_OK, "{err}");
    let line: Value = serde_json::from_str(out.trim()).expect("JSON-Zeile");
    assert_eq!(line["event"], "result");
    assert_eq!(line["text"], "voll-abc-1");
    let raw = std::fs::read(format!("{}/voll-abc-1.json", dir.path())).expect("Datei");
    let save: Value = serde_json::from_slice(&raw).expect("Spielstand");
    assert!(
        save["stages"].as_array().is_some_and(|s| !s.is_empty()),
        "Stufen fehlen"
    );
    // gleicher Seed: derselbe Stand bis auf savedAt
    let (code, _, err) = call(&input("abc-1", &dir), &["save", "--input", "-"]);
    assert_eq!(code, EXIT_OK, "{err}");
    let again: Value =
        serde_json::from_slice(&std::fs::read(format!("{}/voll-abc-1.json", dir.path())).expect("Datei"))
            .expect("Spielstand");
    assert_eq!(again["stages"], save["stages"]);
}

// Mit fester Uhr steht `savedAt` fest (Vergleich mit dem Go-Orakel möglich).
#[test]
fn feste_uhr_bestimmt_saved_at() {
    let dir = WorkDir::new("save-uhr");
    let src = k3c_data::Source::Embedded;
    let inp = super::input_world::SaveInput {
        seed: "uhr".into(),
        saves: dir.path(),
    };
    let name = super::save::run_with(&src, &inp, || Stamp::from_civil(2026, 10, 1, 12, 0, 0, 0)).expect("save");
    assert_eq!(name, "voll-uhr");
    let save: Value =
        serde_json::from_slice(&std::fs::read(format!("{}/voll-uhr.json", dir.path())).expect("Datei")).expect("JSON");
    assert_eq!(save["savedAt"], "2026-10-01T12:00:00.000Z");
}

// Seed wie Go: ^[a-z0-9-]{1,27}$, sonst Exit 2 und keine Datei.
#[test]
fn falscher_seed_ist_exit_2() {
    assert!(valid_seed("a") && valid_seed(&"a".repeat(27)) && valid_seed("0-z"));
    let dir = WorkDir::new("save-seed");
    for seed in ["", "ABC", "a b", "ä", "a/b", &"a".repeat(28)] {
        let (code, out, _) = call(&input(seed, &dir), &["save", "--input", "-"]);
        assert!(
            code == EXIT_USAGE && out.contains("\"event\":\"error\""),
            "Seed {seed:?}: Exit {code}, {out}"
        );
    }
    assert!(dir.files("", "").is_empty(), "es darf nichts angelegt werden");
}

// Ohne Ordner, mit unbekanntem Feld und mit --window ist der Aufruf falsch (Exit 2).
#[test]
fn aufruf_fehler_sind_exit_2() {
    let dir = WorkDir::new("save-aufruf");
    for (stdin, args) in [
        (r#"{"seed":"abc"}"#.to_string(), vec!["save", "--input", "-"]),
        (
            r#"{"seed":"abc","saves":"x","nope":1}"#.to_string(),
            vec!["save", "--input", "-"],
        ),
        (input("abc", &dir), vec!["save", "--input", "-", "--window"]),
    ] {
        let (code, _, err) = call(&stdin, &args);
        assert_eq!(code, EXIT_USAGE, "{args:?}: {err}");
    }
    assert!(dir.files("", "").is_empty());
}
