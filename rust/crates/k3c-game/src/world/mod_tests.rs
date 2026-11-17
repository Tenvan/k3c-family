//! Tests zu `mod.rs` (world.go): `new_roles` aus camera_test.go und der Welt-Drawer gegen `NullCanvas` (AC-01).

use std::rc::Rc;

use k3c_platform::NullCanvas;
use k3c_sim::state::World as SimWorld;

use super::*;
use crate::view::frame::{Seat, Stage};
use crate::world::testutil::{cycle, layer_cell};

/// New meldet genau eine Kamera-Regel und eine Asset-Quelle (game nimmt höchstens eine Regel an).
#[test]
fn new_roles() {
    let p = new(None);
    assert!(
        p.camera_rule.is_some() && p.asset_source.is_some(),
        "Regel oder Quelle fehlt"
    );
    for d in &p.drawers {
        assert!(d.layer().per_cell(), "Ebene {} nicht je Zelle", d.layer());
    }
    let layers: Vec<Layer> = p.drawers.iter().map(|d| d.layer()).collect();
    assert_eq!(
        layers,
        [
            Layer::FAR,
            Layer::MID,
            Layer::NEAR,
            Layer::GROUND,
            Layer::RIVER,
            Layer::RIVER
        ]
    );
}

fn day_frame() -> Frame {
    let mut wd = SimWorld {
        width_units: 100.0,
        cycle: cycle("day", 0.5),
        ..SimWorld::default()
    };
    wd.biome.id = "forest".into();
    Frame {
        clock: 1.0,
        stages: vec![Stage {
            index: 0,
            level: "forest".into(),
            world: Some(Rc::new(wd)),
            ..Stage::default()
        }],
        seats: vec![Seat::default()],
        ..Frame::default()
    }
}

/// fill zeichnet ein Rechteck mit Farbe, Helligkeit und Deckkraft; ohne Fläche oder Deckkraft nichts.
#[test]
fn fill_draws_rect() {
    let w = World::new(None);
    let mut cv = NullCanvas::default();
    w.fill(&mut cv, 0.0, 0.0, 10.0, 10.0, [255, 0, 0, 255], 1.0, 1.0);
    assert_eq!(cv.rects, 1);
    w.fill(&mut cv, 0.0, 0.0, 0.0, 10.0, [255, 0, 0, 255], 1.0, 1.0);
    w.fill(&mut cv, 0.0, 0.0, 10.0, 10.0, [255, 0, 0, 255], 1.0, 0.0);
    assert_eq!(cv.rects, 1, "leere Fläche und Deckkraft 0 zeichnen nichts");
}

/// AC-01: Der Welt-Drawer zeichnet je Ebene in der Reihenfolge von Go: fern = Himmel, Boden = Ufer, Fluss = Kacheln,
/// Licht am Tag nichts. Ohne Grafik (lib = None) bleiben die Kulissen leer.
#[test]
fn draw_order_and_counts() {
    let p = new(None);
    let (f, c) = (day_frame(), layer_cell(50.0));
    let mut counts = Vec::new();
    for d in &p.drawers {
        let mut cv = NullCanvas::default();
        d.draw(&mut cv, &f, &c);
        counts.push((d.layer(), cv.rects, cv.sprites));
    }
    assert_eq!(counts[0], (Layer::FAR, 1, 0), "fern: Himmel");
    assert_eq!(counts[1], (Layer::MID, 0, 0), "mitte ohne Grafik");
    assert_eq!(counts[2], (Layer::NEAR, 0, 0), "nah ohne Grafik");
    assert_eq!(counts[3], (Layer::GROUND, 1, 0), "Boden: Ufer-Streifen");
    assert!(counts[4].2 > 0 && counts[4].1 == 0, "Fluss: Kacheln {:?}", counts[4]);
    assert_eq!(counts[5], (Layer::RIVER, 0, 0), "Licht am Tag, ohne Übergang");
}

/// Ohne Stufe mit Welt zeichnet kein Drawer etwas.
#[test]
fn draw_without_world() {
    let p = new(None);
    let c = layer_cell(50.0);
    for d in &p.drawers {
        let mut cv = NullCanvas::default();
        d.draw(&mut cv, &Frame::default(), &c);
        assert_eq!((cv.rects, cv.sprites, cv.textures), (0, 0, 0), "Ebene {}", d.layer());
    }
}

/// Die Asset-Quelle nennt die Kulissen des Levels, ein unbekanntes Level keine.
#[test]
fn asset_source() {
    let p = new(None);
    let src = p.asset_source.unwrap();
    let w = SimWorld::default();
    assert!(!src.assets("forest", &w).is_empty());
    assert!(src.assets("gibtsnicht", &w).is_empty());
}
