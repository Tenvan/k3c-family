use std::collections::BTreeSet;

use super::backdrops;
use crate::testutil::{biome, generate};

/// Gleiche Eingabe gleiche Folge, Varianten 1 … n, Nachbarn verschieden, deckt die Weltbreite (GEB18, B-618/AC-03).
#[test]
fn backdrops_folge() {
    let a = backdrops("forest", "abc", 5, 480.0);
    assert_eq!(a, backdrops("forest", "abc", 5, 480.0), "gleicher Seed, andere Folge");
    assert_eq!(a.len(), 21, "Länge, erwartet 480/24+1 = 21");
    let mut seen = BTreeSet::new();
    for (i, &v) in a.iter().enumerate() {
        assert!((1..=5).contains(&v), "Kachel {i}: Variante {v} außerhalb 1 … 5");
        assert!(
            i == 0 || v != a[i - 1],
            "Kachel {i}: gleiche Variante {v} wie der Nachbar"
        );
        seen.insert(v);
    }
    assert!(
        seen.len() >= 4,
        "nur {} verschiedene Varianten in 21 Kacheln: {a:?}",
        seen.len()
    );
    assert!(
        a != backdrops("forest", "xyz", 5, 480.0) && a != backdrops("cave", "abc", 5, 480.0),
        "anderer Seed oder anderes Biom ergibt dieselbe Folge"
    );
    let eine = backdrops("forest", "abc", 1, 100.0);
    assert!(
        eine.len() == 6 && eine[0] == 1 && eine[5] == 1,
        "eine Variante: {eine:?}"
    );
    assert!(backdrops("forest", "abc", 0, 100.0).is_empty(), "ohne Varianten");
}

/// Das Level trägt die Folge aus den Biom-Daten, Chunks und Entities bleiben wie ohne sie.
#[test]
#[ignore = "wartet auf RU6.5"]
fn level_backdrops() {
    for id in ["forest", "cave", "mine", "ironhold", "crystal"] {
        let mut b = biome(id);
        assert_eq!(b.backdrop_variants, 5, "{id}: backdropVariants, erwartet 5");
        let l = generate(&b, "golden");
        assert_eq!(
            l.backdrops,
            backdrops(id, "golden", 5, l.width_units),
            "{id}: Folge im Level weicht ab"
        );
        b.backdrop_variants = 0;
        let ohne = generate(&b, "golden");
        assert!(
            ohne.chunks == l.chunks && ohne.entities == l.entities,
            "{id}: die Hintergrund-Folge verändert das Level"
        );
    }
}
