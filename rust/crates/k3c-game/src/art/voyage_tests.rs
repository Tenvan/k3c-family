//! Tests der Schiffsfahrt-Werte (Go: `voyage_test.go`) gegen das echte `data/`.

use super::datafile::read_in;
use super::testutil::real_data;
use super::voyage::{VoyageLook, parse_voyage, voyage};

/// GS2.4: Die Werte stehen in data/native.json › voyage; Datei und Standard im Code sind gleich, kein Wert ist unzulässig.
#[test]
fn voyage_wie_daten() {
    let raw = read_in(&real_data(), "native.json").unwrap();
    let (v, bad, err) = parse_voyage(&raw);
    assert!(
        err.is_none() && bad.is_empty(),
        "native.json: Fehler {err:?}, unzulässig {bad:?}"
    );
    assert_eq!(v, VoyageLook::default(), "native.json › voyage gegen Standard");
    // `voyage()` liest `data/` relativ zum Arbeitsordner (hier nicht vorhanden): ohne Datei gilt der Standard.
    assert_eq!(voyage(), v);
}

/// Fehlende Werte behalten den Standard, unzulässige fallen auf ihn zurück und werden genannt; 0 ist bei Bogen,
/// Schaukeln und Welle erlaubt. Kaputtes JSON liefert den Standard mit Fehler.
#[test]
fn parse_voyage_faelle() {
    let def = VoyageLook::default();
    let (v, bad, err) = parse_voyage(br#"{"stepBudgetMs": 4}"#);
    assert!(
        err.is_none() && bad.is_empty() && v == def,
        "ohne Block: {v:?}, {bad:?}, {err:?}"
    );

    let (v, bad, err) =
        parse_voyage(br#"{"voyage": {"seconds": 5, "arcPx": 0, "bobPx": -1, "flagAt": 2, "scale": 0}}"#);
    let want = VoyageLook {
        seconds: 5.0,
        arc: 0.0,
        ..def
    };
    assert!(err.is_none(), "{err:?}");
    assert_eq!(v, want, "teils unzulässig");
    assert_eq!(bad, ["scale", "bobPx", "flagAt"]);

    let (v, _, err) = parse_voyage(br#"{"voyage": 3"#);
    assert!(err.is_some() && v == def, "kaputtes JSON: {v:?}, {err:?}");
}
