//! Schalter und Eingabe-JSON der Läufe (Port von `engine/simcli/input.go`, RU33.1). Unbekannte JSON-Felder sind ein
//! Aufruffehler (Exit 2) mit Gos Text (`json: unknown field "x"`), damit Tippfehler nicht still den Standard nehmen.

use std::io::Read;

use serde_json::{Map, Value};

use super::RunError;
use super::enginetools_run::Segment;

/// Die Schalter eines Aufrufs hinter dem Unterbefehl.
#[derive(Debug, Clone, PartialEq)]
pub struct Flags {
    /// `--input`: JSON-Datei, `-` = stdin.
    pub input: String,
    pub headless: bool,
    pub window: bool,
    /// `--speed N` (nur mit `--window`).
    pub speed: Option<i64>,
    /// `--data ORDNER`.
    pub data: Option<String>,
    pub stop_stdin: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            input: "-".into(),
            headless: false,
            window: false,
            speed: None,
            data: None,
            stop_stdin: false,
        }
    }
}

fn usage(msg: impl Into<String>) -> RunError {
    RunError::Usage(msg.into())
}

/// Liest die Schalter wie Gos `flag`-Paket (`-x`, `--x`, `--x=wert`, `--x wert`); das erste Wort ohne Strich beendet sie.
pub fn parse_flags(args: &[String]) -> Result<Flags, RunError> {
    let mut f = Flags::default();
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        i += 1;
        if a == "--" {
            rest.extend(args[i..].iter().cloned());
            break;
        }
        let Some(body) = a
            .strip_prefix("--")
            .or_else(|| a.strip_prefix('-'))
            .filter(|b| !b.is_empty())
        else {
            rest.push(a.clone());
            rest.extend(args[i..].iter().cloned());
            break;
        };
        let (name, inline) = match body.split_once('=') {
            Some((n, v)) => (n, Some(v.to_string())),
            None => (body, None),
        };
        match name {
            "headless" | "window" | "stop-stdin" => set_switch(&mut f, name, inline.as_deref())?,
            "input" | "speed" | "data" => {
                let value = match inline {
                    Some(v) => v,
                    None if i < args.len() => {
                        i += 1;
                        args[i - 1].clone()
                    }
                    None => return Err(usage(format!("flag needs an argument: -{name}"))),
                };
                set_value(&mut f, name, value)?;
            }
            _ => return Err(usage(format!("flag provided but not defined: -{name}"))),
        }
    }
    if !rest.is_empty() {
        return Err(usage(format!("unerwartete Argumente: [{}]", rest.join(" "))));
    }
    Ok(f)
}

fn set_switch(f: &mut Flags, name: &str, inline: Option<&str>) -> Result<(), RunError> {
    let on = match inline {
        None | Some("true" | "1" | "t" | "T" | "TRUE" | "True") => true,
        Some("false" | "0" | "f" | "F" | "FALSE" | "False") => false,
        Some(v) => return Err(usage(format!("invalid boolean value \"{v}\" for -{name}"))),
    };
    match name {
        "headless" => f.headless = on,
        "window" => f.window = on,
        _ => f.stop_stdin = on,
    }
    Ok(())
}

fn set_value(f: &mut Flags, name: &str, value: String) -> Result<(), RunError> {
    match name {
        "input" => f.input = value,
        "data" => f.data = Some(value),
        _ => {
            let n = value
                .parse::<i64>()
                .map_err(|_| usage(format!("--speed muss eine ganze Zahl sein, war \"{value}\"")))?;
            f.speed = Some(n);
        }
    }
    Ok(())
}

/// Liest den Text der Eingabe: aus der Datei oder (`-`) von stdin.
pub fn read_source(path: &str, stdin: &mut dyn Read) -> Result<String, RunError> {
    if path == "-" {
        let mut raw = Vec::new();
        stdin.read_to_end(&mut raw).map_err(|e| usage(format!("stdin: {e}")))?;
        return Ok(String::from_utf8_lossy(&raw).into_owned());
    }
    std::fs::read(path)
        .map(|raw| String::from_utf8_lossy(&raw).into_owned())
        .map_err(|e| usage(format!("open {path}: {e}")))
}

/// Das JSON-Objekt der Eingabe; `owner` ist der Go-Typ für die Meldung („runInput“).
fn read_object(raw: &str, owner: &str) -> Result<Map<String, Value>, RunError> {
    let mut it = serde_json::Deserializer::from_str(raw).into_iter::<Value>();
    let first = match it.next() {
        None => return Err(usage("Eingabe-JSON: EOF")),
        Some(Err(e)) => return Err(usage(format!("Eingabe-JSON: {e}"))),
        Some(Ok(v)) => v,
    };
    if it.next().is_some() {
        return Err(usage("Eingabe-JSON: Zusatzdaten nach dem Objekt"));
    }
    match first {
        Value::Object(m) => Ok(m),
        // Go dekodiert `null` in eine Struct ohne Fehler (alle Felder bleiben leer)
        Value::Null => Ok(Map::new()),
        other => Err(usage(format!(
            "Eingabe-JSON: json: cannot unmarshal {} into Go value of type simcli.{owner}",
            kind_of(&other)
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

/// Prüft die Schlüssel (Go vergleicht ohne Groß- und Kleinschreibung) und gibt je Feld den Wert zurück.
fn fields<'a>(
    obj: &'a Map<String, Value>,
    allowed: &[&'static str],
) -> Result<Vec<(&'static str, &'a Value)>, RunError> {
    let mut found = Vec::new();
    for (k, v) in obj {
        match allowed.iter().find(|a| a.eq_ignore_ascii_case(k)) {
            Some(name) => found.push((*name, v)),
            None => return Err(usage(format!("Eingabe-JSON: json: unknown field \"{k}\""))),
        }
    }
    Ok(found)
}

fn mismatch(v: &Value, path: &str, want: &str) -> RunError {
    let what = match v {
        Value::Number(n) => format!("number {n}"),
        other => kind_of(other).to_string(),
    };
    usage(format!(
        "Eingabe-JSON: json: cannot unmarshal {what} into Go struct field {path} of type {want}"
    ))
}

fn text(v: &Value, path: &str) -> Result<Option<String>, RunError> {
    match v {
        Value::Null => Ok(None),
        Value::String(s) => Ok(Some(s.clone())),
        other => Err(mismatch(other, path, "string")),
    }
}

fn int(v: &Value, path: &str) -> Result<Option<i64>, RunError> {
    match v {
        Value::Null => Ok(None),
        Value::Number(n) => n.as_i64().map(Some).ok_or_else(|| mismatch(v, path, "int")),
        other => Err(mismatch(other, path, "int")),
    }
}

fn float(v: &Value, path: &str) -> Result<Option<f64>, RunError> {
    match v {
        Value::Null => Ok(None),
        Value::Number(n) => Ok(n.as_f64()),
        other => Err(mismatch(other, path, "float64")),
    }
}

fn boolean(v: &Value, path: &str) -> Result<Option<bool>, RunError> {
    match v {
        Value::Null => Ok(None),
        Value::Bool(b) => Ok(Some(*b)),
        other => Err(mismatch(other, path, "bool")),
    }
}

/// Die Eingabe von `sim run`: Felder wie das MCP-Tool `sim_run`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RunInput {
    pub biome: String,
    pub seed: String,
    pub ticks: i64,
    pub inputs: Vec<Segment>,
}

/// Die Eingabe von `sim level`: Felder wie das MCP-Tool `level_generate`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelInput {
    pub biome: String,
    pub seed: String,
}

/// Liest die Eingabe von `run`.
pub fn read_run_input(raw: &str) -> Result<RunInput, RunError> {
    let obj = read_object(raw, "runInput")?;
    let mut r = RunInput::default();
    for (name, v) in fields(&obj, &["biome", "seed", "ticks", "inputs"])? {
        let path = format!("runInput.{name}");
        match name {
            "biome" => r.biome = text(v, &path)?.unwrap_or_default(),
            "seed" => r.seed = text(v, &path)?.unwrap_or_default(),
            "ticks" => r.ticks = int(v, &path)?.unwrap_or(0),
            _ => r.inputs = segments(v)?,
        }
    }
    Ok(r)
}

fn segments(v: &Value) -> Result<Vec<Segment>, RunError> {
    let list = match v {
        Value::Null => return Ok(Vec::new()),
        Value::Array(l) => l,
        other => return Err(mismatch(other, "runInput.inputs", "[]enginetools.Segment")),
    };
    list.iter().map(segment).collect()
}

fn segment(v: &Value) -> Result<Segment, RunError> {
    let obj = match v {
        Value::Object(m) => m,
        Value::Null => return Ok(Segment::default()),
        other => return Err(mismatch(other, "runInput.inputs", "enginetools.Segment")),
    };
    let mut s = Segment::default();
    for (name, v) in fields(obj, &["player", "fromTick", "toTick", "moveX", "sprint", "pay"])? {
        let path = format!("Segment.inputs.{name}");
        match name {
            "player" => s.player = int(v, &path)?.unwrap_or(0),
            "fromTick" => s.from_tick = int(v, &path)?.unwrap_or(0),
            "toTick" => s.to_tick = int(v, &path)?.unwrap_or(0),
            "moveX" => s.move_x = float(v, &path)?.unwrap_or(0.0),
            "sprint" => s.sprint = boolean(v, &path)?.unwrap_or(false),
            _ => s.pay = boolean(v, &path)?.unwrap_or(false),
        }
    }
    Ok(s)
}

/// Liest die Eingabe von `level`.
pub fn read_level_input(raw: &str) -> Result<LevelInput, RunError> {
    let obj = read_object(raw, "levelInput")?;
    let mut r = LevelInput::default();
    for (name, v) in fields(&obj, &["biome", "seed"])? {
        let path = format!("levelInput.{name}");
        let value = text(v, &path)?.unwrap_or_default();
        if name == "biome" {
            r.biome = value;
        } else {
            r.seed = value;
        }
    }
    Ok(r)
}
