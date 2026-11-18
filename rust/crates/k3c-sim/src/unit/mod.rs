//! Truppen (Port der `engine/sim/unit_*.go`). RU11.2 liefert die Bogenschützen; die übrigen Gruppen folgen.

pub mod archer;

#[cfg(test)]
mod archer_tests;
// RU11.3: pub mod common; pub mod escort; pub mod flee; pub mod hammer;
// RU11.4: pub mod professions; pub mod warrior;
