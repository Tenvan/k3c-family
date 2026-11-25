//! Richtwerte der Kennzahlen und Hebel je Ziel (Port von `engine/balance/hints.go`, B-632): data/balance-hints.json,
//! Startwerte nach docs/rules/zielkorridore.md § 5. Die Auswertung (insights) liest sie; die Simulation nicht.
//!
//! Die Typen sind die aus `hints.go` (nicht `k3c_data::balance_hints`), damit Feldfolge und Prüfung wie in Go bleiben.

use std::collections::BTreeMap;

use k3c_data::Source;
use serde::{Deserialize, Deserializer, Serialize};

use crate::fehler::Fehler;
use crate::insights::{LIGHT_GREEN, LIGHT_RED, LIGHT_YELLOW};
use crate::summary::Entry;
use crate::targets::measure_known;

/// `[von, bis]` mit `None` = offen (Go: `Band`).
pub type Band = [Option<f64>; 2];

/// Der Richtwert einer Kennzahl: grün im Band `green`, gelb im Band `yellow`, sonst rot. `all_grades`: auch für die Grade
/// außer normal bewerten (sonst nur normal, z. B. fällt das Überleben bei hard und ultra gewollt stärker ab).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FigureHint {
    #[serde(rename = "name", deserialize_with = "null_default")]
    pub name: String,
    #[serde(rename = "unit", deserialize_with = "null_default")]
    pub unit: String,
    #[serde(rename = "green")]
    pub green: Band,
    #[serde(rename = "yellow")]
    pub yellow: Band,
    #[serde(rename = "allGrades", skip_serializing_if = "is_false")]
    pub all_grades: bool,
}

/// Ein Hebel: Wert in data/ (`datei.json:a.b.c`), Richtung „+“ oder „-“ und relative Schrittweite (0.1 = 10 %).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Lever {
    #[serde(rename = "path", deserialize_with = "null_default")]
    pub path: String,
    #[serde(rename = "direction", deserialize_with = "null_default")]
    pub direction: String,
    #[serde(rename = "step", deserialize_with = "null_default")]
    pub step: f64,
    #[serde(rename = "label", deserialize_with = "null_default")]
    pub label: String,
    /// Werte-Gruppe (groups); die Datei der Richtwerte verlangt eine.
    #[serde(
        rename = "group",
        skip_serializing_if = "String::is_empty",
        deserialize_with = "null_default"
    )]
    pub group: String,
}

/// Die Hebel eines Ziels: `low` bei einem Wert unter der Untergrenze, `high` über der Obergrenze.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Levers {
    #[serde(rename = "low", deserialize_with = "null_default")]
    pub low: Vec<Lever>,
    #[serde(rename = "high", deserialize_with = "null_default")]
    pub high: Vec<Lever>,
}

/// Der Inhalt von data/balance-hints.json.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Hints {
    #[serde(rename = "version", deserialize_with = "null_default")]
    pub version: i64,
    #[serde(rename = "notes", deserialize_with = "null_default")]
    pub notes: String,
    #[serde(rename = "figures", deserialize_with = "null_default")]
    pub figures: BTreeMap<String, FigureHint>,
    #[serde(rename = "levers", deserialize_with = "null_default")]
    pub levers: BTreeMap<String, Levers>,
}

/// Die Kennzahlen, die die Auswertung aus `figures` berechnet (Go: `figureKeys`, insights_figures).
pub const FIGURE_KEYS: [&str; 7] = [
    "firstWallTick",
    "firstWorkshopTick",
    "idleShare",
    "survivalDrop",
    "goldBalance",
    "woodBalance",
    "stoneBalance",
];

/// Go lässt JSON `null` bei Zahlen, Texten, Listen und Maps den Nullwert; serde braucht dafür diesen Weg.
fn null_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// `datei.json:a.b.c` wie bei Varianten und Tuning (ohne Platzhalter); Go: `leverPath`, die Regex
/// `^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_-]+(\.[A-Za-z0-9_-]+)*$` (hier ohne Regex-Crate).
pub fn lever_path(p: &str) -> bool {
    let word = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    match p.split_once(".json:") {
        Some((file, rest)) => word(file) && rest.split('.').all(word),
        None => false,
    }
}

/// Lädt die eingebettete data/balance-hints.json (Go: `DefaultHints`).
pub fn default_hints() -> Result<Hints, Fehler> {
    let raw = Source::Embedded
        .read("balance-hints.json")
        .map_err(|e| Fehler::Daten(e.to_string()))?;
    let h = load_hints(&raw)?;
    h.check_groups()
        .map_err(|e| Fehler::Daten(format!("balance-hints: {e}")))?;
    Ok(h)
}

/// Liest und prüft eine Richtwert-Datei; unbekannte Felder, Kennzahlen und Ziele sind ein Fehler (Go: `LoadHints`).
pub fn load_hints(raw: &str) -> Result<Hints, Fehler> {
    let h: Hints = serde_json::from_str(raw).map_err(|e| Fehler::Daten(format!("balance-hints: {e}")))?;
    h.validate().map_err(|e| Fehler::Daten(format!("balance-hints: {e}")))?;
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
            if !measure_known(key) {
                return Err(format!("hebel: ziel-kennzahl {key:?} unbekannt"));
            }
            for lv in l.low.iter().chain(&l.high) {
                lv.validate().map_err(|e| format!("hebel {key}: {e}"))?;
            }
        }
        Ok(())
    }

    /// Hebel eines Ziels in die nötige Richtung (unter der Untergrenze `low`, sonst `high`). Nie fehlend: Ein Ziel ohne
    /// Eintrag in balance-hints.json (z. B. ein neues) liefert die leere Liste, sonst trägt das JSON `null` und die
    /// Oberfläche stürzt bei `.length` ab (Go: `leversFor`).
    pub fn levers_for(&self, e: &Entry) -> &[Lever] {
        let (t, value) = (&e.verdict.target, e.verdict.value);
        let Some(l) = self.levers.get(&t.measure) else {
            return &[];
        };
        if let Some(lo) = t.lower
            && value < lo
        {
            return &l.low;
        }
        if let Some(hi) = t.upper
            && value > hi
        {
            return &l.high;
        }
        if let Some(lo) = t.lower
            && t.upper.is_none_or(|hi| value - lo < hi - value)
        {
            return &l.low; // knapp an der Untergrenze
        }
        &l.high
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

    /// Die Ampel eines Werts nach dem Richtwert (Go: `Light`).
    pub fn light(&self, v: f64) -> &'static str {
        if band_contains(&self.green, v) {
            LIGHT_GREEN
        } else if band_contains(&self.yellow, v) {
            LIGHT_YELLOW
        } else {
            LIGHT_RED
        }
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

/// Von ≤ bis (offene Enden erlaubt; Go: `Band.valid`).
pub fn band_valid(b: &Band) -> bool {
    match (b[0], b[1]) {
        (Some(lo), Some(hi)) => lo <= hi,
        _ => true,
    }
}

/// `b` enthält `inner` (offene Enden von `b` umfassen alles; Go: `Band.covers`).
pub fn band_covers(b: &Band, inner: &Band) -> bool {
    let lo = b[0].is_none_or(|b0| inner[0].is_some_and(|i0| i0 >= b0));
    let hi = b[1].is_none_or(|b1| inner[1].is_some_and(|i1| i1 <= b1));
    lo && hi
}

/// `v` liegt im Band (Go: `Band.contains`).
pub fn band_contains(b: &Band, v: f64) -> bool {
    b[0].is_none_or(|lo| v >= lo) && b[1].is_none_or(|hi| v <= hi)
}
