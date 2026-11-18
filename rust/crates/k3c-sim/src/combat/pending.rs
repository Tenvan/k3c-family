//! Platzhalter der Gruppe `combat` (Port-Leitfaden § 7): Funktionen aus `combat_squad.go` (RU15.3), die `combat/mod.rs` und
//! `combat/spots.rs` rufen. RU15.3 löscht diese Datei; RU22 prüft, dass keiner übrig ist. Go-Zeiger auf Truppen und Spieler
//! sind hier Indizes (`ti`, `pi`).
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf RU15.3 (combat_squad)

use crate::state::World;

/// Go: `freeWarriors` (combat_squad.go): die Indizes der freien Soldaten (ohne Gruppe).
pub fn free_warriors(_w: &World) -> Vec<usize> {
    unimplemented!("RU15: free_warriors (RU15.3)")
}

/// Go: `leadersOf` (combat_squad.go): die Indizes der Gruppenführer der Seite.
pub fn leaders_of(_w: &World, _side: f64) -> Vec<usize> {
    unimplemented!("RU15: leaders_of (RU15.3)")
}

/// Go: `stepGroups` (combat_squad.go): Gruppen führen (Variante groups).
pub fn step_groups(_w: &mut World) {
    unimplemented!("RU15: step_groups (RU15.3)")
}

/// Go: `stepBanners` (combat_squad.go): Banner und Trupps führen (Variante general).
pub fn step_banners(_w: &mut World) {
    unimplemented!("RU15: step_banners (RU15.3)")
}

/// Go: `sendGroups` (combat_squad.go): schickt alle Gruppen der Seite los.
pub fn send_groups(_w: &mut World, _side: f64) {
    unimplemented!("RU15: send_groups (RU15.3)")
}

/// Go: `takeBanner` (combat_squad.go): Monarch `pi` nimmt das Banner an der Seite auf.
pub fn take_banner(_w: &mut World, _pi: usize, _side: i64) {
    unimplemented!("RU15: take_banner (RU15.3)")
}
