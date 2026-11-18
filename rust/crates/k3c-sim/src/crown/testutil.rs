//! Hilfen der Tests dieser Gruppe: `crownWorld` und `fell` aus `crown_test.go`, `probe` aus `crown_helpers_test.go`.
//! Hilfen, die `Step` (RU14) oder `applyDamage` (RU18) brauchen, fehlen; ihre Tests tragen `ignore`.

use std::sync::Arc;

use super::CROWN_GROUND;
use crate::boss::testutil::boss_values;
use crate::enemies::testutil::bare_world;
use crate::state::shared::Crown;
use crate::state::{Player, World, WorldValues};

/// Go: `probeGold`: Das Gold, mit dem Tests Treffer messen.
pub const PROBE_GOLD: i64 = 20;

/// Go: `crownWorld` ohne `quietWorld`: zwei Monarchen weit weg von der Burg (P0 bei Hub + 30, P1 bei Hub + 34) und ein
/// Portal bei Hub + 90.
pub fn crown_world() -> (World, Arc<WorldValues>) {
    let mut w = bare_world();
    let v = boss_values();
    w.vals = Some(v.clone());
    w.width_units = 400.0;
    for (index, dx) in [(0, 30.0), (1, 34.0)] {
        let id = w.new_id();
        w.players.push(Player {
            id,
            index,
            x: w.hub_x + dx,
            ..Player::default()
        });
    }
    w.portals = vec![w.hub_x + 90.0];
    (w, v)
}

/// Go: `probe`: gibt den Monarchen `PROBE_GOLD` Münzen.
pub fn probe(w: &mut World) {
    for p in &mut w.players {
        p.gold = PROBE_GOLD;
    }
}

/// Go: `fell`: lässt die Krone von `pi` am Boden liegen, ohne Flug.
pub fn fell(w: &mut World, pi: usize) {
    let p = &mut w.players[pi];
    p.crown = Some(Crown {
        owner: p.index,
        state: CROWN_GROUND.into(),
        x: p.x,
        ..Crown::default()
    });
}
