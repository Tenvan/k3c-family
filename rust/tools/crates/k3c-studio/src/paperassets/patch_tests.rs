//! Port von `patch_test.go`.

use super::super::test_helpers::{repo_root, unter};
use super::*;

#[test]
fn merge_patch_faelle() {
    let faelle = [
        ("Feld ersetzen", r#"{"a":1,"b":2}"#, r#"{"a":3}"#, r#"{"a":3,"b":2}"#),
        ("null löscht", r#"{"a":1,"b":2}"#, r#"{"a":null}"#, r#"{"b":2}"#),
        (
            "verschachtelt",
            r#"{"u":{"m":"rot","l":5},"x":1}"#,
            r#"{"u":{"m":"purpur"}}"#,
            r#"{"u":{"m":"purpur","l":5},"x":1}"#,
        ),
        (
            "Array ersetzt komplett",
            r#"{"p":[1,2,3]}"#,
            r#"{"p":[9]}"#,
            r#"{"p":[9]}"#,
        ),
        (
            "neues Feld hinten",
            r#"{"a":1}"#,
            r#"{"z":{"k":null,"v":2}}"#,
            r#"{"a":1,"z":{"v":2}}"#,
        ),
        ("Patch kein Objekt", r#"{"a":1}"#, "[1,2]", "[1,2]"),
        ("Ziel kein Objekt", "[1]", r#"{"a":1}"#, r#"{"a":1}"#),
        (
            "Zahlen bleiben roh",
            r#"{"r":[-60, -145, 85.25, 151]}"#,
            "{}",
            r#"{"r":[-60,-145,85.25,151]}"#,
        ),
    ];
    for (name, ziel, patch, want) in faelle {
        assert_eq!(merge_patch(ziel, patch).unwrap(), want, "{name}");
    }
    assert!(merge_patch(r#"{"a":1}"#, r#"{"a":"#).is_err(), "kaputter Patch");
}

/// Neu: die Rekursion ist begrenzt.
#[test]
fn zu_tiefer_patch_abgelehnt() {
    let tief = format!("{}1{}", r#"{"a":"#.repeat(100_000), "}".repeat(100_000));
    assert!(merge_patch("{}", &tief).is_err());
    let rand = format!("{}1{}", r#"{"a":"#.repeat(128), "}".repeat(128));
    assert!(merge_patch("{}", &rand).is_ok());
}

#[test]
fn format_wie_fixtures() {
    let got = format(r#"{"id":"x","pivot":[0,-84],"teile":[{"id":"a","lagen":[]}]}"#).unwrap();
    let want = "{\n  \"id\": \"x\",\n  \"pivot\": [0, -84],\n  \"teile\": [\n    {\n      \"id\": \"a\",\n      \
                \"lagen\": []\n    }\n  ]\n}\n";
    assert_eq!(got, want);
    // Eine unveränderte Fixture-Spec kommt Byte für Byte gleich heraus (kein Diff-Rauschen nach assets_set).
    for name in ["monarch", "pferd", "mauer"] {
        let pfad = unter(
            &repo_root(),
            &std::format!("tools/paper/fixtures/specs/{name}.paper.json"),
        );
        let orig = std::fs::read_to_string(pfad).unwrap().replace("\r\n", "\n");
        let neu = merge_patch(&orig, "{}").unwrap();
        assert!(format(&neu).unwrap() == orig, "{name}: Format weicht vom Original ab");
    }
}

#[test]
fn schluessel_und_felder() {
    let raw = r#"{"$schema":"x","id":"m","umhang":{"material":"rot"},"ausgabe":{"hoehe_px":1},"animationen":{"idle":{},"lauf":{}}}"#;
    assert_eq!(schluessel(raw, &["animationen"]), ["idle", "lauf"]);
    let oben = schluessel(raw, &[]);
    assert_eq!((oben.len(), oben[0].as_str()), (5, "$schema"));
    assert_eq!(
        felder(raw, &["ausgabe", "umhang"]).unwrap(),
        r#"{"umhang":{"material":"rot"},"ausgabe":{"hoehe_px":1}}"#
    );
    let alle = felder(raw, &[]).unwrap();
    assert!(!alle.contains("$schema") && alle.contains(r#""animationen""#), "{alle}");
    assert_eq!(
        felder(raw, &["farbe"]).unwrap_err().to_string(),
        "Feld „farbe“ fehlt; vorhanden: id, umhang, ausgabe, animationen"
    );
}
