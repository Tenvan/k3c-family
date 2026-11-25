//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Die Dinge hier haben einen Rumpf mit den echten
//! Werten, damit `hints.rs` schon läuft (Ziel-Kennzahlen, Gruppen, Ampel); die liefernde Session ersetzt sie durch ihr Original.

use serde::{Deserialize, Serialize};

use crate::evaluate::median;
use crate::metrics::Metrics;
use crate::run::Report;

/// RU31.1 (`insights.go`): die Ampel eines Ziels.
pub const LIGHT_GREEN: &str = "green";
/// RU31.1 (`insights.go`).
pub const LIGHT_YELLOW: &str = "yellow";
/// RU31.1 (`insights.go`).
pub const LIGHT_RED: &str = "red";

/// RU28.5 (`curves.go`, Go: `medianOrNil`): der Median oder `None` ohne Werte.
pub fn median_or_nil(vals: &[f64]) -> Option<f64> {
    if vals.is_empty() { None } else { Some(median(vals)) }
}

/// RU28.4 (`ship_need.go`, Go: `ShipFigures`): Bedarf gegen Bestand je Abschnitt im Bericht.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipFigures {}

/// RU28.4 (`ship_need.go`, Go: `shipFiguresOf`): verdichtet die Abschnitte der Läufe; der Platzhalter liefert nichts.
pub fn ship_figures_of(_r: &Report) -> Option<ShipFigures> {
    None
}

/// RU28.4 (`ship_need.go`, Go: `ShipNeed`): je Schiff-Abschnitt der Tag, ab dem er bezahlbar gewesen wäre.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ShipNeed {}

/// RU28.4 (`ship_need.go`, Go: `ShipStage`): ein Abschnitt der Schiff-Tabelle.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShipStage {}

/// RU28.4 (`ship_need.go`, Go: `LoadShip`): lädt die Schiff-Tabelle; der Platzhalter liefert keine Abschnitte.
pub fn load_ship() -> Result<Vec<ShipStage>, String> {
    Ok(Vec::new())
}

/// RU28.4 (`ship_need.go`, Go: `shipNeeds`): rechnet den Bedarf gegen den Bestand; der Platzhalter liefert nichts.
pub fn ship_needs(_stages: &[ShipStage], _m: &Metrics) -> Vec<ShipNeed> {
    Vec::new()
}
