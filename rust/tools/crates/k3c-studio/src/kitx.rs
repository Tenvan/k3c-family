//! Lücken in `k3c-kit` und `k3c-assetkit`, im Studio nachgebaut (V10): Go-gleiche JSON-Bytes. `k3c-assetkit::gojson`
//! ist dort privat und kennt weder `json.Indent` mit Präfix noch `json.Compact`.

use std::collections::BTreeMap;

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

/// Go `json.Marshal` eines `json.RawMessage`: kompakt, `<`, `>`, `&`, U+2028 und U+2029 maskiert.
fn marshal_raw(src: &str) -> String {
    escape_separators(&strip_space(src))
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
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
