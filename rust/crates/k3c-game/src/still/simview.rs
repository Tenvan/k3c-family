//! `FixedSimView` (RU16.6): die Antworten der Go-Sim-Funktionen hinter `props::simview::SimView` aus einer Datei
//! (`props-<name>.simview.json`, geschrieben von `native/k3c-game/props/rustref_test.go`). Das ist die vereinbarte
//! Zwischenlösung (Orakel), bis RU15 und RU18 die Funktionen nach Rust bringen; sie werden hier nicht portiert.
//!
//! Plätze erkennt die Sicht an der ID, Zahlziele an Art und Seite, Monarchen am Index. Eine Frage, zu der die Datei
//! nichts sagt, ist ein Fehler im Test-Aufbau und bricht ab (`panic`), statt still etwas Falsches zu zeichnen.
#![allow(clippy::panic)] // Test-Fixture: fehlende Antwort in der Fixture bricht den Test ab

use std::path::Path;

use k3c_sim::state::{Player, Site, World};
use serde::Deserialize;

use crate::props::pending::{CombatSpot, Economy, SpotState};
use crate::props::simview::SimView;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct SiteAnswer {
    id: i64,
    visible: bool,
    #[serde(rename = "goldLeft")]
    gold_left: i64,
    #[serde(rename = "goldPayable")]
    gold_payable: bool,
    payable: bool,
    #[serde(rename = "needsClearing")]
    needs_clearing: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct RackAnswer {
    x: f64,
    stack: i64,
    visible: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct SpotAnswer {
    spot: usize,
    player: i64,
    price: i64,
    paid: i64,
    payable: bool,
    near: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
struct LairAnswer {
    player: i64,
    #[serde(rename = "atLair")]
    at_lair: bool,
}

/// Antworten der Sim-Funktionen an einem festen Zustand.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FixedSimView {
    economy: Economy,
    sites: Vec<SiteAnswer>,
    #[serde(rename = "shieldRack")]
    shield_rack: RackAnswer,
    #[serde(rename = "combatSpots")]
    combat_spots: Vec<CombatSpot>,
    #[serde(rename = "spotStates")]
    spot_states: Vec<SpotAnswer>,
    #[serde(rename = "atLair")]
    at_lair: Vec<LairAnswer>,
}

impl FixedSimView {
    /// Liest die Antworten aus JSON.
    pub fn parse(raw: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(raw).map_err(|e| format!("Sim-Antworten: {e}"))
    }

    /// Liest die Datei `path`.
    pub fn load(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::parse(&raw).map_err(|e| format!("{}: {e}", path.display()))
    }

    fn site(&self, s: &Site) -> &SiteAnswer {
        self.sites
            .iter()
            .find(|a| a.id == s.id)
            .unwrap_or_else(|| panic!("Sim-Antworten: Platz {} fehlt", s.id))
    }

    /// Antwort zum Zahlziel `s` für den Monarchen `p` (Index des Ziels in `combat_spots`).
    fn spot(&self, s: &CombatSpot, p: &Player) -> &SpotAnswer {
        let i = self
            .combat_spots
            .iter()
            .position(|o| o.kind == s.kind && o.side == s.side)
            .unwrap_or_else(|| panic!("Sim-Antworten: Zahlziel {} {} fehlt", s.kind, s.side));
        self.spot_states
            .iter()
            .find(|a| a.spot == i && a.player == p.index)
            .unwrap_or_else(|| panic!("Sim-Antworten: Zahlstand {} für Monarch {} fehlt", s.kind, p.index))
    }
}

impl SimView for FixedSimView {
    fn economy_of(&self, _w: &World) -> Economy {
        self.economy.clone()
    }
    fn site_visible(&self, _w: &World, s: &Site) -> bool {
        self.site(s).visible
    }
    fn site_gold_left(&self, _w: &World, s: &Site) -> (i64, bool) {
        let a = self.site(s);
        (a.gold_left, a.gold_payable)
    }
    fn site_payable(&self, _w: &World, s: &Site) -> bool {
        self.site(s).payable
    }
    fn site_needs_clearing(&self, _w: &World, s: &Site) -> bool {
        self.site(s).needs_clearing
    }
    fn shield_rack(&self, _w: &World) -> (f64, i64, bool) {
        (self.shield_rack.x, self.shield_rack.stack, self.shield_rack.visible)
    }
    fn combat_spots(&self, _w: &World) -> Vec<CombatSpot> {
        self.combat_spots.clone()
    }
    fn at_lair(&self, _w: &World, p: &Player) -> bool {
        self.at_lair.iter().any(|a| a.player == p.index && a.at_lair)
    }
    fn spot_near(&self, _w: &World, s: &CombatSpot, p: &Player) -> bool {
        self.spot(s, p).near
    }
    fn spot_state(&self, _w: &World, s: &CombatSpot, p: &Player) -> SpotState {
        let a = self.spot(s, p);
        SpotState {
            price: a.price,
            paid: a.paid,
            payable: a.payable,
            near: a.near,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antworten_lesen() {
        let v = FixedSimView::parse(
            br#"{"economy":{"hubLevel":2,"fighters":1,"troopLimit":3},"sites":[{"id":7,"visible":true,"goldLeft":4,"goldPayable":true}],
            "shieldRack":{"x":454,"stack":1,"visible":true},"combatSpots":[{"kind":"shield","side":0,"x":454}],
            "spotStates":[{"spot":0,"player":1,"price":10,"paid":2,"payable":true,"near":true}],"atLair":[{"player":0,"atLair":true}]}"#,
        )
        .unwrap();
        let w = World::default();
        assert_eq!(v.economy_of(&w).hub_level, 2);
        let s = Site {
            id: 7,
            ..Site::default()
        };
        assert_eq!(v.site_gold_left(&w, &s), (4, true));
        assert!(v.site_visible(&w, &s));
        assert_eq!(v.shield_rack(&w), (454.0, 1, true));
        let spot = v.combat_spots(&w).remove(0);
        let p = Player {
            index: 1,
            ..Player::default()
        };
        assert_eq!(v.spot_state(&w, &spot, &p).price, 10);
        assert!(v.spot_near(&w, &spot, &p));
        assert!(!v.at_lair(&w, &p));
        assert!(FixedSimView::parse(b"7").is_err());
    }
}
