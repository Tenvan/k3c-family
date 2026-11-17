//! Port von `native/k3c-game/fx/fx.go` (Paket fx): Effekte der Welt (Sprint NG6, B-799).
//!
//! RU17.1 liefert die rechnenden Teile (effects, looks, status, texts); den Inhalt von `fx.go` (Drawer, Listener) und
//! `paint`/`paintmarks` liefert RU17.2.

pub mod effects;
pub mod looks;
pub mod pending;
pub mod status;
pub mod texts;

#[cfg(test)]
mod effects_tests;
#[cfg(test)]
mod status_tests;
