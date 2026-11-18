//! Port von `engine/sim/world_values_test.go` und `world_values_b692_test.go` (B-680, B-682, B-692): Welt-Werte erben und
//! übersteuern. Merge (`merge_file`) und Basis-Wertesatz (`default_values`) kommen seit RU14.4; was `values_from` bis zum
//! Ende braucht (Presets, Hub-Plätze: RU18), trägt `ignore`. Die Formfehler laufen ohne `values_from` in
//! `values_merge_tests.rs`; die Fälle, die vor diesen Schritten enden (Kette, Zyklus, unbekannte Datei, Entfernen), laufen
//! hier. Tests des Welt-Dateikopfs (`TestParseWorldExtendsOverrides`) stehen bei `k3c_data::world`.

use std::collections::BTreeMap;

use k3c_data::{Fehler, Source};
use serde_json::Value;

use super::values::{
    LookupFn, check_world_removable, override_files, resolve_world_files, resolve_world_values, world_children,
};
use super::values_room::default_values;
use crate::island::WorldDef;
use crate::island::worlds::{STANDARD_WORLD_ID, world_path};
use crate::state::WorldValues;

/// Die eingebettete Basis wie `data.Files`.
fn base_read(name: &str) -> Result<String, Fehler> {
    Source::Embedded.read(name)
}

/// Go: `testWorlds`: `lookup` über die Welten `ws` (nur ID, Extends, Overrides zählen für die Auflösung).
fn test_worlds(ws: Vec<WorldDef>) -> impl Fn(&str) -> Result<WorldDef, String> {
    move |id| {
        ws.iter()
            .find(|w| w.id == id)
            .cloned()
            .ok_or_else(|| format!("{}: Datei fehlt", world_path(id)))
    }
}

/// Go: `ov`: Overrides aus JSON je Datei.
fn ov(files: &[(&str, &str)]) -> BTreeMap<String, Value> {
    files
        .iter()
        .map(|(k, v)| ((*k).to_string(), serde_json::from_str(v).unwrap_or(Value::Null)))
        .collect()
}

fn world(id: &str, extends: &str, overrides: &[(&str, &str)]) -> WorldDef {
    WorldDef {
        id: id.into(),
        extends: extends.into(),
        overrides: ov(overrides),
        ..WorldDef::default()
    }
}

fn base_values() -> WorldValues {
    (*default_values()).clone()
}

fn resolve(lookup: LookupFn, id: &str) -> Result<WorldValues, String> {
    resolve_world_values(&base_read, lookup, id)
}

/// B-680/AC-02: Ohne Overrides gleich der Basis (auch über extends).
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn world_values_ohne_overrides() {
    let lookup = test_worlds(vec![
        world(STANDARD_WORLD_ID, "", &[]),
        world("kind", STANDARD_WORLD_ID, &[]),
    ]);
    for id in [STANDARD_WORLD_ID, "kind"] {
        let v = resolve(&lookup, id).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(v, base_values(), "{id}: Wertesatz weicht von der Basis ab");
    }
}

/// B-680/AC-02: Ein Override ändert genau das genannte Feld; Einträge in Listen je Index bzw. je id.
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn world_values_override_aendert_nur_feld() {
    let w = world(
        "nordmeer",
        "",
        &[
            (
                "buildings",
                r#"{"wall": {"cost": {"wood": 7}, "levels": {"1": {"hp": 650}}}}"#,
            ),
            ("ship", r#"{"stages": {"hull": {"cost": {"gold": 9}}}}"#),
            ("waves", r#"{"perExtraPlayer": 0.37}"#),
            ("enemies", r#"{"wolf": {"gold": [4, 6]}}"#),
        ],
    );
    let v = resolve(&test_worlds(vec![w]), "nordmeer").unwrap_or_else(|e| panic!("{e}"));
    let mut want = base_values();
    if let Some(wall) = want.buildings.get_mut("wall") {
        wall.cost.wood = 7;
        wall.levels[1].hp = 650.0;
    }
    if let Some(ship) = want.buildings.get_mut("ship") {
        ship.levels[1].cost.gold = 9;
    }
    want.waves.per_extra_player = 0.37;
    if let Some(wolf) = want.enemies.get_mut("wolf") {
        wolf.gold = [4, 6];
    }
    assert_eq!(v, want);
    let base = base_values();
    assert_ne!(base.buildings["wall"].cost.wood, 7, "Auflösung hat die Basis verändert");
    assert_ne!(base.waves.per_extra_player, 0.37, "Auflösung hat die Basis verändert");
}

/// B-680/AC-03: Eine unbekannte Datei in `overrides` wird mit Welt und Datei abgelehnt (endet vor `merge_file`).
#[test]
fn world_values_ungueltig_unbekannte_datei() {
    let w = world("nordmeer", "", &[("biomes", "{}")]);
    let err = resolve(&test_worlds(vec![w]), "nordmeer").err().unwrap_or_default();
    assert!(err.starts_with("data/worlds/nordmeer.json: "), "{err}");
    assert!(err.contains(r#"overrides: unbekannte Datei "biomes""#), "{err}");
    assert!(err.contains("erlaubt: buildings, economy, enemies"), "{err}");
}

/// Go: die Fälle von `TestWorldValuesUngueltig` (Datei, Override, erwartete Meldung), ohne den Fall `biomes`.
type Case = (&'static str, &'static str, &'static str);

fn invalid_cases() -> Vec<Case> {
    let mut v = invalid_shape_cases().to_vec();
    v.extend(invalid_value_cases());
    v
}

/// Formfehler (buildings, ship, enemies.gold).
fn invalid_shape_cases() -> [Case; 9] {
    [
        (
            "buildings",
            r#"{"wall": {"size": 3}}"#,
            "overrides › data/buildings.json › wall.size: unbekanntes Feld",
        ),
        (
            "buildings",
            r#"{"palast": {"hp": 3}}"#,
            "overrides › data/buildings.json › palast: unbekannter Eintrag",
        ),
        (
            "buildings",
            r#"{"wall": {"cost": {"wood": "viel"}}}"#,
            "overrides › data/buildings.json › wall.cost.wood: Ganzzahl erwartet, ist Text",
        ),
        (
            "buildings",
            r#"{"wall": {"cost": {"wood": 2.5}}}"#,
            "wall.cost.wood: Ganzzahl erwartet, ist Zahl",
        ),
        (
            "buildings",
            r#"{"wall": {"levels": [{"hp": 1}]}}"#,
            "wall.levels: Liste von Objekten je Eintrag übersteuern",
        ),
        (
            "buildings",
            r#"{"wall": {"levels": {"9": {"hp": 1}}}}"#,
            "wall.levels.9: unbekannter Eintrag",
        ),
        (
            "buildings",
            r#"{"wall": {"HP": 1}}"#,
            r#"wall.HP: Schreibweise wie in der Basis: "hp""#,
        ),
        (
            "ship",
            r#"{"stages": {"mast": {"hp": 1}}}"#,
            "overrides › data/ship.json › stages.mast: unbekannter Eintrag",
        ),
        (
            "enemies",
            r#"{"wolf": {"gold": [1]}}"#,
            "wolf.gold: genau 2 Werte erwartet, sind 1",
        ),
    ]
}

/// Prüfungen nach dem Merge (Traits, Pools, Wellen, Presets, goals).
fn invalid_value_cases() -> [Case; 6] {
    [
        (
            "enemies",
            r#"{"wolf": {"traits": ["teleports"]}}"#,
            r#"overrides: data/enemies.json: wolf: unbekanntes Trait "teleports""#,
        ),
        (
            "enemies",
            r#"{"caveTroll": {"tier": "standard"}}"#,
            "overrides: data/biomes/cave.json: Portal-Pool braucht Standard- und Elite-Gegner",
        ),
        (
            "waves",
            r#"{"perExtraPlayer": -1}"#,
            "overrides: data/waves.json: perExtraPlayer und bossHpPerExtraPlayer dürfen nicht negativ sein",
        ),
        (
            "waves",
            r#"{"bossHpPerExtraPlayer": null}"#,
            "bossHpPerExtraPlayer: Zahl erwartet, ist null",
        ),
        (
            "monarch",
            r#"{"presets": {"tank": {"activeSkills": ["fliegen"]}}}"#,
            "overrides: data/monarch.json: preset tank nennt unbekannten Skill fliegen",
        ),
        ("goals", "7", "overrides › data/goals.json › Objekt erwartet, ist Zahl"),
    ]
}

/// B-680/AC-03: Ungültige Overrides werden mit Welt, Datei und Pfad abgelehnt (Tabelle wie in Go; ohne den Fall `biomes`).
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn world_values_ungueltig() {
    let cases = invalid_cases();
    for (file, json, want) in cases {
        let w = world("nordmeer", "", &[(file, json)]);
        let err = resolve(&test_worlds(vec![w]), "nordmeer").err().unwrap_or_default();
        assert!(
            err.starts_with("data/worlds/nordmeer.json: ") && err.contains(want),
            "{file} {json}: Fehler {err}, erwartet …{want}…"
        );
    }
}

/// B-680/AC-05: Kette Basis → A → B; das Kind gewinnt, Geerbtes bleibt, der Fehler nennt die auslösende Welt.
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn world_values_kette() {
    let a = world("a", "", &[("goals", r#"{"gold": 111, "days": 5}"#)]);
    let b = world("b", "a", &[("goals", r#"{"days": 9}"#)]);
    let v = resolve(&test_worlds(vec![a, b.clone()]), "b").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!((v.goals.gold, v.goals.days), (111, 9), "Gold 111 aus a, Days 9 aus b");
    let bad = world("a", "", &[("waves", r#"{"perExtraPlayer": -2}"#)]);
    let err = resolve(&test_worlds(vec![bad, b]), "b").err().unwrap_or_default();
    assert!(err.starts_with("data/worlds/a.json: "), "Fehler der Eltern-Welt: {err}");
}

/// Go: `chain`-Hilfe des Tests: `ids[0]` erbt von `ids[1]` usw.
fn chain(ids: &[&str]) -> Vec<WorldDef> {
    ids.iter()
        .enumerate()
        .map(|(i, id)| world(id, ids.get(i + 1).copied().unwrap_or(""), &[]))
        .collect()
}

/// B-680/AC-05: Zyklus, unbekannte Eltern-Welt und zu tiefe Kette werden abgelehnt.
#[test]
fn world_values_kette_abgelehnt() {
    let cases: [(&str, Vec<WorldDef>, &str); 3] = [
        (
            "Zyklus",
            vec![world("b", "a", &[]), world("a", "b", &[])],
            "data/worlds/b.json: extends: Zyklus b → a → b",
        ),
        (
            "unbekannt",
            vec![world("b", "fehlt", &[])],
            r#"data/worlds/b.json: extends: Eltern-Welt "fehlt" unbekannt"#,
        ),
        (
            "zu tief",
            chain(&["e", "d", "c", "b", "a"]),
            "data/worlds/e.json: extends: Kette zu lang (e → d → c → b → a, höchstens 4 Welten)",
        ),
    ];
    for (name, worlds, want) in cases {
        let first = worlds[0].id.clone();
        let err = resolve(&test_worlds(worlds), &first).err().unwrap_or_default();
        assert!(err.contains(want), "{name}: Fehler {err}, erwartet …{want}…");
    }
}

/// B-680/AC-05: Eine Kette aus 4 Welten ist erlaubt (der Fall in `world_values_kette_abgelehnt` von Go).
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn world_values_kette_aus_vier_welten() {
    let r = resolve(&test_worlds(chain(&["d", "c", "b", "a"])), "d");
    assert!(r.is_ok(), "Kette aus 4 Welten abgelehnt: {:?}", r.err());
}

/// B-680/AC-05: Löschen oder Umbenennen einer Welt mit Kind-Welten wird verweigert, die Meldung nennt die Kinder.
#[test]
fn world_removable() {
    let all = vec![
        world("a", "", &[]),
        world("c", "a", &[]),
        world("b", "a", &[]),
        world("d", "b", &[]),
    ];
    assert_eq!(
        check_world_removable(&all, "a"),
        Err("data/worlds/a.json: Welt hat Kind-Welten (b, c), Löschen und Umbenennen abgelehnt".to_string())
    );
    assert_eq!(check_world_removable(&all, "d"), Ok(()), "d ohne Kinder");
    assert_eq!(world_children(&all, "a"), vec!["b".to_string(), "c".to_string()]);
}

/// B-682: `resolve_world_files` liefert die Basis-Dateien nach der Kette (Kind sieht den Override der Eltern-Welt), leer = Basis.
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn resolve_world_files_nach_kette() {
    let a = world("a", "", &[("waves", r#"{"perExtraPlayer": 0.37}"#)]);
    let b = world("b", "a", &[]);
    let files = resolve_world_files(&base_read, &test_worlds(vec![a, b]), "b").unwrap_or_else(|e| panic!("{e}"));
    let waves: Value = serde_json::from_str(&files["waves"]).unwrap_or(Value::Null);
    assert_eq!(waves["perExtraPlayer"], 0.37, "waves nach der Kette");
    let none = test_worlds(vec![]);
    let basis = resolve_world_files(&base_read, &none, "").unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(basis.len(), override_files().len());
    assert_eq!(basis["buildings"], base_read("buildings.json").unwrap_or_default());
}

/// B-692 Punkt 5: `resolve_world_files` mit leerer ID prüft die Basis wie `resolve_world_values`.
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn b692_resolve_world_files_prueft_basis() {
    let lookup = test_worlds(vec![world(STANDARD_WORLD_ID, "", &[])]);
    assert!(resolve_world_files(&base_read, &lookup, "").is_ok());
    let broken = |name: &str| {
        let raw = base_read(name)?;
        Ok(if name == "waves.json" {
            raw.replacen(r#""perExtraPlayer":"#, r#""perExtraPlayer": -1, "x":"#, 1)
        } else {
            raw
        })
    };
    assert!(
        resolve_world_files(&broken, &lookup, "").is_err(),
        "kaputte Basis (perExtraPlayer < 0) ohne Fehler"
    );
}

/// B-692 Punkt 3: Felder, die der Level-Generator aus der Basis liest, sind nicht übersteuerbar (welten.md § 1.3). Die
/// Ablehnung selbst prüft `values_locked_tests`; hier der Weg durch `resolve_world_values` mit Pfad der Welt.
#[test]
#[ignore = "wartet auf RU18.4 (check_presets)"]
fn b692_gesperrte_felder() {
    let cases: [(&str, &str, &str); 4] = [
        ("economy.payRangeUnits", "economy", r#"{"payRangeUnits": 3}"#),
        (
            "hub.sitePlacement",
            "hub",
            r#"{"sitePlacement": {"zone1WallUnits": 5}}"#,
        ),
        (
            "nursery.plantation",
            "buildings",
            r#"{"nursery": {"plantation": {"stripUnits": 5}}}"#,
        ),
        (
            "buildings.nursery.hp ok",
            "buildings",
            r#"{"nursery": {"hp": 120, "plantation": {"slots": 4}}}"#,
        ),
    ];
    for (name, file, json) in cases {
        let w = world("gesperrt", STANDARD_WORLD_ID, &[(file, json)]);
        let lookup = test_worlds(vec![world(STANDARD_WORLD_ID, "", &[]), w]);
        let r = resolve(&lookup, "gesperrt");
        if name.ends_with(" ok") {
            assert!(r.is_ok(), "{name}: {:?}", r.err());
            continue;
        }
        let err = r.err().unwrap_or_default();
        assert!(
            err.contains("nicht übersteuerbar") && err.contains("data/worlds/gesperrt.json"),
            "{name}: Ablehnung mit Pfad erwartet, war {err}"
        );
    }
}

// Die übrigen Tests aus `world_values_b692_test.go` stehen in `values_b692_tests.rs` (RU14.4).
