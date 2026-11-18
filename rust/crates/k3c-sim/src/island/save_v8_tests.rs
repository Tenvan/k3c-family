//! Spielstand Version 8 (Go: `island_save_v8_test.go`, WZ18.3, B-488/AC-03, docs/rules/welten.md § 5): Welt-ID und Insel-ID
//! statt des Index, Migration alter Stände auf die Standard-Welt, unbekannte Welt oder Insel ist ein Fehler. Die Go-Tests, die
//! eine Welt-Datei laden (`LoadWorld`, RU10.5) oder eine Insel bauen und laden (`create_world_with`, `values_of`, RU14),
//! tragen `ignore`; ihre Lese-Seite (Migration, `island_in`, Pflichtfelder) läuft.

#![allow(clippy::unimplemented)] // Rümpfe der ignorierten Tests: warten auf den genannten Sprint

use serde_json::Value;

use super::save_tests::{Env, fixture, world_of};
use super::save_v5::restore_progress;
use super::save_v8::{island_in, legacy_island_id};
use super::testutil::*;
use super::{ISLAND_SAVE_VERSION, Island, IslandSave};
use crate::state::World;

/// Go: `nordmeer` (die Welt mit zwei Inseln), von Hand statt aus einer Welt-Datei.
fn nordmeer() -> super::WorldDef {
    let mut w = world_of(&["island1", "eisinsel"]);
    w.id = "nordmeer".into();
    w
}

/// Go: `TestSpielstandMigrationAufStandard` (Lese-Seite, AC-03): Stände bis Version 7 lesen in die Standard-Welt, Insel am
/// alten Index (fehlt er: island1). Ein Index außerhalb der Standard-Welt ist ein Fehler wie bisher.
#[test]
fn spielstand_migration_auf_standard() {
    let env = Env::standard();
    for v in 1..ISLAND_SAVE_VERSION {
        let s = env.parse(&fixture(v)).unwrap_or_else(|e| panic!("v{v}: {e}"));
        assert_eq!(
            (s.version, s.world.as_str(), s.island_id.as_str()),
            (ISLAND_SAVE_VERSION, "standard", "island1"),
            "v{v}"
        );
    }
    let raw = fixture(7).replacen("\"version\": 7,", "\"version\": 7, \"island\": 1,", 1);
    assert!(raw.contains("\"island\": 1"), "Fixture v7 hat die erwartete Kopfzeile");
    let err = env.parse(&raw).expect_err("v7 mit Index 1");
    assert!(err.to_string().contains("Insel 1"), "v7 mit Index 1: Fehler {err}");
    assert_eq!(legacy_island_id(&nordmeer(), 1).expect("Index 1"), "eisinsel");
    assert_eq!(
        legacy_island_id(&nordmeer(), 2).expect_err("Index 2").to_string(),
        "spielstand: Insel 2, die Daten kennen 2"
    );
    assert!(legacy_island_id(&nordmeer(), -1).is_err(), "negativer Index");
}

/// Go: `TestSpielstandMigrationAufStandard` (Lade-Seite): Die Stände bis Version 7 laden als Insel der Standard-Welt.
#[test]
#[ignore = "wartet auf RU14"]
fn spielstand_migration_auf_standard_insel() {
    let data = test_data();
    for v in 1..ISLAND_SAVE_VERSION {
        let isl = load_island(&data, &fixture(v));
        assert_eq!(
            (isl.world_id(), isl.island_id()),
            ("standard", "island1"),
            "v{v}: Insel aus dem Stand"
        );
    }
}

/// Ein Fall: Name, Welt, Änderung am Stand, erwarteter Fehlertext.
type Case = (&'static str, super::WorldDef, fn(&mut IslandSave), &'static str);

/// Go: `TestSpielstandWeltOderInselUnbekannt` (AC-03): Unbekannte Welt oder Insel lädt nicht; der Fehler nennt Welt und Insel
/// (`island_in`, wie `from_island_save_in` es ruft). Ein Stand der Version 9 ohne `world` oder `islandId` wird abgelehnt.
#[test]
fn spielstand_welt_oder_insel_unbekannt() {
    let env = Env::new();
    let base = env.parse(&fixture(ISLAND_SAVE_VERSION)).expect("v9");
    let cases: [Case; 3] = [
        (
            "Welt unbekannt",
            world_of(&["island1"]),
            |s| s.world = "nordmeer".into(),
            "spielstand: Welt \"nordmeer\" unbekannt",
        ),
        (
            "Insel unbekannt",
            world_of(&["island1"]),
            |s| s.island_id = "eisinsel".into(),
            "spielstand: Insel \"eisinsel\" gibt es in Welt \"standard\" nicht",
        ),
        (
            "andere Welt",
            nordmeer(),
            |_| {},
            "spielstand: Welt \"standard\" unbekannt",
        ),
    ];
    for (name, world, edit, want) in cases {
        let mut s = base.clone();
        edit(&mut s);
        let err = island_in(&world, &s).expect_err(name);
        assert_eq!(err.to_string(), want, "{name}");
    }
    assert_eq!(island_in(&world_of(&["a", "island1"]), &base).expect("gefunden"), 1);
    for field in ["world", "islandId"] {
        let mut m: Value = serde_json::from_str(&fixture(ISLAND_SAVE_VERSION)).expect("JSON");
        m.as_object_mut().expect("Objekt").remove(field);
        let err = env.parse(&m.to_string()).expect_err(field);
        assert!(err.to_string().contains(field), "ohne {field}: Fehler {err}");
    }
}

/// Go: `TestSpielstandWeltOderInselUnbekannt` (Lade-Seite): dieselben Fälle über `from_island_save_in`.
#[test]
#[ignore = "wartet auf RU14"]
fn spielstand_welt_oder_insel_unbekannt_laden() {
    let data = test_data();
    let world = standard_world(&data);
    let defaults = crate::state::pending::values_of(&world).expect("Basis-Wertesatz");
    let t = super::SaveTables {
        data: &data,
        defaults: &defaults,
        standard_world: &world,
    };
    let mut s = super::parse_island_save(&fixture(ISLAND_SAVE_VERSION), &t).expect("v9");
    s.world = "nordmeer".into();
    let err = super::from_island_save_in(&t, &s, ISLAND_SPEED, &world).expect_err("Welt unbekannt");
    assert_eq!(err.to_string(), "spielstand: Welt \"nordmeer\" unbekannt");
}

/// Go: `TestSpielstandV8WeltUndInsel` (AC-03): Version 8 speichert `world` und `islandId`, nicht mehr den Index `island`;
/// Speichern, Laden, Speichern ist gleich (braucht `LoadWorld` RU10.5, `complete_ship`, `create_world_with` RU14).
#[test]
#[ignore = "wartet auf RU14"]
fn spielstand_v8_welt_und_insel() {
    let data = test_data();
    let w = nordmeer_loaded(&data);
    let cur = super::switch_testutil::nordmeer_island(&data, &w, 2);
    let next = super::next_island(&data, &cur, &w.islands[1]).expect("nächste Insel");
    let s = next.to_save("2026-10-19T20:00:00Z").expect("Spielstand");
    let first = serde_json::to_string(&s).expect("Spielstand als JSON");
    assert!(
        s.version == ISLAND_SAVE_VERSION
            && s.world == "nordmeer"
            && s.island_id == "eisinsel"
            && first.contains(r#""world":"nordmeer","islandId":"eisinsel""#)
            && !first.contains(r#""island":"#),
        "Version {}, Welt {:?}, Insel {:?}: {first}",
        s.version,
        s.world,
        s.island_id
    );
    let defaults = crate::state::pending::values_of(&w).expect("Wertesatz");
    let t = super::SaveTables {
        data: &data,
        defaults: &defaults,
        standard_world: &w,
    };
    let parsed = super::parse_island_save(&first, &t).expect("Spielstand lesen");
    let got = super::from_island_save_in(&t, &parsed, ISLAND_SPEED, &w).expect("Insel laden");
    assert!(
        got.world_id() == "nordmeer"
            && got.island_id() == "eisinsel"
            && got.number == 1
            && got.scaling == w.islands[1].depth_scaling
            && got.is_last(),
        "Welt {:?}, Insel {:?} ({}), letzte {}",
        got.world_id(),
        got.island_id(),
        got.number,
        got.is_last()
    );
    let second = serde_json::to_string(&got.to_save("2026-10-19T20:00:00Z").expect("Spielstand")).expect("JSON");
    assert_eq!(first, second, "Speichern, Laden, Speichern ergibt etwas anderes");
}

/// Der Händler im Stand: `saved_merchant` ohne Händler ist `None`; mit Händler in einer Stufe kommt er heraus.
#[test]
fn own_saved_merchant_findet_den_haendler_der_insel() {
    let mut isl = Island {
        stages: vec![World::default(), World::default()],
        ..Island::default()
    };
    assert_eq!(super::save_v5::saved_merchant(&isl), None);
    isl.stages[1].merchant = Some(crate::state::shared::Merchant {
        resource: "stone".into(),
        leaves: 2,
        buy_paid: 1,
        raid: true,
        hp: 60.0,
        max_hp: 100.0,
    });
    let m = super::save_v5::saved_merchant(&isl).expect("Händler");
    assert_eq!(
        (m.resource.as_str(), m.leaves, m.buy_paid, m.raid, m.hp, m.max_hp),
        ("stone", 2, 1, true, 60.0, 100.0)
    );
}

/// Go: `TestSpielstandBossAlteVersionen` (Lese-Seite): Versionen vor 5 lesen mit leerer Bossliste und Zählern 0, die
/// Fixtures ab v5 mit ihrem Boss; `restore_progress` setzt Zähler und Endboss-Flag (ohne Händler braucht es keine Stufe).
#[test]
fn own_alte_versionen_lesen_boss_felder_und_restore_progress() {
    let env = Env::standard();
    for v in 1..5 {
        let s = env.parse(&fixture(v)).unwrap_or_else(|e| panic!("v{v}: {e}"));
        assert!(s.defeated_bosses.is_empty() && s.gold_collected == 0 && !s.won, "v{v}");
    }
    for v in 5..=ISLAND_SAVE_VERSION {
        let mut s = env.parse(&fixture(v)).unwrap_or_else(|e| panic!("v{v}: {e}"));
        assert_eq!(
            (s.defeated_bosses.as_slice(), s.gold_collected),
            (["goblinLeader".to_string()].as_slice(), 37),
            "v{v}"
        );
        s.merchant = None; // der Händler braucht `hub_of` (RU14)
        let mut isl = Island::default();
        restore_progress(&mut isl, &s, &env.data);
        assert_eq!(
            (isl.defeated_bosses.clone(), isl.gold_collected, isl.endboss_defeated),
            (s.defeated_bosses.clone(), 37, false),
            "v{v}"
        );
    }
    let mut s = env.parse(&fixture(ISLAND_SAVE_VERSION)).expect("v9");
    s.merchant = None;
    s.defeated_bosses = env
        .data
        .bosses
        .end_bosses
        .iter()
        .map(|b| b.id.clone())
        .take(1)
        .collect();
    let mut isl = Island::default();
    restore_progress(&mut isl, &s, &env.data);
    assert!(isl.endboss_defeated, "ein Endboss in der Liste setzt das Flag");
}
