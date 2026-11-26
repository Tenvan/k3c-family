//! JSON byte-gleich zu Go `encoding/json` (V5): kompaktieren, einrücken wie `indent.go`, Escapes und `float64`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::value::RawValue;

/// Go `json.MarshalIndent(v, "", "  ")` (`escape_html`) bzw. der Encoder mit `SetEscapeHTML(false)` ohne das
/// abschließende `\n`. U+2028 und U+2029 maskiert Go in beiden Fällen.
pub(crate) fn marshal_indent<T: Serialize + ?Sized>(v: &T, escape_html: bool) -> serde_json::Result<Vec<u8>> {
    let raw = serde_json::to_string(v)?;
    Ok(escape(&indent(&compact(&raw)), escape_html).into_bytes())
}

/// Entfernt Leerraum außerhalb von Strings (ein `RawValue` bringt seine eigene Einrückung mit).
fn compact(src: &str) -> String {
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

fn newline(out: &mut String, depth: usize) {
    out.push('\n');
    for _ in 0..depth {
        out.push_str("  ");
    }
}

/// Go `appendIndent` für kompaktes JSON: leere `{}` und `[]` bleiben in einer Zeile.
fn indent(src: &str) -> String {
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
            newline(&mut out, depth);
        }
        match c {
            '{' | '[' => {
                need_indent = true;
                out.push(c);
            }
            ',' => {
                out.push(c);
                newline(&mut out, depth);
            }
            ':' => out.push_str(": "),
            '}' | ']' => {
                if need_indent {
                    need_indent = false;
                } else {
                    depth = depth.saturating_sub(1);
                    newline(&mut out, depth);
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

/// Maskiert die Zeichen, die Go als `\uXXXX` schreibt; sie kommen in JSON nur innerhalb von Strings vor.
fn escape(src: &str, escape_html: bool) -> String {
    let mut out = String::with_capacity(src.len());
    for c in src.chars() {
        match c {
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            '<' if escape_html => out.push_str("\\u003c"),
            '>' if escape_html => out.push_str("\\u003e"),
            '&' if escape_html => out.push_str("\\u0026"),
            _ => out.push(c),
        }
    }
    out
}

/// `float64` wie Go: ganze Zahlen ohne `.0`, Exponentschreibweise erst unter 1e-6 und ab 1e21.
pub(crate) fn float<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if !v.is_finite() {
        return Err(serde::ser::Error::custom(format!("json: unsupported value: {v}")));
    }
    let abs = v.abs();
    let text = if abs != 0.0 && !(1e-6..1e21).contains(&abs) {
        let e = format!("{v:e}");
        if e.contains("e-") { e } else { e.replace('e', "e+") }
    } else {
        format!("{v}")
    };
    RawValue::from_string(text)
        .map_err(serde::ser::Error::custom)?
        .serialize(s)
}

/// Go `json.Unmarshal` lässt ein Feld bei `null` auf seinem Nullwert.
pub(crate) fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

#[cfg(test)]
#[path = "gojson_tests.rs"]
mod tests;
