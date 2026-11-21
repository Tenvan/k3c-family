//! RU36.7 (B-1166): Wald-Levels sind dicht bewachsen, Lichtungen selten (Standard-Seeds, `data/biomes/forest.json`).

use crate::testutil::{biome, generate};

const SEEDS: usize = 60;

/// (Anteil Lichtungen an allen Chunks, Bäume je Chunk) über viele Seeds.
fn measure() -> (f64, f64) {
    let b = biome("forest");
    let (mut chunks, mut clearings, mut trees) = (0usize, 0usize, 0usize);
    for i in 0..SEEDS {
        let l = generate(&b, &format!("wald-{i}"));
        chunks += l.chunks.len();
        clearings += l.chunks.iter().filter(|c| c.kind == "clearing").count();
        trees += l.entities.iter().filter(|e| e.kind == "tree").count();
    }
    (clearings as f64 / chunks as f64, trees as f64 / chunks as f64)
}

#[test]
fn wald_ist_dicht_und_lichtungen_sind_selten() {
    let (clear, trees) = measure();
    eprintln!("🐛 Lichtungen {clear:.3}, Bäume je Chunk {trees:.1}");
    assert!(
        clear <= 0.10,
        "Lichtungen {clear:.3} der Chunks, erwartet höchstens 0,10"
    );
    assert!(trees >= 15.0, "{trees:.1} Bäume je Chunk, erwartet mindestens 15");
}
