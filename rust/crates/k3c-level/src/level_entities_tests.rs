use crate::testutil::{biome, p, test_gen};
use crate::types::Chunk;

fn chunk(index: i64, kind: &str, cw: f64) -> Chunk {
    Chunk {
        index,
        kind: kind.into(),
        start_units: index as f64 * cw,
    }
}

/// B-1079: Gibt es keinen Chunk für Skill-Punkte (nur Hub, Rand oder geplant), meldet Rust einen Fehler statt wie Go
/// zu panicken (Entscheidung 012).
#[test]
fn entities_leerer_versteck_pool_ist_fehler() {
    let mut b = biome("forest");
    b.skill_points.min = 1;
    b.skill_points.max = 1;
    let mut g = test_gen(&b, "forest:versteck", 4);
    let cw = g.cw;
    g.planned.insert(2, p(2, "plain", &[]));
    let chunks = [
        chunk(0, "edge", cw),
        chunk(1, "hub", cw),
        chunk(2, "plain", cw),
        chunk(3, "edge", cw),
    ];
    let err = g
        .entities(&chunks)
        .expect_err("ohne Versteck-Chunk muss ein Fehler kommen");
    assert_eq!(err, "keine Chunks für Skill-Punkte");
}

/// Gegenprobe: Mit einem freien Chunk gibt es den Skill-Punkt.
#[test]
fn entities_versteck_pool_mit_chunk_setzt_skill_punkt() {
    let mut b = biome("forest");
    b.skill_points.min = 1;
    b.skill_points.max = 1;
    let mut g = test_gen(&b, "forest:versteck", 4);
    let cw = g.cw;
    let chunks = [
        chunk(0, "edge", cw),
        chunk(1, "hub", cw),
        chunk(2, "plain", cw),
        chunk(3, "edge", cw),
    ];
    let out = g.entities(&chunks).expect("Chunk 2 ist ein Versteck");
    assert_eq!(out.iter().filter(|e| e.kind == "skillPoint").count(), 1);
}
