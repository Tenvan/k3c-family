//! Test-Hilfen der Ereignisse (Go: `moonIsland`, `setCycle`, `cycleSeconds` aus `events_moon_test.go`).

use k3c_data::Data;

use crate::island::testutil::{must_island, test_data};
use crate::island::{Island, add_island_player};
use crate::state::island_ref::IslandEvents;
use crate::state::{CycleInfo, World};
use crate::world::base::base;
use crate::world::cycle::{cycle_at, cycle_phases};

/// Eine Stufe mit Insel-Spiegel (Stufe 0, keine Dev-Nächte), ohne `create_island`.
pub fn island_world() -> World {
    World {
        island_events: Some(IslandEvents {
            first_stage: true,
            ..Default::default()
        }),
        ..World::default()
    }
}

/// Stellt die Stufe auf die Nacht `n` (Phase night, Tag n).
pub fn night_of(w: &mut World, n: i64) {
    w.cycle = CycleInfo {
        phase: "night".into(),
        day: n,
        ..Default::default()
    };
}

/// Go: `cycleSeconds`: Beginn der Nacht `n` und Länge eines Zyklus in Zyklus-Sekunden.
pub fn cycle_seconds(n: i64) -> (f64, f64) {
    let dn = &base().expect("eingebettete Daten").day_night;
    let (phases, length) = cycle_phases(dn);
    ((n - 1) as f64 * length + phases[2].start, length)
}

/// Go: `setCycle`: stellt alle Stufen auf Nacht `n` plus `offset` Sekunden Spielzeit (negativ = davor).
pub fn set_cycle(isl: &mut Island, n: i64, offset: f64) {
    let (start, _) = cycle_seconds(n);
    let dn = &base().expect("eingebettete Daten").day_night;
    for w in &mut isl.stages {
        w.time = start / w.cycle_speed + offset;
        w.cycle = cycle_at(dn, w.time * w.cycle_speed);
    }
}

/// Go: `moonIsland`: eine Insel (Oberwelt und Höhle) mit zwei Spielern, kurz vor Einbruch der Nacht `n`.
pub fn moon_island(n: i64) -> (Data, Island) {
    let data = test_data();
    let mut isl = must_island(&data, "mond", &[0, 1]);
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 0);
    set_cycle(&mut isl, n, -0.5);
    (data, isl)
}
