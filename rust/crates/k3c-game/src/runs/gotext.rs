//! Go-Textformen für die Ausgabe der Läufe: `%q` (`strconv.Quote`) und `%g` (`go_float`).

/// Schreibt `s` wie Gos `%q` (`strconv.Quote`): in Anführungszeichen, `"` und `\` mit Rückstrich, Steuerzeichen als
/// `\n`, `\t` … oder `\xNN`; druckbare Zeichen bleiben stehen.
pub fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            c if u32::from(c) < 0x20 || u32::from(c) == 0x7f => out.push_str(&format!("\\x{:02x}", u32::from(c))),
            c if c.is_control() => {
                let n = u32::from(c);
                if n < 0x10000 {
                    out.push_str(&format!("\\u{n:04x}"));
                } else {
                    out.push_str(&format!("\\U{n:08x}"));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `%g` einer Zahl.
pub fn g(x: f64) -> String {
    k3c_det::golden::go_float(x)
}
