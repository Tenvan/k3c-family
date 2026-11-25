//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Die Dinge hier haben einen Rumpf mit den echten
//! Werten, damit `hints.rs` schon läuft (Ziel-Kennzahlen, Gruppen, Ampel); die liefernde Session ersetzt sie durch ihr Original.

use k3c_sim::island::Island;
use serde::{Deserialize, Serialize};

use crate::hints::Hints;

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

/// RU28.1 (`metrics.go`, Go: `StageMetrics`): eine Stufe des Laufs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StageMetrics {
    pub arrival_day: Option<i64>,
}

/// RU28.1 (`metrics.go`, Go: `BossMetrics`): ein besiegter Boss.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BossMetrics {
    pub boss: String,
    pub day: i64,
}

/// RU28.1 (`metrics.go`, Go: `DawnMetrics`): der Stand zum Tagesbeginn.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DawnMetrics {
    pub day: i64,
    pub gold: Vec<i64>,
    pub hub_level: i64,
    pub fighters: i64,
}

/// RU28.1 (`metrics.go`): die Kennzahlen eines Laufs; nur die Felder, die `run.rs`, `progress.rs` und ihre Tests lesen.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lost: String,
    pub first_wall_tick: Option<i64>,
    pub castle_fall_tick: Option<i64>,
    pub endboss_day: Option<i64>,
    pub stages: Vec<StageMetrics>,
    pub bosses: Vec<BossMetrics>,
    pub dawns: Vec<DawnMetrics>,
}

/// RU28.1 (`metrics.go`, Go: `collector`): sammelt die Kennzahlen eines Laufs; hier ohne Messung.
pub struct Collector;

impl Collector {
    /// Go: `newCollector(isl, threshold)`.
    pub fn new(_isl: &Island, _threshold: i64) -> Self {
        Collector
    }

    /// Go: `observe(tick)` (der Sammler hält die Insel in Go; hier kommt sie als Parameter).
    pub fn observe(&mut self, _isl: &Island, _tick: i64) {}

    /// Go: `result()`.
    pub fn result(&mut self, _isl: &Island) -> Metrics {
        Metrics::default()
    }
}
