//! Tests zu `ship_need.rs` (Port von `engine/balance/ship_need_test.go`, BS3.3/AC-04, B-638/AC-02).

use k3c_data::hub::Stock;

use crate::figures::figures_of;
use crate::insights::{LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW};
use crate::metrics::{DawnMetrics, DayMetrics, Metrics};
use crate::run::{Matrix, Report, RunResult, Scenario, run};
use crate::ship_need::{load_ship, ship_figures_of, ship_needs};

/// Zehn Tage mit 8 Gold und 30 Holz; Tag 4 bringt Stein und Kupfer (Rumpf), Tag 7 Eisen und Kristall (Segel).
fn ship_run(end_day: Option<i64>) -> Metrics {
    let mut days: Vec<DayMetrics> = (1..=10)
        .map(|day| DayMetrics {
            day,
            income: 8,
            stock_in: Stock {
                wood: 30,
                ..Stock::default()
            },
            ..DayMetrics::default()
        })
        .collect();
    (days[3].stock_in.stone, days[3].stock_in.copper) = (100, 40);
    (days[6].stock_in.iron, days[6].stock_in.crystal) = (40, 20);
    Metrics {
        dawns: vec![DawnMetrics {
            gold: vec![14, 14],
            ..DawnMetrics::default()
        }],
        days,
        endboss_day: end_day,
        ..Metrics::default()
    }
}

fn report_of(m: Metrics) -> Report {
    Report {
        matrix: Matrix::default(),
        results: vec![RunResult {
            scenario: Scenario::default(),
            valid: true,
            error: String::new(),
            ticks: 0,
            metrics: Some(m),
            replay: None,
        }],
    }
}

#[test]
fn schiff_bedarf_aus_dem_verlauf() {
    let stages = load_ship().expect("ship.json");
    assert_eq!(stages.len(), 3, "{stages:?}");
    let mut m = ship_run(None);
    let got = ship_needs(&stages, &m);
    // Kiel: 10 Gold + 80 Holz → Holz reicht ab Tag 3 (90), Gold schon am Start (28).
    // Rumpf (kumuliert 30 Gold, 80 Holz, 60 Stein, 40 Kupfer): Stein erst Tag 4.
    // Segel (kumuliert 60 Gold, 80 Holz, 60 Stein, 40 Kupfer, 40 Eisen, 20 Kristall): Eisen und Kristall erst Tag 7, Gold 28+8·d ≥ 60 ab Tag 4.
    for (i, want) in [3, 4, 7].into_iter().enumerate() {
        assert_eq!(got[i].affordable_day, Some(want), "{} bezahlbar", got[i].id);
    }
    m.days.truncate(5); // Lauf endet vor Tag 7: das Segel war nie bezahlbar
    assert_eq!(ship_needs(&stages, &m)[2].affordable_day, None, "Segel ohne Eisen");
}

/// Ampel gegen den Korridor „Endboss-Sieg −2 bis +1 Tag“ (zielkorridore.md § 6).
#[test]
fn schiff_ampel() {
    let stages = load_ship().expect("ship.json");
    let cases = [
        (Some(8), LIGHT_GREEN, "Ziel T6–T9"),        // Segel T7 liegt im Korridor
        (Some(11), LIGHT_YELLOW, "zu früh"),         // Segel T7, Korridor T9–T12: 2 Tage davor
        (Some(20), LIGHT_RED, "zu früh"),            // weit davor
        (None, LIGHT_GREY, "Endboss nicht besiegt"), // kein Ziel
    ];
    for (end, light, want) in cases {
        let mut m = ship_run(end);
        m.ship = ship_needs(&stages, &m);
        let f = ship_figures_of(&report_of(m)).expect("Schiff-Werte");
        assert!(
            f.light == light && f.text.contains(want),
            "Endboss {end:?}: {f:?}, erwartet {light} mit {want:?}"
        );
        assert!(
            f.text
                .contains("Schiff (Bedarf): Kiel bezahlbar T3, Rumpf bezahlbar T4, Segel bezahlbar T7"),
            "Zeile: {}",
            f.text
        );
    }
    let mut m = ship_run(Some(8));
    m.ship = ship_needs(&stages, &m);
    let f = ship_figures_of(&report_of(m)).expect("Schiff-Werte");
    let mut b = String::new();
    f.markdown(&mut b);
    assert!(b.contains("🟢 Schiff (Bedarf):"), "Markdown: {b:?}");
}

/// Ein Lauf ohne World hat keinen Schiff-Bedarf (altes JSON bleibt gleich).
#[test]
fn schiff_nur_im_insel_lauf() {
    let m = Matrix {
        seeds: vec!["1".into()],
        players: vec![1],
        bots: vec!["passive".into()],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    };
    let r = run(&m).expect("Lauf");
    let ship = &r.results[0].metrics.as_ref().expect("Kennzahlen").ship;
    assert!(ship.is_empty(), "Ein-Stufen-Lauf mit Schiff: {ship:?}");
    let f = figures_of(&r).expect("Kennzahlen");
    assert!(
        f.ship.is_none(),
        "Kennzahlen ohne World dürfen kein Schiff nennen: {:?}",
        f.ship
    );
}
