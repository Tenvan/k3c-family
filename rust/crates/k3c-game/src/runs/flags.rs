//! Kleiner Schalter-Parser wie Gos `flag`-Paket für `sim balance` (RU40.1, kein `clap`): `-name` und `--name`,
//! `--name=wert` und `--name wert`, Schalter ohne Wert, wiederholbare Werte. Das erste Wort ohne Strich und `--` beenden die
//! Schalter; der Rest sind Argumente. Die Fehltexte folgen Gos Paket sinngemäß (Exit 2 und Zeile `error` genügen).

use std::collections::HashMap;

use super::RunError;

/// Art eines Schalters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Ohne Wert (`--name`, `--name=false`).
    Bool,
    /// Ganze Zahl.
    Int,
    /// Text; der letzte gewinnt.
    Text,
    /// Text, den jedes Vorkommen ergänzt (Go: `flag.Func`, `flag.Var`).
    List,
}

/// Ein deklarierter Schalter mit Standardwert und Hilfetext.
#[derive(Debug, Clone, Copy)]
pub struct Spec {
    pub name: &'static str,
    pub kind: Kind,
    /// Standardwert als Text (leer = keiner).
    pub default: &'static str,
    pub usage: &'static str,
}

/// Kurzform für die Tabelle der Schalter.
pub const fn spec(name: &'static str, kind: Kind, default: &'static str, usage: &'static str) -> Spec {
    Spec {
        name,
        kind,
        default,
        usage,
    }
}

/// Ergebnis des Parsens: die gesetzten Werte, die übrigen Argumente und ob Hilfe verlangt war.
#[derive(Debug, Default)]
pub struct Parsed {
    values: HashMap<&'static str, Vec<String>>,
    defaults: HashMap<&'static str, &'static str>,
    /// Argumente nach den Schaltern (Go: `fs.Args()`).
    pub rest: Vec<String>,
    /// `-h`, `-help` oder `--help` stand in den Argumenten.
    pub help: bool,
}

impl Parsed {
    /// Steht der Schalter in den Argumenten (Go: `fs.Visit`)?
    pub fn is_set(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Schalter oder Standard.
    pub fn flag(&self, name: &str) -> bool {
        self.last(name).is_some_and(|v| v == "true")
    }

    /// Text-Wert oder Standard.
    pub fn text(&self, name: &str) -> String {
        self.last(name).unwrap_or_default()
    }

    /// Ganze Zahl; ein Fehler kann nicht vorkommen, `parse` hat den Wert geprüft.
    pub fn int(&self, name: &str) -> i64 {
        self.last(name).and_then(|v| v.parse().ok()).unwrap_or(0)
    }

    /// Alle Vorkommen in der Reihenfolge der Argumente.
    pub fn list(&self, name: &str) -> Vec<String> {
        self.values.get(name).cloned().unwrap_or_default()
    }

    fn last(&self, name: &str) -> Option<String> {
        match self.values.get(name).and_then(|v| v.last()) {
            Some(v) => Some(v.clone()),
            None => self.defaults.get(name).map(|d| (*d).to_string()),
        }
    }
}

fn usage(msg: impl Into<String>) -> RunError {
    RunError::Usage(msg.into())
}

/// Liest `args` gegen die deklarierten Schalter.
pub fn parse(specs: &[Spec], args: &[String]) -> Result<Parsed, RunError> {
    let mut p = Parsed {
        defaults: specs.iter().map(|s| (s.name, s.default)).collect(),
        ..Parsed::default()
    };
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        i += 1;
        if a == "--" {
            p.rest.extend(args[i..].iter().cloned());
            break;
        }
        let Some(body) = a
            .strip_prefix("--")
            .or_else(|| a.strip_prefix('-'))
            .filter(|b| !b.is_empty())
        else {
            p.rest.push(a.clone());
            p.rest.extend(args[i..].iter().cloned());
            break;
        };
        let (name, inline) = match body.split_once('=') {
            Some((n, v)) => (n, Some(v.to_string())),
            None => (body, None),
        };
        if matches!(name, "h" | "help") && !specs.iter().any(|s| s.name == name) {
            p.help = true;
            return Ok(p);
        }
        let Some(s) = specs.iter().find(|s| s.name == name) else {
            return Err(usage(format!("flag provided but not defined: -{name}")));
        };
        let value = match (s.kind, inline) {
            (Kind::Bool, None) => "true".to_string(),
            (_, Some(v)) => v,
            (_, None) if i < args.len() => {
                i += 1;
                args[i - 1].clone()
            }
            (_, None) => return Err(usage(format!("flag needs an argument: -{name}"))),
        };
        check(s, &value)?;
        let value = if s.kind == Kind::Bool { bool_text(&value) } else { value };
        p.values.entry(s.name).or_default().push(value);
    }
    Ok(p)
}

/// Prüft einen Wert gegen die Art (Go: `invalid value … for flag`).
fn check(s: &Spec, value: &str) -> Result<(), RunError> {
    let ok = match s.kind {
        Kind::Bool => parse_bool(value).is_some(),
        Kind::Int => value.parse::<i64>().is_ok(),
        Kind::Text | Kind::List => true,
    };
    if ok {
        return Ok(());
    }
    let what = if s.kind == Kind::Bool { "boolean" } else { "int" };
    Err(usage(format!("invalid {what} value \"{value}\" for flag -{}", s.name)))
}

fn parse_bool(v: &str) -> Option<bool> {
    match v {
        "1" | "t" | "T" | "true" | "TRUE" | "True" => Some(true),
        "0" | "f" | "F" | "false" | "FALSE" | "False" => Some(false),
        _ => None,
    }
}

fn bool_text(v: &str) -> String {
    parse_bool(v).unwrap_or(false).to_string()
}

/// Die Hilfe wie `fs.PrintDefaults()`: je Schalter eine Zeile mit Text und Standard.
pub fn defaults_text(specs: &[Spec]) -> String {
    let mut out = String::new();
    for s in specs {
        let arg = match s.kind {
            Kind::Bool => "",
            Kind::Int => " int",
            Kind::Text | Kind::List => " string",
        };
        out.push_str(&format!("  -{}{arg}\n    \t{}", s.name, s.usage));
        if !s.default.is_empty() && s.kind != Kind::Bool {
            out.push_str(&format!(" (Standard {})", s.default));
        }
        out.push('\n');
    }
    out
}
