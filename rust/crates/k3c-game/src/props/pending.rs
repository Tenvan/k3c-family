//! Platzhalter für Sim-Typen und Hilfen, die spätere Sprints liefern (Leitfaden § 7).
//!
//! `Economy`, `HubUpgrade` und `CombatSpot` heißen und serialisieren wie die Go-Typen (`engine/sim/economy_view.go`,
//! `combat.go`); RU15 (`CombatSpot`) und RU18 (`Economy`, `HubUpgrade`) ersetzen sie durch die Typen der Sim und
//! löschen sie hier.

use k3c_data::hub::Stock;
use serde::{Deserialize, Serialize};

fn is_zero_i64(v: &i64) -> bool {
    *v == 0
}

fn is_false(v: &bool) -> bool {
    !*v
}

/// `Economy` ist der abgeleitete Wirtschaftsstand einer Stufe (Go: `sim.Economy`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Economy {
    /// Maximum je Rohstoff der Insel; fehlt ohne Insel (unbegrenzt).
    #[serde(rename = "stockMax", skip_serializing_if = "is_zero_i64")]
    pub stock_max: i64,
    /// Gold-Maximum je Spieler; fehlt ohne Dev-Bonus.
    #[serde(rename = "purseMax", skip_serializing_if = "is_zero_i64")]
    pub purse_max: i64,
    /// Hub-Stufe (1 bis Anzahl der `levels` in data/hub.json).
    #[serde(rename = "hubLevel")]
    pub hub_level: i64,
    /// Ausbauschritt, den `hub_upgrade` nennt (1 bis `HubUpgrade::steps`); fehlt mit `hub_upgrade`.
    #[serde(rename = "hubStep", skip_serializing_if = "is_zero_i64")]
    pub hub_step: i64,
    /// Ausbau auf die nächste Hub-Stufe; fehlt auf der letzten Stufe.
    #[serde(rename = "hubUpgrade", skip_serializing_if = "Option::is_none")]
    pub hub_upgrade: Option<HubUpgrade>,
    /// Nacht oder Gegner da, Bau und Ausbau warten.
    #[serde(rename = "danger", skip_serializing_if = "is_false")]
    pub danger: bool,
    /// Zahl der Kämpfer der Stufe (ohne omitempty).
    #[serde(rename = "fighters")]
    pub fighters: i64,
    /// Truppen-Limit der Stufe (ohne omitempty).
    #[serde(rename = "troopLimit")]
    pub troop_limit: i64,
}

/// `HubUpgrade` sind Kosten und Stand des laufenden Ausbauschritts auf die nächste Hub-Stufe (Go: `sim.HubUpgrade`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct HubUpgrade {
    /// Goldkosten des Schritts.
    #[serde(rename = "gold")]
    pub gold: i64,
    /// Materialkosten des Schritts samt Material-Tor.
    #[serde(rename = "material")]
    pub material: Stock,
    /// Schon bezahltes Gold.
    #[serde(rename = "paid")]
    pub paid: i64,
    /// waitingMaterial, waitingNextIsland, waitingWorker; fehlt, solange Gold offen ist.
    #[serde(rename = "state", skip_serializing_if = "String::is_empty")]
    pub state: String,
    /// Zahl der Ausbauschritte auf die nächste Hub-Stufe (B-592).
    #[serde(rename = "steps")]
    pub steps: i64,
}

/// `CombatSpot` ist ein Zahlziel der Kampfvariante: Kind `shield` (Burg), `trigger` (Auslöser) oder `banner`;
/// Side -1 links, 1 rechts, 0 an der Burg (Go: `sim.CombatSpot`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CombatSpot {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "side")]
    pub side: i64,
    #[serde(rename = "x")]
    pub x: f64,
}

/// `SpotState` ist der Zahlstand eines Zahlziels für einen Monarchen (Go: `sim.CombatSpotState`): Preis in Münzen,
/// bereits gezahlter Betrag, ob er hier gerade zahlen kann und ob er in Zahl-Reichweite plus einer Unit steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SpotState {
    pub price: i64,
    pub paid: i64,
    pub payable: bool,
    pub near: bool,
}

/// `PendingSimView` meldet bei jedem Aufruf, dass die Sim-Funktion noch fehlt (siehe `SimView`).
#[derive(Debug, Clone, Copy, Default)]
pub struct PendingSimView;
