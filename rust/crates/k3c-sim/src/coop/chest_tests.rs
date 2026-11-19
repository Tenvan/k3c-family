//! Doppel-Truhe: Solo, Koop, Ausnahmen (RU62.3, AC-02).
use super::testutil::{add_chest, chest_event, coop_data, coop_island, gold_of, tick};
use crate::state::shared::Crown;

const NORMAL: std::ops::RangeInclusive<i64> = 30..=60; // data/economy.json › chestGold
const DOUBLE: std::ops::RangeInclusive<i64> = 90..=180; // x goldFactor 3

#[test]
fn solo_oeffnet_allein_mit_normalem_gold() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 1);
    add_chest(&mut isl, 0, x);
    let ev = tick(&mut isl, &data);
    let c = chest_event(&ev).expect("Truhe offen");
    assert!(NORMAL.contains(&gold_of(c)), "normales Gold: {c:?}");
    assert!(c.get("double").is_none());
    assert!(isl.stages[0].coop.chests.is_empty() && isl.stages[0].pickups.is_empty());
}

#[test]
fn koop_wartet_und_nennt_den_wartenden() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    let id = add_chest(&mut isl, 0, x);
    isl.stages[0].players[1].x = x + 40.0;
    let ev = tick(&mut isl, &data);
    assert!(chest_event(&ev).is_none(), "bleibt zu");
    let w = &isl.stages[0];
    assert_eq!(w.pickups.len(), 1);
    assert_eq!(w.coop.chests.len(), 1);
    let c = &w.coop.chests[0];
    assert_eq!((c.id, c.double, c.waiting), (id, true, Some(w.players[0].index)));
    let json = serde_json::to_value(w).expect("JSON");
    assert_eq!(json["coop"]["chests"][0]["waiting"], w.players[0].index);
    assert_eq!(json["coop"]["chests"][0]["double"], true);
}

#[test]
fn koop_ohne_wartenden_zeigt_nur_das_kennzeichen() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    add_chest(&mut isl, 0, x);
    for p in &mut isl.stages[0].players {
        p.x = x + 40.0;
    }
    tick(&mut isl, &data);
    let c = &isl.stages[0].coop.chests[0];
    assert!(c.double && c.waiting.is_none());
    let json = serde_json::to_value(&isl.stages[0]).expect("JSON");
    assert!(json["coop"]["chests"][0].get("waiting").is_none());
}

#[test]
fn koop_oeffnet_mit_gold_mal_faktor_und_zusatz_fuer_beide() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    add_chest(&mut isl, 0, x);
    let ev = tick(&mut isl, &data);
    let c = chest_event(&ev).expect("Truhe offen");
    let gold = gold_of(c);
    assert!(DOUBLE.contains(&gold), "Gold x Faktor: {gold}");
    assert_eq!(c.get("double").and_then(serde_json::Value::as_bool), Some(true));
    assert_eq!(isl.chests_opened, 2, "chest_skill_point je Monarch");
    assert_eq!(isl.stages[0].coop.chests.len(), 0);
    assert!(isl.stages[0].pickups.is_empty());
    assert_eq!(isl.stages[0].coins.len() as i64, gold, "Fontäne wirft alles aus");
}

#[test]
fn mehr_als_zwei_spieler_zwei_genuegen() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 3);
    add_chest(&mut isl, 0, x);
    isl.stages[0].players[2].x = x + 40.0;
    let ev = tick(&mut isl, &data);
    assert!(chest_event(&ev).is_some_and(|c| DOUBLE.contains(&gold_of(c))));
}

#[test]
fn monarch_ohne_krone_zaehlt_mit() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    add_chest(&mut isl, 0, x);
    isl.stages[0].players[1].crown = Some(Crown::default());
    let ev = tick(&mut isl, &data);
    assert!(chest_event(&ev).is_some_and(|c| DOUBLE.contains(&gold_of(c))));
}

#[test]
fn sinkende_spielerzahl_macht_die_truhe_normal() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    add_chest(&mut isl, 0, x);
    isl.stages[0].players[1].x = x + 40.0;
    assert!(chest_event(&tick(&mut isl, &data)).is_none());
    isl.stages[0].players[1].free = true; // niemand steuert ihn mehr
    let ev = tick(&mut isl, &data);
    let c = chest_event(&ev).expect("öffnet allein");
    assert!(NORMAL.contains(&gold_of(c)), "normales Gold: {c:?}");
    assert!(c.get("double").is_none());
}

#[test]
fn kommt_ein_zweiter_spieler_ist_sie_wieder_doppelt() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    isl.stages[0].players[1].free = true;
    add_chest(&mut isl, 0, x);
    for p in &mut isl.stages[0].players {
        p.x = x + 40.0; // niemand davor
    }
    tick(&mut isl, &data);
    assert!(isl.stages[0].coop.chests.is_empty(), "solo: keine Doppel-Truhe");
    isl.stages[0].players[1].free = false;
    tick(&mut isl, &data);
    assert_eq!(isl.stages[0].coop.chests.len(), 1);
}

#[test]
fn verschiedene_stufen_bleiben_zu() {
    let data = coop_data();
    let (mut isl, x) = coop_island(&data, 2);
    add_chest(&mut isl, 0, x);
    let p = isl.stages[0].players.remove(1);
    isl.stages[1].players.push(p);
    let ev = tick(&mut isl, &data);
    assert!(chest_event(&ev).is_none(), "ein Monarch je Stufe: zu");
    assert_eq!(isl.stages[0].coop.chests.len(), 1);
}

#[test]
fn stufe_ohne_truhe_hat_keine_doppel_truhe() {
    let data = coop_data();
    let (mut isl, _) = coop_island(&data, 2);
    tick(&mut isl, &data);
    assert!(isl.stages.iter().all(|w| w.coop.chests.is_empty()));
}

#[test]
fn welt_ohne_koop_regeln_bleibt_normal() {
    let mut data = coop_data();
    data.coop.worlds.clear(); // wie `standard`: Welt nicht in coop.json
    let (mut isl, x) = coop_island(&data, 2);
    assert!(isl.stages[0].coop_cfg.is_none());
    add_chest(&mut isl, 0, x);
    isl.stages[0].players[1].x = x + 40.0;
    let ev = tick(&mut isl, &data);
    let c = chest_event(&ev).expect("öffnet allein");
    assert!(NORMAL.contains(&gold_of(c)));
    let json = serde_json::to_value(&isl.stages[0]).expect("JSON");
    assert!(json.get("coop").is_none(), "Zustand unverändert");
}
