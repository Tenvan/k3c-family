//! Test-Hilfen der Dev-Aktionen (Go: `coinsAt`, `mustIsland` mit Spieler).

use k3c_data::Data;

use crate::island::testutil::{must_island, test_data};
use crate::island::{Island, add_island_player};
use crate::state::World;

/// Eine Insel mit einer Stufe (Tiefe 0) und einem Spieler; liefert Daten, Insel und den Spielerindex.
pub fn island_with_player(seed: &str) -> (Data, Island, i64) {
    let data = test_data();
    let mut isl = must_island(&data, seed, &[0]);
    let pi = add_island_player(&mut isl, 0);
    let index = isl.stages[0].players[pi].index;
    (data, isl, index)
}

/// Go: `coinsAt`: zählt die Münzen der Welt genau bei `x`.
pub fn coins_at(w: &World, x: f64) -> usize {
    w.coins.iter().filter(|c| c.x == x).count()
}
