//! Xbox-Berichte (`reports/*.json`) und Spielstände (`saves/*.json`) für Agenten lesbar (Port von `gamedata`, B-063):
//! nur lesen, verdichtet zu Text, Pfade nie außerhalb der beiden Ordner.

mod reports;
mod saves;

use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde_json::Value;

pub use reports::{report_read, reports_list};
pub use saves::{ENV_SAVES_DIR, saves_dir, saves_list};

#[cfg(test)]
#[path = "gamedata_tests.rs"]
mod tests;

/// Namen der `*.json` in `dir`, nach Bytes sortiert wie Go `filepath.Glob`; ein fehlender Ordner ist leer.
fn json_files(dir: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut files: Vec<(String, PathBuf)> = entries
        .filter_map(Result::ok)
        .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
        .filter(|(name, _)| name.ends_with(".json"))
        .collect();
    files.sort();
    files
}

/// Entfernt `null`-Felder, die Go beim Lesen wie fehlende behandelt.
fn drop_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            map.values_mut().for_each(drop_nulls);
        }
        Value::Array(list) => list.iter_mut().for_each(drop_nulls),
        _ => {}
    }
}

/// Liest eine JSON-Datei annähernd wie Go `json.Unmarshal` in eine Struktur: oberste Ebene Objekt oder `null`, fehlende
/// und `null`-Felder sind leer, unbekannte zählen nicht.
fn read_json<T: DeserializeOwned + Default>(path: &Path) -> Result<T, String> {
    let data = std::fs::read(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut value: Value = serde_json::from_str(&String::from_utf8_lossy(&data)).map_err(|e| e.to_string())?;
    match value {
        Value::Null => Ok(T::default()),
        Value::Object(_) => {
            drop_nulls(&mut value);
            serde_json::from_value(value).map_err(|e| e.to_string())
        }
        _ => Err("kein JSON-Objekt".to_owned()),
    }
}

/// RFC-3339-Zeit als lokale Zeit `JJJJ-MM-TT HH:MM`.
fn local_minute(text: &str) -> Option<String> {
    let time = chrono::DateTime::parse_from_rfc3339(text).ok()?;
    Some(time.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M").to_string())
}
