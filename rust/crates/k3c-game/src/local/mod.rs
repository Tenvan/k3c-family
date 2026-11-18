//! Port von `native/k3c-game/local/` (RU27): die Sitzung des nativen Spiels über `k3c-room`, synchron getaktet.
//! Teil 1 (RU27.1): Session-Gerüst, Einstellungen, Welten, Route. Teil 2 (RU27.2): Takt (`step`, `peer`), Kampagne.
//! `saves`, `actions`, `dev` folgen in RU27.3; bis dahin stehen ihre Platzhalter in `pending.rs`.

pub mod campaign;
pub mod config;
pub mod error;
pub mod peer;
pub mod pending;
pub mod route;
pub mod session;
pub mod settings;
pub mod step;
pub mod view_impl;
pub mod watched;
pub mod worlds;

#[cfg(test)]
mod campaign_tests;
#[cfg(test)]
mod load_tests;
#[cfg(test)]
mod outcome_tests;
#[cfg(test)]
mod route_tests;
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
