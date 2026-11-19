//! Zustand des Einsiedlers für die Anzeige (RU62.10, B-972; Quelle `hermit/`). `World::hermit` trägt ihn in jeder Stufe einer
//! Welt mit Einsiedler (data/hermits.json); sonst fehlt das Feld. Die Insel schreibt ihn je Takt in alle Stufen; die Hütte
//! steht nur in der Stufe der Tiefe 1, das Bauwerk liegt als Platz `hubSite.id` in der Stufe der Tiefe 0 (`World::sites`).
use serde::{Deserialize, Serialize};

/// Wo der Einsiedler gerade ist (nur Anzeige: der Weg in den Hub kann nicht angegriffen werden).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HermitPhase {
    /// Er wartet an der Hütte, noch nicht bezahlt.
    #[default]
    #[serde(rename = "hut")]
    Hut,
    /// Er ist in den Hub gezogen und baut; `HermitState::progress` ist der Baufortschritt (0 bis 1).
    #[serde(rename = "walking")]
    Walking,
    /// Das Bauwerk steht.
    #[serde(rename = "built")]
    Built,
    /// Das Bauwerk wurde zerstört und wartet auf den Wiederaufbau (keine zweite Zahlung).
    #[serde(rename = "fallen")]
    Fallen,
}

/// Die Hütte des Einsiedlers (Stufe der Tiefe 1).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HermitHut {
    /// Ort der Hütte in Units.
    #[serde(rename = "x")]
    pub x: f64,
    /// Reichweite des Einwurfs in Units.
    #[serde(rename = "range")]
    pub range: f64,
}

/// Zustand des Einsiedlers einer Insel in einer Stufe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HermitState {
    /// Katalog-ID des Einsiedlers (`swordsmith`, `priestess`).
    #[serde(rename = "hermit")]
    pub hermit: String,
    /// Name des Bauwerks (`dojo`, `shrine`).
    #[serde(rename = "building")]
    pub building: String,
    /// Münzen an der Hütte.
    #[serde(rename = "price")]
    pub price: i64,
    /// Die Hütte; nur in der Stufe der Tiefe 1.
    #[serde(rename = "hut", skip_serializing_if = "Option::is_none")]
    pub hut: Option<HermitHut>,
    /// Phase des Einsiedlers.
    #[serde(rename = "phase")]
    pub phase: HermitPhase,
    /// Baufortschritt des Bauwerks (0 bis 1), solange `phase` `walking` ist.
    #[serde(rename = "progress")]
    pub progress: f64,
    /// Der Bonus wirkt gerade (Bauwerk steht, nicht zerstört).
    #[serde(rename = "bonus")]
    pub bonus: bool,
    /// Die Hütte nimmt Münzen an (nicht bezahlt, Hub-Stufe erreicht, nichts ausstehend); je Takt gesetzt, ohne JSON.
    #[serde(skip)]
    pub open: bool,
    /// Ein Monarch dieser Stufe hat in diesem Takt gezahlt; die Insel startet den Bau nach den Stufen, ohne JSON.
    #[serde(skip)]
    pub pending: bool,
    /// Schadensfaktor der Krieger und Schützen (1 = ohne Wirkung); je Takt gesetzt, ohne JSON.
    #[serde(skip)]
    pub damage_factor: f64,
    /// Heilanteil der Mauern bei Tagesanbruch (0 = ohne Wirkung); je Takt gesetzt, ohne JSON.
    #[serde(skip)]
    pub wall_heal: f64,
}
