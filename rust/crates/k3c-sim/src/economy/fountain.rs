//! Gold-Fontäne und Gold aus Bäumen und Gegnern (Port von `engine/sim/economy_fountain.go`, docs/rules/wirtschaft.md § 9,
//! B-952, B-953):
//!   - Eine Truhe wirft ihr Gold als Münzen im Bogen auf den Boden (`fountain_coins`): jede Münze startet gestaffelt
//!     (`launch_at`) an der Truhe, fliegt zu einem Zielpunkt (±`spread_units`) und liegt ab `lands_at` aufsammelbar; Zeiten
//!     und Ziele kommen aus dem rng, deterministisch.
//!   - Ein gefällter Baum wirft `treeGold` Münzen, das Gold der Gegner skaliert `enemyGoldFactor` (`scaled_gold`).

use crate::state::links::values;
use crate::state::{Coin, World, WorldValues};

/// Go: `fountainCoins`: lässt `count` Münzen aus der Truhe bei `x` als Fontäne auf den Boden fallen.
pub fn fountain_coins(w: &mut World, v: &WorldValues, x: f64, count: i64) {
    let f = &v.economy.chest_fountain;
    for i in 0..count {
        let to = (x + (w.rng.next() * 2.0 - 1.0) * f.spread_units)
            .max(0.0)
            .min(w.width_units);
        let launch = w.time + i as f64 * f.stagger_seconds;
        let flight = f.flight_seconds * (0.8 + 0.4 * w.rng.next());
        let id = w.new_id();
        w.coins.push(Coin {
            id,
            x: to,
            from_x: x,
            launch_at: launch,
            lands_at: launch + flight,
            ..Coin::default()
        });
    }
}

/// Go: `scaledGold` skaliert das Gold eines Gegners mit `economy.enemyGoldFactor` (gerundet; Faktor 0 = unverändert).
pub fn scaled_gold(w: &World, gold: i64) -> i64 {
    let f = values(w).economy.enemy_gold_factor;
    if f <= 0.0 {
        return gold;
    }
    (gold as f64 * f).round() as i64
}
