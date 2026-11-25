//! Wert-Varianten eines Laufs (Port von `engine/balance/variant.go`, WZ28.4): `datei.json:a.b.c=wert` setzt eine Zahl aus
//! data/ auf einen festen Wert, nur im Speicher wie die Sensitivität (`vary.rs`); data/*.json bleiben unberührt.

use std::collections::BTreeMap;

use k3c_data::Source;
use serde_json::{Number, Value};

use crate::fehler::Fehler;
use crate::island_world::base_shared;
use crate::vary::{check_reloaded, child_mut, with_data};

/// Alle Dateien aus `data/` im Speicher mit den Varianten. Unbekannter Pfad, keine Zahl oder kein Zahlwert: Fehler mit der
/// Variante (Go: `VariantFS`).
pub fn variant_source(specs: &[String]) -> Result<Source, Fehler> {
    let mut mem = base_shared()?
        .source
        .files()
        .map_err(|e| Fehler::Daten(e.to_string()))?;
    for s in specs {
        set_variant(&mut mem, s).map_err(|e| Fehler::Daten(format!("variante {s:?}: {e}")))?;
    }
    Ok(Source::Mem(mem))
}

/// Setzt eine Variante in `mem` (Go: `setVariant`).
fn set_variant(mem: &mut BTreeMap<String, String>, spec: &str) -> Result<(), String> {
    let (path, value) = spec.split_once('=').unzip();
    let (file, keys) = path.and_then(|p: &str| p.split_once(':')).unzip();
    let (Some(value), Some(file), Some(keys)) = (value, file, keys.filter(|k| !k.is_empty())) else {
        return Err("erwartet datei.json:a.b.c=wert".into());
    };
    let number = parse_number(value)?;
    check_reloaded(file)?;
    let raw = mem.get(file).ok_or_else(|| format!("datei {file:?} fehlt in data/"))?;
    let mut root: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    set_number(&mut root, &keys.split('.').collect::<Vec<_>>(), number)?;
    let text = serde_json::to_string(&root).map_err(|e| e.to_string())?;
    mem.insert(file.to_string(), text);
    Ok(())
}

/// Der Wert als JSON-Zahl; Ganzzahlen bleiben ganz, alles andere ist eine Gleitkommazahl (Go: `ParseFloat` und `json.Number`).
fn parse_number(value: &str) -> Result<Number, String> {
    let not_number = || format!("wert {value:?} ist keine Zahl");
    let f: f64 = value.parse().map_err(|_| not_number())?;
    if let Ok(i) = value.parse::<i64>() {
        return Ok(Number::from(i));
    }
    Number::from_f64(f).ok_or_else(not_number)
}

/// Ersetzt die Zahl unter `keys` in `node` (Go: `setNumber`).
fn set_number(node: &mut Value, keys: &[&str], value: Number) -> Result<(), String> {
    let (last, parents) = keys.split_last().ok_or("leerer Pfad")?;
    let mut node = node;
    for k in parents {
        node = child_mut(node, k).ok_or_else(|| format!("{k:?} fehlt"))?;
    }
    let slot = child_mut(node, last).ok_or_else(|| format!("{last:?} fehlt"))?;
    if !slot.is_number() {
        return Err(format!("{last:?} ist keine Zahl"));
    }
    *slot = Value::Number(value);
    Ok(())
}

/// Rechnet `f` mit den Varianten (Sim und Bot-Preise) und stellt danach `data/` wieder her; ohne Varianten direkt (Go:
/// `WithVariants`). Gilt für diesen Thread und seine Läufe (`vary::with_data`).
pub fn with_variants<T>(specs: &[String], f: impl FnOnce() -> Result<T, Fehler>) -> Result<T, Fehler> {
    if specs.is_empty() {
        return f();
    }
    with_data(variant_source(specs)?, f)
}
