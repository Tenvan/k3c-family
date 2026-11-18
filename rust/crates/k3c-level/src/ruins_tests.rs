use std::collections::{BTreeMap, BTreeSet};

use k3c_data::buildings::BuildingData;
use serde_json::json;

use super::{RUIN_GAP_UNITS, check_ruin_place, ruin_hp};
use crate::planned::{Placed, geometry_of, mark_spots};
use crate::planned_gen::Options;
use crate::testutil::{PlannedCase, biome, gen2, generate_with, obj, p, plan_ok, rules, with_ruin};
use crate::types::{PlannedBuilding, PlannedChunk, Ruin};

#[test]
fn ruinen_werte_aus_daten() {
    let r = &rules().ruin;
    assert!(
        r.hp_share == 0.25 && r.kinds.len() == 8,
        "data/ruins.json: Anteil {}, {} Arten",
        r.hp_share,
        r.kinds.len()
    );
    let hp = rules().ruin_buildings["smithy"].hp;
    assert_eq!(
        ruin_hp(rules(), hp),
        63.0,
        "Schmiede: erwartet 63 (25 % von 250, aufgerundet)"
    );
}

fn building(hp: f64) -> BuildingData {
    serde_json::from_value(json!({ "hp": hp })).expect("BuildingData")
}

#[test]
fn ruinen_werte_lehnt_ab() {
    let buildings: BTreeMap<String, BuildingData> = [("smithy", 250.0), ("castle", 1000.0), ("stairsUp", 500.0)]
        .into_iter()
        .map(|(k, hp)| (k.to_string(), building(hp)))
        .collect();
    let cases = [
        ("Anteil 0", 0.0, vec!["smithy"], "hpShare"),
        ("Anteil über 1", 1.5, vec!["smithy"], "hpShare"),
        ("keine Art", 0.25, vec![], "keine Art"),
        ("unbekannte Art", 0.25, vec!["volcano"], "unbekannte Gebäudeart"),
        ("Burg", 0.25, vec!["castle"], "gehört dem Hub"),
        ("Treppe", 0.25, vec!["stairsUp"], "gehört dem Hub"),
        ("Anteil genau 1", 1.0, vec!["smithy"], ""),
        ("gültige Ruinen", 0.25, vec!["smithy"], ""),
    ];
    for (name, share, kinds, want) in cases {
        let raw = json!({ "hpShare": share, "kinds": kinds }).to_string();
        let err = k3c_data::ruins::load(&raw, &buildings)
            .map(|_| ())
            .map_err(|e| e.to_string());
        let ok = if want.is_empty() {
            err.is_ok()
        } else {
            err.as_ref().is_err_and(|e| e.contains(want))
        };
        assert!(ok, "{name}: Fehler {err:?}, erwartet {want:?}");
    }
}

#[test]
fn ruine_gueltig() {
    let forest = biome("forest");
    let seed = PlannedChunk {
        slot: -3,
        kind: "clearing".into(),
        seed: "lichtung".into(),
        ..PlannedChunk::default()
    };
    let cases: Vec<(&str, Vec<PlannedChunk>)> = vec![
        ("Schmiede", vec![with_ruin(p(3, "forest", &[]), "smithy", 10.0)]),
        ("Schmiede am Rand", vec![with_ruin(p(3, "forest", &[]), "smithy", 2.0)]),
        (
            "Baumschule mit Plantage",
            vec![with_ruin(p(3, "forest", &[]), "nursery", 3.0)],
        ),
        (
            "neben festem Objekt",
            vec![with_ruin(p(3, "clearing", &[obj("tree", 10.0)]), "barracks", 14.0)],
        ),
        ("mit Chunk-Seed", vec![with_ruin(seed, "storage", 25.0)]),
        // innerhalb der Mauern, kein Abstand zu Bauplätzen
        (
            "an einer Mauerlinie",
            vec![with_ruin(p(2, "forest", &[]), "tavern", 25.0)],
        ),
        // 90 Units vom Hub, im Tor-Bereich von Linie 3
        ("links am Tor", vec![with_ruin(p(-1, "forest", &[]), "healer", 10.0)]),
    ];
    for (name, plan) in cases {
        if let Err(e) = plan_ok(&forest, &plan) {
            panic!("{name}: {e}");
        }
    }
}

type RuinCase = (&'static str, crate::biome::Biome, Vec<PlannedChunk>, &'static str);

/// Ruinen neben festen Objekten anderer Chunks.
fn ruin_cases_neighbors(forest: &crate::biome::Biome) -> Vec<RuinCase> {
    let f = |plan: Vec<PlannedChunk>, want| (forest.clone(), plan, want);
    let cases = [
        (
            "Portal im Nachbar",
            f(
                vec![
                    p(3, "portal", &[obj("portal", 50.0)]),
                    with_ruin(p(4, "forest", &[]), "smithy", 2.0),
                ],
                "portal auf Platz +3",
            ),
        ),
        (
            "Camp im Nachbar",
            f(
                vec![
                    with_ruin(p(-3, "forest", &[]), "smithy", 2.0),
                    p(-4, "recruitCamp", &[obj("recruitCamp", 50.0)]),
                ],
                "recruitCamp auf Platz -4",
            ),
        ),
    ];
    cases.into_iter().map(|(n, (b, plan, w))| (n, b, plan, w)).collect()
}

fn ruin_cases(
    forest: &crate::biome::Biome,
    cave: &crate::biome::Biome,
) -> Vec<(&'static str, crate::biome::Biome, Vec<PlannedChunk>, &'static str)> {
    let mut two = p(3, "forest", &[]);
    two.buildings = vec![
        PlannedBuilding {
            kind: "smithy".into(),
            at: 10.0,
        },
        PlannedBuilding {
            kind: "nursery".into(),
            at: 30.0,
        },
    ];
    let f = |plan: Vec<PlannedChunk>, want| (forest.clone(), plan, want);
    let r3 = |kind: &str, at| vec![with_ruin(p(3, "forest", &[]), kind, at)];
    let cases = [
        ("zwei Ruinen", f(vec![two], "höchstens eine Ruine")),
        ("Burg", f(r3("castle", 10.0), "nicht erlaubt")),
        ("Mauer", f(r3("wall", 10.0), "nicht erlaubt")),
        ("unbekannte Art", f(r3("volcano", 10.0), "nicht erlaubt")),
        ("am Rand", f(r3("smithy", 1.0), "Chunk-Grenze")),
        ("hinter dem Chunk", f(r3("smithy", 49.0), "Chunk-Grenze")),
        ("Plantage passt nicht", f(r3("nursery", 2.0), "Chunk-Grenze")),
        (
            "am festen Objekt",
            f(
                vec![with_ruin(p(3, "clearing", &[obj("tree", 10.0)]), "smithy", 12.0)],
                "tree",
            ),
        ),
        (
            "an der Truhe",
            f(vec![with_ruin(p(3, "chest", &[]), "smithy", 23.0)], "chest"),
        ),
        (
            "am Portal",
            f(
                vec![with_ruin(p(4, "portal", &[obj("portal", 25.0)]), "smithy", 27.0)],
                "portal",
            ),
        ),
    ];
    let mut out: Vec<_> = cases.into_iter().map(|(n, (b, plan, w))| (n, b, plan, w)).collect();
    let vein = vec![with_ruin(p(3, "cavern", &[obj("stoneVein", 20.0)]), "smithy", 22.0)];
    out.push(("an der Ader", cave.clone(), vein, "stoneVein"));
    out
}

#[test]
fn ruine_lehnt_ab() {
    let mut cases = ruin_cases(&biome("forest"), &biome("cave"));
    cases.extend(ruin_cases_neighbors(&biome("forest")));
    for (name, b, plan, want) in cases {
        let err = plan_ok(&b, &plan);
        assert!(
            err.as_ref().is_err_and(|e| e.contains(want)),
            "{name}: Fehler {err:?}, erwartet mit {want:?}"
        );
    }
}

/// Doppeltes kind@x ist mit höchstens einer Ruine je Chunk und getrennten Arten im Level nicht erreichbar; `mark_spots`
/// lehnt es trotzdem ab, auch zwischen Objekten und Ruinen.
#[test]
fn mark_spots_lehnt_doppelt_ab() {
    let a = Placed {
        kind: "smithy".into(),
        slot: 3,
        at: 10.0,
        dist: 160.0,
        fixed: false,
    };
    let err = mark_spots(
        &mut BTreeSet::new(),
        &[std::slice::from_ref(&a), std::slice::from_ref(&a)],
    );
    assert!(
        err.as_ref().is_err_and(|e| e.contains("doppelt")),
        "Fehler {err:?}, erwartet doppelt"
    );
    let b = Placed {
        kind: "smithy".into(),
        slot: -3,
        at: 40.0,
        dist: 160.0,
        fixed: false,
    };
    let ok = mark_spots(
        &mut BTreeSet::new(),
        &[std::slice::from_ref(&a), std::slice::from_ref(&b)],
    );
    assert!(ok.is_ok(), "links und rechts sind verschiedene Lagen: {ok:?}");
}

#[test]
fn ruine_fehler_nennt_den_platz() {
    let err = plan_ok(&biome("forest"), &[with_ruin(p(-4, "forest", &[]), "castle", 10.0)]);
    assert!(
        err.as_ref().is_err_and(|e| e.contains("Platz -4")),
        "Fehler {err:?}, erwartet mit Platz -4"
    );
}

#[test]
fn ruine_liest_streng() {
    let strict = |raw: &str| serde_json::from_str::<PlannedChunk>(raw);
    let bad = strict(r#"{"slot":3,"kind":"forest","buildings":[{"kind":"smithy","att":3}]}"#);
    assert!(bad.is_err(), "unbekanntes Feld in einer Ruine muss abgelehnt werden");
    let ok = strict(r#"{"slot":3,"kind":"forest","buildings":[{"kind":"smithy","at":3}]}"#);
    assert!(
        ok.as_ref().is_ok_and(|c| c.buildings.len() == 1),
        "gültige Ruine: {ok:?}"
    );
    let out = serde_json::to_string(&p(3, "forest", &[])).expect("JSON");
    assert!(!out.contains("buildings"), "ohne Ruine kein Feld buildings: {out}");
}

/// testdata/planned/forest-ruin.json ist gültig und trägt je Seed alle drei Ruinen (Fixture für die Sim, WZ14.3).
#[test]
#[ignore = "wartet auf RU6.5"]
fn ruinen_fixture() {
    let raw = std::fs::read_to_string(k3c_det::golden::testdata().join("planned/forest-ruin.json")).expect("Fixture");
    let c: PlannedCase = serde_json::from_str(&raw).expect("PlannedCase");
    for seed in &c.seeds {
        let o = Options {
            planned: c.planned.clone(),
            ..Options::default()
        };
        let l = generate_with(&biome(&c.biome), seed, &o).unwrap_or_else(|e| panic!("Seed {seed:?}: {e}"));
        assert_eq!(l.ruins.len(), 3, "Seed {seed:?}: {} Ruinen, erwartet 3", l.ruins.len());
    }
}

/// Die Ruine würfelt nichts: Level mit und ohne Ruine sind bis auf `Layout::ruins` gleich.
#[test]
#[ignore = "wartet auf RU6.5"]
fn ruine_im_level_ohne_wuerfe() {
    let forest = biome("forest");
    let plain = [p(3, "clearing", &[obj("tree", 10.0)])];
    let with = [with_ruin(p(3, "clearing", &[obj("tree", 10.0)]), "smithy", 30.0)];
    for seed in ["a", "ruine", "Käse🧀"] {
        let (a, mut b) = (gen2(&forest, seed, &plain), gen2(&forest, seed, &with));
        let want = vec![Ruin {
            kind: "smithy".into(),
            x: b.hub_center_units + 50.0 + 2.0 * 50.0 + 30.0,
        }];
        assert_eq!(b.ruins, want, "Seed {seed:?}: Ruinen");
        assert!(a.ruins.is_empty(), "Seed {seed:?}: ohne Ruine {:?}", a.ruins);
        b.ruins.clear();
        assert_eq!(a, b, "Seed {seed:?}: die Ruine ändert das Level");
    }
}

/// B-523: Eine Ruine auf der Lage eines Hub-Platzes gleicher Art wäre für die Sim ein Hub-Platz (isRuin: Art und Lage).
#[test]
fn ruine_auf_hub_platz_gleicher_art_wird_abgelehnt() {
    let g = geometry_of(&biome("forest"), 0, rules().hub_sites.clone());
    let at = |kind: &str, slot, dist| Placed {
        kind: kind.into(),
        slot,
        at: 25.0,
        dist,
        fixed: false,
    };
    let cases = [
        ("Rüstkammer rechts", at("armory", 1, 40.0), true),
        ("Werkstatt links", at("workshop", -1, 16.0), true),
        ("Lager (Insel-Platz)", at("storage", -1, 24.0), true),
        ("Bogner links", at("bowyer", -1, 28.0), true),
        ("andere Art", at("tavern", 1, 40.0), false),
        ("andere Seite", at("workshop", 1, 16.0), false),
    ];
    for (name, pl, want_err) in cases {
        let err = check_ruin_place(&pl, &g, &[], rules());
        let ok = if want_err {
            err.as_ref().is_err_and(|e| e.contains("Hub-Platz"))
        } else {
            err.is_ok()
        };
        assert!(ok, "{name}: Fehler {err:?}, erwartet Fehler {want_err}");
    }
}

/// B-522: Zufalls-Ressourcen halten Abstand zur Ruine ihres Chunks; ohne Ruine bleibt der Inhalt unverändert.
#[test]
#[ignore = "wartet auf RU6.5"]
fn zufalls_ressourcen_halten_abstand_zur_ruine() {
    let forest = biome("forest");
    for at in [10.0, 25.0, 40.0] {
        let plan = [with_ruin(p(3, "forest", &[]), "smithy", at)];
        for seed in 0..300 {
            let l = gen2(&forest, &seed.to_string(), &plan);
            assert_eq!(l.ruins.len(), 1, "Seed {seed}: {} Ruinen", l.ruins.len());
            let start = l.hub_center_units + 50.0 + 2.0 * forest.chunk_width_units;
            let inner: Vec<_> = l
                .entities
                .iter()
                .filter(|e| e.x >= start && e.x < start + forest.chunk_width_units)
                .collect();
            for e in &inner {
                assert!(
                    (e.x - l.ruins[0].x).abs() >= RUIN_GAP_UNITS as f64,
                    "Seed {seed}, Ruine bei {at}: {} bei {} liegt auf der Ruine",
                    e.kind,
                    e.x - start
                );
            }
            assert!(!inner.is_empty(), "Seed {seed}: der Chunk enthält keine Ressourcen");
        }
    }
}
