//! Hilfen der props-Tests: Welten und Plätze als Literal (die Go-Tests bauen `&sim.World{…}` ebenso) und eine
//! `FixedSimView` mit festen Werten für die Tests der Zeichner (RU16.2 bis RU16.5, hud).

use k3c_sim::state::{Player, Site, World};

use super::pending::{CombatSpot, Economy};
use super::simview::SimView;

/// `wall_at` ist eine Mauer an x (Go: `wallAt` in buildings_test.go).
pub fn wall_at(id: i64, x: f64, level: i64, state: &str) -> Site {
    Site {
        id,
        kind: "wall".to_string(),
        x,
        level,
        state: state.to_string(),
        ..Default::default()
    }
}

/// `FixedSimView` liefert in jedem Aufruf dieselben, vom Test gesetzten Werte.
#[derive(Debug, Clone, Default)]
pub struct FixedSimView {
    pub economy: Economy,
    pub gold_left: (i64, bool),
    pub payable: bool,
    pub needs_clearing: bool,
    pub shield_rack: (f64, i64, bool),
    pub combat_spots: Vec<CombatSpot>,
    pub at_lair: bool,
}

impl SimView for FixedSimView {
    fn economy_of(&self, _w: &World) -> Economy {
        self.economy.clone()
    }
    fn site_gold_left(&self, _w: &World, _s: &Site) -> (i64, bool) {
        self.gold_left
    }
    fn site_payable(&self, _w: &World, _s: &Site) -> bool {
        self.payable
    }
    fn site_needs_clearing(&self, _w: &World, _s: &Site) -> bool {
        self.needs_clearing
    }
    fn shield_rack(&self, _w: &World) -> (f64, i64, bool) {
        self.shield_rack
    }
    fn combat_spots(&self, _w: &World) -> Vec<CombatSpot> {
        self.combat_spots.clone()
    }
    fn at_lair(&self, _w: &World, _p: &Player) -> bool {
        self.at_lair
    }
}

#[test]
fn fixed_sim_view_liefert_feste_werte() {
    let v = FixedSimView {
        payable: true,
        shield_rack: (3.0, 2, true),
        ..Default::default()
    };
    let w = World::default();
    assert!(v.site_payable(&w, &Site::default()));
    assert_eq!(v.shield_rack(&w), (3.0, 2, true));
    assert!(!v.at_lair(&w, &Player::default()));
}
