//! Platzhalter für Funktionen späterer Sprints (Port-Leitfaden § 7). Der Sprint, der die Funktion liefert, löscht den
//! Platzhalter; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Spieler und Truppen sind hier Indizes (`pi`, `ti`).

use std::sync::Arc;

use k3c_data::monarch::SkillEffect;

use super::shared::WorldValues;
use super::{Enemy, Player, World};

/// Go: `merchantID` (merchant.go), die ID des Händlers als Ziel.
pub const MERCHANT_ID: i64 = -1;

/// Go: `crowned` (crown.go).
pub fn crowned(_p: &Player) -> bool {
    unimplemented!("RU15: crowned")
}

/// Go: `dropCrown` (crown.go).
pub fn drop_crown(_w: &mut World, _pi: usize, _cause: &str) {
    unimplemented!("RU15: drop_crown")
}

/// Go: `hitCoins` (crown.go).
pub fn hit_coins(_w: &mut World, _pi: usize, _n: i64) {
    unimplemented!("RU15: hit_coins")
}

/// Go: `hitEvent` (events.go).
pub fn hit_event(_w: &mut World, _target: &str, _id: i64, _x: f64, _damage: f64) {
    unimplemented!("RU19: hit_event")
}

/// Go: `troopDamage` (monarch_passives.go).
pub fn troop_damage(_w: &World, _ti: usize, _damage: f64) -> f64 {
    unimplemented!("RU18: troop_damage")
}

/// Go: `eachPassive` (monarch_passives.go); `f` bekommt den Besitzer und den Effekt.
pub fn each_passive(_w: &World, _f: &mut dyn FnMut(&Player, &SkillEffect)) {
    unimplemented!("RU18: each_passive")
}

/// Go: `invulnerable` (enemies_traits.go).
pub fn invulnerable(_w: &World, _e: &Enemy) -> bool {
    unimplemented!("RU11: invulnerable")
}

/// Go: `isWaveEnemy` (enemies_guard.go).
pub fn is_wave_enemy(_e: &Enemy) -> bool {
    unimplemented!("RU11: is_wave_enemy")
}

/// Go: `damageMerchant` (merchant.go).
pub fn damage_merchant(_w: &mut World, _damage: f64) {
    unimplemented!("RU18: damage_merchant")
}

/// Go: `protectedNight` (World-Methode, Nacht-Schutz der Gebäude).
pub fn protected_night(_w: &World) -> bool {
    unimplemented!("RU18: protected_night")
}

/// Go: `isRuin` (building_ruin.go).
pub fn is_ruin(_w: &World, _kind: &str, _x: f64) -> bool {
    unimplemented!("RU18: is_ruin")
}

/// Go: `World.values` (world_values_room.go), der Wertesatz der Welt (Standard, wenn `vals` fehlt).
pub fn values(_w: &World) -> Arc<WorldValues> {
    unimplemented!("RU10: values")
}
