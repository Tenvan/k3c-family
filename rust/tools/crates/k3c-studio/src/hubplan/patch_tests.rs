//! Port von `patch_test.go` (B-462/AC-06): format-stabiles Schreiben, unabhängig von der Form des Kandidaten.

use serde_json::{Value, json};

use super::super::editor_tests::{line_diff, remarshal};
use super::{patch_json, same_json};

const PATCH_DOC: &str = "{\r\n  \"a\": [\r\n    { \"k\": \"x\", \"n\": 1.50 },\r\n    { \"k\": \"y\", \"n\": 2 }\r\n  ],\r\n  \"b\": { \"u\": [1, 2, 3], \"g\": 2 },\r\n  \"c\": \"ü\"\r\n}\r\n";

fn patched(old: &str, cand: &str) -> String {
    patch_json(old, cand).unwrap()
}

#[test]
fn patch_unveraendert_ist_byte_gleich() {
    assert_eq!(patched(PATCH_DOC, &remarshal(PATCH_DOC, |_| {})), PATCH_DOC);
}

#[test]
fn patch_eine_zahl_eine_zeile() {
    let cand = remarshal(PATCH_DOC, |m| m["a"][1]["n"] = json!(3));
    let (removed, added) = line_diff(PATCH_DOC, &patched(PATCH_DOC, &cand));
    assert_eq!(removed, [r#"    { "k": "y", "n": 2 }"#]);
    assert_eq!(added, [r#"    { "k": "y", "n": 3 }"#]);
}

#[test]
fn patch_schluessel_und_eintraege() {
    let cand = remarshal(PATCH_DOC, |m| {
        m["a"][0]["d"] = json!(true);
        m["a"].as_array_mut().unwrap().push(json!({"k": "z"}));
        m["b"]["u"] = json!([1, 3]);
    });
    let got = patched(PATCH_DOC, &cand);
    for want in [
        r#"    { "k": "x", "n": 1.50, "d": true },"#,
        r#"    { "k": "y", "n": 2 },"#,
        "    { \"k\": \"z\" }\r\n  ],",
        r#""u": [1, 3]"#,
    ] {
        assert!(got.contains(want), "fehlt {want:?} in\n{got}");
    }
    assert!(got.ends_with("\"c\": \"ü\"\r\n}\r\n"), "Ende verändert:\n{got}");
}

#[test]
fn patch_eintrag_entfernen() {
    let cand = remarshal(PATCH_DOC, |m| {
        m["a"].as_array_mut().unwrap().remove(0);
    });
    let (removed, added) = line_diff(PATCH_DOC, &patched(PATCH_DOC, &cand));
    assert_eq!(removed, [r#"    { "k": "x", "n": 1.50 },"#]);
    assert!(added.is_empty(), "{added:?}");
}

#[test]
fn patch_mehrzeiliger_neuer_eintrag() {
    let doc = "{\n  \"s\": [\n    {\n      \"k\": \"x\"\n    }\n  ]\n}\n";
    let got = patched(doc, r#"{"s":[{"k":"x"},{"k":"y","h":2}]}"#);
    let want =
        "{\n  \"s\": [\n    {\n      \"k\": \"x\"\n    },\n    {\n      \"k\": \"y\",\n      \"h\": 2\n    }\n  ]\n}\n";
    assert_eq!(got, want);
}

#[test]
fn patch_ungueltiger_kandidat() {
    assert!(patch_json(PATCH_DOC, r#"{"a": ["#).is_err());
}

/// Tabellenfälle: Zeilenenden, Zahlenformat, Umlaute, doppelter Schlüssel. `want` leer: nur Bedeutung gleich `cand`.
#[test]
fn patch_faelle() {
    let cases = [
        (
            "LF bleibt LF",
            "{\n  \"a\": [1, 2],\n  \"b\": 1\n}\n",
            r#"{"a":[1,2,3],"b":1}"#,
            "{\n  \"a\": [1, 2, 3],\n  \"b\": 1\n}\n",
        ),
        (
            "CRLF bleibt CRLF",
            "{\r\n  \"s\": [\r\n    { \"k\": 1 }\r\n  ]\r\n}\r\n",
            r#"{"s":[{"k":1},{"k":2}]}"#,
            "{\r\n  \"s\": [\r\n    { \"k\": 1 },\r\n    { \"k\": 2 }\r\n  ]\r\n}\r\n",
        ),
        (
            "44.0 gleich 44",
            "{ \"w\": 44.0, \"g\": 2 }\n",
            r#"{"w":44,"g":3}"#,
            "{ \"w\": 44.0, \"g\": 3 }\n",
        ),
        (
            "Umlaut im Klartext",
            "{\n  \"n\": \"Rüstkammer\",\n  \"x\": 1\n}\n",
            r#"{"n":"Rüstkammer","x":1,"m":"Lager für Erz"}"#,
            "{\n  \"n\": \"Rüstkammer\",\n  \"x\": 1,\n  \"m\": \"Lager für Erz\"\n}\n",
        ),
        ("doppelter Schlüssel", "{ \"a\": 1, \"a\": 2 }\n", r#"{"a":3}"#, ""),
    ];
    for (name, old, cand, want) in cases {
        let got = patched(old, cand);
        if !want.is_empty() {
            assert_eq!(got, want, "{name}");
        }
        assert!(same_json(&got, cand), "{name}: Bedeutung weicht ab: {got:?}");
    }
}

/// Neu: Die Tiefe ist begrenzt, tiefe Eingaben sind ein Fehler statt eines Stapelüberlaufs; Anführungszeichen und
/// Klammern in Strings stören die Lagen nicht.
#[test]
fn patch_tiefe_und_strings() {
    let deep = format!("{}{}", "[".repeat(100_000), "]".repeat(100_000));
    assert!(patch_json("{}", &deep).is_err());
    assert!(patch_json(&deep, "{}").is_err());
    let old = "{ \"a\": \"]}\\\" ,\", \"b\": 1 }\n";
    let cand: Value = json!({"a": "]}\" ,", "b": 2});
    assert_eq!(patched(old, &cand.to_string()), "{ \"a\": \"]}\\\" ,\", \"b\": 2 }\n");
}
