//! RU3/AC-04 (Teil 2): Bäume der übrigen Dateien und der Standard-Welt wie in Go; jede Datei aus Go ist geprüft.

use std::collections::BTreeSet;

use crate::testutil::{go_tree, raw, same_as_go, same_as_go_zahlen_aus_daten};
use crate::{
    Fehler, buildings, island_sizes, light, monarch, native, paper, render, ruins, ship, sizes, sounds, story,
    stresstests, troops, waves, world,
};

#[test]
fn rest_wie_go() {
    let b = buildings::load(&raw(buildings::FILE)).expect("buildings.json");
    same_as_go(
        island_sizes::FILE,
        &island_sizes::load(&raw(island_sizes::FILE)).expect("laden"),
    );
    // light.json: Fackeln der Hub-Gebäude (LA1.8) sind neue Schlüssel unter torches; Go ist eingefroren.
    same_as_go_zahlen_aus_daten(light::FILE, &light::load(&raw(light::FILE)).expect("laden"));
    same_as_go(monarch::FILE, &monarch::load(&raw(monarch::FILE)).expect("laden"));
    same_as_go(native::FILE, &native::load(&raw(native::FILE)).expect("laden"));
    // paper.json: Maße ändern die Asset-Sprints weiter (BB1, #772); Schlüssel aus Go und Texte bleiben wie Go, neue Einträge sind erlaubt.
    same_as_go_zahlen_aus_daten(paper::FILE, &paper::load(&raw(paper::FILE)).expect("laden"));
    same_as_go(sizes::FILE, &sizes::load(&raw(sizes::FILE)).expect("laden"));
    same_as_go(render::FILE, &render::load(&raw(render::FILE)).expect("laden"));
    same_as_go(ruins::FILE, &ruins::load(&raw(ruins::FILE), &b).expect("laden"));
    same_as_go(ship::FILE, &ship::load(&raw(ship::FILE)).expect("laden"));
    same_as_go(sounds::FILE, &sounds::load(&raw(sounds::FILE)).expect("laden"));
    same_as_go(story::FILE, &story::load(&raw(story::FILE)).expect("laden"));
    same_as_go(
        stresstests::FILE,
        &stresstests::load(&raw(stresstests::FILE)).expect("laden"),
    );
    same_as_go(troops::FILE, &troops::load(&raw(troops::FILE)).expect("laden"));
    same_as_go(waves::FILE, &waves::load(&raw(waves::FILE)).expect("laden"));
}

#[test]
fn standard_welt_wie_go() {
    let w = world::parse(&raw("worlds/standard.json"), "standard").expect("laden");
    same_as_go("worlds/standard.json", &w);
}

/// Jede Datei im Baum aus Go hat hier einen Vergleich (die Namen der Tests decken alle Schlüssel ab).
#[test]
fn jede_datei_aus_go_ist_geprueft() {
    let checked: BTreeSet<&str> = [
        "balance-hints.json",
        "balance-targets.json",
        "bosses.json",
        "buildings.json",
        "difficulty.json",
        "economy.json",
        "enemies.json",
        "events.json",
        "goals.json",
        "guards.json",
        "hub.json",
        "island-sizes.json",
        "light.json",
        "monarch.json",
        "native.json",
        "paper.json",
        "render.json",
        "ruins.json",
        "ship.json",
        "sizes.json",
        "sounds.json",
        "story.json",
        "stresstests.json",
        "troops.json",
        "waves.json",
        "worlds/standard.json",
    ]
    .into_iter()
    .collect();
    for name in go_tree().keys() {
        assert!(
            checked.contains(name.as_str()) || name.starts_with("biomes/"),
            "{name} aus data-tree.json.gz hat keinen Rust-Vergleich"
        );
    }
}

#[test]
fn pruefungen_rest_wie_go() {
    let b = buildings::load(&raw(buildings::FILE)).expect("buildings.json");
    let cases: [(Result<(), Fehler>, &str); 7] = [
        (
            island_sizes::load(r#"{"sizes": []}"#).map(drop),
            "data/island-sizes.json: keine Inselgröße",
        ),
        (
            island_sizes::load(r#"{"sizes": [{"name": "a", "chunkCount": 3}]}"#).map(drop),
            "data/island-sizes.json: a: Chunk-Anzahl 3 ist nicht gerade und größer 0",
        ),
        (
            waves::load("{}").map(drop),
            "data/waves.json: bossHpPerExtraPlayer fehlt",
        ),
        (
            ruins::load(r#"{"hpShare": 0.5, "kinds": ["wall"]}"#, &b).map(drop),
            "ruins.json: \"wall\" gehört dem Hub und ist keine Ruine",
        ),
        (
            monarch::load(r#"{"presets": {"p": {"activeSkills": ["x"]}}}"#).map(drop),
            "data/monarch.json: preset p nennt unbekannten Skill x",
        ),
        (
            world::parse(r#"{"id": "w", "name": "W", "islands": [{}], "foo": 1}"#, "w").map(drop),
            "data/worlds/w.json: unbekanntes Feld \"foo\"",
        ),
        (
            world::parse(
                r#"{"id": "w", "name": "W", "islands": [{}], "overrides": {"x": {}}}"#,
                "w",
            )
            .map(drop),
            "data/worlds/w.json: overrides: unbekannte Datei \"x\" (erlaubt: buildings, economy, enemies, events, \
             goals, hub, monarch, ship, troops, waves)",
        ),
    ];
    for (got, want) in cases {
        assert_eq!(got.expect_err(want).to_string(), want);
    }
    let e = world::parse("{}", "W!").expect_err("ID");
    assert_eq!(
        e.to_string(),
        "Welt-ID \"W!\" ungültig (a-z, 0-9, -, höchstens 32 Zeichen)"
    );
    let e = world::parse(r#"{"name": "W", "islands": [{}]}"#, "w").expect_err("Pflicht");
    assert_eq!(e.to_string(), "data/worlds/w.json: id fehlt (Pflichtfeld)");
}
