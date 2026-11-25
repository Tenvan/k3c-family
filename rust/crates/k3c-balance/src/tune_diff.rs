//! Vorschlag des Abstimmens als Diff (Port von `proposalDiff`, `locateNumber` und Verwandten aus `tuneparam.go`, WZ28.6): Die
//! Zahl wird im Text der Datei aus data/ ersetzt, damit nur die Zeile mit dem Wert im Diff steht. data/ bleibt unberührt.

use std::fmt::Write;

use k3c_data::Source;

use crate::fehler::Fehler;
use crate::tuneparam::TuneParam;

/// Liest den JSON-Text und sucht den Wert unter dem Pfad (Objektschlüssel, Listenindex).
struct Scanner<'a> {
    b: &'a [u8],
    pos: usize,
    keys: &'a [&'a str],
    path: Vec<String>,
}

impl Scanner<'_> {
    fn skip_ws(&mut self) {
        while self.b.get(self.pos).is_some_and(u8::is_ascii_whitespace) {
            self.pos += 1;
        }
    }

    fn eat(&mut self, c: u8) -> bool {
        self.skip_ws();
        let hit = self.b.get(self.pos) == Some(&c);
        self.pos += usize::from(hit);
        hit
    }

    /// Eine JSON-Zeichenkette ab dem öffnenden Anführungszeichen; der Inhalt ohne Auflösung der Escapes.
    fn string(&mut self) -> String {
        let start = self.pos + 1;
        self.pos = start;
        while self.pos < self.b.len() && self.b[self.pos] != b'"' {
            self.pos += if self.b[self.pos] == b'\\' { 2 } else { 1 };
        }
        let s = String::from_utf8_lossy(&self.b[start..self.pos.min(self.b.len())]).into_owned();
        self.pos += 1;
        s
    }

    /// Ein Wert; `Some(Spanne)`, wenn der Pfad hier endet und der Wert eine Zahl ist.
    fn value(&mut self) -> Result<Option<(usize, usize)>, String> {
        self.skip_ws();
        let start = self.pos;
        let Some(&c) = self.b.get(start) else {
            return Ok(None);
        };
        if self.path.iter().map(String::as_str).eq(self.keys.iter().copied()) {
            if c == b'-' || c.is_ascii_digit() {
                while self.b.get(self.pos).is_some_and(|b| b"+-0123456789.eE".contains(b)) {
                    self.pos += 1;
                }
                return Ok(Some((start, self.pos)));
            }
            return Err(format!("{:?} ist keine Zahl", self.keys.join(".")));
        }
        match c {
            b'{' | b'[' => self.container(c == b'['),
            b'"' => {
                self.string();
                Ok(None)
            }
            _ => {
                while self.b.get(self.pos).is_some_and(|b| !b",]} \t\r\n".contains(b)) {
                    self.pos += 1;
                }
                Ok(None)
            }
        }
    }

    /// Objekt oder Liste: jedes Kind unter seinem Schlüssel bzw. Index.
    fn container(&mut self, arr: bool) -> Result<Option<(usize, usize)>, String> {
        self.pos += 1;
        let close = if arr { b']' } else { b'}' };
        let mut idx = 0;
        while !self.eat(close) && self.pos < self.b.len() {
            let key = if arr {
                idx += 1;
                (idx - 1).to_string()
            } else {
                self.skip_ws();
                let k = self.string();
                self.eat(b':');
                k
            };
            self.path.push(key);
            let found = self.value()?;
            self.path.pop();
            if found.is_some() {
                return Ok(found);
            }
            self.eat(b',');
        }
        Ok(None)
    }
}

/// Die Byte-Spanne der Zahl unter `keys` im JSON-Text (Listen über den Index; Go: `locateNumber`).
pub(crate) fn locate_number(raw: &str, keys: &[&str]) -> Result<(usize, usize), String> {
    let mut sc = Scanner {
        b: raw.as_bytes(),
        pos: 0,
        keys,
        path: Vec::new(),
    };
    sc.value()?.ok_or_else(|| format!("{:?} fehlt", keys.join(".")))
}

/// Ersetzt die Zahl des Parameters durch den formatierten Wert (Go: `replaceNumber`).
fn replace_number(raw: &str, p: &TuneParam, v: f64) -> Result<String, String> {
    let keys: Vec<&str> = p.path.split('.').collect();
    let (start, end) = locate_number(raw, &keys)?;
    Ok(format!("{}{}{}", &raw[..start], p.format(v), &raw[end..]))
}

/// Der Vorschlag als Unified-Diff-artiger Text: je betroffener Datei die geänderten Zeilen alt → neu, erzeugt aus dem Text
/// von data/ im Speicher. Leer, wenn kein Wert sich ändert (Go: `proposalDiff`).
pub(crate) fn proposal_diff(params: &[TuneParam], values: &[f64]) -> Result<String, Fehler> {
    let mut files: Vec<&str> = Vec::new();
    for p in params {
        if !files.contains(&p.file.as_str()) {
            files.push(&p.file);
        }
    }
    let source = Source::from_env();
    let mut out = String::new();
    for file in files {
        let raw = source.read(file).map_err(|e| Fehler::Daten(e.to_string()))?;
        let mut next = raw.clone();
        let mut lines: Vec<usize> = Vec::new();
        for (p, &v) in params.iter().zip(values) {
            if p.file != file || p.format(v) == p.format(p.base) {
                continue;
            }
            next = replace_number(&next, p, v).map_err(Fehler::Daten)?;
            let keys: Vec<&str> = p.path.split('.').collect();
            let (start, _) = locate_number(&next, &keys).map_err(Fehler::Daten)?;
            let l = next.as_bytes()[..start].iter().filter(|&&b| b == b'\n').count();
            if !lines.contains(&l) {
                lines.push(l);
            }
        }
        write_hunks(&mut out, file, &raw, &next, &mut lines);
    }
    Ok(out)
}

/// Schreibt die geänderten Zeilen (0-basiert) einer Datei; die Zeilenzahl bleibt gleich (Go: `writeHunks`).
fn write_hunks(out: &mut String, file: &str, old: &str, next: &str, lines: &mut [usize]) {
    if lines.is_empty() {
        return;
    }
    lines.sort_unstable();
    let before: Vec<&str> = old.split('\n').collect();
    let after: Vec<&str> = next.split('\n').collect();
    let _ = write!(out, "--- data/{file}\n+++ data/{file}\n");
    for &l in lines.iter() {
        let _ = write!(
            out,
            "@@ -{n} +{n} @@\n-{}\n+{}\n",
            before[l].trim_end_matches('\r'),
            after[l].trim_end_matches('\r'),
            n = l + 1
        );
    }
}
