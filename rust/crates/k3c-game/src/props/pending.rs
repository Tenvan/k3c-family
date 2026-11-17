//! Platzhalter für Sim-Typen und Hilfen, die spätere Sprints liefern (Leitfaden § 7).
//!
//! `Economy`, `HubUpgrade` und `CombatSpot` heißen und serialisieren wie die Go-Typen (`engine/sim/economy_view.go`,
//! `combat.go`); RU15 (`CombatSpot`) und RU18 (`Economy`, `HubUpgrade`) ersetzen sie durch die Typen der Sim und
//! löschen sie hier. `nature_look`, `COIN_LOOK` und `COMBAT_ASSETS` sind Go-Werte aus `nature.go` (RU16.2) und
//! `plan_combat.go` (RU16.4), die `levelAssets` braucht; die Sessions löschen die Kopien hier beim Portieren.

use k3c_data::hub::Stock;
use serde::{Deserialize, Serialize};

use super::buildings::Look;

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

/// `PendingSimView` meldet bei jedem Aufruf, dass die Sim-Funktion noch fehlt (siehe `SimView`).
#[derive(Debug, Clone, Copy, Default)]
pub struct PendingSimView;

/// `nature_look` ist der Look des Bioms: 1 Wald, 2 Höhle, 3 Mine und alle übrigen; leere ID = Wald (Go: `natureLook`,
/// RU16.2 liefert `nature.rs`).
pub fn nature_look(biome: &str) -> i64 {
    match biome {
        "" | "forest" => 1,
        "cave" => 2,
        _ => 3,
    }
}

/// Animation aller Natur- und Objekt-Assets (Go: `idle`, RU16.2).
pub const IDLE: &str = "idle";

/// `COIN_LOOK` ist die Münze (Go: `coinLook` in nature.go, RU16.2).
pub const COIN_LOOK: Look = Look::new("icons/muenze", IDLE);

/// `COMBAT_ASSETS` sind die Icons der Kampfregeln, die props in jedem Level anmeldet (Go: `combatAssets` in
/// plan_combat.go, RU16.4: Krone, Schild, Auslöser, Banner, Gestell).
pub const COMBAT_ASSETS: [&str; 5] = [
    "icons/icon-krone",
    "icons/zahlstelle-schild",
    "objekte/horn-pfahl",
    "icons/zahlstelle-banner",
    "objekte/schild-gestell",
];
