//! Port von engine/rng/rng_test.go: Golden testdata/golden/rng.json und Fehlerfälle.

use super::{Rng, Weight, hash_seed, pick, shuffle};
use serde_json::Value;
use std::fmt::Debug;

struct Golden {
    int: (i64, i64),
    weights: Vec<Weight>,
    pick: Vec<String>,
    shuffle: i64,
    seeds: Vec<Value>,
}

fn load() -> Golden {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../testdata/golden/rng.json");
    let raw = std::fs::read_to_string(path).expect("rng.json lesen");
    let g: Value = serde_json::from_str(&raw).expect("rng.json parsen");
    let p = &g["params"];
    let int = (as_i64(&p["int"][0]), as_i64(&p["int"][1]));
    let weights = p["weights"]
        .as_array()
        .expect("weights")
        .iter()
        .map(|w| Weight {
            key: w[0].as_str().expect("Schlüssel").to_string(),
            w: w[1].as_f64().expect("Gewicht"),
        })
        .collect();
    let seeds = g["seeds"].as_array().expect("seeds").clone();
    assert!(!seeds.is_empty(), "rng.json enthält keine Seeds");
    Golden {
        int,
        weights,
        pick: strings(&p["pick"]),
        shuffle: as_i64(&p["shuffle"]),
        seeds,
    }
}

fn as_i64(v: &Value) -> i64 {
    v.as_i64().expect("ganze Zahl")
}

fn strings(v: &Value) -> Vec<String> {
    let list = v.as_array().expect("Liste");
    list.iter().map(|s| s.as_str().expect("Text").to_string()).collect()
}

fn floats(v: &Value) -> Vec<f64> {
    let list = v.as_array().expect("Liste");
    list.iter().map(|x| x.as_f64().expect("Zahl")).collect()
}

fn ints(v: &Value) -> Vec<i64> {
    v.as_array().expect("Liste").iter().map(as_i64).collect()
}

/// `same` meldet die erste Abweichung mit Seed, Funktion und Position.
fn same<T: PartialEq + Debug>(seed: &str, func: &str, got: &[T], want: &[T]) {
    assert_eq!(
        got.len(),
        want.len(),
        "Seed {seed:?}: {func} liefert {} Werte, erwartet {}",
        got.len(),
        want.len()
    );
    for i in 0..want.len() {
        assert!(
            got[i] == want[i],
            "Seed {seed:?}: {func}[{i}] = {:?}, erwartet {:?}",
            got[i],
            want[i]
        );
    }
}

fn times<T>(n: usize, mut f: impl FnMut() -> T) -> Vec<T> {
    (0..n).map(|_| f()).collect()
}

#[test]
fn golden() {
    let g = load();
    for e in &g.seeds {
        let seed = e["seed"].as_str().expect("seed");
        let hash = e["hash"].as_u64().expect("hash");
        assert_eq!(u64::from(hash_seed(seed)), hash, "Seed {seed:?}: hash");

        // Zahlen bitgleich vergleichen, nicht nur numerisch.
        let want: Vec<u64> = floats(&e["next"]).iter().map(|x| x.to_bits()).collect();
        let mut r = Rng::new(seed);
        let got: Vec<u64> = times(want.len(), || r.next().to_bits());
        same(seed, "next", &got, &want);

        let want = ints(&e["int"]);
        let mut r = Rng::new(seed);
        let got = times(want.len(), || r.int(g.int.0, g.int.1));
        same(seed, "int", &got, &want);

        let want = strings(&e["weighted"]);
        let mut r = Rng::new(seed);
        let got = times(want.len(), || r.weighted(&g.weights));
        same(seed, "weighted", &got, &want);

        let items: Vec<i64> = (0..g.shuffle).collect();
        let got = shuffle(&mut Rng::new(seed), items);
        same(seed, "shuffle", &got, &ints(&e["shuffle"]));

        let want = strings(&e["pick"]);
        let mut r = Rng::new(seed);
        let got = times(want.len(), || pick(&mut r, &g.pick));
        same(seed, "pick", &got, &want);
    }
}

#[test]
fn umlaute_und_emoji() {
    let seeds: Vec<String> = load()
        .seeds
        .iter()
        .map(|e| e["seed"].as_str().expect("seed").to_string())
        .collect();
    for s in ["Käse🧀", "Ärger über Öl", "👨‍👩‍👧‍👦"] {
        assert!(
            seeds.iter().any(|x| x == s),
            "rng.json deckt Seed {s:?} nicht ab (B-043/AC-01)"
        );
    }
}

#[test]
#[should_panic(expected = "Pick auf leerer Liste")]
fn fehler_pick_leer() {
    let leer: Vec<i64> = Vec::new();
    pick(&mut Rng::new("x"), &leer);
}

#[test]
#[should_panic(expected = "Weighted braucht mindestens ein positives Gewicht")]
fn fehler_weighted_ohne_positives_gewicht() {
    Rng::new("x").weighted(&[Weight {
        key: "a".to_string(),
        w: 0.0,
    }]);
}

/// Umlauf wie in Go: ein Zustand nahe `u32::MAX` bricht im Debug-Bau nicht ab.
#[test]
fn ueberlauf_rechnet_umlaufend() {
    let mut r = Rng { state: u32::MAX };
    let x = r.next();
    assert!((0.0..1.0).contains(&x));
    assert_eq!(r.state, u32::MAX.wrapping_add(0x6d2b_79f5));
}
