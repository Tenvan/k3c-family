use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::value::RawValue;

use super::{Span, patch_overrides, remove_key, same_json, set_key, value_span};
use crate::world::edit::set_field;
use crate::world::values::parse_entries;

fn raw(text: &str) -> Box<RawValue> {
    RawValue::from_string(text.to_owned()).unwrap()
}

fn overrides(text: &str) -> BTreeMap<String, Box<RawValue>> {
    BTreeMap::from([("economy".to_owned(), raw(text))])
}

// Go `override_test.go` › TestPatchOverridesKeepsCRLF, vorgezogen, weil `patch.rs` in RU70.3 entsteht.
#[test]
fn patch_overrides_behaelt_crlf() {
    let before = b"{\r\n  \"id\": \"x\"\r\n}\r\n";
    let out = patch_overrides(before, &overrides(r#"{"a":1}"#)).unwrap();
    let text = String::from_utf8(out.clone()).unwrap();
    assert!(!text.replace("\r\n", "").contains('\n'), "LF in CRLF-Datei: {text:?}");
    assert_eq!(patch_overrides(&out, &BTreeMap::new()).unwrap(), before);
}

// Bytes wie Go `json.Marshal` + `json.Indent(…, "  ", "  ")`: Schlüssel sortiert, HTML-Escapes, Zahlen unverändert.
#[test]
fn patch_overrides_schreibt_wie_go() {
    let mut ov = overrides(r#"{ "b": 1.50, "a": "<&>", "c": {}, "d": [1, 2] }"#);
    ov.insert("buildings".to_owned(), raw("{}"));
    let out = patch_overrides(b"{\n  \"id\": \"x\"\n}\n", &ov).unwrap();
    let want = concat!(
        "{\n  \"id\": \"x\", \"overrides\": {\n",
        "    \"buildings\": {},\n",
        "    \"economy\": {\n",
        "      \"b\": 1.50,\n",
        "      \"a\": \"\\u003c\\u0026\\u003e\",\n",
        "      \"c\": {},\n",
        "      \"d\": [\n        1,\n        2\n      ]\n",
        "    }\n  }\n}\n",
    );
    assert_eq!(String::from_utf8(out.clone()).unwrap(), want);
    assert_eq!(patch_overrides(&out, &ov).unwrap(), out, "gleicher Inhalt");
}

#[test]
fn setzen_und_entfernen_treffen_nur_den_wert() {
    let before = b"{\n  \"a\": 1,\n  \"b\": 2\n}\n";
    let whole = Span::whole(before);
    assert_eq!(
        set_key(before, whole, "a", b"3").unwrap(),
        b"{\n  \"a\": 3,\n  \"b\": 2\n}\n"
    );
    let at = value_span(before, whole, "a").unwrap().unwrap();
    assert_eq!(remove_key(before, at, "a"), b"{\n  \"b\": 2\n}\n");
    let at = value_span(before, whole, "b").unwrap().unwrap();
    assert_eq!(remove_key(before, at, "b"), b"{\n  \"a\": 1\n}\n");
    assert!(value_span(b"[]", Span::whole(b"[]"), "a").is_err());
    assert!(same_json(br#"{"a":1.0,"b":[2]}"#, br#"{ "b": [2], "a": 1 }"#));
    assert!(!same_json(b"1", b"2"));
    assert!(!same_json(b"{", b"{"));
}

/// Jede Welt-Datei des Repos (nur gelesen): denselben Wert erneut setzen ändert kein Byte.
#[test]
fn welten_des_repos_bleiben_byte_gleich() {
    let dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../data/worlds"));
    let mut seen = 0;
    for entry in std::fs::read_dir(dir).unwrap() {
        let before = std::fs::read(entry.unwrap().path()).unwrap();
        let fields = parse_entries(&before).unwrap();
        for (key, value) in &fields {
            let out = set_field(&before, Span::whole(&before), key, value.get(), true).unwrap();
            assert_eq!(out, before, "{key}");
        }
        let current = fields
            .iter()
            .find(|(k, _)| k == "overrides")
            .map_or_else(BTreeMap::new, |(_, v)| {
                let items = parse_entries(v.get().as_bytes()).unwrap();
                items.into_iter().map(|(k, v)| (k, v.to_owned())).collect()
            });
        assert_eq!(patch_overrides(&before, &current).unwrap(), before);
        seen += 1;
    }
    assert!(seen > 0);
}
