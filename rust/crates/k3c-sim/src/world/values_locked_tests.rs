//! Eigene Tests zu `world/values_locked.rs` (Go prüft `checkLocked` nur über `TestB692GesperrteFelder` mit `mergeFile`;
//! hier direkt, damit sie ohne Merge laufen). docs/rules/welten.md § 1.3, B-692.

use serde_json::{Value, json};

use super::values_locked::{check_locked, locked_fields};

fn err(name: &str, ov: Value) -> Option<String> {
    check_locked(name, &ov).err()
}

#[test]
fn own_locked_fields_je_datei() {
    assert_eq!(locked_fields("economy"), ["payRangeUnits"]);
    assert_eq!(locked_fields("hub"), ["sitePlacement"]);
    assert!(locked_fields("buildings").is_empty());
    assert!(locked_fields("waves").is_empty());
}

#[test]
fn own_economy_pay_range_units_gesperrt() {
    let e = err("economy", json!({"payRangeUnits": 3})).unwrap_or_default();
    assert_eq!(
        e,
        "payRangeUnits: nicht übersteuerbar (der Level-Generator liest die Basis, welten.md § 1.3)"
    );
}

#[test]
fn own_hub_site_placement_gesperrt() {
    let e = err("hub", json!({"sitePlacement": {"zone1WallUnits": 5}})).unwrap_or_default();
    assert!(e.starts_with("sitePlacement: nicht übersteuerbar"), "{e}");
}

#[test]
fn own_andere_felder_der_gesperrten_dateien_frei() {
    assert_eq!(err("economy", json!({"payIntervalSeconds": 1})), None);
    assert_eq!(err("hub", json!({"travel": {"seconds": 2}})), None);
}

#[test]
fn own_plantation_strip_units_gesperrt_mit_pfad() {
    let e = err("buildings", json!({"nursery": {"plantation": {"stripUnits": 5}}})).unwrap_or_default();
    assert!(
        e.starts_with("nursery.plantation.stripUnits: nicht übersteuerbar"),
        "{e}"
    );
}

#[test]
fn own_plantation_andere_felder_frei() {
    assert_eq!(
        err("buildings", json!({"nursery": {"hp": 120, "plantation": {"slots": 4}}})),
        None
    );
}

#[test]
fn own_plantation_null_ist_in_go_ein_eintrag() {
    // Go: `json.RawMessage("null")` ist nicht nil, der Schlüssel zählt als gesetzt.
    let e = err("buildings", json!({"nursery": {"plantation": {"stripUnits": null}}}));
    assert!(e.is_some());
    assert_eq!(err("buildings", json!({"nursery": {"plantation": null}})), None);
}

#[test]
fn own_erste_gesperrte_art_in_sortierter_reihenfolge() {
    let ov = json!({"zebra": {"plantation": {"stripUnits": 1}}, "alpha": {"plantation": {"stripUnits": 2}}});
    let e = err("buildings", ov).unwrap_or_default();
    assert!(e.starts_with("alpha.plantation.stripUnits"), "{e}");
}

#[test]
fn own_formfehler_meldet_merge_file_nicht_check_locked() {
    assert_eq!(err("economy", json!(7)), None);
    assert_eq!(err("buildings", json!({"wall": 5})), None);
    assert_eq!(err("buildings", json!({"wall": {"plantation": 5}})), None);
}
