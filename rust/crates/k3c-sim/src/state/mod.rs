//! Zustand der Simulation (Port von `engine/sim/types.go` und Verwandten).
//!
//! Modell: `docs/standards/rust-sim-zustand.md`. Die Welt besitzt alles, Verweise sind IDs.

pub mod action_view;
pub mod cargo_view;
pub mod common;
pub mod coop_view;
pub mod data;
pub mod fehler;
pub mod festival_view;
pub mod hermit_view;
pub mod island_ref;
pub mod links;
pub mod mount_view;
pub mod node_mark;
pub mod omit;
pub mod purse;
pub mod save;
pub mod save_apply;
pub mod save_economy;
pub mod shared;
pub mod site_view;
pub mod torches;
pub mod types;
pub mod types_enemy;
pub mod types_things;
pub mod types_world;
pub mod weather_view;

pub use fehler::Fehler;
pub use island_ref::{IslandRef, IslandView};
pub use save::{HubSave, PlayerSave, SaveGame, SiteSave, TroopSave};
pub use save_economy::{CraftSave, PlotSave};
pub use shared::{
    ActiveEvent, CombatRuntime, CombatSettings, Crown, CycleInfo, ItemDrop, Merchant, SkillTable, SwitchState,
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
mod save_economy_tests;
#[cfg(test)]
mod save_tests;
#[cfg(test)]
mod types_tests;
#[cfg(test)]
mod views_tests;
