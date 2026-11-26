//! Richtwerte der Kennzahlen und Hebel je Ziel (B-632): `data/balance-hints.json`, Startwerte nach
//! `docs/rules/zielkorridore.md` § 5. Die Auswertung liest sie; die Simulation nicht.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::datafiles;
use super::insights::{LIGHT_GREEN, LIGHT_RED, LIGHT_YELLOW};
use super::report_types::read_go;
use super::summary::Entry;
use super::targets::{known_keys, measure};
use crate::kitx::go_quoted;

/// `[von, bis]` mit `null` = offen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
pub struct Band(pub [Option<f64>; 2]);

impl<'de> Deserialize<'de> for Band {
    /// Wie Go bei `[2]*float64`: fehlende Einträge sind `null`, überzählige zählen nicht.
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut list = Vec::<Option<f64>>::deserialize(d)?;
        list.resize(2, None);
        Ok(Self([list[0], list[1]]))
    }
}

/// Richtwert einer Kennzahl: grün im Band `green`, gelb im Band `yellow`, sonst rot.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct FigureHint {
    pub name: String,
    pub unit: String,
    pub green: Band,
    pub yellow: Band,
    /// Auch für die Grade außer normal bewerten.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub all_grades: bool,
}

/// Ein Hebel: Wert in `data/` (`datei.json:a.b.c`), Richtung `+` oder `-` und relative Schrittweite (0.1 = 10 %).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Lever {
    pub path: String,
    pub direction: String,
    pub step: f64,
    pub label: String,
    /// Werte-Gruppe; die Datei der Richtwerte verlangt eine.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub group: String,
}

/// Hebel eines Ziels: `low` bei einem Wert unter der Untergrenze, `high` über der Obergrenze.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Levers {
    pub low: Vec<Lever>,
    pub high: Vec<Lever>,
}

/// Inhalt von `data/balance-hints.json`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Hints {
    pub version: i64,
    pub notes: String,
    pub figures: BTreeMap<String, FigureHint>,
    pub levers: BTreeMap<String, Levers>,
}

/// Kennzahlen, die die Auswertung aus den Figures berechnet.
const FIGURE_KEYS: [&str; 7] = [
    "firstWallTick",
    "firstWorkshopTick",
    "idleShare",
    "survivalDrop",
    "goldBalance",
    "woodBalance",
    "stoneBalance",
];

/// `datei.json:a.b.c` wie bei Varianten und Tuning (ohne Platzhalter).
static LEVER_PATH: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_-]+(\.[A-Za-z0-9_-]+)*$").ok());

/// Lädt `data/balance-hints.json` samt Prüfung der Gruppen.
pub fn default_hints(root: &Path) -> Result<Hints, String> {
    let raw = datafiles::read(root, "balance-hints.json")?;
    let h = load_hints(&raw)?;
    h.check_groups().map_err(|e| format!("balance-hints: {e}"))?;
    Ok(h)
}

/// Liest und prüft eine Richtwert-Datei; unbekannte Felder, Kennzahlen und Ziele sind ein Fehler.
pub fn load_hints(raw: &[u8]) -> Result<Hints, String> {
    parse_strict(raw)
        .and_then(|h| h.validate().map(|()| h))
        .map_err(|e| format!("balance-hints: {e}"))
}

/// Die Werte eines Objekts, bei `null` oder fehlendem Objekt keine.
fn entries(value: Option<&Value>) -> impl Iterator<Item = &Value> {
    value.and_then(Value::as_object).into_iter().flat_map(|m| m.values())
}

fn parse_strict(raw: &[u8]) -> Result<Hints, String> {
    let value: Value = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    known_keys(Some(&value), &["version", "notes", "figures", "levers"])?;
    for figure in entries(value.get("figures")) {
        known_keys(Some(figure), &["name", "unit", "green", "yellow", "allGrades"])?;
    }
    for levers in entries(value.get("levers")) {
        known_keys(Some(levers), &["low", "high"])?;
        for side in ["low", "high"] {
            for lever in levers.get(side).and_then(Value::as_array).into_iter().flatten() {
                known_keys(Some(lever), &["path", "direction", "step", "label", "group"])?;
            }
        }
    }
    read_go(raw)
}

impl Hints {
    fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!("version {} unbekannt, erwartet 1", self.version));
        }
        for (key, f) in &self.figures {
            if !FIGURE_KEYS.contains(&key.as_str()) {
                return Err(format!(
                    "kennzahl {} unbekannt (bekannt: {})",
                    go_quoted(key),
                    FIGURE_KEYS.join(", ")
                ));
            }
            f.validate().map_err(|e| format!("kennzahl {key}: {e}"))?;
        }
        for (key, l) in &self.levers {
            if measure(key).is_none() {
                return Err(format!("hebel: ziel-kennzahl {} unbekannt", go_quoted(key)));
            }
            for lv in l.low.iter().chain(&l.high) {
                lv.validate().map_err(|e| format!("hebel {key}: {e}"))?;
            }
        }
        Ok(())
    }

    /// Hebel eines Ziels in die nötige Richtung; nie `null` im JSON (die Oberfläche liest `.length`).
    pub fn levers_for(&self, e: &Entry) -> Vec<Lever> {
        let Some(l) = self.levers.get(&e.verdict.target.measure) else {
            return Vec::new();
        };
        let (value, lower, upper) = (e.verdict.value, e.verdict.target.lower, e.verdict.target.upper);
        if lower.is_some_and(|lo| value < lo) {
            return l.low.clone();
        }
        if upper.is_some_and(|hi| value > hi) {
            return l.high.clone();
        }
        // knapp an der Untergrenze
        if lower.is_some_and(|lo| upper.is_none_or(|hi| value - lo < hi - value)) {
            return l.low.clone();
        }
        l.high.clone()
    }
}

impl FigureHint {
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name ist Pflicht".to_owned());
        }
        if !self.green.valid() || !self.yellow.valid() {
            return Err("band: von ≤ bis nötig".to_owned());
        }
        if !self.yellow.covers(self.green) {
            return Err("yellow muss green umfassen".to_owned());
        }
        Ok(())
    }

    /// Ampel eines Werts nach dem Richtwert.
    pub fn light(&self, v: f64) -> &'static str {
        if self.green.contains(v) {
            LIGHT_GREEN
        } else if self.yellow.contains(v) {
            LIGHT_YELLOW
        } else {
            LIGHT_RED
        }
    }
}

impl Lever {
    fn validate(&self) -> Result<(), String> {
        let path = &self.path;
        if !LEVER_PATH.as_ref().is_some_and(|re| re.is_match(path)) {
            return Err(format!("pfad {}: erwartet datei.json:a.b.c", go_quoted(path)));
        }
        if self.direction != "+" && self.direction != "-" {
            return Err(format!("pfad {path}: direction + oder -"));
        }
        if self.step <= 0.0 || self.step > 1.0 {
            return Err(format!("pfad {path}: step über 0 bis 1"));
        }
        if self.label.is_empty() {
            return Err(format!("pfad {path}: label ist Pflicht"));
        }
        Ok(())
    }
}

impl Band {
    fn valid(self) -> bool {
        match self.0 {
            [Some(lo), Some(hi)] => lo <= hi,
            _ => true,
        }
    }

    /// Enthält dieses Band `inner`? Offene Enden umfassen alles.
    fn covers(self, inner: Band) -> bool {
        let lo = self.0[0].is_none_or(|lo| inner.0[0].is_some_and(|i| i >= lo));
        let hi = self.0[1].is_none_or(|hi| inner.0[1].is_some_and(|i| i <= hi));
        lo && hi
    }

    pub(crate) fn contains(self, v: f64) -> bool {
        self.0[0].is_none_or(|lo| v >= lo) && self.0[1].is_none_or(|hi| v <= hi)
    }
}

#[cfg(test)]
#[path = "hints_tests.rs"]
mod tests;
