//! Eingabe-JSON von `sim world` und `sim hub` (Port von `worldInput` und `hubInput` aus `engine/simcli`, RU33.2).

use serde_json::Value;

use super::RunError;
use super::gojson::raw_field;
use super::input::{fields, int, read_object, text};

/// Die Eingabe von `sim world`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldInput {
    pub op: String,
    pub world: String,
    pub island: String,
    pub stage: i64,
    pub seed: String,
    /// Go: `json.RawMessage`, der Text des Entwurfs der Welt-Datei (leer, wenn das Feld fehlt).
    pub draft: String,
}

/// Die Eingabe von `sim hub`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HubInput {
    pub op: String,
    /// Der Text einer hub.json.
    pub raw: String,
}

/// Die Eingabe von `sim save` (RU40.2): Seed und Ordner der Spielstände.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SaveInput {
    pub seed: String,
    pub saves: String,
}

/// Liest die Eingabe von `save`.
pub fn read_save_input(raw: &str) -> Result<SaveInput, RunError> {
    let obj = read_object(raw, "saveInput")?;
    let mut r = SaveInput::default();
    for (name, v) in fields(&obj, &["seed", "saves"])? {
        let value = text(v, &format!("saveInput.{name}"))?.unwrap_or_default();
        if name == "seed" {
            r.seed = value;
        } else {
            r.saves = value;
        }
    }
    Ok(r)
}

/// Liest die Eingabe von `world`.
pub fn read_world_input(raw: &str) -> Result<WorldInput, RunError> {
    let obj = read_object(raw, "worldInput")?;
    let mut r = WorldInput::default();
    for (name, v) in fields(&obj, &["op", "world", "island", "stage", "seed", "draft"])? {
        let path = format!("worldInput.{name}");
        match name {
            "op" => r.op = text(v, &path)?.unwrap_or_default(),
            "world" => r.world = text(v, &path)?.unwrap_or_default(),
            "island" => r.island = text(v, &path)?.unwrap_or_default(),
            "stage" => r.stage = int(v, &path)?.unwrap_or(0),
            "seed" => r.seed = text(v, &path)?.unwrap_or_default(),
            _ => r.draft = raw_field(raw, "draft").map(ToString::to_string).unwrap_or_default(),
        }
    }
    Ok(r)
}

/// Liest die Eingabe von `hub`.
pub fn read_hub_input(raw: &str) -> Result<HubInput, RunError> {
    let obj = read_object(raw, "hubInput")?;
    let mut r = HubInput::default();
    for (name, v) in fields(&obj, &["op", "raw"])? {
        let path = format!("hubInput.{name}");
        let value = match v {
            Value::Null => String::new(),
            other => text(other, &path)?.unwrap_or_default(),
        };
        if name == "op" {
            r.op = value;
        } else {
            r.raw = value;
        }
    }
    Ok(r)
}
