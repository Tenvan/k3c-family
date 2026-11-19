//! Hilfen der Koop-Tests (RU62.3): eine Test-Welt mit Koop-Regeln, eine Insel mit `n` Spielern und einer Truhe am Steg.
use k3c_data::Data;

use crate::island::switch_testutil::{gate_island, test_islands};
use crate::island::testutil::{DT, test_data};
use crate::island::{Island, step_island};
use crate::state::{Event, Pickup};

/// Die Daten mit der Test-Welt `test` unter den Koop-Welten; jede Truhe ist doppelt (`everyNth` 1), Faktor 3.
pub fn coop_data() -> Data {
    let mut data = test_data();
    data.coop.worlds.push("test".to_string());
    data.coop.double_chest.every_nth = 1;
    data.coop.double_chest.gold_factor = 3.0;
    data
}

/// Eine Insel der Welt `test` mit `n` gesteuerten Spielern am Steg (Stufe 0), ohne Truhen am Boden und mit lebendem
/// Endboss (kein Inselwechsel). Liefert Insel und Steg-X; eine Truhe legt `add_chest` an.
pub fn coop_island(data: &Data, n: usize) -> (Island, f64) {
    let (world, _) = test_islands(data);
    let mut isl = gate_island(data, &world, n);
    isl.endboss_defeated = false;
    for w in &mut isl.stages {
        w.pickups.clear();
    }
    let x = isl.stages[0].players.first().map_or(0.0, |p| p.x);
    (isl, x)
}

/// Legt eine Truhe in Stufe `stage` bei `x`; liefert ihre ID.
pub fn add_chest(isl: &mut Island, stage: usize, x: f64) -> i64 {
    let w = &mut isl.stages[stage];
    let id = w.new_id();
    w.pickups.push(Pickup {
        id,
        kind: "chest".to_string(),
        x,
    });
    id
}

/// Rechnet einen Takt und liefert die Ereignisse der Stufe 0.
pub fn tick(isl: &mut Island, data: &Data) -> Vec<Event> {
    step_island(isl, &[], &data.difficulty, DT);
    isl.stages[0].events.clone()
}

/// Das erste Truhen-Ereignis, falls eines kam.
pub fn chest_event(events: &[Event]) -> Option<&Event> {
    events
        .iter()
        .find(|e| e.get("type").and_then(|t| t.as_str()) == Some("chest"))
}

/// Das Gold einer Truhe aus dem Ereignis.
pub fn gold_of(ev: &Event) -> i64 {
    ev.get("gold").and_then(serde_json::Value::as_i64).unwrap_or(-1)
}
