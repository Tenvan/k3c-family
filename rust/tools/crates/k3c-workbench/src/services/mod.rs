//! Entwicklungs-Dienste (Port von `internal/services`): Konfiguration aus `services.json`, ein Controller, der je Dienst
//! serialisiert startet, prüft, überwacht und stoppt, und die Zustände für MCP und Oberfläche.

mod adopt;
mod config;
mod controller;
mod filewatch;
mod levels;
mod lifecycle;
mod options;
mod outlog;
mod process;
mod query;
mod reload;
mod runtime;
mod status;
mod system;

#[cfg(test)]
mod adopt_tests;
#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod controller_tests;
#[cfg(test)]
mod filewatch_tests;
#[cfg(test)]
mod game_tests;
#[cfg(test)]
mod outlog_tests;
#[cfg(test)]
mod reload_tests;
#[cfg(test)]
mod system_tests;
#[cfg(test)]
pub(crate) mod testutil;

pub use config::{HEALTH_PROCESS, Service, load, shift};
pub use controller::{Controller, Failure, Outcome};
pub use filewatch::Watch;
pub use levels::LevelCounts;
pub use options::{Cancel, Checker, Exit, KillPid, Listener, Options, OutFn, Sampler, Spawned, Starter, WaitError};
pub use status::{State, Status};
pub use system::Metrics;
