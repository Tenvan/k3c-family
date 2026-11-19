//! Hilfen der Reittier-Tests (RU62.8): eine Test-Welt `test` mit Welt-Reittier Kamel, eine Insel mit `n` Spielern.
use k3c_data::Data;

use crate::island::switch_testutil::{gate_island, test_islands};
use crate::island::testutil::{DT, test_data};
use crate::island::{Island, WorldDef, step_island};
use crate::state::mount_view::MountGate;
use crate::state::{Event, PlayerCommand};
use crate::world::ship::harbor;

/// Die Daten mit der Test-Welt `test` unter den Welten mit Reittier (Kamel).
pub fn mounts_data() -> Data {
    let mut data = test_data();
    data.mounts.worlds.insert("test".to_string(), "camel".to_string());
    data
}

/// Eine Insel der Welt `test` mit `n` Spielern (je 100 Gold) am Steg, Endboss lebt (kein Inselwechsel). Liefert Welt, Insel
/// und Steg-X.
pub fn mounts_island(data: &Data, n: usize) -> (WorldDef, Island, f64) {
    let (world, _) = test_islands(data);
    let mut isl = gate_island(data, &world, n);
    isl.endboss_defeated = false;
    let (wi, si) = harbor(&isl).expect("Hafen");
    let steg = isl.stages[wi].sites[si].x;
    for w in &mut isl.stages {
        for p in &mut w.players {
            p.gold = 100;
        }
    }
    (world, isl, steg)
}

/// Das Gatter der Insel (Stufe 0).
pub fn gate(isl: &Island) -> MountGate {
    isl.stages[0].mount_gate.clone().expect("Gatter")
}

/// Stellt den Spieler `pi` der Stufe 0 an das Gatter.
pub fn stand_at_gate(isl: &mut Island, pi: usize) {
    let x = gate(isl).x;
    isl.stages[0].players[pi].x = x;
}

/// Ein Takt; die Spieler in `paying` halten die Bezahl-Taste. Liefert die Ereignisse der Stufe 0.
pub fn tick(isl: &mut Island, data: &Data, paying: &[usize]) -> Vec<Event> {
    let n = isl.stages[0].players.len();
    let cmds: Vec<PlayerCommand> = (0..n)
        .map(|i| PlayerCommand {
            pay: paying.contains(&i),
            ..PlayerCommand::default()
        })
        .collect();
    step_island(isl, &cmds, &data.difficulty, DT);
    isl.stages[0].events.clone()
}

/// Zahl der Ereignisse der Art `typ`.
pub fn count(events: &[Event], typ: &str) -> usize {
    events
        .iter()
        .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some(typ))
        .count()
}
