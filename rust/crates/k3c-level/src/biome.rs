//! Biome: Laden, `checkPortals`, `checkNature` und `Cycle::check` liegen seit RU3 in `k3c_data::biome`
//! (Port von `engine/level/biome.go`). Hier steht die Wiedergabe unter den Go-Namen.

pub use k3c_data::biome::{Biome, BiomePortals, Cycle, Range, Trait, WeightGroup, load as load_biome};
pub use k3c_data::ordered::{Entry, Ordered};

#[cfg(test)]
#[path = "biome_portals_tests.rs"]
mod biome_portals_tests;
#[cfg(test)]
#[path = "cycle_tests.rs"]
mod cycle_tests;
