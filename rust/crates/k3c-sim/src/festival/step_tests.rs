//! Zustand `festival` und Ereignis `festivalStart` (RU62.12, AC-06): genau einmal je Sieg, in allen Stufen, bis zur Abfahrt;
//! Welten ohne Eintrag und Ziele ohne Endboss bleiben wie heute.
use super::testutil::{count, festival_data, festival_island, run, tick};
use crate::island::testutil::test_data;
use crate::island::{add_island_player, next_island};

/// AC-06: Vor dem Sieg gibt es weder Zustand noch Ereignis; mit dem Sieg nennt jede Stufe das Fest mit dem Effekt der Welt,
/// `festivalStart` kommt in Stufe 0 genau einmal und das Fest bleibt, solange die Insel steht.
#[test]
fn festival_zustand_und_start_genau_einmal() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    let ev = run(&mut isl, &data, 2.0);
    assert_eq!(count(&ev, "festivalStart"), 0);
    assert!(isl.stages.iter().all(|w| w.festival.is_none() && !w.peace));
    isl.endboss_defeated = true; // wie der Sieg in `step_endboss` (`put_stage_ref`)
    let ev = tick(&mut isl, &data);
    assert_eq!(count(&ev, "festivalStart"), 1);
    let start = isl.stages[0]
        .events
        .iter()
        .find(|e| e["type"] == "festivalStart")
        .expect("Ereignis");
    assert_eq!(start["effect"], "lanterns");
    assert!(
        isl.stages
            .iter()
            .all(|w| w.festival.as_ref().is_some_and(|f| f.effect == "lanterns"))
    );
    let ev = run(&mut isl, &data, 30.0);
    assert_eq!(count(&ev, "festivalStart"), 0, "nur einmal");
    assert!(isl.stages.iter().all(|w| w.festival.is_some() && w.peace));
    let json = serde_json::to_value(&isl.stages[0]).expect("Zustand");
    assert_eq!(json["festival"]["effect"], "lanterns");
}

/// AC-06: Der Frieden gilt in beiden Stufen, auch wenn zwei Spieler auf verschiedenen Stufen stehen.
#[test]
fn frieden_in_beiden_stufen_mit_zwei_spielern() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    add_island_player(&mut isl, 1);
    isl.endboss_defeated = true;
    tick(&mut isl, &data);
    tick(&mut isl, &data);
    assert_eq!((isl.stages[0].players.len(), isl.stages[1].players.len()), (1, 1));
    assert!(isl.stages.iter().all(|w| w.peace && w.festival.is_some()));
}

/// AC-06: Nach `next_island` ist die neue Insel nicht friedlich und meldet kein Fest; die Regel der Welt gilt weiter.
#[test]
fn neue_insel_ist_nicht_friedlich() {
    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    isl.endboss_defeated = true;
    tick(&mut isl, &data);
    assert!(isl.festival_active());
    let def = isl.next_def().expect("Ziel");
    let mut next = next_island(&data, &isl, &def).expect("nächste Insel");
    assert!(!next.endboss_defeated && !next.festival_active());
    let ev = run(&mut next, &data, 2.0);
    assert_eq!(count(&ev, "festivalStart"), 0);
    assert!(next.stages.iter().all(|w| w.festival.is_none() && !w.peace));
    assert!(
        next.festival.rule.is_some(),
        "die Welt hat weiter ein Fest für ihren nächsten Sieg"
    );
    next.endboss_defeated = true;
    let ev = tick(&mut next, &data);
    assert_eq!(count(&ev, "festivalStart"), 1, "ein neuer Sieg feiert wieder");
}

/// AC-06: Eine Welt außerhalb von festival.json und ein Ziel ohne Endboss haben weder Frieden noch Fest.
#[test]
fn ohne_eintrag_oder_ziel_kein_fest() {
    let data = test_data();
    let (_, mut isl) = festival_island(&data, 1);
    isl.endboss_defeated = true;
    let ev = run(&mut isl, &data, 2.0);
    assert_eq!(count(&ev, "festivalStart"), 0);
    assert!(isl.stages.iter().all(|w| w.festival.is_none() && !w.peace));
    assert!(isl.festival.rule.is_none());

    let data = festival_data();
    let (_, mut isl) = festival_island(&data, 1);
    isl.options.goal = "gold".to_string();
    isl.endboss_defeated = true;
    let ev = run(&mut isl, &data, 2.0);
    assert_eq!(count(&ev, "festivalStart"), 0);
    assert!(isl.stages.iter().all(|w| w.festival.is_none() && !w.peace));
}
