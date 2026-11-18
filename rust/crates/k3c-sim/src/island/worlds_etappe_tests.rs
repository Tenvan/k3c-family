//! Etappen der Welt-Datei (Go: `island_worlds_etappe_test.go`, B-961 AC-01): Feld `etappe` je Insel, Standardwert
//! Listenplatz + 1, Prüfung lückenlos und nicht fallend; jede Insel außer der einzigen der letzten Etappe braucht einen
//! Hafen. Alle drei Tests laufen.
use super::switch_testutil::{test_island, test_world};
use super::testutil::*;

const HARBOR_PLAN: &str = r#","plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}"#;

/// Go: `etappeWorld`: je Feld eine Insel `a`, `b`, … mit Hafen.
fn etappe_world(fields: &[&str]) -> String {
    let islands: Vec<String> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| test_island(&char::from(b'a' + i as u8).to_string(), &format!("{HARBOR_PLAN}{f}")))
        .collect();
    test_world(&islands.join(","))
}

/// Go: `TestEtappeStandardIstListenplatz`.
#[test]
fn etappe_standard_ist_listenplatz() {
    let w = parse_world_in(&etappe_world(&["", "", ""]), "test").expect("Welt");
    for (i, d) in w.islands.iter().enumerate() {
        let want = i as i64 + 1;
        assert!(
            d.etappe == want && w.etappe_of(i as i64) == want,
            "Insel {i}: Etappe {}",
            d.etappe
        );
    }
}

/// Go: `TestEtappeGeladenUndGeprueft`.
#[test]
fn etappe_geladen_und_geprueft() {
    let w = parse_world_in(&etappe_world(&["", r#","etappe":2"#, r#","etappe":2"#]), "test").expect("Welt");
    let got: Vec<i64> = w.islands.iter().map(|d| d.etappe).collect();
    assert_eq!(got, [1, 2, 2], "Etappen");
    let cases: [(&str, &[&str]); 4] = [
        ("Insel b: etappe 3 lässt Etappe 2 aus", &["", r#","etappe":3"#]),
        (
            "Insel a: etappe 2 lässt Etappe 1 aus",
            &[r#","etappe":2"#, r#","etappe":2"#],
        ),
        (
            "Insel c: etappe 1 fällt unter die Etappe 2",
            &["", r#","etappe":2"#, r#","etappe":1"#],
        ),
        ("Insel a: etappe 0 ungültig", &[r#","etappe":0"#]),
    ];
    for (want, fields) in cases {
        match parse_world_in(&etappe_world(fields), "test") {
            Err(e) if e.contains(want) => {}
            other => panic!("{fields:?}: erwartet {want:?}, Ergebnis {other:?}"),
        }
    }
}

/// Go: `TestEtappeHafenBeiMehrerenInselnDerLetztenEtappe`: Jede Insel außer der einzigen der letzten Etappe braucht einen
/// Hafen.
#[test]
fn etappe_hafen_bei_mehreren_inseln_der_letzten_etappe() {
    let raw = test_world(&format!(
        "{},{},{}",
        test_island("a", HARBOR_PLAN),
        test_island("b", &format!(r#"{HARBOR_PLAN},"etappe":2"#)),
        test_island("c", r#","etappe":2"#)
    ));
    match parse_world_in(&raw, "test") {
        Err(e) if e.contains("Insel c: kein Hafen") => {}
        other => panic!("letzte Etappe mit zwei Inseln: {other:?}"),
    }
}
