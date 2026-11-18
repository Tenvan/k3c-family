use crate::testutil::{biome, biome_ids, generate};
use crate::types::{Chunk, Layout};

/// Kein Lava-Streifen (Chunk-Mitte ± widthUnits/2) liegt auf Mauer, Turm oder Tor einer Linie (B-291/AC-01); Biome ohne
/// Lava bleiben unberührt.
#[test]
fn lava_nicht_auf_linien_500_seeds() {
    for id in biome_ids() {
        let b = biome(&id);
        if b.lava.width_units <= 0.0 {
            continue;
        }
        let mut lava = 0;
        for seed in 0..500 {
            let l = generate(&b, &seed.to_string());
            for c in l.chunks.iter().filter(|c| c.kind == "lava") {
                lava += 1;
                check_lava_abseits_linien(&id, seed, b.lava.width_units, &l, c);
            }
        }
        assert!(lava != 0, "{id}: kein Lava-Chunk in 500 Seeds");
    }
}

/// Prüft, dass ein Lava-Chunk keinen Mauer- oder Turm-Platz trifft.
fn check_lava_abseits_linien(id: &str, seed: i64, width: f64, l: &Layout, c: &Chunk) {
    let mid = c.start_units + l.chunk_width_units / 2.0;
    for site in &l.sites {
        assert!(
            (site.x - mid).abs() > width / 2.0,
            "{id} Seed {seed}: Lava bei {mid:.1} trifft Platz {} bei {:.1}",
            site.kind,
            site.x
        );
    }
}
