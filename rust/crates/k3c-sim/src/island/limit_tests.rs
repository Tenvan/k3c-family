//! EQ1.4 (B-1208): Der Bogner steht nur ganz oben, das Truppen-Limit gilt für die ganze Insel (nur native Regeln).

use std::sync::Arc;

use super::testutil::*;
use crate::state::Troop;
use crate::state::links::{barracks_bonus, island_fighters, island_limit, troop_limit};

fn warrior(x: f64) -> Troop {
    Troop {
        kind: "warrior".into(),
        x,
        hp: 10.0,
        max_hp: 10.0,
        ..Troop::default()
    }
}

/// AC-01: Bogner und Taverne nur in Stufe 0, die Kaserne (Kampfgebäude) auch unten.
#[test]
fn bogner_steht_nur_ganz_oben() {
    let data = test_data();
    let isl = must_island(&data, "bogner-oben", &[0, 1]);
    let has = |i: usize, k: &str| isl.stages[i].sites.iter().any(|s| s.kind == k);
    assert!(has(0, "bowyer") && has(0, "tavern"), "oben: Produktion");
    assert!(!has(1, "bowyer"), "unten kein Bogner");
    assert!(!has(1, "tavern"), "unten keine Taverne");
    assert!(has(1, "barracks"), "unten bleiben Kampfgebäude");
}

/// AC-02: Das Limit zählt Kämpfer aller Stufen; die Basis gilt einmal, der Zuschlag je gebauter Kaserne in jeder Stufe.
#[test]
fn limit_zaehlt_ueber_alle_stufen() {
    let data = test_data();
    let mut isl = must_island(&data, "limit-insel", &[0, 1]);
    let id = isl.stages[0].new_id();
    isl.stages[0].troops.push(Troop { id, ..warrior(0.0) });
    let id = isl.stages[0].new_id();
    isl.stages[0].troops.push(Troop { id, ..warrior(1.0) });
    let base = troop_limit(&isl.stages[1]);
    for i in 0..2 {
        let r = isl.stage_ref(i, &data.difficulty);
        isl.put_stage_ref(r);
    }
    let own0 = isl.stages[0].troops.iter().filter(|t| t.kind == "warrior").count() as i64;
    assert_eq!(
        isl.stages[1].fighters_elsewhere,
        super::errand::island_totals(&isl).0 - own1(&isl)
    );
    assert!(
        isl.stages[1].fighters_elsewhere >= own0,
        "unten zählt die obere Stufe mit"
    );
    assert_eq!(island_limit(&isl.stages[1]), base, "Basis nur einmal");

    let bar = isl.stages[1]
        .sites
        .iter()
        .position(|s| s.kind == "barracks")
        .expect("Kaserne unten");
    isl.stages[1].sites[bar].state = "built".into();
    let bonus = barracks_bonus(&isl.stages[1]);
    assert!(bonus > 0);
    for i in 0..2 {
        let r = isl.stage_ref(i, &data.difficulty);
        isl.put_stage_ref(r);
    }
    assert_eq!(
        island_limit(&isl.stages[0]),
        troop_limit(&isl.stages[0]) + bonus,
        "oben zählt die untere Kaserne"
    );
    assert!(island_fighters(&isl.stages[0]) >= own0);
}

fn own1(isl: &super::Island) -> i64 {
    crate::state::links::fighters(&isl.stages[1])
}

/// AC-03: Mit Go-Regeln gilt das Limit je Stufe.
#[test]
fn ohne_native_regeln_gilt_das_limit_je_stufe() {
    let data = test_data();
    let mut isl = must_island(&data, "limit-go", &[0, 1]);
    let mut v = (*isl.values()).clone();
    v.native_rules = false;
    let v = Arc::new(v);
    isl.stages.iter_mut().for_each(|w| w.vals = Some(Arc::clone(&v)));
    let id = isl.stages[0].new_id();
    isl.stages[0].troops.push(Troop { id, ..warrior(0.0) });
    let r = isl.stage_ref(1, &data.difficulty);
    isl.put_stage_ref(r);
    assert_eq!(
        (isl.stages[1].fighters_elsewhere, isl.stages[1].limit_elsewhere),
        (0, 0)
    );
}

/// Review EQ1.4: Ein Bürger, der in Stufe 0 ein Schwert holt, zählt unten für das Insel-Limit mit.
#[test]
fn holer_anderer_stufen_zaehlen_mit() {
    let data = test_data();
    let mut isl = must_island(&data, "limit-holer", &[0, 1]);
    let si = isl.stages[0]
        .sites
        .iter()
        .position(|s| s.kind == "workshop")
        .expect("Werkstatt");
    isl.stages[0].sites[si].state = "built".into();
    isl.stages[0].sites[si].swords = 1;
    let site_id = isl.stages[0].sites[si].id;
    let id = isl.stages[0].new_id();
    isl.stages[0].troops.push(Troop {
        id,
        kind: "citizen".into(),
        job: Some(crate::state::Job {
            r#type: "fetchSword".into(),
            site_id,
            ..Default::default()
        }),
        ..Troop::default()
    });
    let before = {
        let r = isl.stage_ref(1, &data.difficulty);
        isl.put_stage_ref(r);
        isl.stages[1].fighters_elsewhere
    };
    isl.stages[0].troops.last_mut().expect("Bürger").job = None;
    let r = isl.stage_ref(1, &data.difficulty);
    isl.put_stage_ref(r);
    assert_eq!(before, isl.stages[1].fighters_elsewhere + 1, "Holer zählt mit");
}
