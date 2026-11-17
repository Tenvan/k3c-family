//! Tests zum Drawer des Pakets (`props.go`). Go zeichnet in ein `ebiten.Image`; Rust gegen `NullCanvas` (Ersatz für
//! Zeichentests ohne Bild, der Bildvergleich folgt in RU16.6).

use std::rc::Rc;

use k3c_platform::NullCanvas;
use k3c_sim::state::{Player, ResourceNode, Site, World};

use super::testutil::{FixedSimView, camera, wall_at};
use super::{new, new_pending};
use crate::view::cell::{Cell, rect};
use crate::view::frame::{Frame, Seat, Stage};
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::AssetSource;

fn cell(x: f64) -> Cell {
    Cell {
        index: 0,
        player: 0,
        rect: rect(0, 0, 1280, 360),
        camera: camera(x),
    }
}

fn frame(w: World) -> Frame {
    Frame {
        stages: vec![Stage {
            index: 0,
            level: "forest".to_string(),
            world: Some(Rc::new(w)),
            ..Stage::default()
        }],
        seats: vec![Seat {
            player: 0,
            monarch: 0,
            ..Seat::default()
        }],
        ..Frame::default()
    }
}

fn world() -> World {
    World {
        width_units: 400.0,
        hub_x: 100.0,
        hub_level: 1,
        sites: vec![
            wall_at(1, 110.0, 1, "built"),
            wall_at(2, 118.0, 1, "built"),
            Site::default(),
        ],
        nodes: vec![ResourceNode {
            id: 1,
            kind: "tree".to_string(),
            x: 104.0,
            ..Default::default()
        }],
        players: vec![Player::default()],
        ..Default::default()
    }
}

/// Der Drawer zeichnet einen Zustand ohne Grafik (Platzhalter-Formen) über `Canvas` und steht auf `Layer::BUILDINGS`.
#[test]
fn draw_zeichnet_den_zustand() {
    let mut d = new(None, Box::new(FixedSimView::default()));
    assert_eq!(d.layer(), Layer::BUILDINGS);
    let f = frame(world());
    d.listen(&f);
    let mut canvas = NullCanvas::default();
    d.draw(&mut canvas, &f, &cell(110.0));
    assert!(canvas.rects + canvas.sprites > 0, "nichts gezeichnet: {canvas:?}");
    let before = canvas.textures;
    d.draw(&mut canvas, &f, &cell(110.0));
    assert_eq!(canvas.textures, before, "Vorlagen werden nur einmal angelegt");
}

/// Ohne Platz oder Stufe zeichnet der Drawer nichts, ohne abzustürzen.
#[test]
fn draw_ohne_platz_oder_welt() {
    let d = new(None, Box::new(FixedSimView::default()));
    let mut canvas = NullCanvas::default();
    d.draw(&mut canvas, &Frame::default(), &cell(0.0));
    let mut f = frame(world());
    f.stages[0].world = None;
    d.draw(&mut canvas, &f, &cell(0.0));
    assert_eq!(
        canvas.rects + canvas.sprites + canvas.shapes + canvas.textures as u64,
        0
    );
}

/// Der Drawer nennt die Assets des Levels (AssetSource).
#[test]
fn assets_des_levels() {
    let d = new_pending(None);
    let got = d.assets("forest", &World::default());
    assert!(got.iter().any(|a| a.contains("lagerfeuer")), "Burg fehlt: {got:?}");
    assert!(got.windows(2).all(|p| p[0] < p[1]), "nicht sortiert und einmalig");
}
