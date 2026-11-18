//! Events im Wertesatz der Welt (Port von `engine/sim/events_values.go`, docs/rules/welten.md § 1.1, B-982): Die Stufen einer
//! Insel rechnen mit `WorldValues::events` statt mit der eingebetteten Basis. `NightEvent`, `loadEvents`, `checkNightEvents` und
//! `findEvent` stehen in `k3c_data::events` (RU3).

use k3c_data::events::{NightEvent, find_event};

use crate::state::World;
use crate::state::pending::values;

/// Go: `World.night`: das Event `id` aus dem Wertesatz der Stufe (`None`, wenn es fehlt); eine Kopie, weil der Wertesatz
/// ein `Arc` ist.
pub fn night(w: &World, id: &str) -> Option<NightEvent> {
    find_event(&values(w).events, id).cloned()
}
