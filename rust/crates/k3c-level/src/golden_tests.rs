use k3c_det::golden;
use serde_json::Value;

use crate::planned_gen::Options;
use crate::testutil::{PlannedCase, biome, biome_ids, generate, generate_with, validate};
use crate::types::Layout;

/// Go: `TestGolden`, jedes Level der Golden-Datei je Biom wird mit dem gleichen Seed erzeugt und Baum für Baum verglichen.
#[test]
fn golden() {
    for id in biome_ids() {
        let g = golden::read(golden::testdata().join(format!("golden/level-{id}.json")));
        let levels = g["levels"]
            .as_array()
            .unwrap_or_else(|| panic!("level-{id}.json ohne Level"));
        assert!(g["biome"] == id && !levels.is_empty(), "level-{id}.json unbrauchbar");
        let b = biome(&id);
        for want in levels {
            let seed = want["seed"].as_str().unwrap_or_default();
            let got = generate(&b, seed);
            let d = golden::diff("level", want, &golden::tree(&got));
            assert!(d.is_empty(), "Biom {id}, Seed {seed:?}: {d}");
        }
    }
}

#[test]
fn golden_mit_umlaut_und_emoji() {
    let raw = std::fs::read_to_string(golden::testdata().join("golden/level-forest.json")).unwrap_or_default();
    assert!(
        raw.contains("\"seed\":\"Käse🧀\""),
        "level-forest.json deckt Seed Käse🧀 nicht ab"
    );
}

/// Go: `TestGoldenPlanned`, der Golden-Fall mit Test-Planung (WZ8.3, B-461/AC-03): Der Generator setzt geplante Chunks
/// deterministisch ein.
#[test]
fn golden_planned() {
    let raw = std::fs::read_to_string(golden::testdata().join("planned/forest.json")).unwrap_or_default();
    let c: PlannedCase = serde_json::from_str(&raw).unwrap_or_default();
    assert!(
        !c.seeds.is_empty() && !c.planned.is_empty(),
        "testdata/planned/forest.json unbrauchbar"
    );
    let b = biome(&c.biome);
    let o = Options {
        planned: c.planned.clone(),
        ..Options::default()
    };
    let g = golden::read(golden::testdata().join("golden/level-planned.json"));
    let want = g["levels"].as_array().cloned().unwrap_or_default();
    assert!(
        g["biome"] == c.biome && want.len() == c.seeds.len(),
        "level-planned.json unbrauchbar"
    );
    for (seed, want) in c.seeds.iter().zip(&want) {
        let l: Layout = generate_with(&b, seed, &o).unwrap_or_else(|e| panic!("Seed {seed:?}: {e}"));
        let errs = validate(&l, &b);
        assert!(errs.is_empty(), "Seed {seed:?}: {errs:?}");
        let d = golden::diff("level", want, &golden::tree(&l));
        assert!(d.is_empty(), "Seed {seed:?}: {d}");
    }
}

/// Go: `TestGoldenNatur` (nature.json): je Biom und Seed die geltenden Natur-Werte, die Zahl der Objekte je Art und der
/// Varianten (AC-01, B-891).
#[test]
fn golden_natur() {
    let mut got = Vec::new();
    for id in biome_ids() {
        for seed in ["golden", "Käse🧀"] {
            let l = generate(&biome(&id), seed);
            let (mut kinds, mut variants) = (serde_json::Map::new(), serde_json::Map::new());
            for e in &l.entities {
                bump(&mut kinds, &e.kind);
                if !e.variant.is_empty() {
                    bump(&mut variants, &format!("{}.{}", e.kind, e.variant));
                }
            }
            got.push(serde_json::json!({
                "biome": id, "seed": seed, "nature": golden::tree(&l.nature), "kinds": kinds, "variants": variants,
            }));
        }
    }
    let want = golden::read(golden::testdata().join("golden/nature.json"));
    let d = golden::diff("natur", &want["cases"], &Value::Array(got));
    assert!(d.is_empty(), "{d}");
}

fn bump(m: &mut serde_json::Map<String, Value>, key: &str) {
    let n = m.get(key).and_then(Value::as_i64).unwrap_or(0) + 1;
    m.insert(key.to_string(), Value::from(n));
}
