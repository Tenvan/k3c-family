//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Was hier steht, liefert RU31.4 bis RU31.5;
//! die Typen haben die Felder und JSON-Namen von Go, die Funktionen noch keinen Inhalt (leere Texte und Listen).

use serde::{Deserialize, Serialize};

use crate::fehler::Fehler;
use crate::hints::Lever;
use crate::insights::Insights;
use crate::insights_plain::LeverPlan;

/// RU31.4 (`insights_text.go`): ein Tuning-Hinweis zu einem Ziel.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Hint {
    pub target: String,
    pub light: String,
    pub text: String,
    pub why: String,
    pub levers: Vec<Lever>,
    pub plans: Vec<LeverPlan>,
}

/// RU31.4 (`insights_text.go`): die Stärken.
pub fn strengths(_in: &Insights) -> Vec<String> {
    Vec::new()
}

/// RU31.4 (`insights_text.go`): die Schwächen.
pub fn weaknesses(_in: &Insights) -> Vec<String> {
    Vec::new()
}

/// RU31.4 (`insights_text.go`): die Kurzauswertung.
pub fn summary(_in: &Insights) -> Vec<String> {
    Vec::new()
}

/// RU31.4 (`report_html.go`, Go: `Insights.HTML`): die eigenständige HTML-Seite.
pub fn report_html(_in: &Insights, _root: &str) -> Result<Vec<u8>, Fehler> {
    Ok(Vec::new())
}
