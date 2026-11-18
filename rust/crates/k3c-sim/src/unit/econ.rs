//! Kleine Wirtschaftshilfen der Einheiten (Go: `economy.go`). RU18 verschiebt sie an ihren Ort (`economy`), solange
//! rechnen die Berufe und Krieger von RU11 damit.

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;

use crate::state::Player;

/// Go: `nearest`: Index des nächsten Elements in Reichweite `r` von `x`, für das `ok` gilt; bei Gleichstand das frühere.
pub fn nearest<T>(items: &[T], pos: impl Fn(&T) -> f64, x: f64, r: f64, ok: impl Fn(&T) -> bool) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (i, it) in items.iter().enumerate() {
        let d = (pos(it) - x).abs();
        if d > r || !ok(it) {
            continue;
        }
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some((i, d));
        }
    }
    best.map(|(i, _)| i)
}

/// Go: `clearPending`: verwirft die angefangene Zahlung des Spielers.
pub fn clear_pending(p: &mut Player) {
    p.pay_key = None;
    p.pay_amount = 0;
}

/// Go: `canAfford`: reicht der Vorrat für den Preis (ohne Gold)?
pub fn can_afford(s: &Stock, c: &Cost) -> bool {
    s.wood >= c.wood && s.stone >= c.stone && s.copper >= c.copper && s.iron >= c.iron && s.crystal >= c.crystal
}

/// Go: `spend`: zieht den Preis (ohne Gold) vom Vorrat ab.
pub fn spend(s: &mut Stock, c: &Cost) {
    s.wood -= c.wood;
    s.stone -= c.stone;
    s.copper -= c.copper;
    s.iron -= c.iron;
    s.crystal -= c.crystal;
}
