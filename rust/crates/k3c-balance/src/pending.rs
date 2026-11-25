//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion.
#![allow(clippy::unimplemented, unused_variables)]

use k3c_sim::island::Island;
use k3c_sim::state::{Player, PlayerCommand, World};

use crate::bots::Bot;

/// So viele Kämpfer braucht der Bot, bevor er den Endboss am Bau weckt (Go: `endbossWant`, bot_boss.go, RU25.3 verschiebt
/// die Konstante nach `bot_boss.rs`; B-651: 8 Bögen starben in Minuten an dessen Beschwörung).
pub const ENDBOSS_WANT: i64 = 16;

/// Go: `withBossFight` (bot_boss.go, RU25.3): der Boss-Dekorator um `inner`.
pub fn boss_fight_decide(inner: &Bot, deepest: i64, isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    unimplemented!("RU25.3: boss_fight_decide")
}
