//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Die Dinge hier haben einen Rumpf mit den echten
//! Werten, damit `hints.rs` schon läuft (Ziel-Kennzahlen, Gruppen, Ampel); die liefernde Session ersetzt sie durch ihr Original.

use serde::{Deserialize, Serialize};

use crate::hints::Hints;
use crate::metrics::Metrics;

/// RU31.1 (`insights.go`): die Ampel eines Ziels.
pub const LIGHT_GREEN: &str = "green";
/// RU31.1 (`insights.go`).
pub const LIGHT_YELLOW: &str = "yellow";
/// RU31.1 (`insights.go`).
pub const LIGHT_RED: &str = "red";

/// RU28.2 (`groups.go`): die bekannten Werte-Gruppen (Go: `Groups`).
pub const GROUPS: [&str; 3] = ["basis", "holz", "schiff"];

/// RU28.2 (`groups.go`): `name` ist eine bekannte Gruppe (Go: `IsGroup`).
pub fn is_group(name: &str) -> bool {
    GROUPS.contains(&name)
}

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

impl Hints {
    /// RU28.2 (`groups.go`): verlangt in der Datei der Richtwerte eine bekannte Gruppe an jedem Hebel (Go: `checkGroups`).
    pub fn check_groups(&self) -> Result<(), String> {
        for (key, l) in &self.levers {
            for lv in l.low.iter().chain(&l.high) {
                if !is_group(&lv.group) {
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
