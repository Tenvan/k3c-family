//! Balancing und Testläufe: Port von engine/balance.

pub mod bot_abstieg;
pub mod bot_clear;
pub mod bot_material;
pub mod bot_wirtschaft;
pub mod bots;
pub mod fehler;
pub mod pending;
pub mod prices;
#[cfg(test)]
mod testutil;

#[cfg(test)]
mod bot_abstieg_tests;
#[cfg(test)]
mod bot_clear_tests;
#[cfg(test)]
mod bots_tests;

pub use fehler::Fehler;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_baut() {}
}
