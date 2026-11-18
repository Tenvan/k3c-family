//! Platzhalter der Gruppe `economy` für Funktionen späterer Sessions (Port-Leitfaden § 7). Die Session, die die Funktion
//! liefert (RU18.3 bis RU18.5, RU19), löscht den Platzhalter; RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18, RU19)

use super::PayTarget;
use crate::state::World;

pub use crate::merchant::{merchant_at, pay_merchant, refund_merchant}; // Von RU18.4 portiert (merchant.go)

/// Go: `coinGiveEvent` (events.go, RU19): meldet eine Münze, die ein Ziel bezahlt.
pub fn coin_give_event(_w: &mut World, _pi: usize, _t: &PayTarget) {
    unimplemented!("RU19: coin_give_event")
}

pub use crate::monarch::{chest_skill_point, step_attack}; // Von RU18.4 portiert (monarch.go)

pub use crate::monarch::{mount_of, speed_of}; // Von RU18.5 portiert (monarch_passives.go, monarch_mount.go)
pub use crate::skills::step_skills; // Von RU18.5 portiert (skills.go)

pub use crate::hub::{hub_gate, pay_upgrade, step_upgrade}; // Von RU18.3 portiert (hub_level.go)
