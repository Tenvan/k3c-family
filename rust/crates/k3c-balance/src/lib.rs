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
pub mod parallel;
pub mod pending;
pub mod prices;
pub mod progress;
pub mod run;
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
mod bots_tests;
#[cfg(test)]
mod crown_bot_tests;
#[cfg(test)]
mod helps_tests;
#[cfg(test)]
mod hints_tests;
#[cfg(test)]
mod mix_tests;
#[cfg(test)]
mod parallel_tests;
#[cfg(test)]
mod progress_tests;

pub use fehler::Fehler;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_baut() {}
}
