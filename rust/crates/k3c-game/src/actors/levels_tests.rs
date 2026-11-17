//! Port von `native/k3c-game/actors/levels_test.go`.

use std::collections::BTreeMap;

use k3c_data::{Source, biome};

use super::*;
use crate::actors::catalog::load_catalog;

/// Je Biom nur die Sheets der Figuren, die dort vorkommen können: der Wald keine, der Kristall Spinne, Splitterling und
/// den Endboss, die Mine Rattenschwarm (auch als Beschwörung der Ratten-Königin) und Grubengeist. Jedes Sheet gehört zu
/// mindestens einem Biom.
#[test]
fn level_sheets_je_biom() {
    let c = load_catalog(&Source::Embedded).unwrap();
    let want: BTreeMap<&str, Vec<&str>> = BTreeMap::from([
        ("forest", vec![]),
        ("cave", vec!["gegner/fledermaus"]),
        ("mine", vec!["gegner/grubengeist", "gegner/rattenschwarm"]),
        (
            "crystal",
            vec![
                "bosse/kristallherz-waechter",
                "gegner/kristallspinne",
                "gegner/splitterling",
            ],
        ),
    ]);
    let mut all: Vec<String> = Vec::new();
    for id in ["forest", "cave", "mine", "ironhold", "crystal"] {
        let raw = Source::Embedded.read(&format!("biomes/{id}.json")).unwrap();
        let w = SimWorld {
            biome: biome::load(id, &raw).unwrap(),
            ..SimWorld::default()
        };
        let got = c.level_sheets(&w);
        if let Some(w) = want.get(id) {
            assert_eq!(&got, w, "{id}");
        }
        all.extend(got);
    }
    all.sort_unstable();
    all.dedup();
    assert_eq!(all, c.sheet_assets(), "Sheets ohne Biom: alle Biome gegen Katalog");
    // Go: `LevelSheets(nil)` liefert alle; in Rust gibt es keine fehlende Welt (siehe levels.rs).
}
