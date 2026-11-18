//! Platzhalter für Teile, die spätere Sprints liefern (Leitfaden § 7). RU30.2 liefert `script.go` samt `ParseScript`,
//! tauscht die `pub use`-Zeile in `mod.rs` aus; RU30.1 liefert `Loader`; RU29.3 tauscht `devtools_part` samt Eintrag in
//! `WAITING`. Ist alles geliefert, wird diese Datei gelöscht. RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter nach Leitfaden § 7: wartet auf RU30

use super::parts::Part;
use super::scene::Hold;
use crate::art::Library;
use std::cell::RefCell;
use std::rc::Rc;

/// Go: `ParseScript` (`script.go`, RU30.2): liest die feste Eingabe-Folge „p@t=…,…“.
pub fn parse_script(_s: &str) -> Result<Vec<Hold>, String> {
    unimplemented!("RU30: parse_script")
}

/// Go: `Loader` (`levels.go`, RU30.1): lädt die Grafik der Level (`Parts::art`).
pub trait Loader {}

/// Fachmodule, die `packages()` noch nicht liefern kann: Name und liefernder Sprint. `parts_tests` zählt sie als
/// „wartet auf RUnn“ und prüft, dass der Platzhalter verschwindet, sobald der Modulordner existiert.
#[cfg_attr(not(test), allow(dead_code))]
pub const WAITING: &[(&str, &str)] = &[("devtools", "RU29")];

/// Go: `devtools.New` mit Chronik (Cheat-Menü, Testing, Sitzungsbericht; RU29): bis dahin ein leerer Beitrag mit dem
/// Namen des Moduls (ein `unimplemented!` ließe jeden Zusammenbau abstürzen). RU29.3 tauscht diese Funktion.
pub fn devtools_part(_lib: Option<Rc<RefCell<Library>>>) -> Part {
    Part::named("devtools")
}
