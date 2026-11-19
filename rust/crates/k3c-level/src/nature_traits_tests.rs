//! B-891: Die Eigenschaften je Biom in `data/biomes/*.json` gegen die Tabelle des Tickets.

use crate::testutil::{biome, biome_ids};

/// Wirkung einer Eigenschaft laut B-891: Anzahl (`scales`), Chunk-Gewicht (`chunks`) oder Darstellung (`display`).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Wirkung {
    Skaliert,
    Chunk,
    Anzeige,
}

use Wirkung::{Anzeige, Chunk, Skaliert};

/// B-891 Tabelle: (Biom, Eigenschaft, min, max, Wirkung, Ziele), je Biom in Ticket-Reihenfolge.
type Eigenschaft = (&'static str, &'static str, i64, i64, Wirkung, &'static [&'static str]);

const EIGENSCHAFTEN: &[Eigenschaft] = &[
    ("forest", "forestDensity", 1, 5, Skaliert, &["tree"]),
    ("forest", "rockDensity", 0, 3, Skaliert, &["rock"]),
    ("forest", "bushDensity", 0, 3, Skaliert, &["bush"]),
    ("forest", "clearings", 0, 3, Chunk, &["clearing"]),
    ("forest", "streams", 0, 2, Chunk, &["stream"]),
    ("cave", "rockDensity", 1, 5, Skaliert, &["rock"]),
    ("cave", "ceilingHeight", 1, 3, Anzeige, &[]),
    ("cave", "mushrooms", 0, 3, Skaliert, &["tree", "bush"]),
    ("cave", "stalactites", 0, 3, Anzeige, &[]),
    ("cave", "chasms", 0, 2, Chunk, &["chasm"]),
    ("cave", "stoneVeins", 1, 3, Skaliert, &["stoneVein"]),
    ("mine", "copperDensity", 1, 5, Skaliert, &["copperOre", "copperVein"]),
    ("mine", "railShare", 0, 3, Chunk, &["rails"]),
    ("mine", "decay", 0, 3, Anzeige, &[]),
    ("mine", "rubble", 0, 3, Skaliert, &["rock"]),
    ("mine", "shafts", 0, 2, Chunk, &["shaft"]),
    ("ironhold", "ironDensity", 1, 5, Skaliert, &["ironVein"]),
    ("ironhold", "lavaShare", 0, 3, Chunk, &["lava"]),
    ("ironhold", "forges", 0, 2, Chunk, &["forge"]),
    ("ironhold", "glow", 0, 3, Anzeige, &[]),
    ("ironhold", "galleryWidth", 1, 3, Chunk, &["gallery"]),
    ("crystal", "crystalDensity", 1, 5, Skaliert, &["crystalVein"]),
    ("crystal", "geodes", 0, 3, Chunk, &["geode"]),
    ("crystal", "brightness", 1, 3, Anzeige, &[]),
    ("crystal", "grottoShare", 0, 3, Chunk, &["grotto"]),
];

/// B-891 Tabelle: Gewichtsgruppen (Biom, Gruppe, kind, Varianten), Gewicht 0 bis 5 je Variante.
const GEWICHTE: &[(&str, &str, &str, &[&str])] = &[
    ("forest", "treeSpecies", "tree", &["tanne", "pappel", "birke"]),
    ("crystal", "crystalColors", "crystalVein", &["blau", "violett", "gruen"]),
];

/// B-891: Die echten `data/biomes/*.json` führen je Biom genau die Eigenschaften und Gewichte der Ticket-Tabelle,
/// mit Bereich, genau einer Wirkung und neutralem Default (bei min 0 der Wert 1, bei min 1 die Mitte).
#[test]
fn biome_traits_vollstaendig() {
    let mut ids = biome_ids();
    ids.sort();
    let mut erwartet: Vec<&str> = EIGENSCHAFTEN.iter().map(|e| e.0).collect();
    erwartet.sort();
    erwartet.dedup();
    assert_eq!(ids, erwartet, "Biome in data/biomes weichen von der Tabelle ab");
    for id in &ids {
        let b = biome(id);
        let soll: Vec<_> = EIGENSCHAFTEN.iter().filter(|e| e.0 == id).collect();
        let ist: Vec<&str> = b.traits.0.iter().map(|e| e.key.as_str()).collect();
        let namen: Vec<&str> = soll.iter().map(|e| e.1).collect();
        assert_eq!(ist, namen, "{id}: Eigenschaften (Namen und Reihenfolge)");
        for (_, name, min, max, wirkung, ziele) in soll {
            let Some(t) = b.traits.get(name) else { continue };
            let w = format!("{id}.{name}");
            assert!(!t.name.is_empty(), "{w}: Anzeigename fehlt");
            assert_eq!((t.min, t.max), (*min, *max), "{w}: Bereich");
            let neutral = if *min == 0 { 1 } else { (min + max) / 2 };
            assert_eq!(t.default, neutral, "{w}: neutraler Default");
            let ist_w = (!t.scales.is_empty(), !t.chunks.is_empty(), t.display);
            let soll_w = (*wirkung == Skaliert, *wirkung == Chunk, *wirkung == Anzeige);
            assert_eq!(ist_w, soll_w, "{w}: genau eine Wirkung {wirkung:?}");
            let ist_z = if t.scales.is_empty() { &t.chunks } else { &t.scales };
            assert_eq!(ist_z, ziele, "{w}: Ziele der Wirkung");
        }
        let gruppen: Vec<&str> = b.weights.0.iter().map(|e| e.key.as_str()).collect();
        let soll_g: Vec<&str> = GEWICHTE.iter().filter(|g| g.0 == id).map(|g| g.1).collect();
        assert_eq!(gruppen, soll_g, "{id}: Gewichtsgruppen");
        for (_, gruppe, kind, varianten) in GEWICHTE.iter().filter(|g| g.0 == id) {
            let Some(g) = b.weights.get(gruppe) else { continue };
            let optionen: Vec<&str> = g.options.0.iter().map(|e| e.key.as_str()).collect();
            assert_eq!(g.kind, *kind, "{id}.{gruppe}: kind");
            assert!(!g.name.is_empty(), "{id}.{gruppe}: Anzeigename fehlt");
            assert_eq!(optionen, *varianten, "{id}.{gruppe}: Varianten");
            for e in &g.options.0 {
                assert_eq!(
                    (e.value.min, e.value.max),
                    (0, 5),
                    "{id}.{gruppe}.{}: Gewicht 0 bis 5",
                    e.key
                );
            }
        }
    }
}
