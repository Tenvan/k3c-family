//! Befehle des Bereichs Gegner und Welle (RU56.9, B-1134): Welle, Nacht, Boss, Boss-Phase, Ereignis, Gegner und Truppen
//! erzeugen. Werte werden geprüft, dann laufen sie über dieselbe Dev-Aktion wie im F9-Menü (Gruppe Kampf) und im
//! Stressmodus (`spawn`, `spawnCastle`).

use k3c_devproto::DevCommand;

use super::Host;
use super::commands::{Outcome, run_dev};

/// Größte Zahl Figuren eines Befehls.
const MAX_SPAWN: i64 = 200;

/// dev führt die Kampf-Aktionen aus; `None`: nicht Sache dieses Bereichs.
pub fn dev(host: &mut dyn Host, player: i64, c: &DevCommand) -> Outcome {
    let bad = match c.action.as_str() {
        "boss" if c.boss.is_empty() => Some("Kein Boss gewählt".to_string()),
        "event" if c.event.is_empty() => Some("Kein Ereignis gewählt".to_string()),
        "spawn" | "spawnCastle" => check_spawn(c),
        "wave" | "night" | "boss" | "bossPhase" | "event" => None,
        _ => return None,
    };
    Some(match bad {
        Some(e) => Err(e),
        None => run_dev(host, player, c),
    })
}

fn check_spawn(c: &DevCommand) -> Option<String> {
    if c.kind != "enemy" && c.kind != "troop" {
        return Some(format!("Art „{}“ gibt es nicht (enemy oder troop)", c.kind));
    }
    if c.unit.is_empty() {
        return Some("Keine Sorte gewählt".to_string());
    }
    if !(1..=MAX_SPAWN).contains(&c.amount) {
        return Some(format!("Anzahl {} nicht erlaubt (1 bis {MAX_SPAWN})", c.amount));
    }
    None
}

/// ui: dieser Bereich hat keine Tasten.
pub fn ui(_host: &mut dyn Host, _key: &str, _value: Option<i64>) -> Outcome {
    None
}

#[cfg(test)]
#[path = "foes_tests.rs"]
mod foes_tests;
