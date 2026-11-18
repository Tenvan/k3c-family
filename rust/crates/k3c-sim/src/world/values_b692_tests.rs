//! Port der übrigen Tests aus `engine/sim/world_values_b692_test.go` (Review BAL8.2, B-692); `TestB692GesperrteFelder`
//! und `TestB692ResolveWorldFilesPrueftBasis` stehen in `values_tests.rs`. Die Fälle mit Standard-Welt (`story`: RU19),
//! Skill-Katalog (`CanLearn`, `LearnSkill`: RU18.4) oder Ruinen (`ruin_site`: RU18) tragen `ignore`.
//!
//! Abweichung: Rust hat kein `UseData` (keine Paket-Variablen); `TestB692UseDataFrischMitGoalsUndSkills` prüft hier, dass
//! `default_values_from` aus geänderten Daten einen frischen Wertesatz baut (goals, Skill-Katalog, Gebäude) und der Basis-
//! Wertesatz unberührt bleibt.

use std::sync::Arc;

use k3c_data::{Source, load_all};
use serde_json::json;

use super::base::base;
use super::pending::ruin_site;
use super::values_room::{default_values, default_values_from, resolve_world, values};
use crate::island::testutil::{standard_world, test_data};
use crate::island::worlds::STANDARD_WORLD_ID;
use crate::island::{CreateOptions, Island, WorldDef, add_island_player, create_island_in};
use crate::state::save::HubSave;
use crate::state::save_economy::validate_economy;
use crate::state::{World, WorldValues};

/// Go: `overrideWorld`: erbt von der Standard-Welt und übersteuert mit `ov` (Schlüssel = Basis-Datei).
fn override_world(id: &str, ov: &[(&str, serde_json::Value)]) -> WorldDef {
    let data = test_data();
    let mut w = standard_world(&data);
    (w.id, w.name, w.extends) = (id.into(), id.into(), STANDARD_WORLD_ID.into());
    w.overrides = ov.iter().map(|(k, v)| ((*k).to_string(), v.clone())).collect();
    let lookup = |wid: &str| -> Result<WorldDef, String> {
        if wid == STANDARD_WORLD_ID {
            Ok(standard_world(&test_data()))
        } else {
            Err(format!("Welt {wid:?} unbekannt"))
        }
    };
    let b = base().unwrap_or_else(|e| panic!("{e}"));
    resolve_world(&w, &Source::Embedded, &b.data, &b.rules, &lookup).unwrap_or_else(|e| panic!("{e}"))
}

fn island_of(world: &WorldDef, seed: &str) -> Island {
    let depths = world.islands[0].depths.clone();
    create_island_in(&test_data(), world, seed, &depths, 1.0, CreateOptions::default())
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Go: `CanLearn` (monarch.go), RU18.4.
fn can_learn(_w: &World, _pi: usize, _id: &str) -> Result<(), String> {
    panic!("RU18: can_learn")
}

/// Go: `skillByID` (monarch_data.go), RU18.4.
fn skill_tier(_v: &WorldValues, _id: &str) -> i64 {
    panic!("RU18: skill_by_id")
}

/// Punkt 1: Overrides auf `monarch.skills.*` wirken im Skill-Katalog der Welt (CanLearn), die Basis bleibt.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn b692_skill_override_wirkt_je_welt() {
    let world = override_world("skill", &[("monarch", json!({"skills": {"ironWall": {"tier": 1}}}))]);
    let mut isl = island_of(&world, "skill");
    let pi = add_island_player(&mut isl, 0);
    let w = &mut isl.stages[0];
    crate::island::testutil::add_pool_points(w, 1);
    assert_eq!(skill_tier(&values(w), "ironWall"), 1, "Welt: ironWall Tier");
    assert!(can_learn(w, pi, "ironWall").is_ok(), "Welt mit Tier 1");
    assert_eq!(skill_tier(&default_values(), "ironWall"), 3, "Basis: ironWall Tier");
    let mut base = crate::enemies::testutil::quiet_world();
    let bp = crate::world::add_player(&mut base);
    crate::island::testutil::add_pool_points(&mut base, 1);
    assert!(
        can_learn(&base, bp, "ironWall").is_err(),
        "Basis: ironWall ohne Tier-Gating lernbar"
    );
}

/// Go: `LearnSkill` (monarch.go), RU18.4.
fn learn_skill(_w: &mut World, _pi: usize, _id: &str) -> Result<(), String> {
    panic!("RU18: learn_skill")
}

/// Punkt 2: Der Spielstand wird gegen den Wertesatz seiner Welt geprüft (Tier-Gating aus monarch.tierPoints).
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn b692_spielstand_pruefung_je_welt() {
    let world = override_world("leicht", &[("monarch", json!({"tierPoints": [0, 0, 0, 0]}))]);
    let mut isl = island_of(&world, "leicht");
    let pi = add_island_player(&mut isl, 0);
    crate::island::testutil::add_pool_points(&mut isl.stages[0], 1);
    learn_skill(&mut isl.stages[0], pi, "ironWall").unwrap_or_else(|e| panic!("{e}"));
    let data = test_data();
    let std_world = standard_world(&data);
    let defaults = super::values_room::values_of(&std_world).unwrap_or_else(|e| panic!("{e}"));
    let t = crate::island::SaveTables {
        data: &data,
        defaults: &defaults,
        standard_world: &std_world,
    };
    let raw = crate::island::testutil::save_json(&isl, "");
    let s = crate::island::parse_island_save(&raw, &t).unwrap_or_else(|e| panic!("{e}")); // Parsen prüft ohne Welt
    let r = crate::island::from_island_save_in(&t, &s, 1.0, &world);
    assert!(r.is_ok(), "Welt mit tierPoints 0: {:?}", r.err());
    let r = crate::island::from_island_save(&t, &s, 1.0);
    assert!(
        r.as_ref().err().is_some_and(|e| e.to_string().contains("Tier-Gating")),
        "Standard-Welt: Tier-Gating erwartet, war {:?}",
        r.err()
    );
}

/// Punkt 2: `validate_economy` prüft die Rüstungsstufen gegen den übergebenen Wertesatz, nicht gegen die Basis.
#[test]
fn b692_ruestungsstufe_je_wertesatz() {
    let mut v = (*default_values()).clone();
    if let Some(armory) = v.buildings.get_mut("armory") {
        armory.armor.truncate(1);
    }
    let h = HubSave {
        armor_level: 2,
        ..HubSave::default()
    };
    assert!(
        validate_economy(&v, &h).is_err(),
        "Wertesatz mit 1 Rüstungsstufe: Stufe 2 angenommen"
    );
    let base = default_values();
    assert!(validate_economy(&base, &h).is_ok(), "Basis mit 2 Rüstungsstufen");
}

/// Punkt 3: Die Start-HP einer Ruine folgen den HP der Art im Wertesatz der Welt.
#[test]
#[ignore = "wartet auf RU19 (decode_world_story)"]
fn b692_ruinen_hp_je_welt() {
    let world = override_world("ruine", &[("buildings", json!({"smithy": {"hp": 400}}))]);
    let mut isl = island_of(&world, "ruine");
    let rules = Arc::clone(&base().unwrap_or_else(|e| panic!("{e}")).rules);
    let want = (rules.ruin.hp_share * 400.0).ceil();
    assert_eq!(ruin_site(&mut isl.stages[0], "smithy", 10.0).hp, want, "Ruine smithy");
    let mut quiet = crate::enemies::testutil::quiet_world();
    let base_hp = k3c_level::ruins::ruin_hp(&rules, default_values().buildings["smithy"].hp);
    assert_eq!(ruin_site(&mut quiet, "smithy", 10.0).hp, base_hp, "Basis");
}

/// Punkt 4 (Rust-Gegenstück zu `UseData`): Ein Wertesatz aus geänderten Daten ist frisch (goals, Skill-Katalog,
/// Gebäude), der Basis-Wertesatz bleibt.
#[test]
fn b692_default_values_frisch_mit_goals_und_skills() {
    let before = default_values();
    let (wall_hp, gold) = (before.buildings["wall"].hp, before.goals.gold);
    let mut data = load_all(&Source::Embedded).unwrap_or_else(|e| panic!("{e}"));
    data.goals.gold = 77;
    if let Some(s) = data.monarch.skills.get_mut("ironWall") {
        s.tier = 1;
    }
    let mut wall_x = data.buildings["wall"].clone();
    wall_x.hp = 1.0;
    data.buildings.insert("wallX".into(), wall_x);
    let now = default_values_from(&data).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        now.goals.gold == 77 && gold == 66,
        "goals: Gold {} (vorher {gold})",
        now.goals.gold
    );
    let tier = |v: &WorldValues| v.skills.index.get("ironWall").map(|&i| v.skills.list[i].tier);
    assert_eq!(tier(&now), Some(1), "Skill-Katalog nicht neu gebaut");
    assert_eq!(
        now.skills.list.iter().find(|s| s.id == "ironWall").map(|s| s.tier),
        Some(1),
        "ID im Katalog"
    );
    assert!(
        now.buildings.contains_key("wallX") && now.buildings.contains_key("ship"),
        "Gebäude samt ship"
    );
    assert!(
        !before.buildings.contains_key("wallX") && before.buildings["wall"].hp == wall_hp,
        "Basis geändert"
    );
    assert_eq!(tier(&before), Some(3), "Skill-Katalog der Basis geändert");
}

/// Punkt 6: Das Test-Gerüst `World::default()` bekommt immer denselben Basis-Wertesatz, keine Kopie je Aufruf.
#[test]
fn b692_wertesatz_ohne_create_world_gemerkt() {
    let w = World::default();
    let first = values(&w);
    assert!(
        Arc::ptr_eq(&first, &values(&w)),
        "values() liefert je Aufruf einen neuen Wertesatz"
    );
    assert!(!first.buildings.is_empty(), "Basis-Wertesatz leer");
}
