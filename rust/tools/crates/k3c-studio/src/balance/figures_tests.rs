//! Go `figures_test.go`, dazu Proben der Verdichtung mit von Hand gerechneten Werten (Go hat dafür keine Tests).

use super::figures::figures_of;
use super::report_types::{
    BossMetrics, DayMetrics, IslandMetrics, Metrics, Report, RunResult, Scenario, ShipNeed, StageMetrics, load_ship,
};
use super::summary::read_summary;
use super::test_helpers::repo_root;

/// Ältere Berichte ohne Varianten und Kennzahlen bleiben lesbar.
#[test]
fn alter_bericht_lesbar() {
    let s = read_summary(br#"{"version":1,"seeds":3,"pass":true,"targets":[]}"#).expect("lesen");
    assert!(
        s.seeds == 3 && s.varianten.is_empty() && s.kennzahlen.is_none(),
        "{s:?}"
    );
}

fn island_run(endboss_day: Option<i64>, ship_day: Option<i64>) -> RunResult {
    let ids = load_ship(&repo_root()).expect("ship.json");
    let metrics = Metrics {
        first_wall_tick: endboss_day,
        days: vec![DayMetrics {
            income: 10,
            night_income: 4,
            peasant_ticks: 3,
            peasant_idle_ticks: 1,
            ..DayMetrics::default()
        }],
        island: IslandMetrics {
            stages: vec![StageMetrics {
                arrival_day: Some(1),
                hub_level: 2,
                ..StageMetrics::default()
            }],
            bosses: vec![BossMetrics {
                boss: "b".to_owned(),
                day: 3,
                tick: 1,
            }],
            endboss_day,
            ship: ids
                .iter()
                .map(|s| ShipNeed {
                    id: s.id.clone(),
                    affordable_day: ship_day,
                })
                .collect(),
            ..IslandMetrics::default()
        },
        ..Metrics::default()
    };
    RunResult {
        scenario: Scenario {
            days: 5,
            ..Scenario::default()
        },
        valid: true,
        metrics: Some(metrics),
        ..RunResult::default()
    }
}

#[test]
fn verdichtet_insel_und_schiff() {
    let report = Report {
        results: vec![
            island_run(Some(10), Some(9)),
            island_run(None, None),
            RunResult::default(),
        ],
        ..Report::default()
    };
    let f = figures_of(&repo_root(), &report).expect("Figures");
    assert_eq!((f.runs, f.first_wall_tick_median), (2, Some(10.0)));
    let night = &f.nights[0];
    assert_eq!((night.night, night.runs, night.survival), (1, 2, 100.0));
    assert_eq!(
        (night.day_income, night.night_income, night.idle_share),
        (6.0, 4.0, 33.3)
    );
    let island = f.island.expect("Insel");
    assert_eq!((island.depths[0].arrived, island.depths[0].hub_level), (100.0, 2.0));
    assert_eq!(
        (island.bosses[0].defeated, island.bosses[0].day_median),
        (100.0, Some(3.0))
    );
    assert_eq!((island.endboss.defeated, island.endboss.day_median), (50.0, Some(10.0)));
    let ship = f.ship.expect("Schiff");
    assert_eq!((ship.light.as_str(), ship.stages[0].reached), ("green", 50.0));
    assert!(
        ship.text.starts_with("Schiff (Bedarf): Kiel bezahlbar T9, "),
        "{}",
        ship.text
    );
    assert!(ship.text.ends_with("(Ziel T8–T11)"), "{}", ship.text);
}

#[test]
fn ohne_gueltigen_lauf_keine_figures() {
    assert!(figures_of(&repo_root(), &Report::default()).is_none());
}
