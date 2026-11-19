//! Wetter je Welt (RU62.5, B-969, docs/rules/wetter.md; Entscheidung 013: neue Mechanik nur in Rust, ohne Go-Orakel). Eine
//! Welt hat genau eine Art oder keine (`data/weather.json › worlds`, `standard` hat keine). Der Kern berechnet je Insel
//! die Wetternächte aus Insel-Seed und Tag (`schedule.rs`, `k3c-rng`, nichts im Spielstand), meldet Vorboten am Vortag und
//! die Wirkung von Nachtbeginn bis Tagesanbruch (`World::weather`); der Taifun (`taifun.rs`) bremst alle und verkürzt die
//! Fernkampf-Reichweite, nur an der Oberfläche. Die Flut (`flood.rs`, RU62.6) sperrt die Inselränder für Bodeneinheiten.
//!
//! - `mod.rs`: Plan der Insel, Einschalten, Takt-Vorbereitung.
//! - `schedule.rs`: Wetternächte und Phase.
//! - `taifun.rs`: Zustand und Faktoren je Stufe.
//! - `flood.rs`: gesperrte Randzonen, Heraus-Laufen, Portal-Haltung.
use std::sync::Arc;

use k3c_data::Data;

use crate::island::Island;
use crate::state::weather_view::{WeatherKind, WeatherPhase, WeatherState};

pub mod flood;
pub mod schedule;
pub mod taifun;

#[cfg(test)]
mod flood_tests;
#[cfg(test)]
mod schedule_tests;
#[cfg(test)]
mod taifun_tests;
#[cfg(test)]
mod testutil;

pub use schedule::Schedule;

/// Der Wetter-Plan einer Insel (alle Stufen teilen ihn; nichts davon wird gespeichert).
#[derive(Debug, Clone, PartialEq)]
pub struct WeatherPlan {
    /// Die Art der Welt.
    pub kind: WeatherKind,
    /// Zeitplan aus dem Insel-Seed.
    pub schedule: Schedule,
    /// Taifun: Tempo-Faktor (sonst 1).
    pub speed_factor: f64,
    /// Taifun: Reichweiten-Faktor (sonst 1).
    pub range_factor: f64,
    /// Flut: Breite der Randzone je Rand in Units (sonst 0).
    pub flood_units: f64,
    /// Flut: Tempo des Herauslaufens in Units je Sekunde (sonst 0).
    pub run_units: f64,
}

impl WeatherPlan {
    /// Art und Phase zur Zyklus-Phase am Tag `day`; `None`, wenn nichts ansteht.
    pub fn state(&self, day: i64, cycle_phase: &str) -> Option<WeatherState> {
        self.schedule
            .phase(day, cycle_phase)
            .map(|phase: WeatherPhase| WeatherState { kind: self.kind, phase })
    }
}

/// Die Wetterart der Welt `world_id` (`weather.json › worlds`); `None` = kein Wetter.
pub fn world_kind(data: &Data, world_id: &str) -> Option<WeatherKind> {
    match data.weather.kind_of(world_id) {
        Some("taifun") => Some(WeatherKind::Taifun),
        Some("flut") => Some(WeatherKind::Flut),
        _ => None, // `k3c-data` lässt nur Arten des Katalogs zu
    }
}

/// Baut den Plan der Insel mit dem Seed `seed`; `None`, wenn die Welt kein Wetter hat. Event-Nächte (Vollmond, Blutmond)
/// kommen aus `events.json`.
pub fn plan_of(data: &Data, world_id: &str, seed: &str) -> Option<WeatherPlan> {
    let kind = world_kind(data, world_id)?;
    let moons: Vec<i64> = data
        .events
        .events
        .iter()
        .map(|e| e.every_nights)
        .filter(|&n| n > 0)
        .collect();
    let (every, first, speed_factor, range_factor, flood_units, run_units) = match kind {
        WeatherKind::Taifun => {
            let t = &data.weather.taifun;
            (t.every_nights, t.first_night, t.speed_factor, t.range_factor, 0.0, 0.0)
        }
        WeatherKind::Flut => {
            let f = &data.weather.flut;
            (
                f.every_nights,
                f.first_night,
                1.0,
                1.0,
                f.flood_units as f64,
                f.run_units,
            )
        }
    };
    Some(WeatherPlan {
        kind,
        schedule: Schedule::new(seed, every, first, moons),
        speed_factor,
        range_factor,
        flood_units,
        run_units,
    })
}

impl Island {
    /// Schaltet das Wetter der Insel ein (nur Welten aus `weather.json`; Aufruf beim Bauen der Insel). Jede Insel rechnet
    /// ihren Plan aus ihrem Seed, alle Stufen teilen ihn.
    pub fn enable_weather(&mut self, data: &Data) {
        let Some(plan) = plan_of(data, self.world_id(), self.island_id()) else {
            return;
        };
        let plan = Arc::new(plan);
        for w in &mut self.stages {
            w.weather_cfg = Some(Arc::clone(&plan));
        }
    }

    /// Je Takt vor den Stufen: Zustand und Faktoren jeder Stufe aus ihrem Zyklus. Ohne Wetter-Plan geschieht nichts.
    pub fn prepare_weather(&mut self) {
        for w in &mut self.stages {
            taifun::update(w);
            flood::update(w);
        }
    }
}
