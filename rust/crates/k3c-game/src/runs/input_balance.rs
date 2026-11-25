//! Eingabe von `sim balance` (Port von `balanceArgs`, `cutInput` und `jsonFlag` aus `engine/simcli/input.go`, RU40.1): Das
//! JSON hinter `--input` ist ein Objekt mit Flag-Namen als Schlüssel; es wird zu Schaltern vor den übrigen Argumenten.
//! Liegt in einer eigenen Datei, damit `input.rs` unter 400 Zeilen bleibt (wie `input_world.rs`).

use std::io::Read;

use serde_json::Value;

use super::RunError;
use super::input::{read_source, usage};

/// Nimmt `--input DATEI` bzw. `--input=DATEI` aus den Argumenten (der letzte gewinnt) und gibt Pfad und Rest zurück.
pub fn cut_input(args: &[String]) -> Result<(String, Vec<String>), RunError> {
    let mut path = String::new();
    let mut rest = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if a == "--input" || a == "-input" {
            let Some(next) = args.get(i + 1) else {
                return Err(usage("--input braucht eine Datei oder -"));
            };
            path.clone_from(next);
            i += 1;
        } else if let Some(v) = a.strip_prefix("--input=").or_else(|| a.strip_prefix("-input=")) {
            path = v.to_string();
        } else {
            rest.push(a.clone());
        }
        i += 1;
    }
    Ok((path, rest))
}

/// Setzt das JSON hinter `--input` (falls vorhanden) vor die übrigen Schalter: Schlüssel = Flag-Name, `true` = Schalter,
/// Zahl oder Text = Wert, Feld = Schalter mehrfach. Die Schlüssel stehen sortiert, damit die Folge fest ist.
pub fn balance_args(args: &[String], stdin: &mut dyn Read) -> Result<Vec<String>, RunError> {
    let (path, rest) = cut_input(args)?;
    if path.is_empty() {
        return Ok(rest);
    }
    let raw = read_source(&path, stdin)?;
    let obj = match serde_json::from_str::<Value>(&raw) {
        Ok(Value::Object(m)) => m,
        Ok(Value::Null) => serde_json::Map::new(),
        Ok(other) => {
            return Err(usage(format!(
                "Eingabe-JSON: json: cannot unmarshal {} into Go value of type map[string]interface {{}}",
                kind_of(&other)
            )));
        }
        Err(e) => return Err(usage(format!("Eingabe-JSON: {e}"))),
    };
    let mut flags = Vec::new();
    // `serde_json::Map` ist ohne `preserve_order` nach Schlüsseln sortiert (wie `slices.Sorted(maps.Keys)`)
    for (k, v) in &obj {
        flags.extend(json_flag(k, v)?);
    }
    flags.extend(rest);
    Ok(flags)
}

/// Ein JSON-Wert als Schalter (`--name=wert`); ein Feld ergibt den Schalter mehrfach.
pub fn json_flag(name: &str, val: &Value) -> Result<Vec<String>, RunError> {
    match val {
        Value::Bool(b) => Ok(vec![format!("--{name}={b}")]),
        Value::String(s) => Ok(vec![format!("--{name}={s}")]),
        Value::Number(n) => Ok(vec![format!("--{name}={}", n.as_f64().unwrap_or(0.0))]),
        Value::Array(list) => {
            let mut out = Vec::new();
            for e in list {
                out.extend(json_flag(name, e)?);
            }
            Ok(out)
        }
        _ => Err(usage(format!(
            "Eingabe-JSON: \"{name}\" hat keinen Flag-Wert (Schalter, Zahl, Text oder Feld davon)"
        ))),
    }
}

fn kind_of(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
