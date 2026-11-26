//! Go `targets_test.go`.

use std::collections::HashMap;
use std::fs;

use super::super::evaluate::{STATUS_INVALID, STATUS_NARROW, STATUS_OK, STATUS_VIOLATED, Verdict};
use super::super::report_types::{DawnMetrics, Metrics, Report, RunResult};
use super::super::test_helpers::{metrics_with, repo_root, results, seeds, share_target};
use super::{KIND_MEDIAN, Margin, Target, Targets, default_targets, load_targets, measure};

fn targets_file() -> String {
    fs::read_to_string(repo_root().join("data/balance-targets.json")).expect("balance-targets.json")
}

/// Bewertet das erste Ziel gegen die Läufe.
fn first(ts: &Targets, results: Vec<RunResult>) -> Verdict {
    let report = Report {
        results,
        ..Report::default()
    };
    ts.evaluate(&report).remove(0)
}

fn with_targets(count: usize, target: Target) -> Targets {
    Targets {
        seeds: seeds(count),
        targets: vec![target],
        ..Targets::default()
    }
}

fn run(fallen: bool) -> Option<Metrics> {
    Some(metrics_with(fallen, 1, &[]))
}

/// BAL2/AC-01: Die Datei lädt (100 feste Seeds); ein Ziel ohne bekannte Kennzahl ist ein Ladefehler.
#[test]
fn zielkorridore_laden() {
    let root = repo_root();
    let ts = default_targets(&root).expect("Ziele");
    // k3c-dev und 1–100 (B-634)
    assert_eq!(ts.seeds.len(), 101);
    assert!(!ts.targets.is_empty());
    let raw = targets_file();
    for (name, from, to) in [
        ("Kennzahl unbekannt", r#""castleHeld""#, r#""gibtsNicht""#),
        ("Art passt nicht", r#""kind": "share""#, r#""kind": "median""#),
        ("Bot unbekannt", r#""wirtschaft""#, r#""nobody""#),
        ("Grad unbekannt", r#""grade": "dev""#, r#""grade": "raten""#),
    ] {
        let bad = raw.replacen(from, to, 1);
        assert_ne!(bad, raw, "{name}: Stelle fehlt in der Datei");
        let err = load_targets(&root, bad.as_bytes()).expect_err(name);
        if name == "Kennzahl unbekannt" {
            assert!(err.contains("gibtsNicht"), "Fehler nennt die Kennzahl nicht: {err}");
        }
    }
}

/// BAL2/AC-02: Anteil im Korridor, knapp und verletzt (Korridor 70–90 %, Rand 5 pp, 20 Seeds).
#[test]
fn bewertung_anteil() {
    let mut ts = with_targets(20, share_target());
    ts.margin = Margin {
        share_pp: 5.0,
        ..Margin::default()
    };
    // (Seeds, in denen die Burg hält; Status; erwartete Anzahl FailSeeds)
    for (held, status, fail) in [
        (16, STATUS_OK, 0),
        (14, STATUS_NARROW, 0),
        (18, STATUS_NARROW, 0),
        (12, STATUS_VIOLATED, 8),
        (19, STATUS_VIOLATED, 19),
    ] {
        let ms: Vec<_> = (0..20).map(|i| run(i >= held)).collect();
        let v = first(&ts, results(&ms));
        assert_eq!(
            (v.status.as_str(), v.value, v.fail_seeds.len()),
            (status, f64::from(held) * 5.0, fail),
            "{held} von 20: {:?}",
            v.fail_seeds
        );
    }
}

/// Anteil unter der Untergrenze nennt die Seeds, in denen die Burg fiel; darüber die, in denen sie hielt.
#[test]
fn fail_seeds_anteil() {
    let ts = with_targets(4, share_target());
    let v = first(&ts, results(&[run(false), run(true), run(true), run(false)]));
    assert_eq!(
        (v.status.as_str(), v.fail_seeds),
        (STATUS_VIOLATED, vec!["2".to_owned(), "3".to_owned()])
    );
    let v = first(&ts, results(&[run(false), run(false)]));
    assert_eq!(
        (v.status.as_str(), v.fail_seeds),
        (STATUS_VIOLATED, vec!["1".to_owned(), "2".to_owned()]),
        "100 % über Obergrenze"
    );
}

/// Zerstörungen je Welle (je Seed), Median, Status, FailSeeds.
type MedianCase = ([[i64; 2]; 3], f64, &'static str, &'static [&'static str]);

fn median_cases() -> [MedianCase; 3] {
    [
        ([[0, 0], [0, 1], [0, 0]], 0.0, STATUS_OK, &[]),
        ([[1, 1], [1, 1], [1, 1]], 1.0, STATUS_NARROW, &[]),
        ([[2, 3], [0, 4], [1, 1]], 1.5, STATUS_VIOLATED, &["1", "2"]),
    ]
}

/// BAL2/AC-02: Median im Korridor, knapp und verletzt (nur Obergrenze 1, Rand 10 % von |1| = 0,1).
#[test]
fn bewertung_median() {
    let target = Target {
        measure: "destroyedPerWave".to_owned(),
        name: "y".to_owned(),
        kind: KIND_MEDIAN.to_owned(),
        lower: None,
        upper: Some(1.0),
        ..share_target()
    };
    let mut ts = with_targets(3, target);
    ts.margin = Margin {
        median_fraction: 0.1,
        ..Margin::default()
    };
    // je Seed die Zerstörungen je Welle
    for (destroyed, value, status, fail) in median_cases() {
        let ms: Vec<_> = destroyed.iter().map(|d| Some(metrics_with(false, 1, d))).collect();
        let v = first(&ts, results(&ms));
        assert_eq!((v.status.as_str(), v.value), (status, value), "{destroyed:?}");
        assert_eq!(v.fail_seeds, fail, "{destroyed:?}");
    }
}

/// Ein abgebrochener Lauf macht das Urteil ungültig und nennt den Seed.
#[test]
fn bewertung_ungueltig() {
    let ts = with_targets(2, share_target());
    let v = first(&ts, results(&[run(false), None]));
    assert_eq!(
        (v.status.as_str(), v.invalid_seeds),
        (STATUS_INVALID, vec!["2".to_owned()])
    );
}

/// Morgen-Tick nach Nacht 5.
const DAWN6: i64 = 1000;

fn dawns(list: &[(i64, i64)]) -> Vec<DawnMetrics> {
    list.iter()
        .map(|&(day, tick)| DawnMetrics {
            day,
            tick,
            ..DawnMetrics::default()
        })
        .collect()
}

fn long(fall: Option<i64>) -> Metrics {
    Metrics {
        castle_fall_tick: fall,
        dawns: dawns(&[(1, 0), (6, DAWN6), (41, 9000)]),
        ..Metrics::default()
    }
}

/// B-657: castleHeld wertet nur Nacht 1 bis 5 (bis zum Morgen von Tag 6), auch in einem Lauf über 40 Tage. Den
/// Klartext der Go-Fassung (`TargetInsight.what`) prüft RU70.17.
#[test]
fn castle_held_fenster() {
    let early_end = Metrics {
        castle_fall_tick: Some(500),
        dawns: dawns(&[(1, 0), (3, 400)]),
        ..Metrics::default()
    };
    let too_short = Metrics {
        dawns: dawns(&[(1, 0), (4, 600)]),
        ..Metrics::default()
    };
    let kennzahl = measure("castleHeld").expect("castleHeld");
    let cases: HashMap<&str, (Metrics, Vec<f64>)> = HashMap::from([
        ("AC-01 Burgfall nach Tag 5 (Tag 41)", (long(Some(9500)), vec![1.0])),
        ("AC-02 Burgfall an Tag 3", (long(Some(500)), vec![0.0])),
        ("gehalten über den ganzen Lauf", (long(None), vec![1.0])),
        ("Lauf endet mit Burgfall vor Tag 6", (early_end, vec![0.0])),
        (
            "Lauf kürzer als 5 Tage ohne Burgfall: nicht auswertbar",
            (too_short, vec![]),
        ),
    ]);
    for (name, (metrics, want)) in cases {
        assert_eq!((kennzahl.values)(&metrics), want, "{name}");
    }
    // Ein kurzer Lauf ohne Wert zählt nicht als verletzt, sondern bleibt aus dem Anteil heraus.
    let short = Metrics {
        dawns: dawns(&[(1, 0)]),
        ..Metrics::default()
    };
    let ts = with_targets(2, share_target());
    let v = first(&ts, results(&[Some(long(Some(9500))), Some(short)]));
    assert_eq!((v.value, v.counted), (100.0, 1), "kurzer Lauf verfälscht den Anteil");
}

/// Die Mauer-Kennzahl zählt nur Mauern bis Dämmerungsbeginn von Tag 1.
#[test]
fn mauer_vor_daemmerung() {
    let values = measure("firstWallBeforeDusk1").expect("Kennzahl").values;
    assert_eq!(values(&metrics_with(false, 100, &[])), [1.0]);
    assert_eq!(values(&metrics_with(false, 101, &[])), [0.0]);
    assert_eq!(values(&Metrics::default()), [0.0]);
}

/// Grad aus `data/difficulty.json` → Name in der Spalte Szenario der Tabelle (§ 2).
fn grade_label(grade: &str) -> &'static str {
    match grade {
        "dev" => "Dev",
        "easy" => "Leicht",
        "hard" => "Hart",
        "ultra" => "Ultra",
        _ => "",
    }
}

/// Liest „75 %“, „20 Gold“, „1“ oder „–“ (keine Grenze).
fn table_bound(cell: &str) -> Option<f64> {
    let cell = cell.trim().replace(',', ".");
    cell.split_whitespace().next().and_then(|n| n.parse().ok())
}

/// Datei und Tabelle in `docs/rules/zielkorridore.md` nennen dieselben Grenzen (Zeile = Name als Anfang der Kennzahl,
/// Szenario genau „Standard“, mit Grad „Standard, Grad <Name>“).
#[test]
fn grenzen_wie_tabelle() {
    let root = repo_root();
    let raw = fs::read_to_string(root.join("docs/rules/zielkorridore.md")).expect("zielkorridore.md");
    let rows: Vec<Vec<&str>> = raw
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        .map(|line| line.trim_matches('|').split('|').collect())
        .collect();
    for g in default_targets(&root).expect("Ziele").targets {
        let mut scenario = "Standard".to_owned();
        if !g.grade.is_empty() {
            scenario = format!("{scenario}, Grad {}", grade_label(&g.grade));
        }
        let hit: Vec<_> = rows
            .iter()
            .filter(|r| r.len() >= 4 && r[0].trim().starts_with(&g.name) && r[1].trim() == scenario)
            .collect();
        assert_eq!(hit.len(), 1, "{}: Zeilen in der Tabelle", g.name);
        for (i, got) in [g.lower, g.upper].into_iter().enumerate() {
            assert_eq!(got, table_bound(hit[0][2 + i]), "{}: Grenze {}", g.name, i + 1);
        }
    }
}
