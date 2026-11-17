//! RU3/AC-04: Je Datei ist der JSON-Baum der Rust-Struktur gleich dem der Go-Struktur.

use std::collections::BTreeMap;

use crate::testutil::{raw, same_as_go};
use crate::{
    Fehler, balance_hints, balance_targets, biome, bosses, buildings, difficulty, economy, enemies, events, goals,
    guards, hub,
};

fn enemies() -> BTreeMap<String, enemies::EnemyData> {
    enemies::load(&raw(enemies::FILE)).expect("enemies.json")
}

/// Biome der Platte (wie der Go-Test `biomes/*.json`), sortiert nach Namen.
fn biome_ids() -> Vec<String> {
    crate::data::biome_ids(&crate::Source::Dir(crate::testutil::data_dir())).expect("biomes/")
}

fn biomes() -> Vec<biome::Biome> {
    biome_ids()
        .iter()
        .map(|id| biome::load(id, &raw(&format!("biomes/{id}.json"))).expect("Biom"))
        .collect()
}

#[test]
fn balance_hints_wie_go() {
    let h = balance_hints::load_hints(&raw(balance_hints::FILE)).expect("laden");
    same_as_go(balance_hints::FILE, &h);
}

#[test]
fn balance_targets_wie_go() {
    let t = balance_targets::load_targets(&raw(balance_targets::FILE)).expect("laden");
    same_as_go(balance_targets::FILE, &t);
}

#[test]
fn bosses_wie_go() {
    let depths: Vec<i64> = biomes().iter().map(|b| b.depth).collect();
    let b = bosses::load(&raw(bosses::FILE), &enemies(), &depths).expect("laden");
    same_as_go(bosses::FILE, &b);
}

#[test]
fn buildings_wie_go() {
    same_as_go(buildings::FILE, &buildings::load(&raw(buildings::FILE)).expect("laden"));
}

#[test]
fn difficulty_wie_go() {
    same_as_go(
        difficulty::FILE,
        &difficulty::load(&raw(difficulty::FILE)).expect("laden"),
    );
}

#[test]
fn economy_wie_go() {
    same_as_go(economy::FILE, &economy::load(&raw(economy::FILE)).expect("laden"));
}

#[test]
fn enemies_wie_go() {
    same_as_go(enemies::FILE, &enemies());
}

#[test]
fn events_wie_go() {
    let e = events::load(&raw(events::FILE)).expect("laden");
    events::check_night_events(&e.events, &enemies()).expect("Prüfung");
    same_as_go(events::FILE, &e);
}

#[test]
fn goals_wie_go() {
    same_as_go(goals::FILE, &goals::load(&raw(goals::FILE)).expect("laden"));
}

#[test]
fn guards_wie_go() {
    same_as_go(guards::FILE, &guards::load(&raw(guards::FILE)).expect("laden"));
}

#[test]
fn hub_wie_go() {
    same_as_go(hub::FILE, &hub::load(&raw(hub::FILE)).expect("laden"));
}

#[test]
fn biome_wie_go() {
    let en = enemies();
    for (id, b) in biome_ids().iter().zip(biomes()) {
        biome::check_pool(&b, &en).expect("Pool");
        same_as_go(&format!("biomes/{id}.json"), &b);
    }
}

/// Prüfungen beim Laden: Texte wie in Go.
#[test]
fn pruefungen_wie_go() {
    let cases: [(Result<(), Fehler>, &str); 5] = [
        (
            guards::load(r#"{"aggroRadiusUnits": 0, "maxPerChunk": 1, "minGapUnits": 1}"#).map(drop),
            "guards.json: aggroRadiusUnits 0 ist nicht positiv",
        ),
        (
            difficulty::load(r#"{"grades": {"dev": {}}}"#).map(drop),
            "data/difficulty.json: Grad \"easy\" fehlt",
        ),
        (
            enemies::load(r#"{"wolf": {"name": "Wolf", "traits": ["fliegt"]}}"#).map(drop),
            "data/enemies.json: wolf: unbekanntes Trait \"fliegt\"",
        ),
        (
            balance_hints::load_hints(r#"{"version": 2}"#).map(drop),
            "balance-hints: version 2 unbekannt, erwartet 1",
        ),
        (
            events::check_night_events(&[], &BTreeMap::new()),
            "data/events.json: fullMoon, bloodMoon und merchantRaid fehlen",
        ),
    ];
    for (got, want) in cases {
        assert_eq!(got.expect_err(want).to_string(), want);
    }
}

#[test]
fn balance_lehnt_unbekannte_felder_ab() {
    let e = balance_hints::load_hints(r#"{"version": 1, "neu": 1}"#).expect_err("unbekanntes Feld");
    assert!(matches!(e, Fehler::Json(_)), "{e}");
}

#[test]
fn biom_mit_fehlender_nacht_wie_go() {
    let mut v: serde_json::Value = serde_json::from_str(&raw("biomes/forest.json")).expect("json");
    v["cycle"]["nightMinutes"] = serde_json::json!(0);
    let e = biome::load("forest", &v.to_string()).expect_err("Fehler");
    assert_eq!(
        e.to_string(),
        "biomes/forest.json: cycle.nightMinutes fehlt oder ist nicht positiv (0)"
    );
}
