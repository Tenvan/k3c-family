//! Tests zu `world_hub_sites_test.go` (B-983): `hub.sites` und `hub.islandSites` sind übersteuerbar; Level, Prüfung und
//! Stufe rechnen mit dem Wertesatz der Welt. `sitePlacement` bleibt gesperrt.

use std::collections::BTreeMap;

use k3c_data::Source;
use k3c_data::hub::{HubData, HubSite};
use k3c_level::level::generate_with;
use k3c_level::planned_gen::Options as LevelOptions;
use k3c_level::types::{PlannedBuilding, PlannedChunk};
use serde_json::json;

use super::sites::RULE_BOUNDS;
use super::testutil::{hub, plain_world, validate};
use super::validate::{HubReport, RULE_PLANNED, validate_hub_data_in};
use crate::island::testutil::test_data;
use crate::island::worlds::STANDARD_WORLD_ID;
use crate::island::{CreateOptions, WorldDef, add_island_player, create_island_in};
use crate::state::World;
use crate::world::base::base;
use crate::world::level_sites;
use crate::world::testutil::quiet_world;
use crate::world::values_room::resolve_world;

/// Go: `standardLookup`: nur die Standard-Welt ist bekannt.
fn standard_lookup(id: &str) -> Result<WorldDef, String> {
    if id == STANDARD_WORLD_ID {
        Ok(plain_world(STANDARD_WORLD_ID, &[0, 1, 2, 3, 4]))
    } else {
        Err(format!("Welt {id:?} unbekannt"))
    }
}

/// Go: `ResolveWorld(w, standardLookup)` einer Welt `plaetze`, die die hub.json übersteuert.
fn resolve_hub_override(over: serde_json::Value) -> Result<WorldDef, String> {
    let mut w = plain_world("plaetze", &[0, 1, 2, 3, 4]);
    w.extends = STANDARD_WORLD_ID.into();
    w.overrides = BTreeMap::from([("hub".to_string(), over)]);
    let b = base()?;
    resolve_world(&w, &Source::Embedded, &b.data, &b.rules, &standard_lookup)
}

/// Go: `siteX`: Lage des ersten Platzes der Art (ohne Ruinen).
fn site_x(w: &World, kind: &str) -> Option<f64> {
    w.sites.iter().find(|s| s.kind == kind && !s.ruin).map(|s| s.x)
}

/// AC-01: Welt mit geänderten Plätzen: Level und Simulation zeigen denselben Platz, die Basis bleibt.
#[test]
#[allow(clippy::float_cmp)] // Lage exakt wie in Go
#[ignore = "wartet auf RU18 (values_from: check_presets, RU18.4)"]
fn welt_plaetze_wirken_in_level_und_sim() {
    let world = resolve_hub_override(json!({"sites": {"1": {"offsetUnits": -32}, "5": {"offsetUnits": -28}}}))
        .unwrap_or_else(|e| panic!("{e}"));
    let depths = world.islands[0].depths.clone();
    let mut isl = create_island_in(&test_data(), &world, "plaetze", &depths, 1.0, CreateOptions::default())
        .unwrap_or_else(|e| panic!("{e}"));
    add_island_player(&mut isl, 0);
    add_island_player(&mut isl, 1);
    let w = &isl.stages[0];
    for (kind, want) in [("bowyer", -32.0), ("tavern", -28.0)] {
        let x = site_x(w, kind);
        assert!(
            x == Some(w.hub_x + want),
            "{kind}: Lage {x:?}, erwartet {}",
            w.hub_x + want
        );
    }
    // Der Generator rechnet mit denselben Plätzen wie die Stufe: gleiches Level mit den Plätzen der Welt.
    let values = world.values.clone().expect("aufgelöster Wertesatz");
    let lo = LevelOptions {
        planned: w.planned.clone(),
        chunk_count: w.chunk_count,
        nature: w.level.nature.clone(),
        lair: None,
        hub_sites: Some(level_sites(&values.hub)),
    };
    let b = base().unwrap_or_else(|e| panic!("{e}"));
    let want = generate_with(&w.biome, &w.seed, &lo, &b.rules).unwrap_or_else(|e| panic!("{e}"));
    assert!(
        want.sites == w.level.sites && want.hub_center_units == w.hub_x,
        "Level der Stufe weicht vom Level mit den Plätzen der Welt ab"
    );
    let base_world = quiet_world();
    assert_eq!(
        site_x(&base_world, "bowyer"),
        Some(base_world.hub_x - 28.0),
        "Basis: Bogner"
    );
}

/// AC-02: Fehlerfälle beim Laden: außerhalb, Überlappung, Pflichtplatz, unbekannte Art; Meldung nennt Welt und Datei.
#[test]
#[ignore = "wartet auf RU18 (values_from: check_presets, RU18.4)"]
fn welt_plaetze_fehlerfaelle() {
    let cases = [
        (json!({"sites": {"7": {"offsetUnits": 55}}}), "hub-breite"),
        (json!({"sites": {"7": {"offsetUnits": -50}}}), "hub-breite"),
        (json!({"sites": {"7": {"offsetUnits": 25}}}), "abstand"), // barracks bei 24
        (json!({"islandSites": {"0": {"offsetUnits": -17}}}), "abstand"), // workshop bei -16
        (json!({"sites": {"0": {"kind": "bowyer"}}}), "workshop"),
        (json!({"sites": {"7": {"kind": "volcano"}}}), "unbekannte Art"),
        (json!({"sites": {"7": {"hubLevel": 9}}}), "hubLevel 9"),
    ];
    for (over, want) in cases {
        let err = resolve_hub_override(over.clone()).err().unwrap_or_default();
        assert!(
            err.contains(want) && err.contains("data/hub.json") && err.contains("plaetze"),
            "{over}: Fehler {err:?}, enthält {want:?}, Datei und Welt erwartet"
        );
    }
}

/// AC-02: `sitePlacement` bleibt gesperrt, auch neben erlaubten Plätzen.
#[test]
fn welt_site_placement_bleibt_gesperrt() {
    let over = json!({"sites": {"7": {"offsetUnits": 32}}, "sitePlacement": {"zone1WallUnits": 45}});
    let err = resolve_hub_override(over).err().unwrap_or_default();
    assert!(
        err.contains("sitePlacement: nicht übersteuerbar"),
        "sitePlacement: Ablehnung erwartet, war {err:?}"
    );
}

/// Die Insel-Plätze gehen mit den Hub-Plätzen an den Generator (Dateireihenfolge: sites, dann islandSites).
#[test]
fn level_sites_mit_insel_plaetzen() {
    let site = |kind: &str, at: f64| HubSite {
        kind: kind.into(),
        offset_units: at,
        ..HubSite::default()
    };
    let h = HubData {
        sites: vec![site("workshop", -16.0)],
        island_sites: vec![site("storage", -24.0)],
        ..HubData::default()
    };
    let got: Vec<(String, f64)> = level_sites(&h).into_iter().map(|s| (s.kind, s.offset_units)).collect();
    assert_eq!(
        got,
        vec![("workshop".to_string(), -16.0), ("storage".to_string(), -24.0)]
    );
}

fn found(r: &HubReport, rule: &str) -> bool {
    r.findings.iter().any(|f| f.rule == rule)
}

/// Review B-983: Der Gebäude-Planer prüft Kandidaten wie die Welt: Hub-Breite als Befund, Planung gegen die Kandidaten-Plätze.
#[test]
fn kandidat_plaetze_hub_breite_und_planung() {
    let mut cand = hub();
    cand.sites[7].offset_units = 60.0; // smithy außerhalb der Hub-Breite
    let r = validate(&cand);
    assert!(
        found(&r, RULE_BOUNDS),
        "Kandidat mit Platz bei 60: Befund {RULE_BOUNDS:?} erwartet, war {:?}",
        r.findings
    );
    let mut world = plain_world("test", &[0, 1, 4]);
    let mut d = world.islands[0].clone();
    let depth = d.depths[0];
    let chunk = PlannedChunk {
        slot: 1,
        kind: "forest".into(),
        buildings: vec![PlannedBuilding {
            kind: "smithy".into(),
            at: 10.0,
        }],
        ..PlannedChunk::default()
    };
    d.planned_chunks = BTreeMap::from([(depth.to_string(), vec![chunk])]);
    world.islands = vec![d];
    let r = validate_hub_data_in(&hub(), &world);
    assert!(
        !found(&r, RULE_PLANNED),
        "Basis: Ruine bei 60 ist gültig, war {:?}",
        r.findings
    );
    let r = validate_hub_data_in(&cand, &world);
    assert!(
        found(&r, RULE_PLANNED),
        "Kandidat mit Platz auf der Ruinen-Lage: Befund {RULE_PLANNED:?} erwartet, war {:?}",
        r.findings
    );
}
