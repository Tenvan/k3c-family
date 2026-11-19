//! Befehle des Bereichs Anzeige (RU56.10, B-1134): Debug-Overlay, Leistungsanzeige, Fluss-Fassung, Tastenübersicht,
//! Vollbild und Screenshot laufen über dieselben Befehle wie die F-Tasten (`DevCmd`). Mit Wert wählt `debug` die Seite
//! (0 aus, 1 Geometrie, 2 Werte) und `stats` den Zustand; die Tasten schalten nur weiter, darum schaltet der Host so
//! oft, bis der Zustand stimmt.

use k3c_devproto::DevCommand;

use super::Host;
use super::commands::Outcome;
use crate::view::devkeys::DevCmd;

/// dev: dieser Bereich hat keine Dev-Aktionen.
pub fn dev(_host: &mut dyn Host, _player: i64, _c: &DevCommand) -> Outcome {
    None
}

/// ui führt die Tasten der Anzeige aus.
pub fn ui(host: &mut dyn Host, key: &str, value: Option<i64>) -> Outcome {
    Some(match key {
        "debug" => debug(host, value),
        "stats" => stats(host, value),
        "river" => host.dev_cmd(DevCmd::River),
        "help" => host.dev_cmd(DevCmd::Help),
        "fullscreen" => host.dev_cmd(DevCmd::Fullscreen),
        "shot" => host.dev_cmd(DevCmd::Shot),
        _ => return None,
    })
}

fn debug(host: &mut dyn Host, value: Option<i64>) -> Result<(), String> {
    let Some(page) = value else {
        return host.dev_cmd(DevCmd::Debug);
    };
    if !(0..=2).contains(&page) {
        return Err(format!("Debug-Seite {page} gibt es nicht (0 bis 2)"));
    }
    // höchstens eine Runde der drei Seiten
    for _ in 0..3 {
        if host.state().debug_page == page {
            return Ok(());
        }
        host.dev_cmd(DevCmd::Debug)?;
    }
    Ok(())
}

fn stats(host: &mut dyn Host, value: Option<i64>) -> Result<(), String> {
    if value.is_some_and(|v| (v != 0) == host.state().perf) {
        return Ok(());
    }
    host.dev_cmd(DevCmd::Stats)
}

#[cfg(test)]
#[path = "view_tests.rs"]
mod view_tests;
