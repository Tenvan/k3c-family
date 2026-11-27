//! Hash-Regel wie `tools/paper/schema/hash.ts` (paper-spec.md › Hash-Regel); Go- und Rust-Studio teilen den Cache.

use serde_json::Value;

use crate::kitx::{go_float, sha256_hex};

/// Gleich `RENDERER_VERSION` in `tools/paper/schema/hash.ts`.
pub const RENDERER_VERSION: &str = "paper-0+resvg-js-2.6.2";

/// Kanonische JSON-Form aus `hash.ts`: Schlüssel nach UTF-16-Codeeinheiten sortiert, ohne Leerraum, CRLF in Texten
/// als LF, Zahlen und Texte wie JavaScript `JSON.stringify`. Die Tiefe ist durch das Lesen begrenzt (128 Ebenen).
pub fn kanonisch(wert: &Value) -> String {
    let mut out = String::new();
    schreibe(&mut out, wert);
    out
}

fn schreibe(out: &mut String, wert: &Value) {
    match wert {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::String(s) => out.push_str(&js_text(&s.replace("\r\n", "\n"))),
        Value::Number(n) => out.push_str(&n.as_f64().map_or_else(|| "null".to_owned(), js_zahl)),
        Value::Array(list) => {
            out.push('[');
            for (i, e) in list.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                schreibe(out, e);
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<(&String, &Value)> = map.iter().collect();
            keys.sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));
            out.push('{');
            for (i, (k, v)) in keys.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&js_text(k));
                out.push(':');
                schreibe(out, v);
            }
            out.push('}');
        }
    }
}

/// Wie `Number.prototype.toString`: kürzeste Darstellung, Exponent ab 1e21 und unter 1e-6, nie `-0`.
fn js_zahl(f: f64) -> String {
    if f == 0.0 {
        return "0".to_owned();
    }
    go_float(f)
}

/// Text in Anführungszeichen wie `JSON.stringify` (kein HTML-Escaping, U+2028 bleibt roh).
fn js_text(s: &str) -> String {
    // Ein `str` ist immer serialisierbar.
    serde_json::to_string(s).unwrap_or_default()
}

/// JSON-Text wie Go `json.Unmarshal` nach `any`: ungültiges UTF-8 wird zu U+FFFD.
pub(super) fn lies_wert(raw: &[u8]) -> serde_json::Result<Value> {
    serde_json::from_str(&String::from_utf8_lossy(raw))
}

/// SHA-256 (hex) der kanonischen Form; `$schema` zählt nicht. Gilt ebenso für die Stil-Datei.
pub fn spec_hash(raw: &[u8]) -> serde_json::Result<String> {
    let mut wert = lies_wert(raw)?;
    if let Value::Object(map) = &mut wert {
        map.remove("$schema");
    }
    Ok(sha256_hex(kanonisch(&wert).as_bytes()))
}

/// Cache- und Golden-Schlüssel: SHA-256 über `specHash:stilHash:RENDERER_VERSION`.
pub fn render_key(spec_hash: &str, stil_hash: &str) -> String {
    render_key_mit(spec_hash, stil_hash, RENDERER_VERSION)
}

pub(super) fn render_key_mit(spec_hash: &str, stil_hash: &str, renderer: &str) -> String {
    sha256_hex(format!("{spec_hash}:{stil_hash}:{renderer}").as_bytes())
}

#[cfg(test)]
#[path = "hash_tests.rs"]
mod tests;
