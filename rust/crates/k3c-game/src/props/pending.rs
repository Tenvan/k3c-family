//! Platzhalter für Sim-Typen und Hilfen, die spätere Sprints liefern (Leitfaden § 7).
//!
//! `Economy`, `HubUpgrade` und `CombatSpot` heißen und serialisieren wie die Go-Typen (`engine/sim/economy_view.go`,
//! `combat.go`); RU15 (`CombatSpot`) und RU18 (`Economy`, `HubUpgrade`) ersetzen sie durch die Typen der Sim und
//! löschen sie hier. `COMBAT_ASSETS` ist ein Go-Wert aus `plan_combat.go` (RU16.4), den `levelAssets` braucht; die Session löscht die Kopie hier beim Portieren.

use k3c_data::hub::Stock;
use k3c_sim::state::World;
use serde::{Deserialize, Serialize};

use super::listen::StageInfo;
use super::plan::Planner;
use super::sites::WaitReason;
use crate::view::cell::Camera;

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

/// `COMBAT_ASSETS` sind die Icons der Kampfregeln, die props in jedem Level anmeldet (Go: `combatAssets` in
/// plan_combat.go, RU16.4: Krone, Schild, Auslöser, Banner, Gestell).
pub const COMBAT_ASSETS: [&str; 5] = [
    "icons/icon-krone",
    "icons/zahlstelle-schild",
    "objekte/horn-pfahl",
    "icons/zahlstelle-banner",
    "objekte/schild-gestell",
];

/// `portal_down`: Das Portal an x ist zerstört (`World::portals_down`), sein Bild verschwindet. Go: `portalDown` in
/// plan_combat.go; RU16.4 verschiebt die Funktion nach `plan_combat.rs` und löscht sie hier.
pub fn portal_down(w: &World, x: f64) -> bool {
    w.portals_down.iter().any(|d| (d - x).abs() < 0.01)
}

/// `fell_shrink`: vertikaler Maßstab eines Baums beim Fällen. Go: `fellShrink` in plan_objects.go; RU16.4 verschiebt die
/// Funktion nach `plan_objects.rs` und löscht sie hier.
pub fn fell_shrink(progress: f64) -> f64 {
    1.0 - progress * 0.3
}

/// Planer-Methoden aus RU16.4 (`plan_sites`, `plan_objects`, `plan_combat`) und RU16.5 (`decor`); die Sessions löschen
/// die Platzhalter beim Portieren (Leitfaden § 7).
#[allow(clippy::unused_self)]
impl Planner {
    /// RU16.5: `decor` (decor.go).
    pub fn decor(&mut self, _w: &World, _k: &Camera, _clock: f64) {
        unimplemented!("RU16.5: decor")
    }

    /// RU16.4: `backWalls` (plan_sites.go).
    pub fn back_walls(&mut self, _w: &World, _k: &Camera) {
        unimplemented!("RU16.4: back_walls")
    }

    /// RU16.4: `sites` (plan_sites.go).
    pub fn sites(&mut self, _w: &World, _info: Option<&StageInfo>, _k: &Camera) {
        unimplemented!("RU16.4: sites")
    }

    /// RU16.4: `plantation` (plan_objects.go).
    pub fn plantation(&mut self, _w: &World, _k: &Camera) {
        unimplemented!("RU16.4: plantation")
    }

    /// RU16.4: `pickups` (plan_objects.go).
    pub fn pickups(&mut self, _w: &World, _info: Option<&StageInfo>, _k: &Camera, _clock: f64) {
        unimplemented!("RU16.4: pickups")
    }

    /// RU16.4: `coins` (plan_objects.go).
    pub fn coins(&mut self, _w: &World, _k: &Camera) {
        unimplemented!("RU16.4: coins")
    }

    /// RU16.4: `combat` (plan_combat.go).
    pub fn combat(&mut self, _w: &World, _info: Option<&StageInfo>, _k: &Camera) {
        unimplemented!("RU16.4: combat")
    }

    /// RU16.4: `price` (plan_sites.go): Preisschild mit Münz-Plätzen.
    pub fn price(&mut self, _x: f64, _y: f64, _total: i64, _paid: i64, _text: &str, _enough: bool) {
        unimplemented!("RU16.4: price")
    }

    /// RU16.4: `status` (plan_sites.go): Wartegrund-Schild.
    pub fn status(&mut self, _x: f64, _y: f64, _r: WaitReason, _text: &str) {
        unimplemented!("RU16.4: status")
    }
}
