//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion.
#![allow(clippy::unimplemented, unused_variables)]

use k3c_data::buildings::Cost;
use k3c_data::hub::Stock;
use k3c_sim::island::Island;
use k3c_sim::state::{Player, PlayerCommand, ResourceNode, Site, World};

use crate::bots::Bot;

/// So viele Bauern wirbt `wirtschaft` höchstens an (Go: `peasantLimit`, bot_wirtschaft.go).
pub const PEASANT_LIMIT: i64 = 4;

/// Go: `wirtschaft` (bot_wirtschaft.go, RU25.2).
pub fn wirtschaft(isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    unimplemented!("RU25.2: wirtschaft")
}

/// Go: `hubStep` (bot_wirtschaft.go, RU25.2): Ort zum Zahlen für den Hub-Ausbau oder die Rücklage.
pub fn hub_step(isl: &Island, w: &World, p: &Player) -> (Option<f64>, i64) {
    unimplemented!("RU25.2: hub_step")
}

/// Go: `nearestPayable` (bot_wirtschaft.go, RU25.2).
pub fn nearest_payable<'a>(w: &'a World, p: &Player, kind: &str, state: &str, reserve: i64) -> Option<&'a Site> {
    unimplemented!("RU25.2: nearest_payable")
}

/// Go: `peasants` (bot_wirtschaft.go, RU25.2).
pub fn peasants(w: &World) -> i64 {
    unimplemented!("RU25.2: peasants")
}

/// Go: `covers` (bot_wirtschaft.go, RU25.2): deckt der Vorrat die Materialkosten?
pub fn covers(stock: &Stock, cost: &Cost) -> bool {
    unimplemented!("RU25.2: covers")
}

/// Go: `clearNode` (bot_clear.go, RU25.2): der nächste Baum oder Fels, der einen Mauer- oder Turm-Platz sperrt.
pub fn clear_node<'a>(w: &'a World, p: &Player) -> Option<&'a ResourceNode> {
    unimplemented!("RU25.2: clear_node")
}

/// Go: `wantSet` (bot_clear.go, RU25.2).
pub struct WantSet<'a> {
    pub w: &'a World,
}

impl WantSet<'_> {
    /// Go: `wantSet.wanted`.
    pub fn wanted(&self, s: &Site) -> bool {
        unimplemented!("RU25.2: WantSet::wanted")
    }
}

/// Go: `withDescent` (bot_abstieg.go, RU25.2): der Abstieg-Dekorator um `inner`.
pub fn descent_decide(inner: &Bot, deepest: i64, isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    unimplemented!("RU25.2: descent_decide")
}

/// Go: `withBossFight` (bot_boss.go, RU25.3): der Boss-Dekorator um `inner`.
pub fn boss_fight_decide(inner: &Bot, deepest: i64, isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    unimplemented!("RU25.3: boss_fight_decide")
}
