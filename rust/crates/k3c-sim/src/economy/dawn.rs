//! Morgengold (Port von `engine/sim/economy_dawn.go`).

use super::sites::give_gold;
use crate::crown::crowned;
use crate::state::island_ref::IslandRef;
use crate::state::{World, WorldValues};

/// Go: `dawnGold` ist das Morgengold je Monarch der Stufe: Grundeinkommen plus Zuschlag je Tiefe (tiefere Stufen müssen
/// nach jeder Welle wieder aufbauen und liefern selbst keine Münzen, B-645).
pub fn dawn_gold(w: &World, v: &WorldValues) -> i64 {
    let e = &v.economy;
    e.dawn_gold_per_player + e.dawn_gold_per_depth * w.biome.depth
}

/// Go: `payDawnIncome`: morgens Steuern für jeden lebenden Monarchen.
pub fn pay_dawn_income(w: &mut World, v: &WorldValues, isl: Option<&IslandRef>) {
    for pi in 0..w.players.len() {
        if crowned(&w.players[pi]) {
            let (gold, x) = (dawn_gold(w, v), w.players[pi].x);
            give_gold(w, isl, pi, gold, x);
        }
    }
}
