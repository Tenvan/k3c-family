//! Benannte Baselines (B-640, BS7.2): ein durchgelaufener Bericht unter einem Namen wie M1, mit der Konfiguration des
//! Laufs. Ablage `reports/baseline/<name>/` (`balance.json` = Bericht, `run.json` = [`BaselineInfo`]).

use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use chrono::{Local, SecondsFormat};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;

use super::evaluate::STATUS_INVALID;
use super::summary::{Summary, read_summary};
use super::targets::default_targets;
use super::writeguard::{mkdir, mkdir_all, remove_all, write_file};
use crate::kitx::{go_quoted, indent, marshal_raw};

/// Ordner der benannten Baselines, relativ zum Checkout.
pub const BASELINE_DIR: &str = "reports/baseline";

/// Name, Zeitpunkt, Herkunft und Konfiguration einer gesicherten Baseline.
///
/// `spec` ist rohes JSON: nur mit `serde_json::from_str` lesen (nicht über `Value`), geschrieben mit
/// [`BaselineInfo::json`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BaselineInfo {
    #[serde(deserialize_with = "or_default")]
    pub name: String,
    #[serde(deserialize_with = "or_default")]
    pub saved: String,
    /// Berichtsordner, aus dem gesichert wurde.
    #[serde(deserialize_with = "or_default")]
    pub run: String,
    #[serde(deserialize_with = "or_default")]
    pub seeds: i64,
    /// dev, all oder auswahl.
    #[serde(deserialize_with = "or_default")]
    pub seed_mode: String,
    /// Leere Listen als `[]`: Go legt sie so an.
    #[serde(deserialize_with = "or_default")]
    pub bots: Vec<String>,
    #[serde(deserialize_with = "or_default")]
    pub worlds: Vec<String>,
    #[serde(deserialize_with = "or_default")]
    pub pass: bool,
    /// Eingefrorene Gruppen zum Zeitpunkt des Sicherns; `None` (`null`), wenn die Ziel-Datei keine nennt.
    pub frozen: Option<Vec<String>>,
    /// Auftrag des Laufs als JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<RawValue>>,
}

/// Wie Go `json.Unmarshal`: `null` lässt das Feld leer.
fn or_default<'de, D: Deserializer<'de>, T: Default + Deserialize<'de>>(d: D) -> Result<T, D::Error> {
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

impl BaselineInfo {
    /// Wie Go `json.MarshalIndent`: auch `spec` wird neu eingerückt (`kitx::go_marshal_indent` schriebe ihn roh).
    pub fn json(&self) -> Result<String, String> {
        let compact = serde_json::to_string(self).map_err(|e| e.to_string())?;
        Ok(indent(&marshal_raw(&compact), "", "  "))
    }
}

/// Prüft einen Namen: Buchstaben, Ziffern, `_` und `-`, höchstens 40 Zeichen.
pub fn valid_baseline_name(name: &str) -> Result<(), String> {
    let allowed = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
    if (1..=40).contains(&name.len()) && name.bytes().all(allowed) {
        return Ok(());
    }
    Err(format!(
        "baseline {}: Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen",
        go_quoted(name)
    ))
}

/// Bericht der Baseline, relativ zum Checkout (mit `/`).
pub fn baseline_file(name: &str) -> String {
    format!("{BASELINE_DIR}/{name}/balance.json")
}

/// Liegt die benannte Baseline im Checkout?
pub fn has_baseline(root: &Path, name: &str) -> bool {
    fs::metadata(root.join(baseline_file(name))).is_ok()
}

/// Sichert den Bericht `report_path` (`balance-*.json`) unter `name`. Nur ein durchgelaufener Lauf (Pass) wird
/// gesichert, außer `open` ist gesetzt (offener Meilenstein); ein vergebener Name wird nie überschrieben. `spec` ist
/// der Auftrag des Laufs.
pub fn save_baseline(
    root: &Path,
    name: &str,
    report_path: &Path,
    spec: Option<Box<RawValue>>,
    open: bool,
) -> Result<BaselineInfo, String> {
    valid_baseline_name(name)?;
    let raw = fs::read(report_path).map_err(|e| format!("open {}: {e}", report_path.display()))?;
    let s = checked_report(name, report_path, &raw, open)?;
    let run = report_path.parent().and_then(Path::file_name);
    let run = run.map_or_else(|| ".".to_owned(), |r| r.to_string_lossy().into_owned());
    let mut info = info_of(root, name, run, &s);
    info.spec = spec;
    let meta = info.json()?;

    let base = root.join(BASELINE_DIR);
    let dir = base.join(name);
    mkdir_all(root, &base).map_err(|e| e.to_string())?;
    // atomar: genau ein Aufruf legt den Namen an
    mkdir(root, &dir).map_err(|e| match e.kind() {
        ErrorKind::AlreadyExists => {
            format!("baseline {name} gibt es schon ({BASELINE_DIR}/{name}); anderer Name oder Ordner von Hand löschen")
        }
        _ => e.to_string(),
    })?;
    // run.json zuletzt: ein Ordner ohne run.json ist ein Fehlschlag und wird entfernt
    let written = write_file(root, dir.join("balance.json"), &raw)
        .and_then(|()| write_file(root, dir.join("run.json"), meta.as_bytes()));
    if let Err(e) = written {
        let _ = remove_all(root, &dir); // der Schreibfehler zählt
        return Err(e.to_string());
    }
    Ok(info)
}

/// Liest den Bericht und lehnt ab, was nicht gesichert wird.
fn checked_report(name: &str, report_path: &Path, raw: &[u8], open: bool) -> Result<Summary, String> {
    let file = report_path.file_name().unwrap_or_default().to_string_lossy();
    let s = read_summary(raw).map_err(|e| format!("{file}: {e}"))?;
    if !s.pass && !open {
        return Err(format!(
            "baseline {name}: der Lauf hat sein Ziel nicht erreicht (Pass fehlt); gesichert werden nur durchgelaufene \
             Läufe, bei einem offenen Meilenstein ohnePass setzen"
        ));
    }
    if s.seeds == 0 || !s.targets.iter().any(|e| e.verdict.status != STATUS_INVALID) {
        return Err(format!(
            "baseline {name}: der Bericht hat keine gültige Messung (Lauf abgestürzt?)"
        ));
    }
    Ok(s)
}

/// Fügt `value` an, wenn es noch fehlt.
fn push_new(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|v| v == value) {
        list.push(value.to_owned());
    }
}

fn info_of(root: &Path, name: &str, run: String, s: &Summary) -> BaselineInfo {
    let mut info = BaselineInfo {
        name: name.to_owned(),
        saved: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        run,
        seeds: s.seeds,
        seed_mode: s.seed_mode.clone(),
        pass: s.pass,
        frozen: Some(Vec::new()),
        ..BaselineInfo::default()
    };
    for e in &s.targets {
        push_new(&mut info.bots, &e.verdict.target.bot);
        if !e.verdict.target.world.is_empty() {
            push_new(&mut info.worlds, &e.verdict.target.world);
        }
    }
    // Ohne lesbare Ziel-Datei bleibt die Liste leer.
    if let Ok(targets) = default_targets(root) {
        info.frozen = targets.frozen;
    }
    info
}

/// Die gesicherten Baselines, nach Name sortiert; ein Ordner ohne lesbares `run.json` fehlt in der Liste.
pub fn list_baselines(root: &Path) -> Result<Vec<BaselineInfo>, String> {
    let base = root.join(BASELINE_DIR);
    let fail = |e: std::io::Error| format!("open {}: {e}", base.display());
    let entries = match fs::read_dir(&base) {
        Ok(entries) => entries,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(fail(e)),
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(fail)?;
        let Ok(raw) = fs::read_to_string(entry.path().join("run.json")) else {
            continue;
        };
        if let Ok(info) = serde_json::from_str::<BaselineInfo>(&raw)
            && entry.file_name() == info.name.as_str()
        {
            out.push(info);
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

#[cfg(test)]
#[path = "baselines_tests.rs"]
mod tests;
