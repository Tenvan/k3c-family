//! Parameter des automatischen Balancings (WZ28.6): `datei.json:a.b.c=min..max` zeigt auf eine Zahl in `data/`. Der
//! Wert wird nur im Speicher gesetzt (Varianten); der Vorschlag ist ein Diff-Text der Datei, `data/` bleibt unberührt.

use std::path::Path;

use serde::de::{Deserializer as _, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::datafiles;
use super::summary::go_fixed;
use crate::kitx::go_quoted;

/// Ein abgestimmter Wert: Datei, Pfad, Bereich und Ausgangswert. `int`: der Wert ist in der Datei ganzzahlig, alle
/// Punkte werden gerundet.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TuneParam {
    pub file: String,
    pub path: String,
    pub min: f64,
    pub max: f64,
    pub base: f64,
    pub int: bool,
}

/// Liest `datei.json:a.b.c=min..max` und den Ausgangswert aus `data/` des Checkouts `root`.
pub fn parse_tune_param(root: &Path, spec: &str) -> Result<TuneParam, String> {
    parse(root, spec).map_err(|e| format!("parameter {}: {e}", go_quoted(spec)))
}

fn parse(root: &Path, spec: &str) -> Result<TuneParam, String> {
    let parts = spec.split_once('=').and_then(|(path, range)| {
        let (file, keys) = path.split_once(':')?;
        let (lo, hi) = range.split_once("..")?;
        Some((file, keys, lo, hi))
    });
    let Some((file, keys, lo, hi)) = parts.filter(|p| !p.1.is_empty()) else {
        return Err("erwartet datei.json:a.b.c=min..max".to_owned());
    };
    let bounds = || "min..max: zwei Zahlen mit min < max".to_owned();
    let min: f64 = lo.parse().map_err(|_| bounds())?;
    let max: f64 = hi.parse().map_err(|_| bounds())?;
    if min >= max || (max - min).is_infinite() {
        return Err(bounds());
    }
    let raw = read_text(root, file)?;
    let (start, end) = locate_number(&raw, &split(keys))?;
    let text = &raw[start..end];
    Ok(TuneParam {
        file: file.to_owned(),
        path: keys.to_owned(),
        min,
        max,
        base: text.parse().map_err(|e| format!("{text}: {e}"))?,
        int: !text.contains(['.', 'e', 'E']),
    })
}

fn read_text(root: &Path, file: &str) -> Result<String, String> {
    String::from_utf8(datafiles::read(root, file)?).map_err(|e| format!("{file}: {e}"))
}

fn split(keys: &str) -> Vec<&str> {
    keys.split('.').collect()
}

impl TuneParam {
    /// Ein Wert des Parameters als Text (ganzzahlig gerundet bzw. auf vier Nachkommastellen).
    fn format(&self, v: f64) -> String {
        if self.int {
            // wie Go `int64(math.Round(v))`
            return format!("{}", v.round() as i64);
        }
        go_fixed((v * 1e4).round() / 1e4, None)
    }
}

/// Die Einträge eines Objekts in Dateireihenfolge, doppelte Schlüssel bleiben.
struct Entries;

impl<'de> Visitor<'de> for Entries {
    type Value = Vec<(String, &'de RawValue)>;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ein Objekt")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut out = Vec::new();
        while let Some(entry) = map.next_entry()? {
            out.push(entry);
        }
        Ok(out)
    }
}

/// Die Kinder eines Knotens mit ihrem Pfad-Teil (Listen über den Index); eine Zahl oder ein Text hat keine.
fn children(node: &RawValue) -> Result<Vec<(String, &RawValue)>, String> {
    let text = node.get();
    let fail = |e: serde_json::Error| e.to_string();
    if text.starts_with('{') {
        return serde_json::Deserializer::from_str(text)
            .deserialize_map(Entries)
            .map_err(fail);
    }
    if text.starts_with('[') {
        let list: Vec<&RawValue> = serde_json::from_str(text).map_err(fail)?;
        return Ok(list.into_iter().enumerate().map(|(i, v)| (i.to_string(), v)).collect());
    }
    Ok(Vec::new())
}

/// Der erste Wert unter `keys` in Dateireihenfolge.
fn find<'a>(node: &'a RawValue, keys: &[&str]) -> Result<Option<&'a RawValue>, String> {
    let Some((key, rest)) = keys.split_first() else {
        return Ok(Some(node));
    };
    for (name, child) in children(node)? {
        if name == *key
            && let Some(found) = find(child, rest)?
        {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

/// Die Byte-Spanne der Zahl unter `keys` im JSON-Text `raw`.
fn locate_number(raw: &str, keys: &[&str]) -> Result<(usize, usize), String> {
    let root: &RawValue = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    let path = go_quoted(&keys.join("."));
    let Some(found) = find(root, keys)? else {
        return Err(format!("{path} fehlt"));
    };
    let text = found.get();
    if !text.starts_with(|c: char| c == '-' || c.is_ascii_digit()) {
        return Err(format!("{path} ist keine Zahl"));
    }
    // `text` ist ein Ausschnitt aus `raw` (RawValue kopiert nicht).
    let start = text.as_ptr() as usize - raw.as_ptr() as usize;
    Ok((start, start + text.len()))
}

/// Der Vorschlag als Unified-Diff-artiger Text: je betroffener Datei die geänderten Zeilen alt → neu, erzeugt aus den
/// Bytes von `data/` im Speicher. Leer, wenn kein Wert sich ändert.
pub fn proposal_diff(root: &Path, params: &[TuneParam], values: &[f64]) -> Result<String, String> {
    let mut files: Vec<&str> = Vec::new();
    for p in params {
        if !files.contains(&p.file.as_str()) {
            files.push(&p.file);
        }
    }
    let mut out = String::new();
    for file in files {
        let raw = read_text(root, file)?;
        let mut next = raw.clone();
        let mut lines = Vec::new();
        for (p, &value) in params.iter().zip(values) {
            if p.file != file || p.format(value) == p.format(p.base) {
                continue;
            }
            let (start, end) = locate_number(&next, &split(&p.path))?;
            next.replace_range(start..end, &p.format(value));
            let line = next[..start].matches('\n').count();
            if !lines.contains(&line) {
                lines.push(line);
            }
        }
        write_hunks(&mut out, file, &raw, &next, lines);
    }
    Ok(out)
}

/// Die geänderten Zeilen (ab 0) einer Datei; die Zeilenzahl bleibt gleich.
fn write_hunks(out: &mut String, file: &str, old: &str, next: &str, mut lines: Vec<usize>) {
    if lines.is_empty() {
        return;
    }
    lines.sort_unstable();
    let before: Vec<&str> = old.split('\n').collect();
    let after: Vec<&str> = next.split('\n').collect();
    out.push_str(&format!("--- data/{file}\n+++ data/{file}\n"));
    for l in lines {
        let (a, b) = (before[l].trim_end_matches('\r'), after[l].trim_end_matches('\r'));
        out.push_str(&format!("@@ -{n} +{n} @@\n-{a}\n+{b}\n", n = l + 1));
    }
}

#[cfg(test)]
#[path = "tuneparam_tests.rs"]
mod tests;
