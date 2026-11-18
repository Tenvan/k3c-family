//! Hintergrund-Kacheln (Port von `engine/level/backdrop.go`).

use k3c_rng::Rng;

/// Breite einer Hintergrund-Kachel in Units (Regeldatei § 10: Schichten 24 × 15 Kacheln).
pub const BACKDROP_TILE_UNITS: f64 = 24.0;

/// Go: `backdrops`, legt je Hintergrund-Kachel eine Variante 1 … n fest (GEB18, B-618): deterministisch nach Biom und
/// Seed aus dem eigenen Strom `<biom>:<seed>:backdrop`, damit der Level-Strom unverändert bleibt. Benachbarte Kacheln
/// haben verschiedene Varianten (bei n ≥ 2). Die Folge reicht über die ganze Weltbreite plus eine Kachel. n ≤ 0 = keine
/// Varianten (leere Liste).
pub fn backdrops(biome_id: &str, seed: &str, n: i64, width_units: f64) -> Vec<i64> {
    if n <= 0 {
        return Vec::new();
    }
    let mut r = Rng::new(&format!("{biome_id}:{seed}:backdrop"));
    let len = (width_units / BACKDROP_TILE_UNITS).ceil() as usize + 1;
    let mut out = vec![0_i64; len];
    for i in 0..len {
        if i == 0 || n == 1 {
            out[i] = r.int(1, n);
            continue;
        }
        let mut v = r.int(1, n - 1); // eine der n-1 Varianten außer der vorigen
        if v >= out[i - 1] {
            v += 1;
        }
        out[i] = v;
    }
    out
}

#[cfg(test)]
#[path = "backdrop_tests.rs"]
mod backdrop_tests;
