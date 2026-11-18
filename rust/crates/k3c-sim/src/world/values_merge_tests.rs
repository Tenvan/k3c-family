//! Tests des Merge (`values_merge`, RU14.4): die Formfehler aus `TestWorldValuesUngueltig` (Go: `world_values_test.go`)
//! direkt an `merge_file` und über `resolve_world_values` (der Formfehler endet vor `values_from`), der Merge je Feld, je
//! Index und je `id`, und Gos Verhalten bei eingebetteten Feldern (`VeinData`).

use std::collections::BTreeMap;

use k3c_data::{Fehler, Source};
use serde_json::Value;

use super::values::resolve_world_values;
use super::values_merge::merge_file;
use crate::island::WorldDef;
use crate::island::worlds::world_path;

fn base(name: &str) -> String {
    Source::Embedded
        .read(&format!("{name}.json"))
        .unwrap_or_else(|e| panic!("{e}"))
}

fn merge(name: &str, ov: &str) -> Result<String, String> {
    let ov: Value = serde_json::from_str(ov).unwrap_or_else(|e| panic!("{e}"));
    merge_file(name, &base(name), &ov)
}

/// Go: die Formfehler aus `TestWorldValuesUngueltig` (Datei, Override, erwartete Meldung des Merge).
fn shape_cases() -> [(&'static str, &'static str, &'static str); 11] {
    [
        ("buildings", r#"{"wall": {"size": 3}}"#, "wall.size: unbekanntes Feld"),
        (
            "buildings",
            r#"{"palast": {"hp": 3}}"#,
            "palast: unbekannter Eintrag (neue Einträge sind nicht erlaubt)",
        ),
        (
            "buildings",
            r#"{"wall": {"cost": {"wood": "viel"}}}"#,
            "wall.cost.wood: Ganzzahl erwartet, ist Text",
        ),
        (
            "buildings",
            r#"{"wall": {"cost": {"wood": 2.5}}}"#,
            "wall.cost.wood: Ganzzahl erwartet, ist Zahl",
        ),
        (
            "buildings",
            r#"{"wall": {"levels": [{"hp": 1}]}}"#,
            r#"wall.levels: Liste von Objekten je Eintrag übersteuern ({"<id oder Index>": {…}}), ist Liste"#,
        ),
        (
            "buildings",
            r#"{"wall": {"levels": {"9": {"hp": 1}}}}"#,
            "wall.levels.9: unbekannter Eintrag (neue Einträge sind nicht erlaubt)",
        ),
        (
            "buildings",
            r#"{"wall": {"HP": 1}}"#,
            r#"wall.HP: Schreibweise wie in der Basis: "hp""#,
        ),
        (
            "ship",
            r#"{"stages": {"mast": {"hp": 1}}}"#,
            "stages.mast: unbekannter Eintrag (neue Einträge sind nicht erlaubt)",
        ),
        (
            "enemies",
            r#"{"wolf": {"gold": [1]}}"#,
            "wolf.gold: genau 2 Werte erwartet, sind 1",
        ),
        (
            "waves",
            r#"{"bossHpPerExtraPlayer": null}"#,
            "bossHpPerExtraPlayer: Zahl erwartet, ist null",
        ),
        ("goals", "7", "Objekt erwartet, ist Zahl"),
    ]
}

/// B-680/AC-03: Der Merge meldet Formfehler mit Pfad, wortgleich zu Go.
#[test]
fn merge_file_formfehler_wie_go() {
    for (file, ov, want) in shape_cases() {
        assert_eq!(merge(file, ov), Err(want.to_string()), "{file} {ov}");
    }
}

/// B-680/AC-03 (Teil Form von `TestWorldValuesUngueltig`): Über `resolve_world_values` trägt der Fehler Welt und Datei.
#[test]
fn world_values_ungueltig_form() {
    for (file, ov, want) in shape_cases() {
        let w = WorldDef {
            id: "nordmeer".into(),
            overrides: BTreeMap::from([(file.to_string(), serde_json::from_str(ov).unwrap_or(Value::Null))]),
            ..WorldDef::default()
        };
        let lookup = |id: &str| {
            (id == w.id)
                .then(|| w.clone())
                .ok_or_else(|| format!("{}: Datei fehlt", world_path(id)))
        };
        let read = |name: &str| -> Result<String, Fehler> { Source::Embedded.read(name) };
        let err = resolve_world_values(&read, &lookup, "nordmeer")
            .err()
            .unwrap_or_default();
        assert_eq!(
            err,
            format!("data/worlds/nordmeer.json: overrides › data/{file}.json › {want}"),
            "{file} {ov}"
        );
    }
}

/// B-680/AC-02: Ein Override ändert genau das genannte Feld; Listen von Objekten je Index (`levels`) und je `id`
/// (`stages`), Wertelisten ganz (`gold`).
#[test]
fn merge_file_aendert_nur_das_feld() {
    let got = k3c_data::buildings::load(
        &merge(
            "buildings",
            r#"{"wall": {"cost": {"wood": 7}, "levels": {"1": {"hp": 650}}}}"#,
        )
        .unwrap_or_default(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let mut want = k3c_data::buildings::load(&base("buildings")).unwrap_or_else(|e| panic!("{e}"));
    let wall = want.get_mut("wall").unwrap_or_else(|| panic!("wall"));
    assert_ne!(wall.cost.wood, 7);
    wall.cost.wood = 7;
    wall.levels[1].hp = 650.0;
    assert_eq!(got, want);

    let ship =
        k3c_data::ship::load(&merge("ship", r#"{"stages": {"hull": {"cost": {"gold": 9}}}}"#).unwrap_or_default())
            .unwrap_or_else(|e| panic!("{e}"));
    let mut want = k3c_data::ship::load(&base("ship")).unwrap_or_else(|e| panic!("{e}"));
    want.stages[1].cost.gold = 9;
    assert_eq!(ship, want, "Abschnitt hull je id");

    let enemies = k3c_data::enemies::load(&merge("enemies", r#"{"wolf": {"gold": [4, 6]}}"#).unwrap_or_default())
        .unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(enemies["wolf"].gold, [4, 6]);
}

/// Ohne Änderung bleibt der Inhalt gleich (leeres Override), Zahlen ohne Rundung.
#[test]
fn merge_file_leeres_override_gleich_der_basis() {
    for name in super::values::override_files() {
        let got: Value = serde_json::from_str(&merge(name, "{}").unwrap_or_default()).unwrap_or_default();
        let want: Value = serde_json::from_str(&base(name)).unwrap_or_default();
        assert_eq!(got, want, "{name}");
    }
    let got = merge("waves", r#"{"perExtraPlayer": 0.37}"#).unwrap_or_default();
    assert!(got.contains(r#""perExtraPlayer":0.37"#), "{got}");
}

/// Gos `fieldFold` kennt bei `VeinData` das eingebettete `Gatherable` nur unter seinem Typnamen: `amount` an einer Ader ist
/// ein unbekanntes Feld (wie in Go; Auffälligkeit im Bericht RU14.4).
#[test]
fn merge_file_ader_eingebettetes_feld_wie_go() {
    assert_eq!(
        merge("economy", r#"{"veins": {"stoneVein": {"amount": 3}}}"#),
        Err("veins.stoneVein.amount: unbekanntes Feld".to_string())
    );
    assert!(merge("economy", r#"{"veins": {"stoneVein": {"maxWorkers": 3}}}"#).is_ok());
    assert_eq!(
        merge("economy", r#"{"veins": {"stoneVein": {"Gatherable": 3}}}"#),
        Err("veins.stoneVein.Gatherable: Objekt erwartet, ist Zahl".to_string())
    );
}
