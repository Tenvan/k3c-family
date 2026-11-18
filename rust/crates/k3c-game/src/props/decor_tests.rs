//! Tests zu `decor.go` und `decor_glow.go` (Go: `decor_test.go`, 8 Tests). Die Welt entsteht wie in Go als Literal
//! aus einem Biom der Daten (`data/biomes/<id>.json`) mit 400 Units Breite und den gesetzten Werten.

use k3c_data::biome;
use k3c_sim::state::{ResourceNode, World};

use super::decor::{CEILING_ROCK, STAL_COLOR, VIEW_TOP};
use super::decor_glow::{BEAM_WOOD, EMBER_COLOR, HAZE_COLOR, RUBBLE_STONE, vein_glow};
use super::plan::{Item, Planner, Shape};
use super::testutil::{FixedSimView, camera};
use crate::view::cell::Camera;

/// `biome_world` baut eine Stufe des Bioms `id` (Daten aus data/biomes) mit 400 Units Breite und den Werten `traits`.
fn biome_world(id: &str, traits: &[(&str, i64)]) -> World {
    let path = format!("{}/../../../data/biomes/{id}.json", env!("CARGO_MANIFEST_DIR"));
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut w = World {
        biome: biome::load(id, &raw).expect("Biom"),
        width_units: 400.0,
        ..Default::default()
    };
    for (k, v) in traits {
        w.level.nature.traits.insert((*k).to_string(), *v);
    }
    w
}

/// `plan_decor` plant nur die Darstellung der Natur-Eigenschaften (ganze Welt).
fn plan_decor(w: &World) -> Vec<Item> {
    let mut p = Planner::new(None);
    p.decor(w, &Camera::default(), 0.0);
    p.body
}

/// `ceiling_y` ist die Unterkante des Decken-Rechtecks; `None` = es gibt keins.
fn ceiling_y(items: &[Item]) -> Option<f64> {
    items
        .iter()
        .find(|it| it.shape == Shape::Rect && it.clr == CEILING_ROCK)
        .map(|it| it.y)
}

/// B-902/AC-01: Decke nur in Biomen mit ceilingHeight.
#[test]
fn decke_nur_mit_eigenschaft() {
    assert!(
        ceiling_y(&plan_decor(&biome_world("cave", &[]))).is_some(),
        "Höhle ohne Decke"
    );
    for id in ["forest", "mine", "ironhold", "crystal"] {
        assert!(
            ceiling_y(&plan_decor(&biome_world(id, &[]))).is_none(),
            "{id} mit Decke"
        );
    }
}

/// B-902/AC-02: Die Unterkante steigt mit dem Wert; ohne Wert gilt der Default (2).
#[test]
fn decke_folgt_dem_wert() {
    let ys: Vec<f64> = (1..=3)
        .map(|v| ceiling_y(&plan_decor(&biome_world("cave", &[("ceilingHeight", v)]))).expect("Decke"))
        .collect();
    assert!(ys[0] < ys[1] && ys[1] < ys[2] && ys[2] < VIEW_TOP, "Unterkanten {ys:?}");
    assert_eq!(
        ceiling_y(&plan_decor(&biome_world("cave", &[]))),
        Some(ys[1]),
        "ohne Wert"
    );
    assert_eq!(
        ceiling_y(&plan_decor(&biome_world("cave", &[("ceilingHeight", 9)]))),
        Some(ys[2]),
        "Wert über dem Bereich"
    );
}

/// `stal_xs` sind die Orte der Tropfsteine.
fn stal_xs(items: &[Item]) -> Vec<f64> {
    items
        .iter()
        .filter(|it| it.shape == Shape::Tri && it.clr == STAL_COLOR)
        .map(|it| it.x)
        .collect()
}

/// B-903/AC-01, AC-02: Wert 0 und Biome ohne Eigenschaft ohne Tropfsteine; Anzahl steigt streng, Orte bleiben.
#[test]
fn tropfsteine_folgen_dem_wert() {
    let n = stal_xs(&plan_decor(&biome_world("cave", &[("stalactites", 0)]))).len();
    assert_eq!(n, 0, "Wert 0: {n} Tropfsteine");
    let n = stal_xs(&plan_decor(&biome_world("mine", &[]))).len();
    assert_eq!(n, 0, "Mine: {n} Tropfsteine");
    let mut prev: Vec<f64> = Vec::new();
    for v in 1..=3 {
        let xs = stal_xs(&plan_decor(&biome_world("cave", &[("stalactites", v)])));
        assert!(
            xs.len() > prev.len(),
            "Wert {v}: {} nicht mehr als {}",
            xs.len(),
            prev.len()
        );
        for x in &prev {
            assert!(xs.contains(x), "Wert {v}: Tropfstein bei {x} fehlt");
        }
        let again = stal_xs(&plan_decor(&biome_world("cave", &[("stalactites", v)])));
        assert_eq!(xs, again, "Wert {v} nicht gleich");
        prev = xs;
    }
}

/// Die Darstellung bleibt im sichtbaren Teil der Welt (`plan_nur_sichtbares` bleibt leer).
#[test]
fn deko_nur_sichtbar() {
    let w = biome_world("cave", &[("stalactites", 3)]);
    let mut p = Planner::new(None);
    p.decor(&w, &camera(-10000.0), 0.0);
    assert!(p.body.is_empty(), "außerhalb der Welt {} Einträge", p.body.len());
    p.decor(&w, &camera(200.0), 0.0);
    let (l, r) = camera(200.0).span();
    for it in &p.body {
        assert!(
            !(it.shape == Shape::Tri && (it.x < l - 3.0 || it.x > r + 3.0)),
            "Tropfstein bei {} außerhalb {l}…{r}",
            it.x
        );
    }
}

/// `count_of` zählt die Einträge der Form `s` in der Farbe `clr`.
fn count_of(items: &[Item], s: Shape, clr: [u8; 4]) -> usize {
    items.iter().filter(|it| it.shape == s && it.clr == clr).count()
}

/// B-904/AC-01, AC-02: Balken nur mit decay, Wert 0 ohne Verfall, verfallene Rahmen (je drei Brocken) steigen streng.
#[test]
fn stuetzbalken_verfallen_nach_wert() {
    let n = count_of(&plan_decor(&biome_world("cave", &[])), Shape::Line, BEAM_WOOD);
    assert_eq!(n, 0, "Höhle mit {n} Balken");
    let intact = plan_decor(&biome_world("mine", &[("decay", 0)]));
    assert!(
        count_of(&intact, Shape::Line, BEAM_WOOD) != 0 && count_of(&intact, Shape::Rect, RUBBLE_STONE) == 0,
        "Wert 0: Balken fehlen oder Verfall da"
    );
    let mut prev = 0;
    for v in 1..=3 {
        let n = count_of(
            &plan_decor(&biome_world("mine", &[("decay", v)])),
            Shape::Rect,
            RUBBLE_STONE,
        ) / 3;
        assert!(n > prev, "Wert {v}: {n} verfallene Rahmen, vorher {prev}");
        prev = n;
    }
}

/// `haze_alpha` ist die Deckkraft des Hitze-Schleiers (0 = keiner).
fn haze_alpha(items: &[Item]) -> f32 {
    items
        .iter()
        .find(|it| it.shape == Shape::Rect && it.clr == HAZE_COLOR)
        .map_or(0.0, |it| it.alpha)
}

/// B-905/AC-01, AC-02: Glut nur mit glow > 0, Anzahl steigt streng, Schleier wird deckender.
#[test]
fn glut_nach_wert() {
    for w in [biome_world("ironhold", &[("glow", 0)]), biome_world("mine", &[])] {
        let items = plan_decor(&w);
        assert!(
            count_of(&items, Shape::Circle, EMBER_COLOR) == 0 && haze_alpha(&items) == 0.0,
            "{}: Glut ohne Wert",
            w.biome.id
        );
    }
    let (mut prev_n, mut prev_a) = (0, 0.0f32);
    for v in 1..=3 {
        let items = plan_decor(&biome_world("ironhold", &[("glow", v)]));
        let (n, a) = (count_of(&items, Shape::Circle, EMBER_COLOR), haze_alpha(&items));
        assert!(
            n > prev_n && a > prev_a,
            "Wert {v}: {n} Glutpunkte (vorher {prev_n}), Schleier {a} (vorher {prev_a})"
        );
        (prev_n, prev_a) = (n, a);
    }
}

/// `crystal_world` ist eine Kristallhöhle mit je einer Ader je Farbe (rot = unbekannt), einem Fels und Leuchtstärke `v`.
fn crystal_world(v: i64) -> World {
    let mut w = biome_world("crystal", &[("brightness", v)]);
    for (i, c) in ["blau", "violett", "gruen", "rot"].into_iter().enumerate() {
        w.nodes.push(ResourceNode {
            id: i as i64,
            kind: "crystalVein".to_string(),
            x: (20 + 10 * i) as f64,
            variant: c.to_string(),
            ..Default::default()
        });
    }
    w.nodes.push(ResourceNode {
        id: 9,
        kind: "rock".to_string(),
        x: 80.0,
        ..Default::default()
    });
    w
}

/// B-906/AC-01: je Kristall-Ader ein Schein in ihrer Farbe vor der Ader in der Zeichenliste; ohne brightness keiner.
#[test]
fn schein_je_kristall_ader() {
    let mut p = Planner::new(None); // ohne Grafik: Ader als Platzhalter-Kreis (volle Deckkraft, alpha 0)
    let mut w = crystal_world(2);
    p.plan(Some(&w), None, &Camera::default(), 0.0, &FixedSimView::default());
    for (i, c) in ["blau", "violett", "gruen", "blau"].into_iter().enumerate() {
        let x = (20 + 10 * i) as f64;
        let glow = p
            .body
            .iter()
            .position(|it| it.shape == Shape::Circle && it.x == x && it.alpha > 0.0);
        let vein = p
            .body
            .iter()
            .position(|it| it.shape == Shape::Circle && it.x == x && it.alpha == 0.0);
        let (Some(g), Some(v)) = (glow, vein) else {
            panic!("Ader {i} ({c}): Schein {glow:?}, Ader {vein:?}");
        };
        assert!(
            Some(p.body[g].clr) == vein_glow(c) && v > g,
            "Ader {i} ({c}): Schein {g}, Ader {v}"
        );
    }
    assert!(
        !p.body
            .iter()
            .any(|it| it.shape == Shape::Circle && it.x == 80.0 && it.alpha > 0.0),
        "Schein am Fels"
    );
    w.biome.traits = Default::default();
    let n = plan_decor(&w).len();
    assert_eq!(n, 0, "ohne brightness {n} Einträge");
}

/// B-906/AC-02: Radius und Deckkraft des Scheins steigen mit dem Wert.
#[test]
fn schein_nach_wert() {
    let (mut prev_r, mut prev_a) = (0.0, 0.0f32);
    for v in 1..=3 {
        let items = plan_decor(&crystal_world(v));
        let Some(it) = items.iter().find(|it| it.shape == Shape::Circle) else {
            panic!("Wert {v}: kein Schein");
        };
        assert!(it.w > prev_r && it.alpha > prev_a, "Wert {v}: Schein {it:?}");
        (prev_r, prev_a) = (it.w, it.alpha);
    }
}
