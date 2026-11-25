//! Hilfen für die Tests des Crates: kleine Welten ohne Lauf.

use k3c_sim::island::Island;
use k3c_sim::state::{CycleInfo, Player, Site, Troop, World};

/// Eine leere Stufe am Tag 3 mit der Burg bei x = 100.
pub fn world() -> World {
    World {
        hub_x: 100.0,
        cycle: CycleInfo {
            phase: "day".into(),
            day: 3,
            ..CycleInfo::default()
        },
        ..World::default()
    }
}

/// Eine leere Insel.
pub fn island() -> Island {
    Island::default()
}

/// Ein Spieler mit Index, Ort und Gold.
pub fn player(index: i64, x: f64, gold: i64) -> Player {
    Player {
        index,
        x,
        gold,
        ..Player::default()
    }
}

/// Ein Bauplatz.
#[allow(dead_code)]
pub fn site(kind: &str, x: f64, state: &str) -> Site {
    Site {
        kind: kind.into(),
        x,
        state: state.into(),
        ..Site::default()
    }
}

/// Ein Landstreicher.
pub fn vagrant(x: f64) -> Troop {
    Troop {
        kind: "vagrant".into(),
        x,
        ..Troop::default()
    }
}
