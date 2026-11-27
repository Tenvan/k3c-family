//! Port von `entry_test.go`, dazu die Eigenheiten, die Go mit `encoding/json` und `time` mitbringt.

use super::entry::parse;

/// `TestParseTrenntCheckoutVonDenDaten`.
#[test]
fn parse_trennt_checkout_von_den_daten() {
    let line = br#"{"time":"2026-09-30T09:00:00Z","level":"info","msg":"x","ns":"mcp","checkout":"bed-projekt","tool":"check_run"}"#;
    let entry = parse(line).unwrap();
    assert_eq!(entry.checkout, "bed-projekt");
    let data = entry.data.unwrap();
    assert!(
        !data.contains_key("checkout") && data["tool"] == "check_run",
        "{data:?}"
    );
}

#[test]
fn zahlen_und_schluessel_wie_go() {
    let line = br#"{"time":"2026-09-30T09:00:00.5000+00:00","z":1.0,"a":{"y":2.5,"b":8080}}"#;
    let entry = parse(line).unwrap();
    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains(r#""time":"2026-09-30T09:00:00.5Z""#), "{json}");
    assert!(json.contains(r#""data":{"a":{"b":8080,"y":2.5},"z":1}"#), "{json}");
}

#[test]
fn ohne_gueltige_zeit_kein_eintrag() {
    for line in [
        "das ist kein JSON",
        "[1]",
        "null",
        r#"{"msg":"x"}"#,
        r#"{"time":"2026-09-30 09:00:00Z"}"#,
        r#"{"time":"2026-09-30t09:00:00z"}"#,
        r#"{"time":5}"#,
    ] {
        assert!(parse(line.as_bytes()).is_none(), "{line}");
    }
}
