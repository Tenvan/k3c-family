//! Tests zu `merchant/mod.rs` (Go: `merchant_raid_target_test.go`, K3.2a, B-373, Sprint K3 AC-04, AC-05): Der Händler ist eine
//! angreifbare Figur, die Insel zählt seine Besuche, beides übersteht Speichern und Laden.
//!
//! Die Tests mit Ankunft über `merchant_dawn` brauchen `pure`, `emit` und `unit_x` aus RU19 und tragen `ignore`. Der
//! Spielstand-Test lädt über `load_island` (RU19: `decode_world_story`). `haendler_spielstand_prueft` läuft mit dem Stand
//! der Fixture Version 9 statt Version 6 (die Prüfung ist dieselbe).

use super::testutil::*;
use super::*;
use crate::enemies::target::{TargetKind, attack, choose_target};
use crate::enemies::testutil::{build_site, spawn};
use crate::island::testutil::{count, load_island, must_island, save_json, test_data};
use crate::island::{SaveTables, parse_island_save, validate_island_save};
use crate::monarch::new_skill_table;
use crate::state::common::apply_damage_by;
use crate::world::values_room::default_values;

/// (a) Jede Ankunft zählt einen Besuch, ohne und mit Taverne.
#[test]
fn haendler_besuche_zaehlen() {
    let data = test_data();
    let every = default_values().economy.merchant.every_days;
    let mut isl = must_island(&data, "haendler", &[0, 1]);
    for day in 1..=3 * every {
        island_arrives(&mut isl, &data, 0, day);
    }
    assert_eq!(isl.merchant_visits, 3, "drei Ankünfte ohne Taverne erwartet");
    build_site(&mut isl.stages[0], "tavern");
    let mut day = 3 * every + 1;
    while isl.merchant_visits < 5 && day < 100 {
        island_arrives(&mut isl, &data, 0, day);
        day += 1;
    }
    let hp = default_values().economy.merchant.hp;
    let ok = isl.stages[0].merchant.as_ref().is_some_and(|m| m.hp == hp);
    assert!(
        isl.merchant_visits == 5 && ok,
        "mit Taverne fünf Besuche und Händler mit {hp} HP erwartet: {}, {:?}",
        isl.merchant_visits,
        isl.stages[0].merchant
    );
}

/// (b) Ein Gegner in Reichweite greift den Händler an; ohne Händler ist er kein Ziel.
#[test]
fn haendler_wird_angegriffen() {
    let m = default_values().economy.merchant.clone();
    let mut w = merchant_world(0);
    merchant_arrives(&mut w, None, m.every_days);
    let x = merchant_x(&w, "buy");
    let e = spawn(&mut w, "wolf", x);
    let v = vals(&w);
    let c = choose_target(&w, &v, e, 1.0, None).expect("Händler als Ziel erwartet");
    assert_eq!(c.kind, TargetKind::Merchant);
    attack(&mut w, &v, e, &c, None);
    let want = m.hp - w.enemies[e].damage;
    assert_eq!(w.merchant.as_ref().map(|mm| mm.hp), Some(want), "Händler-HP");
    w.merchant = None;
    let c = choose_target(&w, &v, e, 1.0, None);
    assert!(
        c.is_none_or(|c| c.kind != TargetKind::Merchant),
        "ohne Händler kein Händler-Ziel"
    );
}

/// (c) Bei 0 HP flieht der Händler (`merchantFled`), ein halber Kauf fällt zu Boden; der nächste Besuch kommt normal.
#[test]
fn haendler_flieht_bei_null_hp() {
    let data = test_data();
    let m = default_values().economy.merchant.clone();
    let mut isl = must_island(&data, "haendler", &[0]);
    island_arrives(&mut isl, &data, 0, m.every_days);
    let w = &mut isl.stages[0];
    w.merchant.as_mut().expect("Händler").buy_paid = 2;
    let coins = w.coins.len();
    w.events.clear();
    apply_damage_by(w, MERCHANT_ID, m.hp, "wolf");
    assert!(
        w.merchant.is_none() && count(&w.events, "merchantFled") == 1 && w.coins.len() == coins + 2,
        "Flucht mit zwei Münzen erwartet: Händler {:?}, Ereignisse {:?}",
        w.merchant,
        w.events
    );
    island_arrives(&mut isl, &data, 0, 2 * m.every_days);
    let mm = isl.stages[0].merchant.as_ref();
    assert!(
        mm.is_some_and(|mm| mm.hp == m.hp) && isl.merchant_visits == 2,
        "nächster Besuch mit vollen HP erwartet: {mm:?}, Besuche {}",
        isl.merchant_visits
    );
}

/// (d) Besuchszähler und anwesender Händler mit HP überstehen Speichern und Laden; Version 5 lädt mit 0 ohne Händler.
#[test]
fn haendler_im_spielstand() {
    let data = test_data();
    let every = default_values().economy.merchant.every_days;
    let mut isl = must_island(&data, "haendler", &[0, 1]);
    crate::island::add_island_player(&mut isl, 0);
    island_arrives(&mut isl, &data, 0, every);
    isl.stages[0].merchant.as_mut().expect("Händler").hp = 40.0;
    let raw = save_json(&isl, "2026-10-09T20:00:00Z");
    let loaded = load_island(&data, &raw);
    let m = loaded.stages[0].merchant.as_ref();
    assert!(
        loaded.merchant_visits == 1 && m.is_some_and(|m| m.hp == 40.0) && m == isl.stages[0].merchant.as_ref(),
        "Besuche {}, Händler {m:?}",
        loaded.merchant_visits
    );
    assert!(loaded.stages[1].merchant.is_none(), "Händler nur in seiner Tiefe");
    let old = load_island(&data, &read_fixture(5));
    assert!(
        old.merchant_visits == 0 && old.stages[0].merchant.is_none(),
        "v5: Besuche {}",
        old.merchant_visits
    );
    let v6 = load_island(&data, &read_fixture(6));
    assert!(
        v6.merchant_visits == 3 && v6.stages[0].merchant.as_ref().is_some_and(|m| m.hp == 60.0),
        "v6-Fixture: Besuche {}, Händler {:?}",
        v6.merchant_visits,
        v6.stages[0].merchant
    );
}

/// Ungültige Händler-Werte im Spielstand werden abgelehnt.
#[test]
fn haendler_spielstand_prueft() {
    let data = test_data();
    let mut defaults = crate::enemies::testutil::test_values();
    defaults.skills = new_skill_table(&data.monarch.skills);
    let world = crate::island::WorldDef::default();
    let t = SaveTables {
        data: &data,
        defaults: &defaults,
        standard_world: &world,
    };
    let mut s = parse_island_save(&read_fixture(9), &t).expect("Version 9 lesen");
    s.merchant = Some(crate::island::MerchantSave {
        resource: "wood".into(),
        leaves: 4,
        hp: 0.0,
        max_hp: 100.0,
        ..Default::default()
    });
    assert!(
        validate_island_save(&s, None, &t).is_err(),
        "Händler mit 0 HP im Stand wird abgelehnt"
    );
    (s.merchant, s.merchant_visits) = (None, -1);
    assert!(
        validate_island_save(&s, None, &t).is_err(),
        "negativer Besuchszähler wird abgelehnt"
    );
}
