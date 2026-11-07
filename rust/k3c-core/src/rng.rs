//! Deterministischer Zufallsgenerator (mulberry32), Port von `engine/rng/rng.go`. Gleicher Seed, gleiche Folge;
//! geprüft gegen `testdata/golden/rng.json`. Rust fusioniert `x*y + z` nie von selbst zu FMA, Zwischenwerte
//! werden trotzdem explizit gerundet (wie `float64(...)` im Go-Code, B-071).

/// Generator mit 32-Bit-Zustand.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u32,
}

/// FNV-1a über UTF-16-Codeeinheiten wie `charCodeAt` in TypeScript.
pub fn hash_seed(seed: &str) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    for c in seed.encode_utf16() {
        h ^= u32::from(c);
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

impl Rng {
    /// Generator für einen Text-Seed.
    pub fn new(seed: &str) -> Self {
        Self {
            state: hash_seed(seed),
        }
    }

    /// Zahl in [0, 1). Name wie `Next` in Go; kein Iterator (der Strom ist unendlich, `next` liefert kein `Option`).
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_add(0x6d2b_79f5);
        let mut t = self.state;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    }

    /// Ganze Zahl in [lo, hi], beide inklusive.
    pub fn int(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() * ((hi - lo + 1) as f64)).floor() as i64
    }

    /// Schlüssel nach Gewicht, in Listenreihenfolge. Panik ohne positives Gewicht wie im Go-Code.
    pub fn weighted<'a>(&mut self, weights: &'a [(String, f64)]) -> &'a str {
        let total: f64 = weights.iter().map(|w| w.1).sum();
        assert!(
            total > 0.0,
            "Weighted braucht mindestens ein positives Gewicht"
        );
        let mut roll: f64 = self.next() * total;
        for (key, w) in weights {
            roll -= w;
            if roll < 0.0 {
                return key;
            }
        }
        &weights[weights.len() - 1].0
    }

    /// Index eines zufälligen Elements (Pick); Panik bei leerer Liste.
    pub fn pick_index(&mut self, len: usize) -> usize {
        assert!(len > 0, "Pick auf leerer Liste");
        self.int(0, len as i64 - 1) as usize
    }

    /// Mischt in place (Fisher-Yates von hinten wie in TS).
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.int(0, i as i64) as usize;
            items.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn golden() -> Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../testdata/golden/rng.json"
        );
        serde_json::from_str(&std::fs::read_to_string(path).expect("rng.json lesbar"))
            .expect("rng.json gültig")
    }

    fn floats(v: &Value) -> Vec<f64> {
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_f64().unwrap())
            .collect()
    }

    #[test]
    fn golden_rng_bitgleich() {
        let g = golden();
        let p = &g["params"];
        let (lo, hi) = (p["int"][0].as_i64().unwrap(), p["int"][1].as_i64().unwrap());
        let weights: Vec<(String, f64)> = p["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| (w[0].as_str().unwrap().to_string(), w[1].as_f64().unwrap()))
            .collect();
        let picks: Vec<&str> = p["pick"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect();
        let shuffle_n = p["shuffle"].as_u64().unwrap() as usize;
        for e in g["seeds"].as_array().unwrap() {
            let seed = e["seed"].as_str().unwrap();
            assert_eq!(
                u64::from(hash_seed(seed)),
                e["hash"].as_u64().unwrap(),
                "Hash {seed:?}"
            );
            let mut r = Rng::new(seed);
            let want = floats(&e["next"]);
            let got: Vec<f64> = (0..want.len()).map(|_| r.next()).collect();
            assert_eq!(
                got.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                want.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                "next {seed:?}"
            );
            let mut r = Rng::new(seed);
            let want = e["int"].as_array().unwrap();
            for (i, w) in want.iter().enumerate() {
                assert_eq!(r.int(lo, hi), w.as_i64().unwrap(), "int[{i}] {seed:?}");
            }
            let mut r = Rng::new(seed);
            for (i, w) in e["weighted"].as_array().unwrap().iter().enumerate() {
                assert_eq!(
                    r.weighted(&weights),
                    w.as_str().unwrap(),
                    "weighted[{i}] {seed:?}"
                );
            }
            let mut items: Vec<i64> = (0..shuffle_n as i64).collect();
            Rng::new(seed).shuffle(&mut items);
            let want: Vec<i64> = e["shuffle"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_i64().unwrap())
                .collect();
            assert_eq!(items, want, "shuffle {seed:?}");
            let mut r = Rng::new(seed);
            for (i, w) in e["pick"].as_array().unwrap().iter().enumerate() {
                assert_eq!(
                    picks[r.pick_index(picks.len())],
                    w.as_str().unwrap(),
                    "pick[{i}] {seed:?}"
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = "Pick auf leerer Liste")]
    fn pick_leer_panikt() {
        Rng::new("x").pick_index(0);
    }
}
