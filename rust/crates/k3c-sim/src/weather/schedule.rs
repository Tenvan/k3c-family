//! Zeitplan des Wetters (RU62.5, docs/rules/wetter.md § 2): jede `everyNights`-te Nacht mit festem Versatz aus dem Seed,
//! frühestens `firstNight`; fällt sie auf eine Nacht eines Nacht-Events (Vollmond, Blutmond), rückt sie eine Nacht weiter.
//! Alles wird aus Seed und Nacht berechnet (`k3c-rng`), nichts davon steht im Spielstand.
use k3c_rng::Rng;

use crate::state::weather_view::WeatherPhase;

/// Der Zeitplan einer Insel: Rhythmus, Versatz und die Rhythmen der Nacht-Events, denen das Wetter ausweicht.
#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    /// Jede n-te Nacht (>= 1).
    pub every: i64,
    /// Frühestens diese Nacht.
    pub first: i64,
    /// Fester Versatz `0..every` aus dem Insel-Seed.
    pub offset: i64,
    /// Rhythmen der Nacht-Events (`events.json` › `everyNights`, 7 und 13); in deren Nächten gibt es kein Wetter.
    pub moons: Vec<i64>,
}

impl Schedule {
    /// Zeitplan der Insel mit dem Seed `seed`; der Versatz kommt aus `k3c-rng` (`<seed>|weather`).
    pub fn new(seed: &str, every: i64, first: i64, moons: Vec<i64>) -> Schedule {
        let every = every.max(1);
        let offset = Rng::new(&format!("{seed}|weather")).int(0, every - 1);
        Schedule {
            every,
            first,
            offset,
            moons,
        }
    }

    fn is_moon(&self, night: i64) -> bool {
        self.moons.iter().any(|&m| m > 0 && night % m == 0)
    }

    fn scheduled(&self, night: i64) -> bool {
        night >= self.first && night % self.every == self.offset
    }

    /// Ist Nacht `night` (Nacht n = `cycle.day` n in der Phase night) eine Wetternacht? Nie in einer Event-Nacht; eine auf
    /// Event-Nächte fallende Nacht des Rhythmus rückt bis zur ersten freien Nacht weiter.
    pub fn is_weather_night(&self, night: i64) -> bool {
        if night < 1 || self.is_moon(night) {
            return false;
        }
        let mut s = night;
        loop {
            if self.scheduled(s) {
                return true;
            }
            s -= 1;
            if s < 1 || !self.is_moon(s) {
                return false;
            }
        }
    }

    /// Die Phase zur Zyklus-Phase `cycle_phase` (`day`, `dusk`, `night`, `dawn`) am Tag `day`: Die Nacht gehört zum Tag
    /// davor (`cycle.day`), also laufen Vorboten am Tag vor der Wetternacht (Morgengrauen, Tag, Dämmerung) und die
    /// Wirkung in der Nacht selbst; im Morgengrauen danach ist der Tag schon der nächste.
    pub fn phase(&self, day: i64, cycle_phase: &str) -> Option<WeatherPhase> {
        if !self.is_weather_night(day) {
            return None;
        }
        Some(if cycle_phase == "night" {
            WeatherPhase::Active
        } else {
            WeatherPhase::Forecast
        })
    }
}
