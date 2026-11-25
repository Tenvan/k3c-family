//! Tests zu `targets.rs` (Port von `engine/balance/targets_test.go`: Laden, Kennzahl-Tabelle, Tabelle der Grenzen; die
//! Bewertungs-Tests stehen in `evaluate_tests.rs`).

use k3c_data::balance_targets::FILE;
use k3c_det::golden;

use crate::evaluate_tests::metrics_with;
use crate::island_world::shared;
use crate::metrics::{DawnMetrics, Metrics};
use crate::targets::{default_targets, load_targets, measure_values};

fn raw() -> String {
    shared().expect("Daten").source.read(FILE).expect("Datei der Ziele")
}

/// BAL2/AC-01: Die eingebettete Datei lädt (100 feste Seeds); ein Ziel ohne bekannte Kennzahl ist ein Ladefehler.
#[test]
fn zielkorridore_laden() {
    let ts = default_targets().expect("Ziele");
    // k3c-dev und 1–100 (B-634)
    assert!(
        ts.seeds.len() == 101 && !ts.targets.is_empty(),
        "{} Seeds, {} Ziele",
        ts.seeds.len(),
        ts.targets.len()
    );
    let raw = raw();
    for (name, from, to) in [
        ("Kennzahl unbekannt", "\"castleHeld\"", "\"gibtsNicht\""),
        ("Art passt nicht", "\"kind\": \"share\"", "\"kind\": \"median\""),
        ("Bot unbekannt", "\"wirtschaft\"", "\"nobody\""),
        ("Grad unbekannt", "\"grade\": \"dev\"", "\"grade\": \"raten\""),
    ] {
        assert!(raw.contains(from), "{name}: {from} nicht in der Datei");
        let err = load_targets(&raw.replacen(from, to, 1)).expect_err(name);
        if name == "Kennzahl unbekannt" {
            assert!(
                err.to_string().contains("gibtsNicht"),
                "Fehler nennt die Kennzahl nicht: {err}"
            );
        }
    }
}

fn dawns(list: &[(i64, i64)]) -> Vec<DawnMetrics> {
    list.iter()
        .map(|&(day, tick)| DawnMetrics {
            day,
            tick,
            ..DawnMetrics::default()
        })
        .collect()
}

/// B-657: castleHeld wertet nur Nacht 1 bis 5 (bis zum Morgen von Tag 6), auch in einem Lauf über 40 Tage.
#[test]
fn castle_held_fenster() {
    const DAWN6: i64 = 1000; // Tick des Morgens nach Nacht 5
    let long = |fall: Option<i64>| Metrics {
        castle_fall_tick: fall,
        dawns: dawns(&[(1, 0), (6, DAWN6), (41, 9000)]),
        ..Metrics::default()
    };
    let cases: Vec<(&str, Metrics, Vec<f64>)> = vec![
        ("AC-01 Burgfall nach Tag 5 (Tag 41)", long(Some(9500)), vec![1.0]),
        ("AC-02 Burgfall an Tag 3", long(Some(500)), vec![0.0]),
        ("gehalten über den ganzen Lauf", long(None), vec![1.0]),
        (
            "Lauf endet mit Burgfall vor Tag 6",
            Metrics {
                castle_fall_tick: Some(500),
                dawns: dawns(&[(1, 0), (3, 400)]),
                ..Metrics::default()
            },
            vec![0.0],
        ),
        (
            "Lauf kürzer als 5 Tage ohne Burgfall: nicht auswertbar",
            Metrics {
                dawns: dawns(&[(1, 0), (4, 600)]),
                ..Metrics::default()
            },
            Vec::new(),
        ),
    ];
    for (name, m, want) in cases {
        assert_eq!(measure_values("castleHeld", &m), want, "{name}");
    }
}

/// Die Mauer-Kennzahl zählt nur Mauern bis Dämmerungsbeginn von Tag 1.
#[test]
fn mauer_vor_daemmerung() {
    let m = |m: &Metrics| measure_values("firstWallBeforeDusk1", m)[0];
    assert!(
        m(&metrics_with(false, 100, &[])) == 1.0
            && m(&metrics_with(false, 101, &[])) == 0.0
            && m(&Metrics::default()) == 0.0,
        "Grenze bei Dämmerungsbeginn falsch"
    );
}

fn grade_label(grade: &str) -> &'static str {
    match grade {
        "dev" => "Dev",
        "easy" => "Leicht",
        "hard" => "Hart",
        "ultra" => "Ultra",
        _ => "",
    }
}

/// „75 %“, „20 Gold“, „1“ oder „–“ (keine Grenze).
fn table_bound(cell: &str) -> Option<f64> {
    cell.trim().replace(',', ".").split_whitespace().next()?.parse().ok()
}

/// Datei und Tabelle in docs/rules/zielkorridore.md nennen dieselben Grenzen (Zeile = Name als Anfang der Kennzahl,
/// Szenario genau „Standard“, mit Grad „Standard, Grad <Name>“).
#[test]
fn grenzen_wie_tabelle() {
    let doc = std::fs::read_to_string(golden::testdata().join("../docs/rules/zielkorridore.md")).expect("Regeltext");
    let rows: Vec<Vec<&str>> = doc
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|'))
        .map(|l| l.trim_matches('|').split('|').collect())
        .collect();
    let ts = default_targets().expect("Ziele");
    for g in &ts.targets {
        let mut scenario = "Standard".to_string();
        if !g.grade.is_empty() {
            scenario += &format!(", Grad {}", grade_label(&g.grade));
        }
        let hit: Vec<&Vec<&str>> = rows
            .iter()
            .filter(|r| r.len() >= 4 && r[0].trim().starts_with(&g.name) && r[1].trim() == scenario)
            .collect();
        assert_eq!(
            hit.len(),
            1,
            "{:?}: {} Zeilen in der Tabelle, erwartet 1",
            g.name,
            hit.len()
        );
        for (i, got) in [g.lower, g.upper].into_iter().enumerate() {
            let want = table_bound(hit[0][2 + i]);
            assert_eq!(
                got,
                want,
                "{:?}: Grenze {} in der Datei {got:?}, in der Tabelle {want:?}",
                g.name,
                i + 1
            );
        }
    }
}
