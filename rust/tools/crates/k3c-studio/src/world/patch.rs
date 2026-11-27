//! Format-stabiles Schreiben der Welt-Datei (Port von `patch.go`): Nur der geänderte Wert bekommt neue Bytes, alle
//! übrigen Felder der Datei bleiben byte-gleich.

use std::collections::BTreeMap;

use serde_json::value::RawValue;

use super::values::parse_entries;
use crate::kitx;

/// Byte-Bereich `[from, to)` im Inhalt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Span {
    pub from: usize,
    pub to: usize,
}

impl Span {
    pub(super) fn whole(raw: &[u8]) -> Span {
        Span { from: 0, to: raw.len() }
    }
}

/// Der Wert zu `key` im JSON-Objekt `raw[outer]`; `None`, wenn der Schlüssel fehlt.
pub(super) fn value_span(raw: &[u8], outer: Span, key: &str) -> Result<Option<Span>, String> {
    let part = &raw[outer.from..outer.to];
    if part.trim_ascii_start().first() != Some(&b'{') {
        return Err("JSON-Objekt erwartet".to_owned());
    }
    let entries = parse_entries(part).map_err(|e| e.to_string())?;
    Ok(entries.into_iter().find(|(k, _)| k == key).map(|(_, v)| {
        // Der Wert ist ein Ausschnitt von `raw`; seine Adresse gibt die Stelle.
        let from = v.get().as_ptr() as usize - raw.as_ptr() as usize;
        Span {
            from,
            to: from + v.get().len(),
        }
    }))
}

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\r' | b'\t')
}

/// Die Stelle vor der schließenden Klammer des Objekts, an der ein fehlendes Feld angehängt wird; Leerraum nach der
/// Klammer (Zeilenende am Dateiende) wird übersprungen.
pub(super) fn insert_span(raw: &[u8], obj: Span) -> Span {
    let skip = |mut end: usize| {
        while end > obj.from && is_space(raw[end - 1]) {
            end -= 1;
        }
        end
    };
    let end = skip(skip(obj.to).saturating_sub(1)); // ohne die schließende Klammer
    Span { from: end, to: end }
}

fn splice(raw: &[u8], at: Span, text: &[u8]) -> Vec<u8> {
    [&raw[..at.from], text, &raw[at.to..]].concat()
}

/// Setzt den Wert von `key` im Objekt `obj`; fehlt der Schlüssel, wird er vor der schließenden Klammer angehängt.
pub(super) fn set_key(raw: &[u8], obj: Span, key: &str, text: &[u8]) -> Result<Vec<u8>, String> {
    if let Some(at) = value_span(raw, obj, key)? {
        return Ok(splice(raw, at, text));
    }
    let mut added = format!(", {}: ", kitx::go_quote(key)).into_bytes();
    added.extend_from_slice(text);
    Ok(splice(raw, insert_span(raw, obj), &added))
}

/// Entfernt den Schlüssel `key` mit seinem Wert (Bereich `at`) und dem Komma davor bzw. danach.
pub(super) fn remove_key(raw: &[u8], at: Span, key: &str) -> Vec<u8> {
    let quoted = kitx::go_quote(key).into_bytes();
    let Some(mut from) = raw[..at.from].windows(quoted.len()).rposition(|w| w == quoted) else {
        return raw.to_vec();
    };
    let mut to = at.to;
    let before = raw[..from].iter().rposition(|b| !is_space(*b));
    match before {
        Some(i) if raw[i] == b',' => from = i,
        _ => {
            while to < raw.len() && (is_space(raw[to]) || raw[to] == b',') {
                to += 1;
            }
        }
    }
    splice(raw, Span { from, to }, &[])
}

/// JSON-Wert, wie Go ihn nach `any` liest (Zahlen als `float64`, doppelte Schlüssel: der letzte).
#[derive(PartialEq)]
enum Any {
    Null,
    Bool(bool),
    Number(f64),
    Text(String),
    List(Vec<Any>),
    Object(BTreeMap<String, Any>),
}

fn any_of(v: &RawValue, depth: usize) -> Option<Any> {
    let text = v.get();
    if depth > 512 {
        return None;
    }
    Some(match text.bytes().next()? {
        b'{' => {
            let mut map = BTreeMap::new();
            for (k, v) in parse_entries(text.as_bytes()).ok()? {
                map.insert(k, any_of(v, depth + 1)?);
            }
            Any::Object(map)
        }
        b'[' => {
            let items = serde_json::from_str::<Vec<&RawValue>>(text).ok()?;
            Any::List(items.into_iter().map(|v| any_of(v, depth + 1)).collect::<Option<_>>()?)
        }
        b'"' => Any::Text(serde_json::from_str(text).ok()?),
        b't' => Any::Bool(true),
        b'f' => Any::Bool(false),
        b'n' => Any::Null,
        _ => Any::Number(text.parse::<f64>().ok().filter(|n| n.is_finite())?),
    })
}

/// `a` und `b` bedeuten denselben Wert (Zahlenformat, Leerraum und Schlüsselreihenfolge egal).
pub(super) fn same_json(a: &[u8], b: &[u8]) -> bool {
    let read = |raw: &[u8]| any_of(serde_json::from_slice::<&RawValue>(raw).ok()?, 0);
    matches!((read(a), read(b)), (Some(x), Some(y)) if x == y)
}

/// Setzt `overrides` der Welt-Datei auf `ov` (Objekt je Basis-Datei): inhaltlich gleich bleibt die Datei byte-gleich,
/// leer entfällt der Schlüssel. Ein neuer Schlüssel steht am Ende des Objekts, hinter den Inseln; die Zeilenenden
/// folgen der Datei (CRLF bleibt CRLF).
pub(super) fn patch_overrides(raw: &[u8], ov: &BTreeMap<String, Box<RawValue>>) -> Result<Vec<u8>, String> {
    let whole = Span::whole(raw);
    let at = value_span(raw, whole, "overrides")?;
    let text = kitx::marshal_map(ov);
    match at {
        Some(at) if same_json(&raw[at.from..at.to], text.as_bytes()) => return Ok(raw.to_vec()),
        None if ov.is_empty() => return Ok(raw.to_vec()),
        Some(at) if ov.is_empty() => return Ok(remove_key(raw, at, "overrides")),
        _ => {}
    }
    let mut out = kitx::indent(&text, "  ", "  ");
    if raw.windows(2).any(|w| w == b"\r\n") {
        out = out.replace('\n', "\r\n"); // Zeilenenden der Datei beibehalten
    }
    set_key(raw, whole, "overrides", out.as_bytes())
}

#[cfg(test)]
#[path = "patch_tests.rs"]
mod tests;
