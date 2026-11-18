//! Port von `native/k3c-game/local/` (RU27): die Sitzung des nativen Spiels über `k3c-room`, synchron getaktet.
//! Teil 1 (RU27.1): Session-Gerüst, Einstellungen, Welten, Route. Teil 2 (RU27.2): Takt (`step`, `peer`), Kampagne.
//! Teil 3 (RU50.1): `saves`, `actions`, `dev` (Spielstände, Aktionen der Spieler, Entwickler-Rolle).

pub mod actions;
pub mod campaign;
pub mod config;
pub mod dev;
pub mod error;
pub mod peer;
pub mod route;
pub mod saves;
pub mod session;
pub mod settings;
pub mod step;
pub mod view_impl;
pub mod watched;
pub mod worlds;

#[cfg(test)]
mod actions_tests;
#[cfg(test)]
mod campaign_tests;
#[cfg(test)]
mod dev_tests;
#[cfg(test)]
mod load_tests;
#[cfg(test)]
mod outcome_tests;
#[cfg(test)]
mod route_tests;
#[cfg(test)]
mod saves_tests;
#[cfg(test)]
mod session_tests;
#[cfg(test)]
mod settings_controls_tests;
#[cfg(test)]
mod settings_stress_tests;
#[cfg(test)]
mod settings_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod world_tests;
