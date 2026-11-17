//! Tests zu `felling_test.go`. Go baut die Welt mit `sim.CreateIsland` (RU10) und ruft `plan` ganz (mit `decor`, RU16.5);
//! hier steht eine Literal-Welt mit gebauter Baumschule und einem Baum auf Platz 2, gezeichnet über `plantation`
//! (die Bäume der Plantage zeichnet `nodes` nicht). Die Grafik kommt aus `StubSheets` statt aus `public/grafik`.

use k3c_sim::state::{ResourceNode, Site, World};

use super::data::values;
use super::nature::{plant_tree_look, plot_x};
use super::plan::{GOLD, Planner, Shape, px};
use super::plan_objects::fell_shrink;
use super::testutil::{StubSheets, camera};

#[test]
fn fell_shrink_wie_im_browser() {
    for (progress, want) in [(0.0, 1.0), (0.5, 0.85), (1.0, 0.7)] {
        let got = fell_shrink(progress);
        assert!(
            (got - want).abs() <= 1e-9,
            "Fortschritt {progress}: Maßstab {got}, erwartet {want}"
        );
    }
}

/// `plantation_world`: Welt mit gebauter Baumschule und einem Baum auf Platz 2 (Fortschritt `progress`).
fn plantation_world(progress: f64) -> (World, f64) {
    let pl = &values().buildings["nursery"].plantation;
    let farm = Site {
        id: 9999,
        kind: "nursery".to_string(),
        state: "built".to_string(),
        x: 5.0,
        ..Default::default()
    };
    let x = plot_x(farm.x, 2, pl.slots, pl.strip_units);
    let w = World {
        sites: vec![farm],
        nodes: vec![ResourceNode {
            id: 9998,
            kind: "tree".to_string(),
            x,
            progress,
            ..Default::default()
        }],
        ..Default::default()
    };
    (w, x)
}

#[test]
fn plantage_baum_schrumpft_mit_balken() {
    let (w, x) = plantation_world(0.5);
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.plantation(&w, &camera(x));
    let tree = p
        .body
        .iter()
        .rfind(|it| it.shape == Shape::Sprite && it.look == plant_tree_look() && it.x == x)
        .expect("niedrige Pappel erwartet");
    assert!((tree.sy - 0.85).abs() <= 1e-9, "Maßstab 0,85 erwartet: {tree:?}");
    let fill = p
        .top
        .iter()
        .rfind(|it| it.shape == Shape::Rect && it.clr == GOLD && it.x < x)
        .expect("Balken erwartet");
    assert!(
        (fill.w - px(25.0)).abs() <= 1e-9,
        "Balken zur Hälfte gefüllt (50 px · 0,5) erwartet: {fill:?}"
    );
}

#[test]
fn plantage_baum_ungefaellt_ohne_balken() {
    let (w, x) = plantation_world(0.0);
    let mut p = Planner::new(Some(StubSheets::rc()));
    p.plantation(&w, &camera(x));
    for it in &p.top {
        assert!(
            !(it.shape == Shape::Rect && it.clr == GOLD),
            "Balken ohne Fortschritt: {it:?}"
        );
    }
    for it in &p.body {
        assert!(
            it.look != plant_tree_look() || it.sy == 1.0,
            "ungefällter Baum mit Maßstab {}",
            it.sy
        );
    }
}
