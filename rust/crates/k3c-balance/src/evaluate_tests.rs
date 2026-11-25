//! Tests zu `evaluate.rs` (Port der Bewertungs-Tests aus `engine/balance/targets_test.go`; Laden und Kennzahl-Tabelle
//! stehen in `targets_tests.rs`). Die Hilfen (`seeds`, `metrics_with`, `report`, `share_target`, `targets`) nutzen auch
//! die Tests zu Zusammenfassung und Wirtschaft.

use k3c_data::balance_targets::{Target, Targets};

use crate::evaluate::{Evaluate, STATUS_INVALID, STATUS_NARROW, STATUS_OK, STATUS_VIOLATED};
use crate::metrics::{DawnMetrics, DayMetrics, Metrics, WaveMetrics};
use crate::run::{Matrix, Report, RunResult, Scenario};

pub(crate) fn seeds(n: usize) -> Vec<String> {
    (1..=n).map(|i| i.to_string()).collect()
}

/// Metrics für feste Werte: gehalten oder gefallen, Tick der ersten Mauer, zerstörte Gebäude je Welle.
pub(crate) fn metrics_with(fallen: bool, wall_tick: i64, destroyed: &[i64]) -> Metrics {
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
            .enumerate()
            .map(|(i, &d)| WaveMetrics {
                wave: i as i64 + 1,
                buildings_destroyed: d,
                ..WaveMetrics::default()
            })
            .collect(),
        ..Metrics::default()
    }
}

pub(crate) fn report(ms: Vec<Option<Metrics>>) -> Report {
    let results = ms
        .into_iter()
        .enumerate()
        .map(|(i, m)| RunResult {
            scenario: Scenario {
                seed: (i + 1).to_string(),
                players: 2,
                bot: "saver".into(),
                depth: 0,
                days: 5,
                ..Scenario::default()
            },
            valid: m.is_some(),
            error: String::new(),
            ticks: 0,
            metrics: m,
            replay: None,
        })
        .collect();
    Report {
        matrix: Matrix::default(),
        results,
    }
}

pub(crate) fn share_target() -> Target {
    Target {
        measure: "castleHeld".into(),
        name: "x".into(),
        kind: "share".into(),
        players: 2,
        bot: "saver".into(),
        days: 5,
        lower: Some(70.0),
        upper: Some(90.0),
        rule: "r".into(),
        ..Target::default()
    }
}

pub(crate) fn targets(n: usize, t: Target) -> Targets {
    Targets {
        seeds: seeds(n),
        targets: vec![t],
        ..Targets::default()
    }
}

/// BAL2/AC-02: Anteil im Korridor, knapp und verletzt (Korridor 70–90 %, Rand 5 pp, 20 Seeds).
#[test]
fn bewertung_anteil() {
    let mut ts = targets(20, share_target());
    ts.margin.share_pp = 5.0;
    // (Seeds, in denen die Burg hält, Status, erwartete Zahl FailSeeds)
    for (held, status, fail) in [
        (16, STATUS_OK, 0),
        (14, STATUS_NARROW, 0),
        (18, STATUS_NARROW, 0),
        (12, STATUS_VIOLATED, 8),
        (19, STATUS_VIOLATED, 19),
    ] {
        let ms = (0..20).map(|i| Some(metrics_with(i >= held, 50, &[]))).collect();
        let v = &ts.evaluate(&report(ms))[0];
        assert!(
            v.status == status && v.value == held as f64 * 5.0 && v.fail_seeds.len() == fail,
            "{held} von 20: {} {} % Fail {:?}, erwartet {status}",
            v.status,
            v.value,
            v.fail_seeds
        );
    }
}

/// Anteil unter der Untergrenze nennt die Seeds, in denen die Burg fiel; darüber die, in denen sie hielt.
#[test]
fn fail_seeds_anteil() {
    let ts = targets(4, share_target());
    let m = |fallen| Some(metrics_with(fallen, 1, &[]));
    let v = &ts.evaluate(&report(vec![m(false), m(true), m(true), m(false)]))[0];
    assert_eq!(
        (v.status.as_str(), v.fail_seeds.clone()),
        (STATUS_VIOLATED, vec!["2".into(), "3".into()])
    );
    let v = &ts.evaluate(&report(vec![m(false), m(false)]))[0];
    assert_eq!(
        (v.status.as_str(), v.fail_seeds.clone()),
        (STATUS_VIOLATED, vec!["1".into(), "2".into()]),
        "100 % über Obergrenze"
    );
}

/// BAL2/AC-02: Median im Korridor, knapp und verletzt (nur Obergrenze 1, Rand 10 % von |1| = 0,1).
#[test]
fn bewertung_median() {
    let g = Target {
        measure: "destroyedPerWave".into(),
        name: "y".into(),
        kind: "median".into(),
        players: 2,
        bot: "saver".into(),
        days: 5,
        upper: Some(1.0),
        rule: "r".into(),
        ..Target::default()
    };
    let mut ts = targets(3, g);
    ts.margin.median_fraction = 0.1;
    type Case = ([[i64; 2]; 3], f64, &'static str, &'static [&'static str]);
    let cases: [Case; 3] = [
        ([[0, 0], [0, 1], [0, 0]], 0.0, STATUS_OK, &[]),
        ([[1, 1], [1, 1], [1, 1]], 1.0, STATUS_NARROW, &[]),
        ([[2, 3], [0, 4], [1, 1]], 1.5, STATUS_VIOLATED, &["1", "2"]),
    ];
    for (destroyed, value, status, fail) in cases {
        let ms = destroyed.iter().map(|d| Some(metrics_with(false, 1, d))).collect();
        let v = &ts.evaluate(&report(ms))[0];
        assert!(
            v.status == status && v.value == value && v.fail_seeds == fail,
            "{destroyed:?}: {} {} {:?}, erwartet {status} {value} {fail:?}",
            v.status,
            v.value,
            v.fail_seeds
        );
    }
}

/// Ein abgebrochener Lauf macht das Urteil ungültig und nennt den Seed.
#[test]
fn bewertung_ungueltig() {
    let ts = targets(2, share_target());
    let v = &ts.evaluate(&report(vec![Some(metrics_with(false, 1, &[])), None]))[0];
    assert_eq!(
        (v.status.as_str(), v.invalid_seeds.clone()),
        (STATUS_INVALID, vec!["2".to_string()])
    );
}

/// B-657: Ein kurzer Lauf ohne Wert zählt nicht als verletzt, sondern bleibt aus dem Anteil heraus.
#[test]
fn kurzer_lauf_verfaelscht_den_anteil_nicht() {
    let long = Metrics {
        castle_fall_tick: Some(9500),
        dawns: [(1, 0), (6, 1000), (41, 9000)]
            .map(|(day, tick)| DawnMetrics {
                day,
                tick,
                ..DawnMetrics::default()
            })
            .to_vec(),
        ..Metrics::default()
    };
    let short = Metrics {
        dawns: vec![DawnMetrics {
            day: 1,
            ..DawnMetrics::default()
        }],
        ..Metrics::default()
    };
    let ts = targets(2, share_target());
    let v = &ts.evaluate(&report(vec![Some(long), Some(short)]))[0];
    assert!(
        v.value == 100.0 && v.counted == 1,
        "{} %, gewertet {}",
        v.value,
        v.counted
    );
}

/// Das Urteil schreibt und liest sich wie Gos `Verdict` (Ziel flach, `failSeeds` nur bei Inhalt).
#[test]
fn urteil_json_wie_go() {
    let ts = targets(2, share_target());
    let v = ts.evaluate(&report(vec![
        Some(metrics_with(false, 1, &[])),
        Some(metrics_with(false, 1, &[])),
    ]));
    let j = serde_json::to_value(&v[0]).expect("JSON");
    assert_eq!(j["measure"], "castleHeld");
    assert_eq!(j["status"], STATUS_VIOLATED);
    assert_eq!(j["failSeeds"], serde_json::json!(["1", "2"]));
    assert!(j.get("invalidSeeds").is_none());
    let back: crate::evaluate::Verdict = serde_json::from_value(j).expect("Urteil lesen");
    assert_eq!(back, v[0]);
}
