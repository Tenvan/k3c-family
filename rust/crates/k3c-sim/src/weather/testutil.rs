//! Hilfen der Wetter-Tests (RU62.5): Daten mit Taifun in der Test-Welt `test`, eine Insel mit zwei Stufen (Tiefe 0 und 1).
use k3c_data::Data;

use crate::island::Island;
use crate::island::switch_testutil::{gate_island, test_islands};
use crate::island::testutil::test_data;

/// Die Daten mit dem Taifun in der Welt `test` (Rhythmus und Faktoren aus `data/weather.json`).
pub fn taifun_data() -> Data {
    let mut data = test_data();
    data.weather.worlds.insert("test".to_string(), "taifun".to_string());
    data
}

/// Eine Insel der Welt `test` (Stufe 0 Tiefe 0, Stufe 1 Tiefe 1) mit `n` Spielern.
pub fn weather_island(data: &Data, n: usize) -> Island {
    let (world, _) = test_islands(data);
    gate_island(data, &world, n)
}

/// Die erste Wetternacht der Insel (Nacht n = `cycle.day` n in der Phase night).
pub fn first_weather_night(isl: &Island) -> i64 {
    let plan = isl.stages[0].weather_cfg.as_ref().expect("Wetter-Plan");
    (1..400)
        .find(|&n| plan.schedule.is_weather_night(n))
        .expect("eine Wetternacht")
}

/// Stellt den Zyklus aller Stufen auf Tag `day` und Phase `phase` und rechnet die Wetter-Vorbereitung eines Takts.
pub fn at(isl: &mut Island, day: i64, phase: &str) {
    for w in &mut isl.stages {
        w.cycle.day = day;
        w.cycle.phase = phase.to_string();
    }
    isl.prepare_weather();
}
