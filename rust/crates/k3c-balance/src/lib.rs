//! Balancing und Testläufe: Port von engine/balance.

pub mod bot_abstieg;
pub mod bot_boss;
pub mod bot_clear;
pub mod bot_learn;
pub mod bot_material;
pub mod bot_wirtschaft;
pub mod bots;
pub mod crown_bot;
pub mod fehler;
pub mod helps;
pub mod hints;
pub mod island_world;
pub mod parallel;
pub mod pending;
pub mod prices;
pub mod progress;
pub mod replay;
pub mod run;
pub mod scope;
pub mod seedmode;
pub mod sha256;
#[cfg(test)]
mod testutil;

#[cfg(test)]
mod balance_tests;
#[cfg(test)]
mod bot_abstieg_tests;
#[cfg(test)]
mod bot_boss_tests;
#[cfg(test)]
mod bot_clear_tests;
#[cfg(test)]
mod bot_learn_tests;
#[cfg(test)]
mod bot_perf_tests;
#[cfg(test)]
mod bots_tests;
#[cfg(test)]
mod crown_bot_tests;
#[cfg(test)]
mod helps_tests;
#[cfg(test)]
mod hints_tests;
#[cfg(test)]
mod island_world_tests;
#[cfg(test)]
mod mix_tests;
#[cfg(test)]
mod parallel_tests;
#[cfg(test)]
mod profiles_tests;
#[cfg(test)]
mod progress_tests;
#[cfg(test)]
mod replay_tests;
#[cfg(test)]
mod seedmode_tests;
#[cfg(test)]
mod sha256_tests;

pub use fehler::Fehler;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_baut() {}
}
