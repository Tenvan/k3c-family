//! Natur-Werte der Insel (Go: `island_nature_test.go`, B-891): lesen, zusammenlegen und prüfen. `neutralNature` steht in
//! `testutil.rs`. Go baut die Inseln der Tests mit `parseTestWorld` (strenges Lesen der Welt-Datei, RU10.5); hier ist es
//! die `IslandDef` von Hand (Insel „a“ mit den Tiefen 0 bis 4, wie `testIsland`), die Prüfung ist dieselbe.

use std::collections::BTreeMap;

use k3c_level::types::{NatureValues, PlannedChunk};

use super::data::IslandDef;
use super::nature::check_nature;
use super::testutil::*;
use super::{CreateOptions, create_island_in};

/// Go: `testIsland`, eine gültige Insel mit den Tiefen 0 bis 4.
fn test_island() -> IslandDef {
    let mut d = IslandDef {
        id: "a".into(),
        name: "Insel".into(),
        depths: vec![0, 1, 2, 3, 4],
        ..IslandDef::default()
    };
    d.depth_scaling.hp = 1.0;
    d.depth_scaling.damage = 1.0;
    d.depth_scaling.speed = 1.0;
    d
}

fn traits(pairs: &[(&str, i64)]) -> NatureValues {
    let mut v = NatureValues::default();
    pairs.iter().for_each(|(k, n)| v.set_trait(k, *n));
    v
}

fn tree_species(tanne: i64, pappel: i64, birke: i64) -> NatureValues {
    let mut v = NatureValues::default();
    v.set_weight("treeSpecies", "tanne", tanne);
    v.set_weight("treeSpecies", "pappel", pappel);
    v.set_weight("treeSpecies", "birke", birke);
    v
}

/// B-891: Natur-Werte der Insel lesen, zusammenlegen und prüfen.
#[test]
fn insel_natur_vorgabe_und_tiefe() {
    let data = test_data();
    let mut isl = test_island();
    isl.nature = traits(&[("rockDensity", 3), ("forestDensity", 2)]);
    let mut by_depth = tree_species(4, 0, 0);
    by_depth.set_trait("forestDensity", 5);
    isl.nature_by_depth.insert("0".into(), by_depth);

    let got = isl.nature_at(&data.biomes, 0);
    assert!(
        got.traits["forestDensity"] == 5 && got.traits["rockDensity"] == 3 && got.weights["treeSpecies"]["tanne"] == 4,
        "Wald: {got:?}"
    );
    // Höhle (Tiefe 1) kennt keine Walddichte, aber die Felsdichte (rockDensity): nur sie gilt aus der Vorgabe.
    let cave = isl.nature_at(&data.biomes, 1);
    assert!(
        cave.traits == BTreeMap::from([("rockDensity".to_string(), 3)]) && cave.weights.is_empty(),
        "Höhle: {cave:?}"
    );
}

/// AC-03: Der fest gesetzte Wert gilt in der erzeugten Stufe, unabhängig vom Seed; die Baumart kommt beim Knoten an.
#[test]
#[ignore = "wartet auf RU14"]
fn insel_natur_fest_im_spiel() {
    let data = test_data();
    let mut world = standard_world(&data);
    let mut by_depth = tree_species(0, 0, 3);
    by_depth.set_trait("forestDensity", 1);
    world.islands[0].nature_by_depth.insert("0".into(), by_depth);
    for seed in ["eins", "zwei"] {
        let opts = world.islands[0].create_options(&data.biomes);
        let isl = create_island_in(&data, &world, seed, &[0], 1.0, opts).expect("Insel");
        let st = &isl.stages[0];
        assert_eq!(st.level.nature.traits["forestDensity"], 1, "Seed {seed}: Walddichte");
        for n in st.nodes.iter().filter(|n| n.kind == "tree") {
            assert_eq!(n.variant, "birke", "Seed {seed}: Baum {n:?} ist keine Birke");
        }
    }
}

/// Spielstand: Die Natur der Stufe wird mitgespeichert und beim Laden wieder verwendet, auch wenn die Welt sie nicht mehr
/// festsetzt. Braucht `create_island` (RU14).
#[test]
#[ignore = "wartet auf RU14"]
fn insel_natur_im_spielstand() {
    let data = test_data();
    let world = standard_world(&data);
    let isl = create_island_in(&data, &world, "natur-stand", &[0], 1.0, CreateOptions::default()).expect("Insel");
    let want = isl.stages[0].level.nature.clone();
    let raw = save_json(&isl, "2026-11-14T00:00:00Z");
    let s: super::IslandSave = serde_json::from_str(&raw).expect("Spielstand");
    assert_eq!(s.stages[0].nature, want, "gespeichert");
    let loaded = load_island(&data, &raw);
    assert_eq!(loaded.stages[0].level.nature, want, "geladen");
}

/// AC-04 und Prüfung beim Laden: unbekannte Schlüssel, Bereich, alle Baumarten 0, falsche Tiefe.
#[test]
fn insel_natur_abgelehnt() {
    let data = test_data();
    let cases: [(&str, IslandDef, &str); 5] = [
        (
            "unbekannt",
            IslandDef {
                nature: traits(&[("pilze", 1)]),
                ..test_island()
            },
            "kein Biom der Insel",
        ),
        (
            "Bereich",
            IslandDef {
                nature: traits(&[("forestDensity", 9)]),
                ..test_island()
            },
            "außerhalb",
        ),
        (
            "Tiefe",
            with_depth("7", traits(&[("forestDensity", 2)])),
            "keine Tiefe der Insel",
        ),
        (
            "fremdes Biom",
            with_depth("1", traits(&[("forestDensity", 2)])),
            "keine Eigenschaft",
        ),
        ("alle 0", with_depth("0", tree_species(0, 0, 0)), "Gewicht 0"),
    ];
    for (name, isl, want) in cases {
        match check_nature(&isl, &data.biomes) {
            Err(e) if e.contains(want) => {}
            other => panic!("{name}: Fehler {other:?}, erwartet {want:?}"),
        }
    }
}

fn with_depth(key: &str, v: NatureValues) -> IslandDef {
    let mut isl = test_island();
    isl.nature_by_depth.insert(key.into(), v);
    isl
}

/// Das sechste Go-Argument (`"nature":{"dichte":{}}`, unbekanntes Feld) ist strenges JSON-Lesen: kommt mit `parse_world`.
#[test]
#[ignore = "wartet auf RU10.5"]
fn insel_natur_abgelehnt_feld() {
    panic!("RU10.5: parse_world (unbekanntes Feld dichte)");
}

// Ab hier: Tests ohne Go-Gegenstück, sie laufen schon.

#[test]
fn insel_natur_ohne_vorgabe_ist_gueltig() {
    let data = test_data();
    assert_eq!(check_nature(&test_island(), &data.biomes), Ok(()));
}

#[test]
fn create_options_zeigt_die_daten_der_insel() {
    let data = test_data();
    let mut isl = test_island();
    isl.nature = traits(&[("rockDensity", 3)]);
    isl.chunk_counts.insert("0".into(), 24);
    isl.planned_chunks.insert(
        "0".into(),
        vec![PlannedChunk {
            slot: -7,
            kind: "harbor".into(),
            ..PlannedChunk::default()
        }],
    );
    let o = isl.create_options(&data.biomes);
    assert_eq!(o.chunk_counts, Some(BTreeMap::from([(0, 24)])));
    assert_eq!(o.planned.as_ref().map(|m| m[&0][0].slot), Some(-7));
    let nature = o.nature.as_ref().expect("Natur gesetzt");
    assert!(nature.contains_key(&0), "der Wald kennt rockDensity");
    assert!(
        !nature.contains_key(&3),
        "ohne Wert für die Tiefe kein Eintrag, soweit das Biom die Eigenschaft nicht kennt"
    );
    assert_eq!(o.planned_for(0, &isl)[0].kind, "harbor");
    assert_eq!(o.count_for(0, &isl), 24);
    // `None` fällt auf die Insel zurück, eine leere Map schaltet ab.
    let leer = CreateOptions {
        planned: Some(BTreeMap::new()),
        chunk_counts: Some(BTreeMap::new()),
        ..Default::default()
    };
    assert!(leer.planned_for(0, &isl).is_empty() && leer.count_for(0, &isl) == 0);
    let auto = CreateOptions::default();
    assert_eq!((auto.planned_for(0, &isl).len(), auto.count_for(0, &isl)), (1, 24));
}
