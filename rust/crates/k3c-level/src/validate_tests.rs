//! Go hat keine `validate_test.go`; dieser Test ist neu in Rust und prüft `validate` schon vor `generate` (RU6.4):
//! Jedes Level der Golden-Dateien ohne Planung ist gültig, und ein verändertes meldet die Go-Texte.

use k3c_det::golden;

use crate::nature::resolve_nature;
use crate::testutil::{biome, validate};
use crate::types::{Layout, NatureValues};

fn golden_levels(id: &str) -> Vec<Layout> {
    let g = golden::read(golden::testdata().join(format!("golden/level-{id}.json")));
    let list = g["levels"]
        .as_array()
        .unwrap_or_else(|| panic!("level-{id}.json ohne levels"));
    list.iter()
        .map(|v| serde_json::from_value(v.clone()).expect("Layout"))
        .collect()
}

#[test]
fn golden_level_sind_gueltig() {
    for id in ["cave", "crystal", "forest", "ironhold", "mine"] {
        let b = biome(id);
        for mut l in golden_levels(id) {
            // Die Natur-Werte stehen nicht im Golden (`json:"-"`): wie `generate` aus dem Seed würfeln.
            l.nature = resolve_nature(&b, &l.seed, &NatureValues::default()).expect("Natur");
            let errs = validate(&l, &b);
            assert!(errs.is_empty(), "Biom {id}, Seed {:?}: {errs:?}", l.seed);
        }
    }
}

/// Wie `TestValidateMeldetKaputtesLevel` (RU6.5), nur mit einem Golden-Level statt `generate`.
#[test]
fn kaputtes_golden_level_meldet_die_go_texte() {
    let b = biome("forest");
    let mut l = golden_levels("forest").remove(0);
    for c in &mut l.chunks {
        if c.kind == "exit" {
            c.kind = "forest".into();
        }
    }
    l.entities.push(crate::types::Entity {
        kind: "castle".into(),
        x: l.width_units + 1.0,
        ..Default::default()
    });
    let want = [
        "Genau ein Tiefen-Eingang erwartet",
        "Genau eine Burg erwartet",
        "Entity außerhalb des Levels",
    ];
    assert_eq!(validate(&l, &b), want);
}
