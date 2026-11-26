//! Gemeinsame Hilfen der Balancing-Tests (aus Go `targets_test.go`; RU70.15 nutzt sie weiter).

use std::path::PathBuf;

use super::report_types::{DawnMetrics, DayMetrics, Metrics, RunResult, Scenario, WaveMetrics};
use super::targets::{KIND_SHARE, Target};

/// Repo-Wurzel für die Daten unter `data/`.
pub fn repo_root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.."))
}

pub fn metrics_with(fallen: bool, wall_tick: i64, destroyed: &[i64]) -> Metrics {
    Metrics {
        days: vec![DayMetrics {
            day: 1,
            dusk_tick: Some(100),
            ..DayMetrics::default()
        }],
        first_wall_tick: Some(wall_tick),
        dawns: vec![
            DawnMetrics {
                day: 1,
                ..DawnMetrics::default()
            },
            DawnMetrics {
                day: 6,
                tick: 1000,
                ..DawnMetrics::default()
            },
        ],
        castle_fall_tick: fallen.then_some(500),
        waves: destroyed
            .iter()
            .zip(1..)
            .map(|(d, wave)| WaveMetrics {
                wave,
                buildings_destroyed: *d,
                ..WaveMetrics::default()
            })
            .collect(),
        ..Metrics::default()
    }
}

/// Läufe mit den Seeds 1, 2, …; `None` ist ein ungültiger Lauf.
pub fn results(ms: &[Option<Metrics>]) -> Vec<RunResult> {
    ms.iter()
        .zip(seeds(ms.len()))
        .map(|(m, seed)| RunResult {
            scenario: Scenario {
                seed,
                players: 2,
                bot: "saver".to_owned(),
                days: 5,
                ..Scenario::default()
            },
            valid: m.is_some(),
            metrics: m.clone(),
            ..RunResult::default()
        })
        .collect()
}

pub fn share_target() -> Target {
    Target {
        measure: "castleHeld".to_owned(),
        name: "x".to_owned(),
        kind: KIND_SHARE.to_owned(),
        players: 2,
        bot: "saver".to_owned(),
        days: 5,
        lower: Some(70.0),
        upper: Some(90.0),
        rule: "r".to_owned(),
        ..Target::default()
    }
}

pub fn seeds(n: usize) -> Vec<String> {
    (1..=n).map(|i| i.to_string()).collect()
}
