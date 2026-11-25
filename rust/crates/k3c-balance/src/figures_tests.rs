//! Tests zu `figures.rs` (Port von `engine/balance/figures_test.go`; `TestAlterBerichtLesbar` gehört mit `ReadSummary` zu
//! RU28.3).

use crate::figures::figures_of;
use crate::run::{Matrix, Report, run};

/// WZ28.4/AC-04 (B-596/AC-07): Die neuen Kennzahlen sind in einem kleinen Szenario plausibel gefüllt.
#[test]
fn neue_kennzahlen_plausibel() {
    let r = run(&Matrix {
        seeds: vec!["1".into(), "2".into()],
        players: vec![2],
        bots: vec!["saver".into()],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    })
    .expect("Lauf");
    let f = figures_of(&r).expect("Kennzahlen");
    assert!(f.runs == 2 && f.nights.len() == 1, "Kennzahlen {f:?}");
    let n = &f.nights[0];
    assert!(
        f.first_wall_tick_median.is_some_and(|t| t > 0.0),
        "Zeit bis zur ersten Mauer fehlt"
    );
    // seit B-652 steht in Nacht 1 oft noch keine Mauer (Freiraum räumen)
    assert!(
        n.night == 1 && n.runs == 2 && (0.0..=100.0).contains(&n.survival),
        "Überlebensquote {n:?}"
    );
    // Einnahmen erst am Morgen von Tag 2, außerhalb des Szenarios
    assert!(n.day_expense > 0.0, "Gold-Bilanz je Phase leer: {n:?}");
    assert!(
        n.resource_in.wood + n.resource_out.wood + n.resource_in.stone + n.resource_out.stone > 0.0,
        "Rohstoff-Fluss leer: {:?} / {:?}",
        n.resource_in,
        n.resource_out
    );
    assert!(
        n.idle_share > 0.0 && n.idle_share <= 100.0,
        "Wartezeit-Anteil {}",
        n.idle_share
    );
    check_days(&r);
    assert!(
        figures_of(&Report {
            matrix: Matrix::default(),
            results: Vec::new()
        })
        .is_none(),
        "ohne gültigen Lauf: None erwartet"
    );
}

/// Bauern gezählt, Nacht-Anteil des Golds nie größer als der ganze Tag.
fn check_days(r: &Report) {
    for res in &r.results {
        let d = &res.metrics.as_ref().expect("Kennzahlen").days[0];
        assert!(
            d.peasant_ticks != 0 && d.night_income <= d.income && d.night_expense <= d.expense,
            "Seed {} Tag 1: {d:?}",
            res.scenario.seed
        );
    }
}
