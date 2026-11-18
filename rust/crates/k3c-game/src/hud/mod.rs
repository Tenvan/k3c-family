//! Port von `native/k3c-game/hud/` (RU20): HUD je Spielerhälfte. Diese Datei trägt bis RU20.4 nur die `mod`-Zeilen; den
//! Inhalt von `hud.go` liefert RU20.4.

pub mod data;
pub mod fonts;
pub mod glyphs;
pub mod layout;
pub mod texts;
mod texts_de;
mod texts_en;
pub mod values;
mod values_texts;

#[cfg(test)]
mod glyphs_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod texts_tests;
#[cfg(test)]
mod values_tests;
