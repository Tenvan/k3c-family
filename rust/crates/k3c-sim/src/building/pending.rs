//! Platzhalter der Gruppe `building` für Funktionen späterer Sessions (Port-Leitfaden § 7). Die Session, die die Funktion
//! liefert (RU18.2, RU18.3), löscht den Platzhalter; RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18)

use crate::state::island_ref::IslandRef;
use crate::state::{Site, World, WorldValues};

/// Go: `levelOf` (hub_level.go, RU18.3): die Ausbau-Stufe des Platzes; der Platzhalter steht in `world/pending.rs`.
pub use crate::world::pending::level_of;

/// Go: `sitePayable` (economy.go, RU18.2): Nimmt der Platz gerade Münzen?
pub fn site_payable(_w: &World, _v: &WorldValues, _isl: Option<&IslandRef>, _s: &Site) -> bool {
    unimplemented!("RU18: site_payable")
}
