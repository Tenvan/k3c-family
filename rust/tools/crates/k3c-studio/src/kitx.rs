//! Lücken in `k3c-kit` und `k3c-assetkit`, im Studio nachgebaut (V10): Go-gleiche JSON-Bytes. `k3c-assetkit::gojson`
//! ist dort privat und kennt weder `json.Indent` mit Präfix noch `json.Compact`. Dazu SHA-256 (kein öffentlicher Hash
//! in k3c-assetkit) und Go `%q`.

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

/// Go `%q` eines Texts (`strconv.Quote`): druckbare Zeichen bleiben, der Rest wird maskiert.
pub fn go_quoted(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' | '\\' => out.extend(['\\', c]),
            '\x07' => out.push_str(r"\a"),
            '\x08' => out.push_str(r"\b"),
            '\x0c' => out.push_str(r"\f"),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '\t' => out.push_str(r"\t"),
            '\x0b' => out.push_str(r"\v"),
            c if c.is_ascii_control() => out.push_str(&format!("\\x{:02x}", u32::from(c))),
            c if c != ' ' && (c.is_control() || c.is_whitespace() || c == '\u{feff}') => {
                out.push_str(&format!("\\u{:04x}", u32::from(c)));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Rundenkonstanten von SHA-256 (FIPS 180-4).
const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98,
    0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8,
    0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819,
    0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
    0xc67178f2,
];

/// Verarbeitet einen Block von 64 Bytes.
fn sha256_block(state: &mut [u32; 8], block: &[u8]) {
    let mut w = [0u32; 64];
    for (word, bytes) in w.iter_mut().zip(block.as_chunks::<4>().0) {
        *word = bytes.iter().fold(0, |acc, b| (acc << 8) | u32::from(*b));
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for (k, word) in SHA256_K.iter().zip(w) {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let choose = (e & f) ^ (!e & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(choose)
            .wrapping_add(*k)
            .wrapping_add(word);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let t2 = s0.wrapping_add((a & b) ^ (a & c) ^ (b & c));
        (h, g, f, e) = (g, f, e, d.wrapping_add(t1));
        (d, c, b, a) = (c, b, a, t1.wrapping_add(t2));
    }
    for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *slot = slot.wrapping_add(value);
    }
}

/// SHA-256 als Hex-Text (Go `sha256.Sum256` samt `hex.EncodeToString`).
pub fn sha256_hex(data: &[u8]) -> String {
    let mut state = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    // Auffüllen: 0x80, Nullen, dann die Länge in Bits.
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&(data.len() as u64).wrapping_mul(8).to_be_bytes());
    for block in padded.as_chunks::<64>().0 {
        sha256_block(&mut state, block);
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{go_marshal, go_quoted, sha256_hex};

    /// Prüfwerte aus FIPS 180-4 (leer, ein Block, zwei Blöcke).
    #[test]
    fn sha256_pruefwerte() {
        for (text, want) in [
            ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (
                "abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ] {
            assert_eq!(sha256_hex(text.as_bytes()), want, "{text:?}");
        }
    }

    #[test]
    fn go_quoted_wie_go() {
        let text = "| ü − \"x\" \\ \r\n\u{a0}\u{1}";
        assert_eq!(go_quoted(text), r#""| ü − \"x\" \\ \r\n\u00a0\x01""#);
    }

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
