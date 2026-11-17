//! Determinismus-Hilfen: Gleitkomma-Funktionen bitgleich zu Go (amd64) und Golden-Vergleich.
//!
//! Rust-Bordmittel (`powf`, `exp`, `ln`, `f64::max`, `f64::min`) sind in den Sim-Crates verboten
//! (Leitfaden § 5); dafür gibt es hier `pow`, `exp`, `log`, `fmax`, `fmin`.

mod bits;
mod dim;
mod exp;
pub mod golden;
mod log;
mod pow;

pub use dim::{fmax, fmin};
pub use exp::exp;
pub use log::log;
pub use pow::pow;

#[cfg(test)]
mod vectors_tests;
