use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::{merge_nature, scale_count};
use crate::biome::load_biome;
use crate::testutil::{TestOptions, biome, biome_ids, count_entities, generate_nature, generate_with, validate};
use crate::types::{Layout, NatureValues};

fn traits(list: &[(&str, i64)]) -> NatureValues {
    let mut v = NatureValues::default();
    for (k, val) in list {
        v.set_trait(k, *val);
    }
    v
}

fn weights(group: &str, list: &[(&str, i64)]) -> NatureValues {
    let mut v = NatureValues::default();
    for (k, val) in list {
        v.set_weight(group, k, *val);
    }
    v
}

/// Go: `natureCounts`, zählt Bäume und Felsen und die Bäume je Baumart.
fn nature_counts(l: &Layout) -> (usize, usize, BTreeMap<String, usize>) {
    let (mut trees, mut rocks, mut species) = (0, 0, BTreeMap::new());
    for e in &l.entities {
        match e.kind.as_str() {
            "tree" => {
                trees += 1;
                *species.entry(e.variant.clone()).or_default() += 1;
            }
            "rock" => rocks += 1,
            _ => {}
        }
    }
    (trees, rocks, species)
}

/// AC-01: gleicher Seed, gleiche Werte und gleiche Natur; die Werte liegen in den Bereichen des Bioms.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_gleicher_seed_gleiche_natur() {
    let b = biome("forest");
    let none = NatureValues::default();
    let (a, c) = (generate_nature(&b, "eiche", &none), generate_nature(&b, "eiche", &none));
    assert!(
        a.nature == c.nature && a.entities == c.entities,
        "gleicher Seed ergibt andere Natur"
    );
    for e in &b.traits.0 {
        let v = a.nature.traits.get(&e.key).copied().unwrap_or(0);
        assert!(
            v >= e.value.min && v <= e.value.max,
            "{} = {v} außerhalb {}–{}",
            e.key,
            e.value.min,
            e.value.max
        );
    }
    for e in &a.entities {
        assert_eq!(
            e.kind == "tree",
            !e.variant.is_empty(),
            "Variante falsch gesetzt: {e:?}"
        );
    }
}

/// AC-02: Zwei Seeds unterscheiden sich im selben Biom in Walddichte oder Mischung.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_seeds_unterscheiden_sich() {
    let b = biome("forest");
    let none = NatureValues::default();
    let first = generate_nature(&b, "seed-0", &none).nature;
    let differs = ["seed-1", "seed-2", "seed-3", "seed-4"]
        .iter()
        .any(|s| generate_nature(&b, s, &none).nature != first);
    assert!(differs, "fünf Seeds ergeben dieselben Natur-Werte");
}

/// AC-02: Die Walddichte ändert die Zahl der Bäume messbar (1 gegen 5).
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_walddichte_skaliert_baeume() {
    let b = biome("forest");
    let (low, _, _) = nature_counts(&generate_nature(&b, "dichte", &traits(&[("forestDensity", 1)])));
    let (high, _, _) = nature_counts(&generate_nature(&b, "dichte", &traits(&[("forestDensity", 5)])));
    assert!(
        high >= 3 * low,
        "Walddichte 5 ergibt {high} Bäume, Walddichte 1 {low}: erwartet etwa Faktor 5"
    );
    let (_, rocks, _) = nature_counts(&generate_nature(&b, "dichte", &traits(&[("rockDensity", 0)])));
    assert_eq!(rocks, 0, "Steindichte 0 ergibt {rocks} Felsen");
}

/// AC-03: Ein fest gesetzter Wert gilt unabhängig vom Seed.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_fester_wert_gilt() {
    let b = biome("forest");
    let mut fixed = traits(&[("forestDensity", 4)]);
    fixed.set_weight("treeSpecies", "birke", 2);
    for seed in ["a", "b", "c", "d"] {
        let n = generate_nature(&b, seed, &fixed).nature;
        let birke = n.weights.get("treeSpecies").and_then(|m| m.get("birke")).copied();
        assert!(
            n.traits.get("forestDensity") == Some(&4) && birke == Some(2),
            "Seed {seed}: fester Wert überschrieben: {n:?}"
        );
    }
}

/// AC-04: Monokultur erzeugt nur die eine Art.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_monokultur() {
    let b = biome("forest");
    let mono = weights("treeSpecies", &[("tanne", 0), ("pappel", 3), ("birke", 0)]);
    let (trees, _, species) = nature_counts(&generate_nature(&b, "mono", &mono));
    assert!(
        trees != 0 && species.get("pappel") == Some(&trees),
        "Monokultur Pappel: {trees} Bäume, Arten {species:?}"
    );
}

/// AC-04: Alle Baumarten fest auf 0 lehnt die Prüfung ab; ebenso ein Wert außerhalb des Bereichs und ein unbekannter
/// Schlüssel.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_feste_werte_abgelehnt() {
    let b = biome("forest");
    let cases = [
        (
            "alle 0",
            weights("treeSpecies", &[("tanne", 0), ("pappel", 0), ("birke", 0)]),
        ),
        ("Bereich", traits(&[("forestDensity", 9)])),
        ("unbekannt", traits(&[("pilze", 1)])),
        ("Art", weights("treeSpecies", &[("eiche", 3)])),
        ("Gruppe", weights("pilze", &[("a", 1)])),
    ];
    for (name, v) in cases {
        let o = TestOptions {
            nature: v,
            ..TestOptions::default()
        };
        assert!(generate_with(&b, "x", &o).is_err(), "{name}: nicht abgelehnt");
    }
}

/// Das Biom `test` aus den Werten der Basis, darüber die Felder von `over`; `portals` sind gültig, damit nur die
/// Natur-Prüfung ablehnen kann.
fn test_biome(over: &Value) -> Result<(), String> {
    let mut v = json!({
        "resourcesPerChunk": {"a": {"tree": [1, 2]}},
        "chunkWeights": {"a": 1},
        "portals": {"hp": 1, "defense": {"intervalSeconds": 1}},
    });
    if let (Some(base), Some(add)) = (v.as_object_mut(), over.as_object()) {
        base.extend(add.clone());
    }
    load_biome("test", &v.to_string())
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Biome mit kaputten Natur-Daten: alle Gewichte 0, Bereich verkehrt, Skalierung einer fremden Art.
fn tr(body: Value) -> Value {
    let mut t = json!({"name": "D"});
    if let (Some(m), Some(add)) = (t.as_object_mut(), body.as_object()) {
        m.extend(add.clone());
    }
    json!({"traits": {"d": t}})
}

fn datenpruefung_faelle() -> Vec<(&'static str, Value)> {
    let opts = |kind: &str, options: Value| json!({"weights": {"g": {"name": "G", "kind": kind, "options": options}}});
    vec![
        ("Gewichte 0", opts("tree", json!({"a": {"min": 0, "max": 0}}))),
        ("Gewicht 6", opts("tree", json!({"a": {"min": 0, "max": 6}}))),
        ("fremde Art", opts("gold", json!({"a": {"min": 0, "max": 2}}))),
        ("ohne Varianten", opts("tree", json!({}))),
        (
            "Bereich",
            tr(json!({"min": 3, "max": 1, "default": 2, "scales": ["tree"]})),
        ),
        (
            "default 0",
            tr(json!({"min": 0, "max": 1, "default": 0, "scales": ["tree"]})),
        ),
        (
            "fremde Res",
            tr(json!({"min": 1, "max": 3, "default": 2, "scales": ["gold"]})),
        ),
        (
            "fremder Chunk",
            tr(json!({"min": 1, "max": 3, "default": 2, "chunks": ["moor"]})),
        ),
        (
            "zwei Wirkungen",
            tr(json!({"min": 1, "max": 3, "default": 2, "scales": ["tree"], "display": true})),
        ),
        ("ohne Wirkung", tr(json!({"min": 1, "max": 3, "default": 2}))),
        (
            "ohne Name",
            json!({"traits": {"d": {"min": 1, "max": 3, "default": 2, "scales": ["tree"]}}}),
        ),
        (
            "doppelt",
            json!({"traits": {
                "d": {"name": "D", "min": 1, "max": 3, "default": 2, "scales": ["tree"]},
                "e": {"name": "E", "min": 1, "max": 3, "default": 2, "scales": ["tree"]},
            }}),
        ),
    ]
}

/// Datenprüfung des Bioms: alle Gewichte 0, Bereich verkehrt, Skalierung einer fremden Art.
#[test]
fn natur_datenpruefung() {
    for (name, raw) in datenpruefung_faelle() {
        let err = test_biome(&raw);
        assert!(
            err.as_ref().is_err_and(|e| !e.contains("portals")),
            "{name}: nicht abgelehnt ({err:?})"
        );
    }
    let mut ok = json!({"min": 1, "max": 3, "default": 2});
    ok["scales"] = json!(["tree"]);
    assert!(test_biome(&tr(ok)).is_ok(), "gültige Eigenschaft abgelehnt");
    for id in biome_ids() {
        let raw = std::fs::read_to_string(crate::testutil::data_dir().join(format!("biomes/{id}.json"))).expect("Biom");
        load_biome(&id, &raw).unwrap_or_else(|e| panic!("{id}: {e}"));
    }
}

/// Ohne Eigenschaften bleibt das Level wie zuvor: neutrale Werte ändern keinen Wurf.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_neutraler_wert_aendert_nichts() {
    let b = biome("forest");
    let mut neutral = NatureValues::default();
    for e in &b.traits.0 {
        neutral.set_trait(&e.key, e.value.default);
    }
    let mut plain = b.clone();
    plain.traits.0.clear();
    plain.weights.0.clear();
    let (want, got) = (
        generate_nature(&plain, "neutral", &NatureValues::default()),
        generate_nature(&b, "neutral", &neutral),
    );
    assert_eq!(
        want.entities.len(),
        got.entities.len(),
        "neutrale Werte ändern das Level"
    );
    for (i, (w, g)) in want.entities.iter().zip(&got.entities).enumerate() {
        assert!(
            w.kind == g.kind && w.x == g.x,
            "Entität {i} weicht ab: {w:?} gegen {g:?}"
        );
    }
}

#[test]
fn natur_scale_count_rundet() {
    for (n, v, def, want) in [
        (22, 3, 3, 22),
        (22, 1, 3, 7),
        (30, 5, 3, 50),
        (3, 0, 2, 0),
        (1, 1, 2, 1),
    ] {
        let got = scale_count(n, v, def);
        assert_eq!(got, want, "scale_count({n}, {v}, {def})");
    }
    let merged = merge_nature(&traits(&[("a", 1)]), &traits(&[("a", 2)]));
    assert_eq!(merged.traits.get("a"), Some(&2), "merge_nature: over gewinnt nicht");
}

/// Chunk-Eigenschaften verschieben die Chunk-Arten: Lichtungen 0 und Bachläufe 0 lassen keine dieser Chunks übrig.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_chunk_gewichte() {
    let b = biome("forest");
    let l = generate_nature(&b, "chunks", &traits(&[("clearings", 0), ("streams", 0)]));
    for c in &l.chunks {
        assert!(
            c.kind != "clearing" && c.kind != "stream",
            "Chunk {} ist {} trotz Gewicht 0",
            c.index,
            c.kind
        );
    }
    // Alle Chunk-Gewichte 0 (Kristallhöhle): Die Gewichte des Bioms bleiben, das Level entsteht.
    generate_nature(
        &biome("crystal"),
        "chunks",
        &traits(&[("geodes", 0), ("grottoShare", 0)]),
    );
}

/// Adern-Eigenschaften ändern die Zahl der Adern, die Prüfung zählt mit den Natur-Werten der Stufe.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_adern() {
    let b = biome("cave");
    for (v, want) in [(1, 1), (2, 2), (3, 3)] {
        let l = generate_nature(&b, "adern", &traits(&[("stoneVeins", v)]));
        let n = count_entities(&l, "stoneVein");
        assert_eq!(n, want, "Steinadern {v}: {n} Adern, erwartet {want}");
        let errs = validate(&l, &b);
        assert!(errs.is_empty(), "Steinadern {v}: {errs:?}");
    }
}

/// Kristallfarben: Gewichts-Gruppe auf Adern, jede Ader bekommt eine Farbe.
#[test]
#[ignore = "wartet auf RU6.5"]
fn natur_kristallfarben() {
    let fixed = weights("crystalColors", &[("blau", 0), ("violett", 4), ("gruen", 0)]);
    let l = generate_nature(&biome("crystal"), "farben", &fixed);
    for e in &l.entities {
        assert!(
            e.kind != "crystalVein" || e.variant == "violett",
            "Ader {e:?} ist nicht violett"
        );
    }
}
