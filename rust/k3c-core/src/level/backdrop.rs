//! Hintergrund-Varianten je Kachel (Port von `engine/level/backdrop.go`, GEB18, B-618) aus dem eigenen Strom
//! `<biom>:<seed>:backdrop`; benachbarte Kacheln haben verschiedene Varianten (bei n ≥ 2).

use crate::rng::Rng;

/// Breite einer Hintergrund-Kachel in Units.
pub const BACKDROP_TILE_UNITS: f64 = 24.0;

/// Variante 1 … n je Kachel über die Weltbreite plus eine Kachel; n ≤ 0 = keine.
pub fn backdrops(biome_id: &str, seed: &str, n: i64, width_units: f64) -> Vec<i64> {
    if n <= 0 {
        return Vec::new();
    }
    let mut r = Rng::new(&format!("{biome_id}:{seed}:backdrop"));
    let len = (width_units / BACKDROP_TILE_UNITS).ceil() as usize + 1;
    let mut out: Vec<i64> = Vec::with_capacity(len);
    for i in 0..len {
        if i == 0 || n == 1 {
            out.push(r.int(1, n));
            continue;
        }
        let mut v = r.int(1, n - 1); // eine der n-1 Varianten außer der vorigen
        if v >= out[i - 1] {
            v += 1;
        }
        out.push(v);
    }
    out
}
