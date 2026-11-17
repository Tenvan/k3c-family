//! Port von `native/k3c-game/props/` (Paketkopf `props.go`): Auswahl-Regeln für Gebäude und Objekte. RU16.1 liefert
//! `assets`, `data`, `buildings`, `coin_fountain` und die Naht `simview`; der Drawer (`New`, `Draw`, `Assets`) folgt in RU16.5.

pub mod assets;
pub mod buildings;
pub mod coin_fountain;
pub mod data;
pub mod pending;
pub mod simview;

#[cfg(test)]
mod assets_tests;
#[cfg(test)]
mod buildings_tests;
#[cfg(test)]
mod coin_fountain_tests;
#[cfg(test)]
mod testutil;
