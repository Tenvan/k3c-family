//! Hilfen der Fest-Tests (RU62.12): eine Test-Welt `test` mit Fest, eine Insel mit zwei Stufen (Tiefe 0 und 1), Endboss lebt.
use k3c_data::Data;
use k3c_data::hub::Stock;

use crate::enemies::spawn_enemy;
use crate::island::switch_testutil::test_islands;
use crate::island::testutil::{DT, ISLAND_SPEED, test_data};
use crate::island::{CreateOptions, Island, WorldDef, add_island_player, create_island_in, step_island};
use crate::state::{Event, PlayerCommand};

/// Die Daten mit der Test-Welt `test` unter den Welten mit Fest (Effekt `lanterns`).
pub fn festival_data() -> Data {
    let mut data = test_data();
    data.festival.worlds.insert("test".to_string(), "lanterns".to_string());
    data
}

/// Eine Insel der Welt `test` (Stufe 0 Tiefe 0, Stufe 1 Tiefe 1), Endboss lebt, leerer Vorrat; `n` Spieler in Stufe 0.
pub fn festival_island(data: &Data, n: usize) -> (WorldDef, Island) {
    let (world, _) = test_islands(data);
    let mut isl =
        create_island_in(data, &world, "fest", &[0, 1], ISLAND_SPEED, CreateOptions::default()).expect("Insel");
    isl.stock = Stock::default();
    for _ in 0..n {
        add_island_player(&mut isl, 0);
    }
    (world, isl)
}

/// Ein Takt ohne Eingaben; liefert die Ereignisse aller Stufen.
pub fn tick(isl: &mut Island, data: &Data) -> Vec<Event> {
    let cmds = vec![PlayerCommand::default(); isl.next_player as usize];
    step_island(isl, &cmds, &data.difficulty, DT);
    isl.stages.iter().flat_map(|w| w.events.clone()).collect()
}

/// Rechnet `seconds` Sekunden und sammelt die Ereignisse.
pub fn run(isl: &mut Island, data: &Data, seconds: f64) -> Vec<Event> {
    let mut out = Vec::new();
    for _ in 0..(seconds / DT).ceil() as usize {
        out.extend(tick(isl, data));
    }
    out
}

/// Zahl der Ereignisse der Art `typ`.
pub fn count(events: &[Event], typ: &str) -> usize {
    events
        .iter()
        .filter(|e| e.get("type").and_then(|t| t.as_str()) == Some(typ))
        .count()
}

/// Gegner in allen Stufen der Insel.
pub fn enemies_total(isl: &Island) -> usize {
    isl.stages.iter().map(|w| w.enemies.len()).sum()
}

/// Gegner und Warteschlange in allen Stufen der Insel.
pub fn pending_total(isl: &Island) -> usize {
    isl.stages.iter().map(|w| w.enemies.len() + w.spawn_queue.len()).sum()
}

/// Stellt einen Gegner der Art `kind` in Stufe `stage` bei `x` (ohne Insel-Bezug, wie ein Dev-Gegner); liefert seinen Index.
pub fn put_enemy(isl: &mut Island, stage: usize, kind: &str, x: f64) -> usize {
    spawn_enemy(&mut isl.stages[stage], kind, x, None)
}

/// Dauer eines Spieltags in Sekunden der Insel (Zykluslänge durch die Zyklus-Geschwindigkeit der Stufe 0).
pub fn day_seconds(isl: &Island) -> f64 {
    crate::island::switch_testutil::cycle_length() / isl.stages[0].cycle_speed
}
