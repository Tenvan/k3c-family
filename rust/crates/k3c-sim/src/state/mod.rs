//! Zustand der Simulation (Port von `engine/sim/types.go` und Verwandten).
//!
//! Modell: `docs/standards/rust-sim-zustand.md`. Die Welt besitzt alles, Verweise sind IDs.

pub mod action_view;
pub mod common;
pub mod data;
pub mod node_mark;
pub mod omit;
pub mod pending;
pub mod purse;
pub mod save;
pub mod shared;
pub mod site_view;
pub mod torches;
pub mod types;
pub mod types_enemy;
pub mod types_things;
pub mod types_world;

pub use shared::{
    ActiveEvent, CombatRuntime, CombatSettings, Crown, CycleInfo, Drop, Merchant, SkillTable, SwitchState,
    SwitchTarget, WarnZone, WorldValues,
};
pub use types::{Coin, Job, Player, PlayerCommand, Troop};
pub use types_enemy::{Camp, Enemy, Event, Hazard, Pickup, Projectile, QueuedSpawn, Storm, Travel};
pub use types_things::{Castle, Plot, ResourceNode, Site};
pub use types_world::World;

#[cfg(test)]
mod common_tests;
#[cfg(test)]
mod data_tests;
#[cfg(test)]
mod types_tests;
#[cfg(test)]
mod views_tests;
