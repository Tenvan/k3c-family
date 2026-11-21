//! Zustand der Simulation, Teil 2: Ressourcen, Bauplätze, Burg (Port von `engine/sim/types.go`).

use serde::{Deserialize, Serialize};

use super::omit::{is_false, is_zero_f64, is_zero_i64};

/// Ein Baum, Fels oder Kupfererz.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ResourceNode {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "paidGold")]
    pub paid_gold: i64,
    #[serde(rename = "marked")]
    pub marked: bool,
    /// Spieler-Index der letzten Markier-Münze, fehlt = unbekannt.
    #[serde(rename = "markedBy", skip_serializing_if = "Option::is_none")]
    pub marked_by: Option<i64>,
    #[serde(rename = "workerId")]
    pub worker_id: Option<i64>,
    #[serde(rename = "progress")]
    pub progress: f64,
    /// Variante aus dem Level (Baumart, Kristallfarbe); nur im Prozess, nicht im Zustand und nicht im Spielstand.
    #[serde(skip)]
    pub variant: String,
    /// Baum einer Baumschule: kein Baum-Gold; ohne JSON.
    #[serde(skip)]
    pub plant: bool,
    /// Abgebaut und aus `nodes` entfernt (Plantage); ohne JSON.
    #[serde(skip)]
    pub gone: bool,
}

/// Go: `sim.plot`, ein Platz der Plantage: der gewachsene Baum oder die Zeit bis zum nächsten. Der Baum ist in Go ein
/// Zeiger auf einen Knoten von `World.nodes`; in Rust seine ID (Vorentscheidung 2).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Plot {
    pub tree: Option<i64>,
    pub grow_in: f64,
}

/// Ein Bauplatz im Hub. `state`: unpaid, waitingMaterial, waitingWorker, built.
///
/// Die Zwischenspeicher `hubW`/`hubLv` und `wall` (mit Rückverweis auf die Welt) entfallen (Vorentscheidung 4): Sie sind
/// reine Ableitungen von Art und X und werden je Aufruf neu gerechnet.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Site {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "paidGold")]
    pub paid_gold: i64,
    #[serde(rename = "buildProgress")]
    pub build_progress: f64,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "maxHp")]
    pub max_hp: f64,
    #[serde(rename = "workerId")]
    pub worker_id: Option<i64>,
    /// Bogner: fertige Bögen im Regal.
    #[serde(rename = "bows")]
    pub bows: i64,
    /// Werkstatt: fertige Hämmer im Regal.
    #[serde(rename = "hammers", skip_serializing_if = "is_zero_i64")]
    pub hammers: i64,
    /// Bezahltes Gold für das Produkt am Platz (Werkstatt: Bogen).
    #[serde(rename = "bowPaidGold")]
    pub bow_paid_gold: i64,
    /// Spielzeit, zu der das Produkt am Platz fertig ist (0 = keins in Arbeit).
    #[serde(rename = "craftDone", skip_serializing_if = "is_zero_f64")]
    pub craft_done: f64,
    /// Bezahltes Gold je Angebot (Index wie `buildings[kind].offers`).
    #[serde(rename = "offerPaid", skip_serializing_if = "Vec::is_empty")]
    pub offer_paid: Vec<i64>,
    /// Wie `craft_done` für das Elite-Krieger-Angebot der Schmiede.
    #[serde(rename = "offerDone", skip_serializing_if = "is_zero_f64")]
    pub offer_done: f64,
    /// Schwerter der Werkstatt, wie `bows`, `bow_paid_gold` und `craft_done`.
    #[serde(rename = "swords", skip_serializing_if = "is_zero_i64")]
    pub swords: i64,
    #[serde(rename = "swordPaidGold", skip_serializing_if = "is_zero_i64")]
    pub sword_paid_gold: i64,
    #[serde(rename = "swordDone", skip_serializing_if = "is_zero_f64")]
    pub sword_done: f64,
    /// Rüstungen im Regal des Rüstungsschmieds (FG1.2, native Regel; Gold und Arbeit in `bow_paid_gold`/`craft_done`).
    #[serde(rename = "armors", skip_serializing_if = "is_zero_i64")]
    pub armors: i64,
    /// Stufe des gebauten Platzes (0 = 1).
    #[serde(rename = "level", skip_serializing_if = "is_zero_i64")]
    pub level: i64,
    /// Zustand des Ausbaus auf die nächste Stufe ("", waitingMaterial, waitingWorker).
    #[serde(rename = "upgrade", skip_serializing_if = "String::is_empty")]
    pub upgrade: String,
    #[serde(rename = "upgradePaid", skip_serializing_if = "is_zero_i64")]
    pub upgrade_paid: i64,
    /// Der Platz ist eine Ruine der Planung; dauerhafte Herkunft, nicht im Spielstand.
    #[serde(rename = "ruin", skip_serializing_if = "is_false")]
    pub ruin: bool,
    /// Plantage einer gebauten Baumschule; ohne JSON.
    #[serde(skip)]
    pub plots: Vec<Plot>,
    /// Zaubertum: Sekunden bis zum nächsten Schuss; ohne JSON.
    #[serde(skip)]
    pub spell_in: f64,
}

/// Die Burg in der Hub-Mitte.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Castle {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "maxHp")]
    pub max_hp: f64,
}
