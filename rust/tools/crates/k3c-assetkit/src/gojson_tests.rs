//! Eigene Tests ohne Go-Gegenstück: Einrückung, Escapes und Zahlen wie Go `encoding/json`.

use super::*;
use serde_json::json;

fn text<T: Serialize>(v: &T, escape_html: bool) -> String {
    String::from_utf8(marshal_indent(v, escape_html).unwrap()).unwrap()
}

#[derive(Serialize)]
struct Raw {
    r: Box<RawValue>,
}

#[derive(Serialize)]
struct Num {
    #[serde(serialize_with = "float")]
    v: f64,
}

#[test]
fn leere_behaelter_bleiben_in_einer_zeile() {
    assert_eq!(
        text(&json!({"a": [], "b": {}}), false),
        "{\n  \"a\": [],\n  \"b\": {}\n}"
    );
    assert_eq!(text(&json!([]), false), "[]");
}

#[test]
fn verschachtelte_objekte() {
    let want = "{\n  \"a\": {\n    \"b\": [\n      1,\n      {\n        \"c\": null\n      }\n    ]\n  }\n}";
    assert_eq!(text(&json!({"a": {"b": [1, {"c": null}]}}), false), want);
}

#[test]
fn zeichen_in_strings_bleiben() {
    let got = text(&json!(["{ [ , : \" \\ ", "}"]), false);
    assert_eq!(got, "[\n  \"{ [ , : \\\" \\\\ \",\n  \"}\"\n]");
}

#[test]
fn raw_value_wird_neu_eingerueckt() {
    let r = RawValue::from_string("{\n\t\t\"b\" :  1.50 ,\"a\": \"x  y\\u0041\", \"e\": { } }".into()).unwrap();
    let want = "{\n  \"r\": {\n    \"b\": 1.50,\n    \"a\": \"x  y\\u0041\",\n    \"e\": {}\n  }\n}";
    assert_eq!(text(&Raw { r }, false), want);
}

#[test]
fn escape_html_wie_go() {
    let v = json!("<a & b>\u{2028}\u{2029}ü");
    assert_eq!(text(&v, true), "\"\\u003ca \\u0026 b\\u003e\\u2028\\u2029ü\"");
    assert_eq!(text(&v, false), "\"<a & b>\\u2028\\u2029ü\"");
}

#[test]
fn float_wie_go() {
    for (v, want) in [
        (6.0, "6"),
        (4.5, "4.5"),
        (-0.0, "-0"),
        (0.00001, "0.00001"),
        (1e-7, "1e-7"),
        (1e20, "100000000000000000000"),
        (1e21, "1e+21"),
    ] {
        assert_eq!(text(&Num { v }, false), format!("{{\n  \"v\": {want}\n}}"), "{v}");
    }
    assert!(marshal_indent(&Num { v: f64::NAN }, false).is_err());
}
