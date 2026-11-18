//! Go: `events_moon_stages_test.go` (KB1.1, B-379/AC-01, B-380/AC-01): Vollmond-Belohnung je bespielter Stufe.

use super::moon::night_events_end;
use super::values::night;
use crate::island::testutil::{must_island, test_data};
use crate::island::{Island, add_island_player};
use crate::state::CycleInfo;

/// Go: `fullMoonReward`: Münzen, die der Tagesanbruch nach Vollmond-Nacht 7 in jeder Stufe der Insel streut.
fn full_moon_reward(isl: &mut Island) -> Vec<usize> {
    isl.stages
        .iter_mut()
        .map(|w| {
            w.cycle = CycleInfo {
                phase: "day".into(),
                day: 8,
                ..Default::default()
            };
            let before = w.coins.len();
            night_events_end(w);
            w.coins.len() - before
        })
        .collect()
}

#[test]
fn vollmond_belohnung_in_der_bespielten_stufe() {
    let data = test_data();
    let mut isl = must_island(&data, "mond", &[0, 1]);
    let reward = night(&isl.stages[0], "fullMoon").expect("fullMoon").reward_gold as usize;
    add_island_player(&mut isl, 1);
    add_island_player(&mut isl, 1);
    assert_eq!(full_moon_reward(&mut isl), vec![0, reward], "beide in Stufe 1");
    isl.stages[1].castle_fell_night = 7; // Burg der Stufe 1 fiel, Stufe 0 hielt
    assert_eq!(full_moon_reward(&mut isl), vec![0, 0], "Burg der Stufe 1 gefallen");
}

#[test]
fn vollmond_belohnung_je_stufe_mit_spielern() {
    let data = test_data();
    let mut isl = must_island(&data, "mond", &[0, 1, 2]);
    let reward = night(&isl.stages[0], "fullMoon").expect("fullMoon").reward_gold as usize;
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 2);
    assert_eq!(
        full_moon_reward(&mut isl),
        vec![reward, 0, reward],
        "Spieler in Stufe 0 und 2"
    );
    isl.stages[2].players[0].free = true; // getrennter Monarch spielt nicht
    assert_eq!(
        full_moon_reward(&mut isl)[2],
        0,
        "nur ein getrennter Monarch in Stufe 2"
    );
}
