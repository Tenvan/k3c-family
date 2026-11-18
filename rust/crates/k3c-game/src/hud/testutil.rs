//! Hilfen der HUD-Tests (Go: `twoPlayers`, `keys1`, `pad` aus `overlay_test.go`; die Insel kommt mit RU10 aus
//! `sim.CreateIsland`, hier baut `two_players` die Welt von Hand).

use std::rc::Rc;

use super::Hud;
use crate::props::pending::{CombatSpot, Economy, SpotState};
use crate::props::simview::SimView;
use crate::view::frame::{Frame, Seat, Stage};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::settings::default_settings;
use k3c_data::biome::Biome;
use k3c_sim::state::{CycleInfo, Player, Site, World};

pub const KEYS1: DeviceID = DeviceID::new(DeviceKind::Keyboard, 0);
pub const PAD: DeviceID = DeviceID::new(DeviceKind::Gamepad, 0);

/// StubView liefert für jede Welt dieselbe, leere Wirtschaft (die Sim-Seite kommt mit RU15 und RU18).
pub struct StubView;

impl SimView for StubView {
    fn economy_of(&self, _w: &World) -> Economy {
        Economy::default()
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

/// hud ist ein HUD ohne Grafik (Go: `New(nil)`).
pub fn hud() -> Hud {
    Hud::new(None, Box::new(StubView))
}

/// two_players baut eine Insel mit zwei Monarchen und freien Skill-Punkten; Spieler 1 an der Tastatur, 2 am Pad.
pub fn two_players(points: i64) -> (World, Frame) {
    let w = World {
        biome: Biome {
            id: "forest".to_string(),
            name: "Wald".to_string(),
            primary_resource: "wood".to_string(),
            ..Default::default()
        },
        cycle: CycleInfo {
            phase: "day".to_string(),
            day: 1,
            seconds_left: 100.0,
            ..Default::default()
        },
        players: (0..2)
            .map(|i| Player {
                index: i,
                id: i + 1,
                x: 10.0 * i as f64,
                ..Default::default()
            })
            .collect(),
        skill_points: points,
        ..Default::default()
    };
    let seat = |player: i64, device: DeviceID| Seat {
        player,
        device,
        monarch: player,
        stage: 0,
        lost: false,
    };
    let mut f = Frame {
        stages: vec![Stage {
            index: 0,
            level: w.biome.id.clone(),
            ..Default::default()
        }],
        seats: vec![seat(0, KEYS1), seat(1, PAD)],
        settings: default_settings(),
        ..Default::default()
    };
    set_world(&mut f, &w);
    (w, f)
}

/// set_world zeigt dem Frame den aktuellen Stand der Welt (der Frame hält eine Kopie).
pub fn set_world(f: &mut Frame, w: &World) {
    f.stages[0].world = Some(Rc::new(w.clone()));
}
