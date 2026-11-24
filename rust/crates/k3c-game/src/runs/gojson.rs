//! JSON wie Gos `encoding/json` für die Berichte von `world` und `hub` (RU33.2): Zahlen in Gos Form (`10`, nicht `10.0`;
//! Exponent ab 1e21 und unter 1e-6), HTML-Escape in Texten, Roh-JSON (`json.RawMessage`) kompakt. Felder und Karten
//! schreibt serde in der Reihenfolge der Typen; ohne `preserve_order` sind Karten sortiert wie bei Go.

use std::io::Write;

use serde::Serialize;
use serde_json::ser::{Formatter, Serializer};

/// Schreibt eine Zahl wie Gos `json.Marshal` (`float64`): ohne Exponent im Bereich 1e-6 bis 1e21, sonst `1e+21`, `1e-7`.
pub fn num(x: f64) -> String {
    let abs = x.abs();
    if abs == 0.0 || (1e-6..1e21).contains(&abs) {
        return format!("{x}");
    }
    let e = format!("{x:e}"); // 1.5e-7, 1e21
    let Some((mant, exp)) = e.split_once('e') else {
        return e;
    };
    let (sign, digits) = match exp.strip_prefix('-') {
        Some(d) => ('-', d),
        None => ('+', exp),
    };
    // Go: zweistelliger Exponent bei positivem Vorzeichen (e+21), ein "e-07" wird zu "e-7"
    let digits = if sign == '+' && digits.len() < 2 {
        format!("0{digits}")
    } else {
        digits.to_string()
    };
    format!("{mant}e{sign}{digits}")
}

struct GoFormatter;

impl Formatter for GoFormatter {
    fn write_f64<W: ?Sized + Write>(&mut self, w: &mut W, value: f64) -> std::io::Result<()> {
        w.write_all(num(value).as_bytes())
    }

    fn write_string_fragment<W: ?Sized + Write>(&mut self, w: &mut W, fragment: &str) -> std::io::Result<()> {
        let mut rest = fragment;
        while let Some(at) = rest.find(['<', '>', '&', '\u{2028}', '\u{2029}']) {
            w.write_all(&rest.as_bytes()[..at])?;
            let c = rest[at..].chars().next().unwrap_or('<');
            write!(w, "\\u{:04x}", u32::from(c))?;
            rest = &rest[at + c.len_utf8()..];
        }
        w.write_all(rest.as_bytes())
    }
}

/// Schreibt `v` als JSON-Text wie Gos `json.Marshal`.
pub fn to_string<T: Serialize>(v: &T) -> String {
    let mut out = Vec::new();
    let mut ser = Serializer::with_formatter(&mut out, GoFormatter);
    if v.serialize(&mut ser).is_err() {
        return "null".to_string();
    }
    String::from_utf8(out).unwrap_or_default()
}

/// Gos `json.RawMessage` beim Schreiben: gültiges JSON, kompakt (ohne Leerraum außerhalb von Texten), HTML-Escape in Texten.
pub fn compact(raw: &str) -> Result<String, String> {
    serde_json::from_str::<serde::de::IgnoredAny>(raw).map_err(|e| e.to_string())?;
    let mut out = String::with_capacity(raw.len());
    let (mut in_str, mut esc) = (false, false);
    for c in raw.chars() {
        if in_str {
            match c {
                _ if esc => esc = false,
                '\\' => esc = true,
                '"' => in_str = false,
                '<' | '>' | '&' | '\u{2028}' | '\u{2029}' => {
                    out.push_str(&format!("\\u{:04x}", u32::from(c)));
                    continue;
                }
                _ => {}
            }
            out.push(c);
        } else if !c.is_whitespace() {
            in_str = c == '"';
            out.push(c);
        }
    }
    Ok(out)
}

/// Die Einträge des obersten Objekts als Schlüssel und Roh-Text ihrer Werte (für `json.RawMessage`-Felder). Der Text
/// muss gültiges JSON sein (vorher geprüft); ein Schlüssel, der mehrfach steht, gilt wie in Go zuletzt.
pub fn raw_entries(text: &str) -> Vec<(String, &str)> {
    let b = text.as_bytes();
    let mut out: Vec<(String, &str)> = Vec::new();
    let mut i = skip_ws(b, 0);
    if b.get(i) != Some(&b'{') {
        return out;
    }
    i += 1;
    loop {
        i = skip_ws(b, i);
        if b.get(i) != Some(&b'"') {
            return out;
        }
        let key_end = scan_string(b, i);
        let Ok(key) = serde_json::from_str::<String>(&text[i..key_end]) else {
            return out;
        };
        i = skip_ws(b, key_end);
        i = skip_ws(b, i + 1); // ':'
        let end = scan_value(b, i);
        out.retain(|(k, _)| *k != key);
        out.push((key, &text[i..end]));
        i = skip_ws(b, end);
        if b.get(i) == Some(&b',') {
            i += 1;
        } else {
            return out;
        }
    }
}

/// Der Roh-Text des Feldes `key` im obersten Objekt (Go vergleicht Schlüssel ohne Groß- und Kleinschreibung).
pub fn raw_field<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    raw_entries(text)
        .into_iter()
        .rev()
        .find(|(k, _)| k.eq_ignore_ascii_case(key))
        .map(|(_, v)| v)
}

fn skip_ws(b: &[u8], mut i: usize) -> usize {
    while b.get(i).is_some_and(u8::is_ascii_whitespace) {
        i += 1;
    }
    i
}

/// Das Ende des Textes, der bei `i` (ein `"`) beginnt.
fn scan_string(b: &[u8], i: usize) -> usize {
    let mut j = i + 1;
    while j < b.len() {
        match b[j] {
            b'\\' => j += 2,
            b'"' => return j + 1,
            _ => j += 1,
        }
    }
    b.len()
}

/// Das Ende des Wertes, der bei `i` beginnt.
fn scan_value(b: &[u8], i: usize) -> usize {
    match b.get(i) {
        Some(b'"') => scan_string(b, i),
        Some(b'{' | b'[') => {
            let (mut depth, mut j) = (0usize, i);
            while j < b.len() {
                match b[j] {
                    b'"' => {
                        j = scan_string(b, j);
                        continue;
                    }
                    b'{' | b'[' => depth += 1,
                    b'}' | b']' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            return j + 1;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            b.len()
        }
        _ => {
            let mut j = i;
            while j < b.len() && !matches!(b[j], b',' | b'}' | b']') && !b[j].is_ascii_whitespace() {
                j += 1;
            }
            j
        }
    }
}
