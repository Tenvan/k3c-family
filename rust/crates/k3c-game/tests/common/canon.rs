//! Kanonische Form und Hash eines JSON-Baums (RU32.1). Gegenstück zu `canon` in
//! `native/k3c-game/e2e/rustref_test.go`; beide müssen für denselben Baum denselben Hash liefern.
//!
//! - Objekte: Schlüssel aufsteigend nach Bytes, `{"k":v,...}`; Listen `[v,...]`; `null` und leere Liste beide `[]`.
//! - Zahlen: 16 Hex-Stellen der `f64`-Bits (minus null zählt als null); Wahrheitswerte `true`/`false`.
//! - Texte: JSON-Text ohne HTML-Escaping: nur `"`, `\` und Steuerzeichen unter U+0020 werden ersetzt (`\n`, `\r`,
//!   `\t`, sonst `\u00xx`), alles andere bleibt UTF-8.
//! - Stufen nach Index aufsteigend, je Stufe `<index>:<Form>\n`; ohne Stufe `-`.
//! - Hash: FNV-1a 64 über die UTF-8-Bytes, 16 Hex-Stellen.

use serde_json::Value;
use std::fmt::Write;

/// Hängt die kanonische Form von `v` an `out`.
pub fn canon(out: &mut String, v: &Value) {
    match v {
        Value::Null => out.push_str("[]"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            let x = n.as_f64().unwrap_or(f64::NAN);
            let x = if x == 0.0 { 0.0 } else { x }; // minus null
            let _ = write!(out, "{:016x}", x.to_bits());
        }
        Value::String(s) => canon_string(out, s),
        Value::Array(a) => {
            out.push('[');
            for (i, e) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                canon(out, e);
            }
            out.push(']');
        }
        Value::Object(o) => {
            // Die Sortierung steht hier, damit sie nicht an der Feature-Wahl von serde_json hängt.
            let mut keys: Vec<&String> = o.keys().collect();
            keys.sort_unstable_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
            out.push('{');
            for (i, k) in keys.into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                canon_string(out, k);
                out.push(':');
                canon(out, &o[k]);
            }
            out.push('}');
        }
    }
}

fn canon_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// FNV-1a 64 als 16 Hex-Stellen.
pub fn fnv_hex(s: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// Kanonischer Hash eines einzelnen Baums.
pub fn tree_hash(v: &Value) -> String {
    let mut s = String::new();
    canon(&mut s, v);
    fnv_hex(&s)
}

/// Kanonischer Welt-Hash: `(Stufen-Index, Baum der Welt)`, `-` ohne Stufe.
pub fn worlds_hash(stages: &[(i64, Value)]) -> String {
    if stages.is_empty() {
        return "-".to_string();
    }
    let mut sorted: Vec<&(i64, Value)> = stages.iter().collect();
    sorted.sort_by_key(|(i, _)| *i);
    let mut s = String::new();
    for (i, v) in sorted {
        let _ = write!(s, "{i}:");
        canon(&mut s, v);
        s.push('\n');
    }
    fnv_hex(&s)
}
