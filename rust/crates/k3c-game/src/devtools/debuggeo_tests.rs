//! Port von `native/k3c-game/devtools/debuggeo_test.go`.

use std::rc::Rc;

use k3c_level::types::{Chunk, Layout};
use k3c_platform::NullCanvas;
use k3c_sim::state::{Enemy, Player, Site, Troop, World};

use super::debuggeo::{
    GEO_CAMERA, GEO_CHUNK, GEO_EDGE, GEO_ENEMY, GEO_HALF, GEO_MONARCH, GEO_SITE, GEO_TROOP, Shape, geometry, pay_range,
};
use super::new;
use crate::view::cell::{Cell, new_camera, rect, window_cell};
use crate::view::devkeys::{DEBUG_GEOMETRY, DevView};
use crate::view::frame::{Frame, Seat, Stage};

/// geo_world ist eine Welt von 100 Units mit Chunks zu 20 Units, Monarch bei 50, Gegner bei 52, Bauplatz bei 46 und
/// Truppe weit außerhalb der Kamera.
pub(super) fn geo_world() -> World {
    let chunks = [0.0, 20.0, 40.0, 60.0, 80.0]
        .iter()
        .map(|s| Chunk {
            start_units: *s,
            ..Chunk::default()
        })
        .collect();
    World {
        width_units: 100.0,
        level: Layout {
            chunks,
            ..Layout::default()
        },
        players: vec![Player {
            index: 0,
            x: 50.0,
            ..Player::default()
        }],
        enemies: vec![Enemy {
            id: 2,
            x: 52.0,
            ..Enemy::default()
        }],
        sites: vec![Site {
            id: 3,
            x: 46.0,
            ..Site::default()
        }],
        troops: vec![Troop {
            id: 4,
            x: 95.0,
            ..Troop::default()
        }],
        ..World::default()
    }
}

/// geo_cell ist eine Zelle 640 × 360 mit Kamera bei x 50 und 32 px je Unit (sichtbar 40–60).
pub(super) fn geo_cell() -> Cell {
    let r = rect(0, 0, 640, 360);
    Cell {
        index: 0,
        player: 0,
        rect: r,
        camera: new_camera(r, 50.0, 32.0),
    }
}

fn count_kind(ss: &[Shape], kind: usize) -> usize {
    ss.iter().filter(|s| s.kind == kind).count()
}

/// AC-04 (B-888): Rahmen sitzen um die Welt-x der Figuren (Mitte der Zelle = Kamera), Bauplätze zeigen die
/// Zahl-Reichweite, unsichtbare Figuren fehlen; Chunk-Grenzen nur im sichtbaren Bereich, dazu die Kamera-Mitte.
#[test]
fn test_geometry() {
    let mut c = geo_cell();
    let mut ss = Vec::new();
    geometry(&mut ss, &geo_world(), &c);
    for s in &ss {
        match s.kind {
            GEO_MONARCH => {
                let mid = (s.x0 + s.x1) / 2.0;
                assert!(
                    mid == 320.0 && s.x1 - s.x0 == 2.0 * GEO_HALF[GEO_MONARCH] * 32.0 && s.y1 == c.ground_y() as f64,
                    "Monarch: {s:?}"
                );
            }
            GEO_ENEMY => assert_eq!((s.x0 + s.x1) / 2.0, 320.0 + 2.0 * 32.0, "Gegner: {s:?}"),
            GEO_SITE => assert_eq!(s.x1 - s.x0, 2.0 * pay_range() * 32.0, "Bauplatz: {s:?}"),
            _ => {}
        }
    }
    assert_eq!(
        (
            count_kind(&ss, GEO_TROOP),
            count_kind(&ss, GEO_CHUNK),
            count_kind(&ss, GEO_CAMERA),
            count_kind(&ss, GEO_EDGE)
        ),
        (0, 2, 1, 0)
    );
    c.camera = new_camera(c.rect, 5.0, 32.0);
    ss.clear();
    geometry(&mut ss, &geo_world(), &c);
    assert_eq!(count_kind(&ss, GEO_EDGE), 1, "linke Weltkante");
}

/// Das Overlay zeichnet ohne Schrift und ohne Welt ohne Absturz.
#[test]
fn test_debug_draw() {
    let parts = new(None);
    let mut dst = NullCanvas::default();
    let c = geo_cell();
    let win = window_cell(rect(0, 0, 640, 360));
    let mut f = Frame {
        cells: vec![c],
        dev: DevView {
            debug: DEBUG_GEOMETRY,
            ..DevView::default()
        },
        ..Frame::default()
    };
    for d in &parts.drawers {
        d.draw(&mut dst, &f, &win);
    }
    assert_eq!(dst.shapes, 0, "ohne Welt keine Formen");
    f.seats = vec![Seat::default()];
    f.stages = vec![Stage {
        index: 0,
        world: Some(Rc::new(geo_world())),
        ..Stage::default()
    }];
    for d in &parts.drawers {
        d.draw(&mut dst, &f, &win);
    }
    assert!(dst.shapes > 0, "keine Formen");
}
