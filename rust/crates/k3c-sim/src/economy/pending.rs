//! Platzhalter der Gruppe `economy` für Funktionen späterer Sessions (Port-Leitfaden § 7). Die Session, die die Funktion
//! liefert (RU18.3 bis RU18.5, RU19), löscht den Platzhalter; RU22 prüft, dass keiner übrig ist.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU18, RU19)

use k3c_data::monarch::MountData;

use super::PayTarget;
use crate::state::island_ref::IslandRef;
use crate::state::{PlayerCommand, World, WorldValues};

/// Go: `merchantAt` (merchant.go, RU18.4): "buy" oder "sell", wenn `x` in Zahl-Reichweite eines Zahlziels des anwesenden
/// Händlers liegt.
pub fn merchant_at(_w: &World, _v: &WorldValues, _x: f64) -> Option<&'static str> {
    unimplemented!("RU18: merchant_at (RU18.4)")
}

/// Go: `payMerchant` (merchant.go, RU18.4): ein Takt A halten am Händler (statt `pay_one_coin`).
pub fn pay_merchant(_w: &mut World, _v: &WorldValues, _isl: Option<&IslandRef>, _pi: usize, _side: &str) {
    unimplemented!("RU18: pay_merchant (RU18.4)")
}

/// Go: `refundMerchant` (merchant.go, RU18.4): zieht bis zu `amount` Münzen aus dem laufenden Kauf zurück.
pub fn refund_merchant(_w: &mut World, _amount: i64) -> i64 {
    unimplemented!("RU18: refund_merchant (RU18.4)")
}

/// Go: `coinGiveEvent` (events.go, RU19): meldet eine Münze, die ein Ziel bezahlt.
pub fn coin_give_event(_w: &mut World, _pi: usize, _t: &PayTarget) {
    unimplemented!("RU19: coin_give_event")
}

/// Go: `stepAttack` (monarch.go, RU18.4): Schlag (Taste X) des lebenden Monarchen `pi`.
pub fn step_attack(_w: &mut World, _v: &WorldValues, _pi: usize, _cmd: &PlayerCommand, _dt: f64) {
    unimplemented!("RU18: step_attack (RU18.4)")
}

/// Go: `chestSkillPoint` (monarch.go, RU18.4): zählt eine geöffnete Truhe der Insel; jede n-te gibt einen Skill-Punkt.
pub fn chest_skill_point(_w: &mut World, _v: &WorldValues, _isl: Option<&mut IslandRef>, _pi: usize) {
    unimplemented!("RU18: chest_skill_point (RU18.4)")
}

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
