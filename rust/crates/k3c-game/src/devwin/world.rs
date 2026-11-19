//! Befehle des Bereichs Welt und Zeit (RU56.6, B-1134): Tageszeit, Zeitraffer, Grad (Dev-Aktionen wie im F9-Menü)
//! sowie Pause, Einzelschritt, Zeitraffer-Taste, Speichern und Neu laden (die F-Tasten F5 bis F8). Wechsel von Welt,
//! Insel und Stufe (`goto`) beantwortet der `Host` selbst.

use k3c_devproto::DevCommand;

use super::Host;
use super::commands::{Outcome, run_dev};
use crate::view::devkeys::{DevCmd, TIMESCALES};

const PHASES: [&str; 4] = ["day", "dusk", "night", "dawn"];
const GRADES: [&str; 4] = ["easy", "normal", "hard", "ultra"];

/// dev führt die Aktionen Tageszeit, Zeitraffer und Grad aus (Werte geprüft, dann über die Cheat-Pfade).
pub fn dev(host: &mut dyn Host, player: i64, c: &DevCommand) -> Outcome {
    let bad = match c.action.as_str() {
        "phase" if !PHASES.contains(&c.phase.as_str()) => Some(format!("Unbekannte Tageszeit „{}“", c.phase)),
        "grade" if !GRADES.contains(&c.grade.as_str()) => Some(format!("Unbekannter Grad „{}“", c.grade)),
        "timescale" if !TIMESCALES.contains(&c.factor) => Some(format!("Zeitraffer {}× gibt es nicht", c.factor)),
        "phase" | "grade" | "timescale" | "nextPhase" | "timescaleAuto" => None,
        _ => return None,
    };
    Some(match bad {
        Some(e) => Err(e),
        None => run_dev(host, player, c),
    })
}

/// ui führt die Tasten Pause, Einzelschritt, Zeitraffer, Speichern und Neu laden aus.
pub fn ui(host: &mut dyn Host, key: &str, value: Option<i64>) -> Outcome {
    Some(match key {
        "pause" => pause(host, value),
        "step" => step(host),
        "speed" => speed(host, value),
        "save" => host.dev_cmd(DevCmd::Save),
        "load" => match host.dev_session() {
            Some(ds) => ds.reload(),
            None => Err(no_session()),
        },
        _ => return None,
    })
}

fn no_session() -> String {
    "Keine Entwickler-Rolle (kein laufendes Spiel)".to_string()
}

/// Pause: mit Wert 1 anhalten, mit 0 weiterlaufen, ohne Wert umschalten (F7 hält an, der Halt ist unabhängig von
/// der Pause des Menüs).
fn pause(host: &mut dyn Host, value: Option<i64>) -> Result<(), String> {
    let ds = host.dev_session().ok_or_else(no_session)?;
    let on = value.map_or(!ds.halted(), |v| v != 0);
    ds.halt(on);
    Ok(())
}

/// Einzelschritt: nur im Halt (wie F7 in der Pause).
fn step(host: &mut dyn Host) -> Result<(), String> {
    let ds = host.dev_session().ok_or_else(no_session)?;
    if !ds.halted() {
        return Err("Einzelschritt geht nur in der Pause".to_string());
    }
    ds.step_once();
    Ok(())
}

/// Zeitraffer-Taste: mit Wert diese Stufe, sonst eine Stufe weiter wie F8.
fn speed(host: &mut dyn Host, value: Option<i64>) -> Result<(), String> {
    let Some(f) = value else {
        return host.dev_cmd(DevCmd::Speed);
    };
    let c = DevCommand {
        action: "timescale".to_string(),
        factor: f,
        ..DevCommand::default()
    };
    dev(host, 0, &c).unwrap_or(Ok(()))
}

#[cfg(test)]
#[path = "world_tests.rs"]
mod world_tests;
