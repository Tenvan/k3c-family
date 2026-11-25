//! Tests zu `economy_measures.rs` und den Wirtschafts-Zielen (Port von `engine/balance/economy_test.go`, BAL6, B-347).

use std::collections::BTreeSet;

use k3c_data::balance_targets::{Target, Targets};
use k3c_data::hub::Stock;

use crate::economy_measures::economy_measure;
use crate::evaluate::Evaluate;
use crate::evaluate_tests::{metrics_with, report, seeds, share_target};
use crate::metrics::{DawnMetrics, DayMetrics, Metrics};
use crate::run::{Matrix, run};
use crate::scope::validate_target;
use crate::summary::Summarize;
use crate::targets::{default_targets, measure_values};
use crate::variant::with_variants;

/// Ein Lauf über 5 Tage (Dämmerung bei Tick 100·n, Tagesbeginn bei 200·(n−1)) mit festen Werten zu Tagesbeginn; `wood` und
/// `fighters` gelten je Tag ab Tag 1, `gold` für beide Monarchen.
fn economy_metrics(
    tower: Option<i64>,
    hub2: Option<i64>,
    hub3: Option<i64>,
    gold: i64,
    wood: &[i64],
    fighters: &[i64],
) -> Metrics {
    let mut m = Metrics {
        first_tower_tick: tower,
        hub_level2_tick: hub2,
        hub_level3_tick: hub3,
        ..Metrics::default()
    };
    for d in 1..=6_usize {
        let day = d as i64;
        m.days.push(DayMetrics {
            day,
            dusk_tick: Some(200 * (day - 1) + 100),
            ..DayMetrics::default()
        });
        m.dawns.push(DawnMetrics {
            day,
            tick: 200 * (day - 1),
            gold: vec![gold, gold + 10],
            stock: Stock {
                wood: wood[d - 1],
                ..Stock::default()
            },
            fighters: fighters[d - 1],
            ..DawnMetrics::default()
        });
    }
    m
}

/// BAL6/AC-01 (B-347): Die Kennzahlen der Wirtschafts-Korridore lesen Tagesbeginn, Turm und Hub-Stufe richtig ab.
#[test]
fn wirtschafts_kennzahlen() {
    let ok = economy_metrics(
        Some(300),
        Some(799),
        Some(900),
        40,
        &[100, 50, 100, 120, 100, 10],
        &[2, 3, 4, 6, 7, 8],
    );
    let late = economy_metrics(
        Some(301),
        Some(800),
        None,
        40,
        &[100, 100, 99, 100, 100, 100],
        &[2, 2, 3, 9, 9, 9],
    );
    let one = |v: f64| vec![v];
    let cases: Vec<(&str, &Metrics, Vec<f64>)> = vec![
        ("firstTowerBeforeDusk2", &ok, one(1.0)),
        ("firstTowerBeforeDusk2", &late, one(0.0)),
        ("woodAtDawnFrom3", &ok, one(1.0)),
        ("woodAtDawnFrom3", &late, one(0.0)), // Tag 6 liegt nach dem Szenario
        ("hubLevel2BeforeDay5", &ok, one(1.0)),
        ("hubLevel2BeforeDay5", &late, one(0.0)),
        ("hubLevel3BeforeDay10", &ok, one(1.0)),
        ("hubLevel3BeforeDay10", &late, one(0.0)), // Lauf endet vor Tag 10
        ("fightersAtDawn", &ok, one(1.0)),
        ("fightersAtDawn", &late, one(0.0)),
        // Tag 2 bis 5, je Monarch
        (
            "goldAtDawnPerPlayer",
            &ok,
            vec![40.0, 50.0, 40.0, 50.0, 40.0, 50.0, 40.0, 50.0],
        ),
    ];
    for (measure, m, want) in cases {
        assert_eq!(measure_values(measure, m), want, "{measure}");
        assert!(
            economy_measure(measure, m).is_some(),
            "{measure} fehlt in der Wirtschafts-Tabelle"
        );
    }
}

/// Ein Ziel mit Grad bewertet nur die Läufe dieses Grads; ohne Grad nur die Läufe mit normalem Grad (leer).
#[test]
fn ziel_mit_grad() {
    let hard = Target {
        grade: "hard".into(),
        ..share_target()
    };
    let mut rep = report(vec![
        Some(metrics_with(true, 1, &[])),
        Some(metrics_with(false, 1, &[])),
    ]);
    rep.results[0].scenario.grade = "hard".into();
    let ts = Targets {
        seeds: seeds(2),
        targets: vec![hard, share_target()],
        ..Targets::default()
    };
    let v = ts.evaluate(&rep);
    assert!(
        v[0].value == 0.0 && v[1].value == 100.0,
        "Grad hard {} %, normal {} %; erwartet 0 und 100",
        v[0].value,
        v[1].value
    );
    let bad = Target {
        grade: "raten".into(),
        days: 1,
        lower: Some(1.0),
        upper: None,
        rule: "x".into(),
        ..share_target()
    };
    assert!(validate_target(&bad).is_err(), "unbekannter Grad angenommen");
}

/// BAL6.1 (B-347): Der Bot wirtschaft baut Turm, Baumschule und Werkstatt und schafft Kämpfer; der Lauf legt jeden
/// Tagesbeginn ab (Tag 1 = Start, zuletzt der Morgen nach dem letzten Tag) mit Gold je Monarch. Mit dem reichen Beutel von
/// vor der Münzskala: Der Test prüft den Bot, die Wirtschaft der Münzskala stimmt BS3.4 ab.
#[test]
fn wirtschafts_bot_und_tagesbeginn() {
    let m = Matrix {
        seeds: vec!["1".into()],
        players: vec![2],
        bots: vec!["wirtschaft".into()],
        depths: vec![0],
        days: 2,
        ..Matrix::default()
    };
    let variants = ["economy.json:purse.startGold=100", "economy.json:purse.maxGold=100"].map(String::from);
    let r = with_variants(&variants, || run(&m)).expect("Lauf");
    let res = &r.results[0];
    assert!(res.valid, "ungültig: {}", res.error);
    let m = res.metrics.as_ref().expect("Kennzahlen");
    let mut days = Vec::new();
    for d in &m.dawns {
        days.push(d.day);
        assert!(d.gold.len() == 2 && d.hub_level >= 1, "Tag {}: {d:?}", d.day);
    }
    assert!(
        days == [1, 2, 3] && m.dawns[0].tick == 0 && m.dawns[0].fighters == 2,
        "Tagesbeginne {days:?}, Start {:?}",
        m.dawns[0]
    );
    assert!(
        m.first_tower_tick.is_some() && m.first_workshop_tick.is_some(),
        "kein Turm ({:?}) oder keine Werkstatt ({:?})",
        m.first_tower_tick,
        m.first_workshop_tick
    );
    assert!(
        m.dawns[2].fighters > m.dawns[0].fighters,
        "keine neuen Kämpfer: {} → {}",
        m.dawns[0].fighters,
        m.dawns[2].fighters
    );
}

/// BAL6/AC-01: Jede Zeile der Korridore der Wirtschaft hat ein Ziel, und jedes Ziel bekommt eine Bewertung (nicht
/// „ungültig“). Ein Seed genügt für die Aussage „bewertet“.
#[test]
#[ignore = "lang"]
fn alle_wirtschafts_zeilen_bewertet() {
    let ts = default_targets().expect("Ziele");
    let mut want: BTreeSet<&str> = [
        "castleHeld",
        "goldAtDawnPerPlayer",
        "firstWallBeforeDusk1",
        "firstTowerBeforeDusk2",
        "woodAtDawnFrom3",
        "hubLevel2BeforeDay5",
        "hubLevel3BeforeDay10",
        "destroyedPerWave",
        "fightersAtDawn",
    ]
    .into();
    let mut grades = BTreeSet::new();
    for g in &ts.targets {
        want.remove(g.measure.as_str());
        if g.measure == "castleHeld" {
            grades.insert(g.grade.as_str());
        }
    }
    assert!(
        want.is_empty() && grades.len() == 5,
        "ohne Ziel: {want:?}; Grade für „Burg hält“: {grades:?}"
    );
    let r = ts.run_targets(1).expect("Läufe");
    for e in ts.summarize(&r).targets {
        let t = &e.verdict.target;
        assert!(
            e.verdict.status != "ungültig",
            "{} ({} Spieler, {}, Tiefe {}, {} Tage): ungültig {:?}",
            t.name,
            t.players,
            t.bot,
            t.depth,
            t.days,
            e.verdict.invalid_seeds
        );
    }
}
