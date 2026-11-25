//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Die Dinge hier haben einen Rumpf mit den echten
//! Werten, damit `hints.rs` schon läuft (Ziel-Kennzahlen, Gruppen, Ampel); die liefernde Session ersetzt sie durch ihr Original.

use serde::{Deserialize, Serialize};

use crate::economy_measures::{b2f, economy_measure};
use crate::evaluate::median;
use crate::metrics::Metrics;
use crate::run::Report;

/// RU31.1 (`insights.go`): die Ampel eines Ziels.
pub const LIGHT_GREEN: &str = "green";
/// RU31.1 (`insights.go`).
pub const LIGHT_YELLOW: &str = "yellow";
/// RU31.1 (`insights.go`).
pub const LIGHT_RED: &str = "red";

/// RU28.3 (`summary.go`, Go: `Entry` mit eingebettetem `Verdict`): ein bewertetes Ziel; nur die Felder, die
/// `Hints::levers_for` liest.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Entry {
    pub measure: String,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub value: f64,
}

/// RU28.3 (`targets.go` und `economy_measures.go`): `key` ist eine Kennzahl der Tabelle `measures`.
pub fn measure_known(key: &str) -> bool {
    matches!(
        key,
        "castleHeld"
            | "firstWallBeforeDusk1"
            | "endbossDefeated"
            | "endbossDay"
            | "caveBeforeDay6"
            | "destroyedPerWave"
            | "goldAtDawnPerPlayer"
            | "firstTowerBeforeDusk2"
            | "woodAtDawnFrom3"
            | "hubLevel2BeforeDay5"
            | "hubLevel3BeforeDay10"
            | "fightersAtDawn"
    )
}

/// RU28.3 (`summary.go`, Go: `num`): eine Zahl so kurz wie möglich mit Komma als Dezimalzeichen.
pub fn num(x: f64) -> String {
    format!("{x}").replace('.', ",")
}

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

/// RU28.3 (`targets.go` und `economy_measures.go`, Go: `measures[key].values(m)`): die Messwerte der Kennzahl `key` für
/// einen Lauf; ein fremder Schlüssel liefert keinen.
pub fn measure_values(key: &str, m: &Metrics) -> Vec<f64> {
    match key {
        "castleHeld" => castle_held(m),
        "firstWallBeforeDusk1" => {
            let dusk = m.days.first().and_then(|d| d.dusk_tick);
            vec![b2f(matches!((m.first_wall_tick, dusk), (Some(w), Some(d)) if w <= d))]
        }
        "endbossDefeated" => vec![b2f(m.endboss_tick.is_some())],
        // Ohne Sieg zählt der Tag nach dem Lauf (Tageslimit + 1).
        "endbossDay" => vec![m.endboss_day.map_or(m.days.len() as f64 + 1.0, |d| d as f64)],
        "caveBeforeDay6" => {
            let ok = m.stages.get(1).and_then(|s| s.arrival_day).is_some_and(|d| d < 6);
            vec![b2f(ok)]
        }
        // Zerstörte Gebäude je Welle 1–5.
        "destroyedPerWave" => m
            .waves
            .iter()
            .filter(|w| w.wave <= 5)
            .map(|w| w.buildings_destroyed as f64)
            .collect(),
        _ => economy_measure(key, m).map(|(_, v)| v).unwrap_or_default(),
    }
}

/// Das Fenster der Kennzahl castleHeld: Nacht 1 bis 5, unabhängig von `Target.days` (B-657).
const CASTLE_HELD_NIGHTS: i64 = 5;

/// B-657: 1, wenn die Burg bis zum Morgen nach Nacht 5 nicht fiel; 0 bei einem Fall davor. Ein Lauf, der diesen Morgen ohne
/// Burgfall nicht erreicht, ist nicht auswertbar (kein Wert statt „verletzt“).
fn castle_held(m: &Metrics) -> Vec<f64> {
    let end = m.dawns.iter().find(|d| d.day == CASTLE_HELD_NIGHTS + 1).map(|d| d.tick);
    let fell = m.castle_fall_tick.is_some_and(|t| end.is_none_or(|e| t < e));
    if !fell && end.is_none() {
        return Vec::new();
    }
    vec![b2f(!fell)]
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
