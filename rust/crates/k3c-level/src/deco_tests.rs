use crate::testutil::{biome, generate};

/// B-531: Die Höhle (`bush` in `resourcesPerChunk`) zeigt Deko-Büsche, der Wald hat je Waldabschnitt 14 bis 20 Bäume
/// (Werte: data/biomes).
#[test]
fn hoehle_mit_bueschen() {
    let count = |id: &str, kind: &str| -> usize {
        let b = biome(id);
        ["b531-1", "b531-2", "b531-3"]
            .iter()
            .map(|seed| generate(&b, seed).entities.iter().filter(|e| e.kind == kind).count())
            .sum()
    };
    assert!(count("cave", "bush") != 0, "Höhle ohne Büsche");
    let forest = biome("forest");
    let trees = forest
        .resources_per_chunk
        .get("forest")
        .and_then(|r| r.get("tree"))
        .copied()
        .unwrap_or_default();
    assert_eq!(trees, [32, 42], "Wald-Abschnitt: tree {trees:?}, erwartet [32 42]");
    assert!(count("forest", "tree") != 0, "Wald ohne Bäume");
}
