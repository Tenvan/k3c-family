//! Zustand des Fests für die Anzeige (RU62.12, B-973; Quelle `festival/`). `World::festival` trägt ihn in jeder Stufe einer
//! Insel, auf der der Endboss besiegt ist und die Welt in data/festival.json › `worlds` steht; sonst fehlt das Feld. Er gilt
//! vom Sieg bis zur Abfahrt, auf der letzten Insel der Welt bis zum Spielende (`docs/rules/kampf.md` § 10).
use serde::{Deserialize, Serialize};

/// Das Fest läuft; das Feld nennt den Himmels-Effekt der Welt (`lanterns`, `torches`).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FestivalState {
    /// Himmels-Effekt des Fests (data/festival.json › `worlds`).
    #[serde(rename = "effect")]
    pub effect: String,
}

/// Die Regel der Welt: Effekt und Werte der Flucht (aus `k3c_data::festival::FestivalFile`, einmal je Insel gebaut). Ohne
/// JSON.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FestivalRule {
    /// Himmels-Effekt des Fests.
    pub effect: String,
    /// Laufgeschwindigkeit fliehender Gegner als Vielfaches ihres Tempos.
    pub flee_speed_factor: f64,
    /// Abstand zum Portal oder Inselrand, ab dem ein fliehender Gegner verschwindet.
    pub exit_margin_units: f64,
}
