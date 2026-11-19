//! Befehle des Bereichs Spieler und Skills (RU56.8, B-1134): Skills lernen und zurücksetzen laufen über dieselben Wege
//! wie das Skill-Menü (`Session::learn`, `Session::respec`). Skillpunkte gibt es als Dev-Aktion nicht in `k3c-sim`:
//! das Spiel antwortet `ok:false` (Ticket), nichts wird an der Sim vorbei erzwungen. Die Skill-Kennung steht im Feld
//! `unit` des Befehls.

use k3c_devproto::DevCommand;

use super::Host;
use super::commands::Outcome;

/// dev führt die Aktionen `learn`, `respec` und `skillPoints` aus.
pub fn dev(host: &mut dyn Host, player: i64, c: &DevCommand) -> Outcome {
    Some(match c.action.as_str() {
        "learn" if c.unit.is_empty() => Err("Kein Skill gewählt".to_string()),
        "learn" => host.learn(player, &c.unit),
        "respec" => host.respec(player),
        "skillPoints" => Err("Skillpunkte: die Simulation hat dafür keine Dev-Aktion (Ticket)".to_string()),
        _ => return None,
    })
}

/// ui: dieser Bereich hat keine Tasten.
pub fn ui(_host: &mut dyn Host, _key: &str, _value: Option<i64>) -> Outcome {
    None
}

#[cfg(test)]
#[path = "players_tests.rs"]
mod players_tests;
