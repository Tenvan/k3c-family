//! Tests zum Himmels-Effekt des Fests (RU62.13, B-1002): je Welt der Effekt aus `World::festival`, ohne Fest, ohne Eintrag
//! und unter Tage nichts, nur mit dem Hub im Bild.

use k3c_platform::NullCanvas;
use k3c_sim::state::World;
use k3c_sim::state::festival_view::FestivalState;

use super::*;
use crate::fx::paint::Painter;
use crate::view::cell::{Cell, new_camera, rect};

fn cell(x: f64) -> Cell {
    let r = rect(0, 0, 1920, 1080);
    Cell {
        player: 0,
        rect: r,
        camera: new_camera(r, x, 1080.0 * 2.0 / 3.0 / 13.0),
        ..Cell::default()
    }
}

fn world(effect: Option<&str>, depth: i64) -> World {
    let mut w = World {
        hub_x: 40.0,
        width_units: 100.0,
        festival: effect.map(|e| FestivalState { effect: e.to_string() }),
        ..Default::default()
    };
    w.biome.depth = depth;
    w
}

fn sprites(w: &World, cam_x: f64) -> u64 {
    let mut p = Painter::default();
    let mut cv = NullCanvas::default();
    let st = p.stencils(&mut cv);
    draw(&st, &mut cv, w, &cell(cam_x), 3.0, None);
    cv.sprites
}

#[test]
fn der_effekt_folgt_dem_zustand_der_welt() {
    assert_eq!(sky_of(&world(Some("lanterns"), 0)), Sky::Lanterns);
    assert_eq!(sky_of(&world(Some("torches"), 0)), Sky::Torches);
    assert_eq!(sky_of(&world(None, 0)), Sky::None, "ohne Fest");
    assert_eq!(sky_of(&world(Some("x"), 0)), Sky::None, "unbekannter Effekt");
    assert_eq!(sky_of(&world(Some("lanterns"), 1)), Sky::None, "unter Tage");
}

#[test]
fn sengoku_laternen_und_nil_fackeln_mit_bluten() {
    let lant = sprites(&world(Some("lanterns"), 0), 40.0);
    let nil = sprites(&world(Some("torches"), 0), 40.0);
    assert!(lant >= LANTERNS as u64, "Laternen: {lant}");
    assert!(nil >= BLOSSOMS as u64 + 4, "Fackeln und Blüten: {nil}");
    assert_eq!(sprites(&world(None, 0), 40.0), 0, "ohne Fest nichts");
    assert_eq!(sprites(&world(Some("lanterns"), 1), 40.0), 0, "unter Tage nichts");
}

#[test]
fn nur_mit_dem_hub_im_bild() {
    let w = world(Some("lanterns"), 0);
    assert!(sprites(&w, 40.0) > 0);
    assert_eq!(sprites(&w, 400.0), 0, "Hub weit außerhalb");
}

#[test]
fn laternen_steigen_und_blenden_an_den_enden() {
    let (_, h0, a0) = lantern(3, 0.0, 40.0);
    let (_, h1, _) = lantern(3, 4.0, 40.0);
    assert!(h1 != h0, "bewegt sich");
    assert!((0.0..=1.0).contains(&a0));
    for i in 0..LANTERNS {
        let (_, h, a) = lantern(i, 7.7, 40.0);
        assert!((1.5..=9.5).contains(&h) && (0.0..=1.0).contains(&a), "{i}: {h} {a}");
    }
}
