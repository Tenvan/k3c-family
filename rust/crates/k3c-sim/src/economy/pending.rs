//! Platzhalter der Gruppe `economy` für Funktionen späterer Sessions (Port-Leitfaden § 7). Die Session, die die Funktion
//! liefert (RU18.3 bis RU18.5, RU19), löscht den Platzhalter; RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18, RU19)

use k3c_data::monarch::MountData;

use super::PayTarget;
use crate::state::{PlayerCommand, World, WorldValues};

pub use crate::merchant::{merchant_at, pay_merchant, refund_merchant}; // Von RU18.4 portiert (merchant.go)

/// Go: `coinGiveEvent` (events.go, RU19): meldet eine Münze, die ein Ziel bezahlt.
pub fn coin_give_event(_w: &mut World, _pi: usize, _t: &PayTarget) {
    unimplemented!("RU19: coin_give_event")
}

pub use crate::monarch::{chest_skill_point, step_attack}; // Von RU18.4 portiert (monarch.go)

/// Go: `stepSkills` (skills.go, RU18.5): Skill-Slots des lebenden Monarchen `pi`.
pub fn step_skills(_w: &mut World, _v: &WorldValues, _pi: usize, _cmd: &PlayerCommand, _dt: f64) {
    unimplemented!("RU18: step_skills (RU18.5)")
}

/// Go: `speedOf` (monarch_passives.go, RU18.5): Grundtempo des Monarchen `pi`.
pub fn speed_of(_w: &World, _v: &WorldValues, _pi: usize) -> f64 {
    unimplemented!("RU18: speed_of (RU18.5)")
}

/// Go: `MountOf` (monarch_mount.go, RU18.5): das Reittier des Monarchen `pi`.
pub fn mount_of(_v: &WorldValues, _pi: usize) -> &MountData {
    unimplemented!("RU18: mount_of (RU18.5)")
}

pub use crate::hub::{hub_gate, pay_upgrade, step_upgrade}; // Von RU18.3 portiert (hub_level.go)
