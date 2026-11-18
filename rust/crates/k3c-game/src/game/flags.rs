//! Ersatz für das Go-Paket `flag` (`flag.FlagSet` mit ContinueOnError, Ausgabe verworfen), ohne neue Abhängigkeit.
//!
//! Erkannt wie in Go: `-x` und `--x`, `--x=wert` und `--x wert`, Bool-Schalter ohne Wert oder mit `=true`/`=false`;
//! das Parsen endet am ersten Argument ohne `-` (es und alles danach bleibt übrig) oder nach `--`. Ein Wert-Schalter
//! nimmt das nächste Argument immer als Wert, auch wenn es mit `-` beginnt (`--measure -1`).

use std::collections::BTreeMap;

/// Art eines Schalters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Bool,
    Str,
    Int,
    Float,
}

/// Ein Schalter mit Namen, Art und Standardwert (als Text).
#[derive(Clone, Copy, Debug)]
pub struct Spec {
    pub name: &'static str,
    pub kind: Kind,
    pub def: &'static str,
}

/// Kurzform für die Schalter-Tabellen.
pub const fn spec(name: &'static str, kind: Kind, def: &'static str) -> Spec {
    Spec { name, kind, def }
}

/// Die Schalter und ihre Werte nach dem Einlesen.
#[derive(Debug)]
pub struct FlagSet {
    kinds: BTreeMap<&'static str, Kind>,
    values: BTreeMap<&'static str, String>,
    rest: Vec<String>,
}

/// `strconv.ParseBool`.
fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "1" | "t" | "T" | "TRUE" | "true" | "True" => Some(true),
        "0" | "f" | "F" | "FALSE" | "false" | "False" => Some(false),
        _ => None,
    }
}

/// `strconv.ParseInt(s, 0, 64)` ohne Unterstriche: Vorzeichen, `0x`, `0b`, `0o`, führende 0 = oktal.
pub fn parse_int(s: &str) -> Option<i64> {
    let (neg, body) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let lower = body.to_ascii_lowercase();
    let (radix, digits) = if let Some(r) = lower.strip_prefix("0x") {
        (16, r.to_string())
    } else if let Some(r) = lower.strip_prefix("0b") {
        (2, r.to_string())
    } else if let Some(r) = lower.strip_prefix("0o") {
        (8, r.to_string())
    } else if lower.len() > 1 && lower.starts_with('0') {
        (8, lower[1..].to_string())
    } else {
        (10, lower)
    };
    if digits.is_empty() || digits.starts_with(['+', '-']) {
        return None;
    }
    let v = i128::from_str_radix(&digits, radix).ok()?;
    i64::try_from(if neg { -v } else { v }).ok()
}

fn valid(kind: Kind, value: &str) -> bool {
    match kind {
        Kind::Bool => parse_bool(value).is_some(),
        Kind::Str => true,
        Kind::Int => parse_int(value).is_some(),
        Kind::Float => value.parse::<f64>().is_ok(),
    }
}

fn kind_word(kind: Kind) -> &'static str {
    if kind == Kind::Bool { "boolean" } else { "value" }
}

impl FlagSet {
    /// Legt die Schalter mit ihren Standardwerten an.
    pub fn new(specs: &[Spec]) -> Self {
        let mut kinds = BTreeMap::new();
        let mut values = BTreeMap::new();
        for s in specs {
            kinds.insert(s.name, s.kind);
            values.insert(s.name, s.def.to_string());
        }
        Self {
            kinds,
            values,
            rest: Vec::new(),
        }
    }

    /// Ändert den Standardwert eines Schalters (vor `parse`).
    pub fn set_default(&mut self, name: &'static str, value: &str) {
        if let Some(v) = self.values.get_mut(name) {
            *v = value.to_string();
        }
    }

    /// Liest `argv`; der Fehlertext nennt den Grund wie Go.
    pub fn parse(&mut self, argv: &[String]) -> Result<(), String> {
        let mut i = 0;
        while i < argv.len() {
            let arg = &argv[i];
            if arg.len() < 2 || !arg.starts_with('-') {
                break;
            }
            i += 1;
            let body = arg.trim_start_matches('-');
            let dashes = arg.len() - body.len();
            if body.is_empty() && dashes == 2 {
                break; // "--" beendet die Schalter
            }
            if dashes > 2 || body.is_empty() || body.starts_with('=') {
                return Err(format!("bad flag syntax: {arg}"));
            }
            let (name, inline) = match body.split_once('=') {
                Some((n, v)) => (n, Some(v.to_string())),
                None => (body, None),
            };
            match self.set(name, inline, argv.get(i)) {
                Ok(used) => i += used,
                Err(e) => {
                    self.rest = argv[i..].to_vec(); // wie Go: `Args()` nennt, was nach dem Fehler übrig ist
                    return Err(e);
                }
            }
        }
        self.rest = argv[i..].to_vec();
        Ok(())
    }

    /// Setzt einen Schalter; liefert, wie viele Folge-Argumente verbraucht wurden (0 oder 1).
    fn set(&mut self, name: &str, inline: Option<String>, next: Option<&String>) -> Result<usize, String> {
        let Some((key, kind)) = self.kinds.get_key_value(name).map(|(k, v)| (*k, *v)) else {
            return Err(format!("flag provided but not defined: -{name}"));
        };
        let (value, used) = match (inline, kind) {
            (Some(v), _) => (v, 0),
            (None, Kind::Bool) => ("true".to_string(), 0),
            (None, _) => match next {
                Some(v) => (v.clone(), 1),
                None => return Err(format!("flag needs an argument: -{name}")),
            },
        };
        if !valid(kind, &value) {
            return Err(format!(
                "invalid {} {value:?} for flag -{name}: parse error",
                kind_word(kind)
            ));
        }
        self.values.insert(key, value);
        Ok(used)
    }

    /// Argumente, die nach den Schaltern übrig bleiben (`fs.Args()`).
    pub fn args(&self) -> &[String] {
        &self.rest
    }

    /// Wert eines Text-Schalters (leer, wenn unbekannt).
    pub fn str(&self, name: &str) -> &str {
        self.values.get(name).map_or("", String::as_str)
    }

    pub fn bool(&self, name: &str) -> bool {
        self.values.get(name).and_then(|v| parse_bool(v)).unwrap_or(false)
    }

    pub fn int(&self, name: &str) -> i64 {
        self.values.get(name).and_then(|v| parse_int(v)).unwrap_or(0)
    }

    pub fn float(&self, name: &str) -> f64 {
        self.values.get(name).and_then(|v| v.parse().ok()).unwrap_or(0.0)
    }
}

#[cfg(test)]
#[path = "flags_tests.rs"]
mod flags_tests;
