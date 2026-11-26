//! JSON-Wert in einer Zeile im Stil der Welt-Dateien (Port von `inline.go`): `{ "hp": 1.5, "damage": 1.3 }`, Listen
//! von Zahlen `[0, 1, 2]`, Listen mit Objekten `[ { "slot": -7 } ]`. Schlüsselreihenfolge und Zahlen bleiben, wie sie
//! sind; ein zweites `inline` über das Ergebnis ändert nichts.

use serde_json::value::RawValue;

use super::values::parse_entries;
use crate::kitx::go_quote;

/// Tiefste Schachtelung (Schutz vor Stapelüberlauf; Welt-Werte haben höchstens eine Handvoll Ebenen).
const MAX_DEPTH: usize = 512;

pub(super) fn inline(raw: &str) -> Result<String, String> {
    // Prüft auch, dass nach dem Wert nichts mehr folgt.
    let value = serde_json::from_str::<&RawValue>(raw).map_err(|e| format!("kein gültiges JSON: {e}"))?;
    inline_value(value, 0)
}

fn inline_value(value: &RawValue, depth: usize) -> Result<String, String> {
    if depth > MAX_DEPTH {
        return Err("kein gültiges JSON: zu tief geschachtelt".to_owned());
    }
    let text = value.get();
    match text.bytes().next() {
        Some(b'{') => inline_object(text, depth),
        Some(b'[') => inline_array(text, depth),
        Some(b'"') => serde_json::from_str::<String>(text)
            .map(|s| go_quote(&s))
            .map_err(|e| e.to_string()),
        _ => Ok(text.to_owned()), // Zahl, true, false, null
    }
}

fn inline_object(text: &str, depth: usize) -> Result<String, String> {
    let mut parts = Vec::new();
    for (key, value) in parse_entries(text.as_bytes()).map_err(|e| e.to_string())? {
        parts.push(format!("{}: {}", go_quote(&key), inline_value(value, depth + 1)?));
    }
    Ok(if parts.is_empty() {
        "{}".to_owned()
    } else {
        format!("{{ {} }}", parts.join(", "))
    })
}

fn inline_array(text: &str, depth: usize) -> Result<String, String> {
    let items = serde_json::from_str::<Vec<&RawValue>>(text).map_err(|e| e.to_string())?;
    let parts = items
        .into_iter()
        .map(|v| inline_value(v, depth + 1))
        .collect::<Result<Vec<_>, _>>()?;
    let nested = parts.iter().any(|p| p.starts_with(['{', '[']));
    Ok(match (parts.is_empty(), nested) {
        (true, _) => "[]".to_owned(),
        (false, true) => format!("[ {} ]", parts.join(", ")),
        (false, false) => format!("[{}]", parts.join(", ")),
    })
}

#[cfg(test)]
#[path = "inline_tests.rs"]
mod tests;
