//! Hilfen der Fracht-Tests (RU62.1): eine Test-Welt mit Fracht, ein Hafen mit Grundschiff (Segel fertig), Spieler an den
//! Ladeplätzen.

use k3c_data::Data;
use k3c_data::hub::Stock;

use super::Bay;
use crate::island::switch_testutil::{gate_island, test_islands};
use crate::island::testutil::{DT, test_data};
use crate::island::{Island, WorldDef, step_island};
use crate::state::{Event, PlayerCommand, Troop};
use crate::world::ship::harbor;

/// Die Daten mit der Test-Welt `test` unter den Fracht-Welten.
pub fn cargo_data() -> Data {
    let mut data = test_data();
    data.cargo.worlds.push("test".to_string());
    data
}

/// Eine Fracht-Insel mit Grundschiff (Rumpf und Segel fertig, ohne Laderaum), `n` Spielern am Steg mit je 30 Gold, leerem
/// Vorrat und noch lebendem Endboss (kein Inselwechsel). Liefert Welt, Insel und Steg-X.
pub fn cargo_island(data: &Data, n: usize) -> (WorldDef, Island, f64) {
    let (world, _) = test_islands(data);
    let mut isl = gate_island(data, &world, n);
    isl.endboss_defeated = false;
    let (wi, si) = harbor(&isl).expect("Hafen");
    isl.stages[wi].sites[si].level = 3; // Grundschiff: Kiel, Rumpf, Segel
    let x = isl.stages[wi].sites[si].x;
    for p in &mut isl.stages[wi].players {
        p.gold = 30; // der Beutel fasst höchstens 30 (economy.json)
    }
    // keine Start-Kämpfer: die Tests stellen ihre Truppen selbst auf
    isl.stages[wi]
        .troops
        .retain(|t| t.kind == "peasant" || t.kind == "vagrant");
    isl.stock = Stock::default();
    isl.sync_stock();
    (world, isl, x)
}

/// Stellt den Spieler `pi` der Hafen-Stufe an den Ladeplatz `bay`.
pub fn stand_at(isl: &mut Island, steg: f64, pi: usize, bay: Bay) {
    let off = bay.offset(isl.cargo.cfg.as_ref().expect("Fracht"));
    let (wi, _) = harbor(isl).expect("Hafen");
    isl.stages[wi].players[pi].x = steg + off;
}

/// Befehle: die Spieler in `paying` halten die Bezahl-Taste, die übrigen nicht (Index = Spielerindex).
pub fn pay_cmds(n: usize, paying: &[usize]) -> Vec<PlayerCommand> {
    (0..n)
        .map(|i| PlayerCommand {
            pay: paying.contains(&i),
            ..PlayerCommand::default()
        })
        .collect()
}

/// Rechnet `seconds` Takte und sammelt die Ereignisse der Hafen-Stufe.
pub fn run(isl: &mut Island, data: &Data, cmds: &[PlayerCommand], seconds: f64) -> Vec<Event> {
    let mut out = Vec::new();
    let mut tm = 0.0;
    while tm < seconds {
        step_island(isl, cmds, &data.difficulty, DT);
        out.extend(isl.stages[0].events.iter().cloned());
        tm += DT;
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

/// Eine Truppe der Art `kind` bei `x` in die Hafen-Stufe stellen; liefert ihre ID.
pub fn add_troop(isl: &mut Island, kind: &str, x: f64) -> i64 {
    let w = &mut isl.stages[0];
    let id = w.new_id();
    w.troops.push(Troop {
        id,
        kind: kind.to_string(),
        x,
        hp: 10.0,
        max_hp: 10.0,
        anchor_x: x,
        target_x: x,
        ..Troop::default()
    });
    id
}
