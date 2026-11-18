//! Platzhalter der Gruppe `dev` für Funktionen späterer Sprints (Port-Leitfaden § 7). Der liefernde Sprint löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18, RU19.2)

use crate::state::World;

/// Go: `merchantArrive` (merchant.go): der Händler kommt in der Stufe an.
pub fn merchant_arrive(_w: &mut World) {
    unimplemented!("RU18: merchant_arrive")
}

/// Go: `raidMerchant` (events_merchant.go): der Besuch des Händlers wird überfallen.
pub fn raid_merchant(_w: &mut World) {
    unimplemented!("RU19: raid_merchant")
}
