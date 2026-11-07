//! Golden-Tests des Rust-Generators: jedes Level aus `testdata/golden/level-*.json` entsteht bitgleich (gleicher
//! JSON-Baum, gleiche Zahlen). Biome und Regeln liest der Test direkt aus `data/` (im Betrieb liefert Go sie).

use super::biome::Input;
use super::{Error, Options, generate, generate_json};
use serde_json::{Value, json};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

fn read(rel: &str) -> Value {
    let path = format!("{ROOT}/{rel}");
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// Regeln wie `LevelInput` in `engine/core/level.go`, aus den Dateien unter `data/`.
fn rules() -> Value {
    let hub = read("data/hub.json");
    let mut sites = hub["sites"].as_array().cloned().unwrap_or_default();
    sites.extend(hub["islandSites"].as_array().cloned().unwrap_or_default());
    let ruins = read("data/ruins.json");
    let buildings = read("data/buildings.json");
    let mut edges = serde_json::Map::new();
    for (kind, b) in buildings.as_object().into_iter().flatten() {
        if let Some(strip) = b.get("plantation").and_then(|p| p.get("stripUnits")) {
            edges.insert(kind.clone(), strip.clone());
        }
    }
    json!({
        "payRangeUnits": read("data/economy.json")["payRangeUnits"],
        "sitePlacement": hub["sitePlacement"],
        "hubSites": sites,
        "guards": read("data/guards.json"),
        "ruinKinds": ruins["kinds"],
        "ruinEdges": edges,
    })
}

fn input(id: &str) -> Vec<u8> {
    let v = json!({ "biome": read(&format!("data/biomes/{id}.json")), "rules": rules() });
    serde_json::to_vec(&v).unwrap()
}

fn check_golden(file: &str, id: &str, opts: &Value) {
    let golden = read(&format!("testdata/golden/{file}"));
    assert_eq!(golden["biome"], id, "{file}");
    let levels = golden["levels"].as_array().expect("levels");
    assert!(!levels.is_empty(), "{file}: keine Level");
    let inp = input(id);
    let opts = serde_json::to_vec(opts).unwrap();
    for want in levels {
        let seed = want["seed"].as_str().expect("seed");
        let got = generate_json(&inp, seed, &opts)
            .unwrap_or_else(|e| panic!("{file}, Seed {seed:?}: {e:?}"));
        let got: Value = serde_json::from_str(&got).expect("Ausgabe ist JSON");
        assert!(
            got == *want,
            "{file}, Seed {seed:?}: {}",
            first_diff("level", want, &got)
        );
    }
}

/// Pfad der ersten Abweichung (wie `golden.Diff`).
fn first_diff(at: &str, want: &Value, got: &Value) -> String {
    match (want, got) {
        (Value::Object(w), Value::Object(g)) => {
            for k in w.keys().chain(g.keys()) {
                match (w.get(k), g.get(k)) {
                    (Some(a), Some(b)) if a != b => return first_diff(&format!("{at}.{k}"), a, b),
                    (Some(_), Some(_)) => {}
                    _ => return format!("{at}.{k} (Schlüssel nur auf einer Seite)"),
                }
            }
            String::new()
        }
        (Value::Array(w), Value::Array(g)) if w.len() == g.len() => {
            let i = (0..w.len()).find(|&i| w[i] != g[i]).unwrap_or(0);
            first_diff(&format!("{at}[{i}]"), &w[i], &g[i])
        }
        _ => format!("{at} = {got}, erwartet {want}"),
    }
}

#[test]
fn golden_biome_bitgleich() {
    for id in ["forest", "cave", "mine", "ironhold", "crystal"] {
        check_golden(&format!("level-{id}.json"), id, &json!({}));
    }
}

#[test]
fn golden_geplant_bitgleich() {
    let case = read("testdata/planned/forest.json");
    check_golden(
        "level-planned.json",
        case["biome"].as_str().unwrap(),
        &json!({ "planned": case["planned"] }),
    );
}

#[test]
fn optionen_mit_go_namen_und_leer() {
    let inp = input("forest");
    let a = generate_json(&inp, "7", b"").unwrap();
    let b = generate_json(&inp, "7", br#"{"Planned":null,"ChunkCount":0}"#).unwrap();
    assert_eq!(a, b);
    let c = generate_json(&inp, "7", br#"{"ChunkCount":24}"#).unwrap();
    assert!(c.contains(r#""widthUnits":1200"#), "{c}");
}

#[test]
fn fehler_wie_go() {
    let inp = input("forest");
    let e = generate_json(&inp, "x", br#"{"chunkCount":17}"#).unwrap_err();
    assert_eq!(
        e,
        Error::Generate("eingestellte Chunk-Anzahl 17 in forest: gerade und mindestens 6".into())
    );
    let e = generate_json(&inp, "x", br#"{"planned":[{"slot":5,"kind":"volcano"}]}"#).unwrap_err();
    assert_eq!(
        e,
        Error::Generate("geplante Chunks: auf Platz +5: unbekannte Chunk-Art \"volcano\"".into())
    );
    let e = generate_json(
        &inp,
        "x",
        br#"{"planned":[{"slot":1,"kind":"forest","object":[]}]}"#,
    )
    .unwrap_err();
    assert!(
        matches!(e, Error::Input(_)),
        "unbekanntes Feld muss abgelehnt werden: {e:?}"
    );
    assert!(matches!(
        generate_json(b"{", "x", b""),
        Err(Error::Input(_))
    ));
}

#[test]
fn sim_felder_und_ruine() {
    let inp: Input = serde_json::from_slice(&input("forest")).unwrap();
    let o: Options = serde_json::from_value(json!({
        "planned": [{ "slot": 3, "kind": "clearing", "objects": [{ "kind": "tree", "at": 10 }],
                      "buildings": [{ "kind": "smithy", "at": 30 }], "enemies": [{ "kind": "goblin", "at": 40 }] }],
    }))
    .unwrap();
    let l = generate(&inp, "ruine", &o).unwrap();
    let start = l.hub + 50.0 + 2.0 * 50.0;
    assert_eq!(l.ruins.len(), 1);
    assert_eq!(l.ruins[0].x, start + 30.0);
    assert_eq!(l.guards[0].x, start + 40.0);
    assert_eq!(l.free_ground.len(), 4, "je Seite Mauer und Turm der Zone 1");
    assert!(l.sites.iter().any(|s| s.kind == "nursery"));
}
