//! Fälle aus src/scenes/render/natureSprites.test.ts und plantation.test.ts; die Werte von `mix` stammen aus dem
//! Browser (node, natureSprites.ts › mix), damit Baumform und Phase an derselben Stelle gleich sind.

use std::collections::BTreeSet;

use super::data::values;
use super::nature::*;

#[test]
fn mix_wie_im_browser() {
    let want = [
        (0.0, 3413276431u32),
        (1.4, 3390830866),
        (-0.5, 3413276431),
        (-137.25, 2787311493),
        (412.0, 2386506617),
        (99999.9, 2345423595),
        (7.0, 2896571549),
        (3.5, 2761611335),
    ];
    for (x, v) in want {
        assert_eq!(mix(x), v, "mix({x})");
    }
    let phases = [
        (0.0, 7),
        (1.4, 0),
        (-0.5, 7),
        (-137.25, 4),
        (412.0, 2),
        (99999.9, 6),
        (7.0, 7),
        (3.5, 0),
    ];
    for (x, v) in phases {
        assert_eq!(anim_phase(x, 8), v, "anim_phase({x})");
    }
    assert_eq!(anim_phase(5.0, 1), 0, "ein Frame: Phase 0");
}

#[test]
fn baum_und_fels_aus_der_position() {
    let trees = [
        (0.0, "pappel"),
        (1.4, "pappel"),
        (-137.25, "tanne"),
        (412.0, "birke"),
        (99999.9, "tanne"),
        (7.0, "birke"),
    ];
    for (x, id) in trees {
        assert_eq!(tree_look(x, "forest"), object(id), "Baum bei {x}");
    }
    assert!(
        tree_look(412.2, "forest") == tree_look(412.4, "forest") && tree_look(7.0, "") == tree_look(7.0, "forest"),
        "gerundet und leere Biom-ID wie Wald"
    );
    let mut kinds = BTreeSet::new();
    for i in 0..200 {
        let x = f64::from(i) * 3.7 - 40.0;
        kinds.insert(tree_look(x, "forest").asset);
        assert!(
            tree_look(x, "cave") == object("wurzel") && tree_look(x, "mine") == object("wurzel"),
            "außerhalb des Walds Wurzeln bei {x}"
        );
        kinds.insert(rock_look(x).asset);
    }
    assert_eq!(
        kinds.len(),
        6,
        "drei Baumarten und drei Fels-Formen erwartet: {kinds:?}"
    );
}

fn roots(x: f64, n: i64, biome: &str) -> Vec<f64> {
    let mut out = Vec::new();
    deco_roots(&mut out, x, biome, n);
    out
}

#[test]
fn deco_roots_wie_im_browser() {
    let cases: [(f64, i64, &[f64]); 3] = [
        (5.0, 3, &[0.5, 0.9]),
        (-137.25, 2, &[0.5]),
        (412.0, 4, &[0.4, -0.6, 0.6]),
    ];
    for (x, n, want) in cases {
        assert_eq!(roots(x, n, "cave"), want, "deco_roots({x}, {n})");
    }
    assert!(
        roots(5.0, 3, "forest").is_empty() && roots(5.0, 1, "mine").is_empty(),
        "im Wald und mit einer Wurzel keine Deko"
    );
    let by = &values().roots_by_biom;
    assert!(
        by.get("cave") == Some(&2) && by.get("mine") == Some(&1),
        "rootsPerNode aus data/biomes: {by:?}"
    );
}

#[test]
fn nature_look_und_schilder() {
    for (biome, want) in [
        ("forest", 1),
        ("", 1),
        ("cave", 2),
        ("mine", 3),
        ("ironhold", 3),
        ("neu", 3),
    ] {
        assert_eq!(nature_look(biome), want, "{biome:?}");
    }
    let signs = [
        ("tree", "schild-axt"),
        ("rock", "schild-spitzhacke"),
        ("copperOre", "schild-spitzhacke"),
        ("ironVein", "schild-spitzhacke"),
        ("bush", ""),
    ];
    for (kind, id) in signs {
        let got = mine_sign_look(kind);
        assert!(
            (id.is_empty() && !got.ok()) || (!id.is_empty() && got == object(id)),
            "{kind}: {got:?}"
        );
    }
    assert!(
        bush_look("forest") == object("busch") && bush_look("cave") == object("hoehlenbusch"),
        "Busch je Biom"
    );
}

#[test]
fn plantage_plaetze() {
    let want = [97.0, 98.2, 99.4, 100.6, 101.8, 103.0];
    for (i, x) in want.iter().enumerate() {
        let got = plot_x(100.0, i as i64, 6, 3.0);
        assert!((got - x).abs() <= 1e-9, "Platz {i}: {got}, erwartet {x}");
    }
    assert!(
        on_plot(plot_x(100.0, 2, 6, 3.0), 100.0, 6, 3.0)
            && !on_plot(100.2, 100.0, 6, 3.0)
            && plot_x(50.0, 0, 1, 3.0) == 50.0,
        "Baum auf dem Platz, Waldbaum im Streifen nicht"
    );
    let pl = &values().buildings["nursery"].plantation;
    assert!(
        pl.slots == 6 && pl.strip_units == 3.0,
        "Plantage aus data/buildings.json: {pl:?}"
    );
}

/// B-891: Die Baumart aus dem Level wählt das Bild, im Wald; unbekannte Varianten und andere Biome bleiben beim alten Bild.
#[test]
fn variant_look_baumart() {
    for id in TREE_KINDS {
        assert_eq!(variant_look("tree", id, "forest"), Some(object(id)), "Baumart {id}");
    }
    let cases = [
        ("tree", "eiche", "forest"),
        ("tree", "tanne", "cave"),
        ("crystalVein", "rot", "crystal"),
        ("crystalVein", "", "crystal"),
        ("tree", "", "forest"),
        ("rock", "blau", "crystal"),
    ];
    for c in cases {
        assert!(variant_look(c.0, c.1, c.2).is_none(), "{c:?}: Variante gewählt");
    }
}

/// B-901/AC-01, AC-02: Kristall-Adern zeigen das Bild ihrer Farbe, unbekannte Farben das Bild ohne Variante.
#[test]
fn variant_look_kristallfarbe() {
    for (variant, id) in [
        ("blau", "ader-kristall"),
        ("violett", "ader-kristall-violett"),
        ("gruen", "ader-kristall-gruen"),
    ] {
        assert_eq!(
            variant_look("crystalVein", variant, "crystal"),
            Some(object(id)),
            "Farbe {variant}"
        );
    }
    assert_eq!(
        node_look("crystalVein", 3.0, "crystal"),
        object("ader-kristall"),
        "ohne Variante"
    );
}

#[test]
fn node_und_entity_looks() {
    assert!(!node_look("unbekannt", 1.0, "forest").ok(), "unbekannte Art: keins");
    assert_eq!(node_look("copperOre", 1.0, "mine"), object("erzbrocken"));
    assert_eq!(entity_look("harbor", "forest").map(|(_, dx)| dx), Some(-HARBOR_SHIFT));
    assert_eq!(
        entity_look("recruitCamp", "cave").map(|(l, _)| l.anim),
        Some("stand.s2")
    );
    assert!(entity_look("tree", "forest").is_none());
    let (stern, lift) = pickup_look("skillPoint");
    assert!(stern == object("stern") && (lift - 44.0 / 32.0).abs() < 1e-12);
    assert!(!pickup_look("coin").0.ok());
}
