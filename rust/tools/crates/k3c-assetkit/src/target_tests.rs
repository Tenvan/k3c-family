//! Port von `target_test.go`.

use std::fs;

use super::*;
use crate::register::{REGISTER_PATH, load};
use crate::testutil::{repo, write_file};

fn units(w: f64, h: f64) -> Option<Units> {
    Some(Units { w, h })
}

/// B-685/AC-03: Zielmaß setzen, entfernen und Schreibregel; nur Grafik-Slots, ungültige Werte ohne Schreibzugriff.
#[test]
fn test_set_target_if() {
    let dir = repo();
    let root = dir.path();
    let stand = stamp(root).unwrap();
    let (slot, next) = set_target_if(root, "farm", units(6.0, 4.5), &stand).unwrap();
    assert!(slot.target == units(6.0, 4.5) && next != stand, "{slot:?}");
    assert_eq!(load(root).unwrap().find("farm").unwrap().target, units(6.0, 4.5));
    let err = set_target_if(root, "farm", units(1.0, 1.0), &stand).unwrap_err();
    assert!(matches!(err, Error::Stale), "{err}");
    let before = fs::read(root.join(REGISTER_PATH)).unwrap();
    for (name, id, t) in [
        ("negativ", "farm", units(-1.0, 2.0)),
        ("zu groß", "farm", units(2.0, 101.0)),
        ("Sound", "sfx:coin", units(1.0, 1.0)),
        ("unbekannt", "x", None),
    ] {
        assert!(set_target_if(root, id, t, &next).is_err(), "{name}: kein Fehler");
    }
    assert_eq!(
        fs::read(root.join(REGISTER_PATH)).unwrap(),
        before,
        "abgelehntes Zielmaß hat geschrieben"
    );
    let (slot, _) = set_target_if(root, "farm", units(0.0, 3.0), &next).unwrap();
    assert_eq!(slot.target, None, "Seite 0 entfernt das Zielmaß");
}

/// B-685: Ein ungültiges Zielmaß im Register ist ein Ladefehler mit Slot.
#[test]
fn load_prueft_ziel() {
    let dir = repo();
    write_file(
        &dir.path().join(REGISTER_PATH),
        r#"{"_comment": "", "slots": [{"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "", "ziel": {"w": -2, "h": 3}, "wahl": []}]}"#,
    );
    let err = load(dir.path()).unwrap_err().to_string();
    assert!(err.contains("farm"), "Fehler {err}, erwartet mit Slot farm");
    assert!(err.contains("ziel -2x3 Units ungültig (je Seite 0 bis 100)"), "{err}");
}

/// Eigener Test: Zahlen der Meldung wie Go `%g`.
#[test]
fn meldung_wie_go_g() {
    for (v, want) in [
        (101.0, "101"),
        (-0.5, "-0.5"),
        (1e6, "1e+06"),
        (1234567.5, "1.2345675e+06"),
        (0.0001, "0.0001"),
        (0.00001, "1e-05"),
        (f64::NAN, "NaN"),
        (f64::INFINITY, "+Inf"),
    ] {
        assert_eq!(go_g(v), want);
    }
}
