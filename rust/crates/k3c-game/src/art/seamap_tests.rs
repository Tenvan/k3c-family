//! Tests der Seekarten-Zuordnung (Go: `seamap_test.go`) gegen das echte `data/`.

use std::collections::BTreeSet;

use super::STANDARD_WORLD;
use super::datafile::read_in;
use super::seamap::SeaMap;
use super::testutil::real_data;

/// B-964/R1, R5: Die Zuordnung in data/paper.json ist vollständig; eine Welt ohne Eintrag nutzt die Miniatur von standard.
#[test]
fn sea_map_aus_daten() {
    let m = SeaMap::parse(&read_in(&real_data(), "paper.json").unwrap()).expect("data/paper.json › seekarte lesbar");
    let std = m.islands[STANDARD_WORLD].as_str();
    assert_eq!(m.island("gibt-es-nicht"), std);
    assert_eq!(m.island(""), std);
    assert_ne!(
        m.island("sengoku"),
        m.island(STANDARD_WORLD),
        "sengoku hat keine eigene Miniatur"
    );
    let a = m.assets();
    assert!(a.is_sorted(), "sortiert: {a:?}");
    assert_eq!(a.iter().collect::<BTreeSet<_>>().len(), a.len(), "ohne Doppelte: {a:?}");
    assert_eq!(a.len(), 6 + m.toppers.len() + m.islands.len(), "Assets {a:?}");
}

/// B-964/R2: Der Aufsatz hängt nur am Hash; jeder Aufsatz kommt vor.
#[test]
fn sea_map_aufsatz_fest() {
    let m = SeaMap {
        toppers: vec!["a".into(), "b".into(), "c".into()],
        ..SeaMap::default()
    };
    let mut seen = BTreeSet::new();
    for seed in 0..9u32 {
        assert_eq!(m.topper(seed), m.toppers[seed as usize % 3], "seed {seed}");
        seen.insert(m.topper(seed));
    }
    assert_eq!(seen.len(), 3, "Aufsätze {seen:?}");
    let leer = SeaMap::default();
    assert_eq!((leer.topper(1), leer.island("x")), ("", ""));
    assert!(
        leer.assets().iter().all(String::is_empty),
        "leere Zuordnung nennt keine echten Assets"
    );
}

/// Unvollständige Zuordnung ist ein Fehler (das HUD zeichnet dann Formen).
#[test]
fn sea_map_unvollstaendig() {
    let sea = r#""meer":{"hinten":"o/h","mitte":"o/m","vorn":"o/v"},"schiff":"o/s""#;
    let marks = r#""zeichen":{"abgeschlossen":"o/f","gesperrt":"o/n"}"#;
    let full = format!(r#"{{"seekarte":{{"inseln":{{"standard":"o/i"}},{sea},"aufsaetze":["o/t"],{marks}}}}}"#);
    SeaMap::parse(full.as_bytes()).expect("vollständig");
    let cases = [
        ("fehlt", "{}".to_string()),
        ("ohne standard", full.replace("standard", "nil")),
        ("ohne aufsatz", full.replace(r#"["o/t"]"#, "[]")),
        ("ohne schiff", full.replace(r#","schiff":"o/s""#, "")),
        ("kaputt", r#"{"seekarte":"#.to_string()),
    ];
    for (name, raw) in cases {
        assert!(SeaMap::parse(raw.as_bytes()).is_err(), "{name}: kein Fehler");
    }
}
