//! Gemeinsame Hilfen der Balancing-Tests (aus Go `targets_test.go`).

use std::path::{Path, PathBuf};

use super::report_types::{DawnMetrics, DayMetrics, Metrics, RunResult, Scenario, WaveMetrics};
use super::summary::{Summary, read_summary};
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

/// Der Bericht `tests/data/insights-run7.json` (Go `fixture`).
pub fn fixture() -> Summary {
    let path = repo_root().join("rust/tools/crates/k3c-studio/tests/data/insights-run7.json");
    read_summary(&std::fs::read(path).expect("Fixture")).expect("Fixture lesbar")
}

/// Temporäre Wurzel unter `.work/` ohne `..` im Pfad (der Schreibschutz lehnt `..` ab).
pub fn temp_root() -> (tempfile::TempDir, PathBuf) {
    let dir = crate::test_helpers::tmp_dir();
    let root = std::path::absolute(dir.path().join("repo")).expect("absolut");
    std::fs::create_dir_all(&root).expect("Wurzel");
    (dir, root)
}

/// Junction `link` auf `target` (nur Windows, `mklink` braucht keine Rechte).
#[cfg(windows)]
pub fn junction(link: &Path, target: &Path) -> bool {
    let mut cmd = std::process::Command::new("cmd");
    cmd.args(["/c", "mklink", "/J"]).arg(link).arg(target);
    cmd.output().is_ok_and(|o| o.status.success())
}
