//! `NeutralSimView` (RU39.1): die Sicht der Bauten und des HUD auf abgeleitete Sim-Werte für die Wiedergabe, ohne
//! Absturz. `LiveSimView` ruft `unimplemented!`, `FixedSimView` (Standbild) bricht bei unbekannten Fragen ab; die
//! Wiedergabe braucht für jeden Takt eine Antwort. Sie liefert neutrale Werte und ruft keine Sim-Funktion.
//! `site_visible` meldet jeden Platz sichtbar: Die Grundregel (`site_view::site_visible`) braucht `level_site` (RU18,
//! `unimplemented!`); Go blendet unbezahlte Plätze ab Hub-Stufe + 2 aus, die Wiedergabe zeichnet also etwas mehr.
// RU15/RU18: ersetzen durch die Sim (`economy_of`, `site_*`, `shield_rack`, `combat_spots`, `at_lair`, `spot_*`).

use k3c_sim::state::{Player, Site, World};

use crate::props::pending::{CombatSpot, Economy, SpotState};
use crate::props::simview::SimView;

/// Neutrale Antworten: keine Wirtschaft, nichts bezahlbar, keine Zahlziele, kein Schildständer, kein Hort.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeutralSimView;

impl SimView for NeutralSimView {
    fn economy_of(&self, _w: &World) -> Economy {
        Economy::default()
    }
    fn site_visible(&self, _w: &World, _s: &Site) -> bool {
        true
    }
    fn site_gold_left(&self, _w: &World, _s: &Site) -> (i64, bool) {
        (0, false)
    }
    fn site_payable(&self, _w: &World, _s: &Site) -> bool {
        false
    }
    fn site_needs_clearing(&self, _w: &World, _s: &Site) -> bool {
        false
    }
    fn shield_rack(&self, _w: &World) -> (f64, i64, bool) {
        (0.0, 0, false)
    }
    fn combat_spots(&self, _w: &World) -> Vec<CombatSpot> {
        Vec::new()
    }
    fn at_lair(&self, _w: &World, _p: &Player) -> bool {
        false
    }
    fn spot_near(&self, _w: &World, _s: &CombatSpot, _p: &Player) -> bool {
        false
    }
    fn spot_state(&self, _w: &World, _s: &CombatSpot, _p: &Player) -> SpotState {
        SpotState::default()
    }
}
