//! Hilfen der Einsiedler-Tests (RU62.10): eine Test-Welt `test` mit Einsiedler, eine Insel mit zwei Stufen (Tiefe 0 und 1).
use k3c_data::Data;

use crate::island::switch_testutil::{gate_island, test_islands};
use crate::island::testutil::{DT, test_data};
use crate::island::{Island, WorldDef, add_island_player, step_island};
use crate::state::hermit_view::HermitHut;
use crate::state::{Event, PlayerCommand, Site};

/// Die Daten mit der Test-Welt `test` unter den Welten mit Einsiedler `id` (`swordsmith`, `priestess`).
pub fn hermit_data(id: &str) -> Data {
    let mut data = test_data();
    data.hermits.worlds.insert("test".to_string(), id.to_string());
    data
}

/// Eine Insel der Welt `test` (Stufe 0 Tiefe 0, Stufe 1 Tiefe 1) mit `n` Spielern in Stufe 0 (je 100 Gold), Hub-Stufe 2, Endboss
/// lebt (kein Inselwechsel). Liefert die Welt und die Insel.
pub fn hermit_island(data: &Data, n: usize) -> (WorldDef, Island) {
    let (world, _) = test_islands(data);
    let mut isl = gate_island(data, &world, n);
    isl.endboss_defeated = false;
    for w in &mut isl.stages {
        w.cycle_speed = 1.0; // Echtzeit: die Tests laufen am Tag, ohne Welle
    }
    isl.stages[0].hub_level = 2;
    for p in &mut isl.stages[0].players {
        p.gold = 100;
    }
    (world, isl)
}

/// Stellt einen neuen Monarchen (100 Gold) in Stufe 1 an die Hütte; liefert seinen inselweiten Index.
pub fn add_deep_player(isl: &mut Island) -> i64 {
    let x = hut(isl).x;
    let pi = add_island_player(isl, 1);
    let p = &mut isl.stages[1].players[pi];
    p.gold = 100;
    p.x = x;
    p.index
}

/// Die Hütte der Insel (Stufe 1).
pub fn hut(isl: &Island) -> HermitHut {
    isl.stages[1].hermit_hut().cloned().expect("Hütte")
}

/// Der Platz des Bauwerks (Stufe 0).
pub fn hall(isl: &Island) -> &Site {
    let (wi, si) = isl.hall_site().expect("Platz des Bauwerks");
    &isl.stages[wi].sites[si]
}

/// Ein Takt; die Spieler mit den inselweiten Indizes in `paying` halten die Bezahl-Taste. Liefert die Ereignisse aller
/// Stufen.
pub fn tick(isl: &mut Island, data: &Data, paying: &[i64]) -> Vec<Event> {
    let cmds: Vec<PlayerCommand> = (0..isl.next_player)
        .map(|i| PlayerCommand {
            pay: paying.contains(&i),
            ..PlayerCommand::default()
        })
        .collect();
    step_island(isl, &cmds, &data.difficulty, DT);
    isl.stages.iter().flat_map(|w| w.events.clone()).collect()
}

/// Rechnet `seconds` Sekunden ohne Eingaben und sammelt die Ereignisse.
pub fn run(isl: &mut Island, data: &Data, seconds: f64) -> Vec<Event> {
    let mut out = Vec::new();
    for _ in 0..(seconds / DT).ceil() as usize {
        out.extend(tick(isl, data, &[]));
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

/// Zahlt mit dem Monarchen `index` in Stufe 1 (Taste loslassen und drücken) und liefert die Ereignisse des Druck-Takts.
pub fn pay_at_hut(isl: &mut Island, data: &Data, index: i64) -> Vec<Event> {
    tick(isl, data, &[]);
    tick(isl, data, &[index])
}

/// Stellt die Zeit aller Stufen kurz vor den Tagesanbruch (der nächste Takt löst `dawn` aus).
pub fn before_dawn(isl: &mut Island) {
    let base = crate::world::base::base().expect("Basis");
    let (phases, _) = crate::world::cycle::cycle_phases(&base.day_night);
    for w in &mut isl.stages {
        w.cycle_speed = 1.0;
        w.time = phases[3].start - 0.02;
        w.cycle = crate::world::cycle::cycle_at(&base.day_night, w.time);
    }
}
