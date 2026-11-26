//! Lücken in `k3c-kit` und `k3c-assetkit`, im Studio nachgebaut (V10): Go-gleiche JSON-Bytes. `k3c-assetkit::gojson`
//! ist dort privat und kennt weder `json.Indent` mit Präfix noch `json.Compact`.

use std::collections::BTreeMap;

use serde_json::Value;
use serde_json::value::RawValue;

/// Maskiert U+2028 und U+2029 wie Go (immer, auch ohne HTML-Escapes).
fn escape_separators(s: &str) -> String {
    s.replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029")
}

/// Text als JSON-String wie Go mit `SetEscapeHTML(false)`, ohne Zeilenende.
pub fn go_quote(s: &str) -> String {
    // Ein `str` ist immer serialisierbar.
    escape_separators(&serde_json::to_string(s).unwrap_or_default())
}

/// Entfernt Leerraum außerhalb von Strings.
fn strip_space(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let (mut in_string, mut escaped) = (false, false);
    for c in src.chars() {
        if in_string {
            out.push(c);
            in_string = escaped || c != '"';
            escaped = !escaped && c == '\\';
        } else if !matches!(c, ' ' | '\t' | '\r' | '\n') {
            in_string = c == '"';
            out.push(c);
        }
    }
    out
}

/// Go `json.Compact`; ungültiges JSON bleibt, wie es ist.
pub fn compact(src: &str) -> String {
    match serde_json::from_str::<&RawValue>(src) {
        Ok(_) => strip_space(src),
        Err(_) => src.to_owned(),
    }
}

/// Maskiert in fertigem JSON `<`, `>`, `&`, U+2028 und U+2029 wie Go mit HTML-Escapes.
fn escape_html(json: &str) -> String {
    escape_separators(json)
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

/// Go `json.Marshal` eines `json.RawMessage`: kompakt und maskiert.
fn marshal_raw(src: &str) -> String {
    escape_html(&strip_space(src))
}

/// Go `json.Marshal` einer `map[string]json.RawMessage` (Schlüssel nach Bytes sortiert).
pub fn marshal_map(map: &BTreeMap<String, Box<RawValue>>) -> String {
    let parts: Vec<String> = map
        .iter()
        .map(|(k, v)| format!("{}:{}", marshal_raw(&go_quote(k)), marshal_raw(v.get())))
        .collect();
    format!("{{{}}}", parts.join(","))
}

fn newline(out: &mut String, prefix: &str, step: &str, depth: usize) {
    out.push('\n');
    out.push_str(prefix);
    for _ in 0..depth {
        out.push_str(step);
    }
}

/// Go `json.Indent(src, prefix, step)` für kompaktes JSON: leere `{}` und `[]` bleiben in einer Zeile, die erste Zeile
/// bekommt kein Präfix.
pub fn indent(src: &str, prefix: &str, step: &str) -> String {
    let mut out = String::with_capacity(src.len() * 2);
    let (mut in_string, mut escaped, mut need_indent) = (false, false, false);
    let mut depth = 0usize;
    for c in src.chars() {
        if in_string {
            out.push(c);
            in_string = escaped || c != '"';
            escaped = !escaped && c == '\\';
            continue;
        }
        if need_indent && c != '}' && c != ']' {
            need_indent = false;
            depth += 1;
            newline(&mut out, prefix, step, depth);
        }
        match c {
            '{' | '[' => {
                need_indent = true;
                out.push(c);
            }
            ',' => {
                out.push(c);
                newline(&mut out, prefix, step, depth);
            }
            ':' => out.push_str(": "),
            '}' | ']' => {
                if need_indent {
                    need_indent = false;
                } else {
                    depth = depth.saturating_sub(1);
                    newline(&mut out, prefix, step, depth);
                }
                out.push(c);
            }
            _ => {
                in_string = c == '"';
                out.push(c);
            }
        }
    }
    out
}

/// Go `float64` in `json.Marshal`: kürzeste Form, Exponent nur unter 1e-6 und ab 1e21.
fn go_float(f: f64) -> String {
    let abs = f.abs();
    if abs == 0.0 || (1e-6..1e21).contains(&abs) {
        return format!("{f}");
    }
    let text = format!("{f:e}");
    match text.split_once('e') {
        Some((mantissa, exponent)) if !exponent.starts_with('-') => format!("{mantissa}e+{exponent}"),
        _ => text,
    }
}

/// Go `json.Marshal` eines nach `any` gelesenen Werts (so sehen die MCP-Tools ihre freien Argumente): Schlüssel nach
/// Bytes sortiert, jede Zahl als `float64`, `<`, `>`, `&`, U+2028 und U+2029 maskiert. Die Tiefe ist durch das Lesen
/// begrenzt (`serde_json` bricht nach 128 Ebenen ab).
pub fn go_marshal(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.as_f64().map_or_else(|| n.to_string(), go_float),
        Value::String(s) => escape_html(&go_quote(s)),
        Value::Array(list) => {
            let parts: Vec<String> = list.iter().map(go_marshal).collect();
            format!("[{}]", parts.join(","))
        }
        Value::Object(map) => {
            let sorted: BTreeMap<&String, &Value> = map.iter().collect();
            let parts: Vec<String> = sorted
                .into_iter()
                .map(|(k, v)| format!("{}:{}", escape_html(&go_quote(k)), go_marshal(v)))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::go_marshal;

    /// Erwartet nach den Regeln von Go `json.Marshal` für Werte nach `any` (Zahlform, Sortierung, HTML-Escapes).
    #[test]
    fn go_marshal_wie_go() {
        let value = json!({"b": [100.0, 1.5, -0.0, 1e-7, 1e21, 99], "a": "x <&> y\u{2028}", "c": null, "d": {}});
        assert_eq!(
            go_marshal(&value),
            r#"{"a":"x \u003c\u0026\u003e y\u2028","b":[100,1.5,-0,1e-7,1e+21,99],"c":null,"d":{}}"#
        );
    }
}
