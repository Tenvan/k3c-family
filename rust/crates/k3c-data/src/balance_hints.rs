//! data/balance-hints.json: Go `balance.Hints` (engine/balance/hints.go) mit `LoadHints` und `checkGroups`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Fehler, check, decode};

/// Go: `balance.Hints`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Hints {
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(rename = "notes")]
    pub notes: String,
    #[serde(rename = "figures")]
    pub figures: BTreeMap<String, FigureHint>,
    #[serde(rename = "levers")]
    pub levers: BTreeMap<String, Levers>,
}

/// Go: `balance.FigureHint`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FigureHint {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "green")]
    pub green: [Option<f64>; 2],
    #[serde(rename = "yellow")]
    pub yellow: [Option<f64>; 2],
    #[serde(rename = "allGrades", skip_serializing_if = "crate::omit::is_false")]
    pub all_grades: bool,
}

/// Go: `balance.Levers`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Levers {
    #[serde(rename = "low")]
    pub low: Vec<Lever>,
    #[serde(rename = "high")]
    pub high: Vec<Lever>,
}

/// Go: `balance.Lever`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Lever {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "step")]
    pub step: f64,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "group", skip_serializing_if = "String::is_empty")]
    pub group: String,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "balance-hints.json";

/// Kennzahlen, die die Auswertung aus Figures berechnet (engine/balance `figureKeys`).
const FIGURE_KEYS: [&str; 7] = [
    "firstWallTick",
    "firstWorkshopTick",
    "idleShare",
    "survivalDrop",
    "goldBalance",
    "woodBalance",
    "stoneBalance",
];

/// Gruppen der Hebel und Ziele (engine/balance `Groups`).
pub const GROUPS: [&str; 3] = ["basis", "holz", "schiff"];

/// `load_hints` liest und prüft eine Richtwert-Datei wie `balance.DefaultHints` (`LoadHints`, dann `checkGroups`).
/// Go durchläuft die Maps zufällig; hier sortiert, bei mehreren Fehlern kann deshalb ein anderer zuerst kommen.
/// Nicht geprüft: die Ziel-Kennzahl der Hebel (Tabelle `measures` aus engine/balance, B-1024).
pub fn load_hints(raw: &str) -> Result<Hints, Fehler> {
    let h: Hints = decode("balance-hints", raw)?;
    check("balance-hints", h.validate())?;
    check("balance-hints", h.check_groups())?;
    Ok(h)
}

impl Hints {
    fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!("version {} unbekannt, erwartet 1", self.version));
        }
        for (key, f) in &self.figures {
            if !FIGURE_KEYS.contains(&key.as_str()) {
                return Err(format!(
                    "kennzahl {key:?} unbekannt (bekannt: {})",
                    FIGURE_KEYS.join(", ")
                ));
            }
            f.validate().map_err(|e| format!("kennzahl {key}: {e}"))?;
        }
        for (key, l) in &self.levers {
            for lv in l.low.iter().chain(&l.high) {
                lv.validate().map_err(|e| format!("hebel {key}: {e}"))?;
            }
        }
        Ok(())
    }

    fn check_groups(&self) -> Result<(), String> {
        for (key, l) in &self.levers {
            for lv in l.low.iter().chain(&l.high) {
                if !GROUPS.contains(&lv.group.as_str()) {
                    return Err(format!(
                        "hebel {key}, pfad {}: group {:?}, erlaubt: {}",
                        lv.path,
                        lv.group,
                        GROUPS.join(", ")
                    ));
                }
            }
        }
        Ok(())
    }
}

impl FigureHint {
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name ist Pflicht".into());
        }
        if !band_valid(&self.green) || !band_valid(&self.yellow) {
            return Err("band: von ≤ bis nötig".into());
        }
        if !band_covers(&self.yellow, &self.green) {
            return Err("yellow muss green umfassen".into());
        }
        Ok(())
    }
}

impl Lever {
    fn validate(&self) -> Result<(), String> {
        if !lever_path(&self.path) {
            return Err(format!("pfad {:?}: erwartet datei.json:a.b.c", self.path));
        }
        if self.direction != "+" && self.direction != "-" {
            return Err(format!("pfad {}: direction + oder -", self.path));
        }
        if self.step <= 0.0 || self.step > 1.0 {
            return Err(format!("pfad {}: step über 0 bis 1", self.path));
        }
        if self.label.is_empty() {
            return Err(format!("pfad {}: label ist Pflicht", self.path));
        }
        Ok(())
    }
}

/// `band_valid`: von ≤ bis (offene Enden erlaubt).
fn band_valid(b: &[Option<f64>; 2]) -> bool {
    match (b[0], b[1]) {
        (Some(lo), Some(hi)) => lo <= hi,
        _ => true,
    }
}

/// `band_covers`: b enthält inner (offene Enden von b umfassen alles).
fn band_covers(b: &[Option<f64>; 2], inner: &[Option<f64>; 2]) -> bool {
    let lo = b[0].is_none_or(|b0| inner[0].is_some_and(|i0| i0 >= b0));
    let hi = b[1].is_none_or(|b1| inner[1].is_some_and(|i1| i1 <= b1));
    lo && hi
}

/// `lever_path` prüft wie die Regex `^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_-]+(\.[A-Za-z0-9_-]+)*$`.
fn lever_path(p: &str) -> bool {
    let word = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    match p.split_once(".json:") {
        Some((file, rest)) => word(file) && rest.split('.').all(word),
        None => false,
    }
}

#[cfg(test)]
mod balance_hints_tests {
    use super::lever_path;

    #[test]
    fn hebel_pfad_wie_regex() {
        for (p, ok) in [
            ("economy.json:purse.startGold", true),
            ("buildings.json:wall", true),
            ("a.b.json:x", false),
            ("economy.json:", false),
            ("economy.json:a..b", false),
            ("economy:a", false),
            ("eco nomy.json:a", false),
        ] {
            assert_eq!(lever_path(p), ok, "{p}");
        }
    }
}
