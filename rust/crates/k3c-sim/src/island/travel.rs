//! Reise einzelner Spieler zwischen Stufen (Port von `engine/sim/island_travel.go`). Rumpf aus RU10.1: nur der Typ, die
//! Funktionen (`step_island_travel`, `Island::players` …) kommen mit RU10.4.

/// Go: `islandTravel`, der Reisefortschritt eines Spielers an einem Punkt.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IslandTravel {
    pub x: f64,
    pub progress: f64,
}
