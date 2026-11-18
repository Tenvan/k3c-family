//! Bau-Regeln der Gebäude (Port von `engine/sim/building_*.go`): Kaserne, Heilplatz, Vorposten, Plantage, Reparatur,
//! Ruinen, Zaubertum, Taverne, Elite und Rüstung. Go hat keine `building.go`; dieses Modul sammelt nur die Dateien.

pub mod barracks;
pub mod healer;
pub mod outpost;
pub mod plantation;
pub mod repair;
pub mod ruin;
pub mod spell_tower;
pub mod tavern;
pub mod upgrades;

#[cfg(test)]
mod testutil;

#[cfg(test)]
mod barracks_tests;
#[cfg(test)]
mod healer_tests;
#[cfg(test)]
mod outpost_tests;
#[cfg(test)]
mod plantation_tests;
#[cfg(test)]
mod repair_tests;
#[cfg(test)]
mod ruin_tests;
#[cfg(test)]
mod spell_tower_tests;
#[cfg(test)]
mod tavern_tests;
#[cfg(test)]
mod upgrades_craft_tests;
#[cfg(test)]
mod upgrades_sites_tests;
#[cfg(test)]
mod upgrades_tests;
