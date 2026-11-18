//! Zufallszahlen: Port von engine/rng (deterministisch, ohne fremde Crates).
//!
//! Der deterministische Zufallsgenerator (mulberry32), Port von src/core/rng.ts.
//! Gleicher Seed => gleiche Zahlenfolge wie in TypeScript; geprüft gegen testdata/golden/rng.json.
//! Go rechnet `uint32` umlaufend; hier deshalb jede Rechnung auf dem Zustand mit `wrapping_*`.

/// Rng liefert eine feste Zahlenfolge je Seed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: u32,
}

/// Weight ist ein Eintrag für `weighted`. Eine Liste statt einer map, weil TS in Einfügereihenfolge zieht.
#[derive(Debug, Clone, PartialEq)]
pub struct Weight {
    pub key: String,
    pub w: f64,
}

/// `hash_seed` macht aus einem Text eine 32-Bit-Zahl (FNV-1a) über UTF-16-Codeeinheiten wie `charCodeAt`.
pub fn hash_seed(seed: &str) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    for c in seed.encode_utf16() {
        h ^= u32::from(c);
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

impl Rng {
    /// `new` erzeugt einen Generator für einen Text-Seed.
    pub fn new(seed: &str) -> Rng {
        Rng { state: hash_seed(seed) }
    }

    /// `next` liefert eine Zahl in [0, 1).
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_add(0x6d2b_79f5);
        let mut t = self.state;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
        f64::from(t ^ (t >> 14)) / 4_294_967_296.0
    }

    /// `int` liefert eine ganze Zahl in [lo, hi], beide inklusive.
    pub fn int(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() * (hi - lo + 1) as f64).floor() as i64
    }

    /// `weighted` zieht einen Schlüssel nach Gewicht, in der Reihenfolge der Liste.
    #[allow(clippy::panic)] // Vertrag wie Go/TS: keine positiven Gewichte ist ein Programmfehler (Test should_panic)
    pub fn weighted(&mut self, weights: &[Weight]) -> String {
        let mut total = 0.0;
        for w in weights {
            total += w.w;
        }
        if total <= 0.0 {
            panic!("Weighted braucht mindestens ein positives Gewicht");
        }
        // Go rundet das Produkt mit float64(…) gegen FMA (B-071); Rust fusioniert nie.
        let mut roll = self.next() * total;
        for w in weights {
            roll -= w.w;
            if roll < 0.0 {
                return w.key.clone();
            }
        }
        weights[weights.len() - 1].key.clone()
    }
}

/// `pick` wählt ein Element; wie in TS ein Fehler bei leerer Liste.
#[allow(clippy::panic)] // Vertrag wie TS: leere Liste ist ein Programmfehler (Test should_panic)
pub fn pick<T: Clone>(r: &mut Rng, items: &[T]) -> T {
    if items.is_empty() {
        panic!("Pick auf leerer Liste");
    }
    items[r.int(0, items.len() as i64 - 1) as usize].clone()
}

/// `shuffle` mischt in place (Fisher-Yates von hinten wie in TS) und gibt die Liste zurück.
pub fn shuffle<T>(r: &mut Rng, mut items: Vec<T>) -> Vec<T> {
    let mut i = items.len() as i64 - 1;
    while i > 0 {
        let j = r.int(0, i);
        items.swap(i as usize, j as usize);
        i -= 1;
    }
    items
}

#[cfg(test)]
mod lib_tests;
