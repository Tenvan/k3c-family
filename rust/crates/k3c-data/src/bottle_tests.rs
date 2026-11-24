//! Tests zu `bottle.rs`: die echte Datei lädt, und jede Verletzung der Prüfung wird abgelehnt.

use super::*;

const OK: &str = r#"{"firstDay":3,"everyDays":4,"edgeMarginMinUnits":2,"edgeMarginMaxUnits":6,
    "pickupRangeUnits":1.5,"targetRangeUnits":1.5}"#;

#[test]
fn echte_datei_laedt_mit_den_startwerten() {
    let raw = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../data/bottle.json"))
        .expect("bottle.json");
    let b = load(&raw).expect("gültig");
    assert_eq!((b.first_day, b.every_days), (3, 4));
}

#[test]
fn gueltiger_satz_wird_angenommen() {
    assert!(load(OK).is_ok());
}

#[test]
fn ungueltige_werte_werden_abgelehnt() {
    let cases = [
        (
            r#"{"firstDay":-1,"everyDays":4,"edgeMarginMinUnits":2,"edgeMarginMaxUnits":6,"pickupRangeUnits":1,"targetRangeUnits":1}"#,
            "firstDay",
        ),
        (
            r#"{"firstDay":3,"everyDays":0,"edgeMarginMinUnits":2,"edgeMarginMaxUnits":6,"pickupRangeUnits":1,"targetRangeUnits":1}"#,
            "everyDays",
        ),
        (
            r#"{"firstDay":3,"everyDays":4,"edgeMarginMinUnits":7,"edgeMarginMaxUnits":6,"pickupRangeUnits":1,"targetRangeUnits":1}"#,
            "edgeMargin",
        ),
        (
            r#"{"firstDay":3,"everyDays":4,"edgeMarginMinUnits":2,"edgeMarginMaxUnits":6,"pickupRangeUnits":0,"targetRangeUnits":1}"#,
            "größer 0",
        ),
    ];
    for (raw, want) in cases {
        let err = load(raw).expect_err("muss abgelehnt werden").to_string();
        assert!(err.contains(want), "{err} enthält nicht {want}");
    }
}
