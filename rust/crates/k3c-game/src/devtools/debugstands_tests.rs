//! WD1.3 (B-1204): Geometrie der Waldstück-Anzeige (AC-03).

use std::sync::Arc;

use k3c_sim::state::{ResourceNode, World, WorldValues};
use k3c_sim::world::stands::OUTER;

use crate::devtools::debuggeo::{GEO_STAND, GEO_STAND_TREE, Shape, geometry};
use crate::devtools::debuggeo_tests::{geo_cell, geo_world};
use crate::devtools::debugstands::{hatch, set_stands_on, stand_shapes, stands_on};

/// Welt von 100 Units (geo_world) mit nativen Regeln und Bäumen bei `xs`.
fn forest_world(xs: &[f64], native: bool) -> World {
    let mut w = geo_world();
    w.level.width_units = 100.0;
    w.vals = Some(Arc::new(WorldValues {
        native_rules: native,
        ..WorldValues::default()
    }));
    for (i, x) in xs.iter().enumerate() {
        w.nodes.push(ResourceNode {
            id: 100 + i64::try_from(i).unwrap(),
            kind: "tree".into(),
            x: *x,
            ..ResourceNode::default()
        });
    }
    w
}

fn count(ss: &[Shape], kind: usize) -> usize {
    ss.iter().filter(|s| s.kind == kind).count()
}

/// Ein Waldstück von 41 bis 59 (Länge 18, sichtbar 40–60) ist klein: eine Fläche, keine Baum-Rahmen.
#[test]
fn kleines_stueck_nur_flaeche() {
    let xs: Vec<f64> = (0..19).map(|i| 41.0 + f64::from(i)).collect();
    let mut ss = Vec::new();
    stand_shapes(&mut ss, &forest_world(&xs, true), &geo_cell());
    assert_eq!((count(&ss, GEO_STAND), count(&ss, GEO_STAND_TREE)), (1, 0));
}

/// Ein großes Stück (Länge ≥ 25) zeigt die äußeren Bäume je Seite, soweit sie im Bild liegen: Kamera 40–60, Stück
/// 38 bis 70 – links liegen 38, 39 außerhalb, drei Rahmen am linken Rand sichtbar; rechts ist keiner sichtbar.
#[test]
fn grosses_stueck_aeussere_baeume_gerahmt() {
    let xs: Vec<f64> = (0..33).map(|i| 38.0 + f64::from(i)).collect();
    let mut ss = Vec::new();
    stand_shapes(&mut ss, &forest_world(&xs, true), &geo_cell());
    assert_eq!(count(&ss, GEO_STAND), 1);
    let sichtbar_links = (0..OUTER).filter(|i| 38.0 + *i as f64 >= 39.5).count();
    assert_eq!(count(&ss, GEO_STAND_TREE), sichtbar_links);
    let area = ss.iter().find(|s| s.kind == GEO_STAND).unwrap();
    assert!(
        area.x0 >= 0.0 && area.x1 <= 640.0,
        "auf die Zelle beschnitten: {area:?}"
    );
    assert!(area.y0 < area.y1);
}

/// Ohne native Regeln (und damit ohne Waldregel) und außerhalb des Bildes gibt es nichts zu zeigen.
#[test]
fn ohne_regel_oder_ausser_bild_leer() {
    let xs: Vec<f64> = (0..19).map(|i| 41.0 + f64::from(i)).collect();
    let mut ss = Vec::new();
    stand_shapes(&mut ss, &forest_world(&xs, false), &geo_cell());
    assert!(ss.is_empty());
    let far: Vec<f64> = (0..19).map(|i| 75.0 + f64::from(i)).collect();
    stand_shapes(&mut ss, &forest_world(&far, true), &geo_cell());
    assert!(ss.is_empty());
}

/// Der Umschalter (F4, Taste `stands`) blendet die Waldstücke in `geometry` aus und wieder ein.
#[test]
fn umschalter_steuert_geometrie() {
    let xs: Vec<f64> = (0..19).map(|i| 41.0 + f64::from(i)).collect();
    let w = forest_world(&xs, true);
    assert!(stands_on(), "Standard ist an");
    let mut ss = Vec::new();
    geometry(&mut ss, &w, &geo_cell());
    assert_eq!(count(&ss, GEO_STAND), 1);
    set_stands_on(false);
    ss.clear();
    geometry(&mut ss, &w, &geo_cell());
    set_stands_on(true);
    assert_eq!(count(&ss, GEO_STAND), 0);
}

/// Schraffur: jede Linie liegt im Rechteck, geht von links unten nach rechts oben und hat 45 Grad.
#[test]
fn schraffur_liegt_im_rechteck() {
    let (x0, y0, x1, y1) = (10.0, 20.0, 110.0, 70.0);
    let ls = hatch(x0, y0, x1, y1, 9.0);
    assert!(ls.len() > 10);
    for [xa, ya, xb, yb] in ls {
        for (x, y) in [(xa, ya), (xb, yb)] {
            assert!((x0..=x1).contains(&x) && (y0..=y1).contains(&y), "({x}, {y})");
        }
        assert!(xb > xa && yb < ya);
        assert!(((xb - xa) - (ya - yb)).abs() < 1e-9, "45 Grad");
    }
    assert!(hatch(5.0, 5.0, 5.0, 9.0, 3.0).is_empty());
}
