//! Auswertung eines Balancing-Berichts ohne KI (B-632): Ampel je Ziel und Kennzahl, Stärken, Schwächen,
//! Tuning-Hinweise, Kurzauswertung und Vorschlag für den nächsten Lauf.
//!
//! Bis RU70.16 stehen hier nur die Typen und Ampeln. Welche Listen Go als `null` schreibt, prüft RU70.16 mit den
//! Funktionen, die sie füllen.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::delta::GroupDelta;
use super::hints::Lever;
use super::insights_figures::FigureInsights;
use super::insights_levers::LeverGroup;
use super::insights_next::NextRun;
use super::insights_plain::LeverPlan;
use super::insights_text::Hint;
use super::progress::{GoalView, Progress, QualityView};
use super::report_types::nil_empty;
use super::summary::{Change, Entry};

/// Ampeln: grün ok, gelb knapp, rot verletzt, grau ungültig bzw. ohne Messwert.
pub const LIGHT_GREEN: &str = "green";
pub const LIGHT_YELLOW: &str = "yellow";
pub const LIGHT_RED: &str = "red";
pub const LIGHT_GREY: &str = "grey";

/// Die Auswertung; das JSON speist den HTML-Bericht und die Oberfläche.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Insights {
    pub title: String,
    pub run: String,
    pub generated: String,
    pub seeds: i64,
    pub pass: bool,
    pub counts: BTreeMap<String, i64>,
    pub summary: Vec<String>,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub targets: Vec<TargetInsight>,
    pub figures: FigureInsights,
    pub hints: Vec<Hint>,
    /// Stellschrauben nach Wirkung zusammengefasst.
    pub lever_groups: Vec<LeverGroup>,
    pub next: NextRun,
    pub baseline: BaselineView,
    /// Vergleich je Werte-Gruppe, leer ohne Baseline.
    pub delta: Vec<GroupDelta>,
    pub varianten: Vec<String>,
    pub goal: Option<GoalView>,
    pub quality: Option<QualityView>,
    pub progress: Option<Progress>,
    /// Lauf, mit dem verglichen wurde.
    pub previous: String,
    pub goal_text: String,
}

/// Ein Ziel mit Ampel. `distance`: Abstand zur nächsten Grenze in % der Korridorbreite, positiv im Korridor.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TargetInsight {
    pub name: String,
    pub group: String,
    pub measure: String,
    pub kind: String,
    pub unit: String,
    pub grade: String,
    pub scenario: String,
    pub value: f64,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub status: String,
    pub light: String,
    pub distance: Option<f64>,
    pub trend: Option<Trend>,
    #[serde(serialize_with = "nil_empty")]
    pub fail_seeds: Vec<String>,
    #[serde(serialize_with = "nil_empty")]
    pub invalid_seeds: Vec<String>,
    pub rule: String,
    pub levers: Vec<Lever>,
    /// Gespielte Läufe.
    pub runs: i64,
    /// Messwert, Sollwert und Lage in Sätzen.
    pub plain: String,
    pub gap: String,
    pub trend_text: String,
    pub plans: Vec<LeverPlan>,
    /// Das bewertete Ziel, nicht im JSON.
    #[serde(skip)]
    pub entry: Entry,
}

/// Vergleich mit der Baseline: `better`, wenn der Abstand zum Korridor größer (die Verletzung kleiner) wurde.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Trend {
    pub before: f64,
    pub delta: f64,
    pub better: bool,
    pub before_status: String,
}

/// Vergleich mit der Baseline für die Anzeige.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BaselineView {
    pub present: bool,
    pub note: String,
    pub changes: Vec<Change>,
}

/// Der Lauf für Kopf und Befehle.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Meta {
    pub run: String,
    pub generated: String,
    pub baseline: String,
    /// Eingefrorene Werte-Gruppen: keine Hebel, Delta nur beim Kippen.
    pub frozen: Vec<String>,
}
