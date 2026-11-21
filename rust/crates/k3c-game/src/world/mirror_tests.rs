//! RU55.3: Wann der Spiegel statt der Wellen gezeichnet wird, und seine Uniforms.

use k3c_data::look::with_profile;
use k3c_platform::{Frame as Batch, NullCanvas};

use super::*;
use crate::view::cell::{new_camera, rect};
use crate::view::frame::Frame;
use crate::view::settings::RIVER_TILES;

fn cell() -> Cell {
    let r = rect(0, 0, 1920, 1080);
    Cell {
        rect: r,
        camera: new_camera(r, 0.0, 33.0),
        ..Cell::default()
    }
}

#[test]
fn spiegel_nur_im_profil_k2c_mit_shader_fassung_und_faehigem_canvas() {
    super::super::register_look();
    let mut w = World::new(None);
    w.start = RiverMode::Shader;
    let f = Frame::default();
    let (mut ok, mut none) = (Batch::new(), NullCanvas::default());
    assert!(with_profile("k2c", || w.mirror_wanted(&f, &ok)));
    assert!(
        !with_profile("k2c", || w.mirror_wanted(&f, &none)),
        "Canvas ohne Spiegel"
    );
    assert!(
        !with_profile("go", || w.mirror_wanted(&f, &ok)),
        "Profil go zeichnet Wellen"
    );
    ok.set_mirror_support(false);
    assert!(
        !with_profile("k2c", || w.mirror_wanted(&f, &ok)),
        "Backend ohne Spiegel"
    );
    // Fassung tiles (F3 oder K3C_RIVER=tiles) bleibt als Rückfall
    let mut tiles = Frame::default();
    tiles.settings.river = RIVER_TILES.to_string();
    let ok = Batch::new();
    assert!(!with_profile("k2c", || w.mirror_wanted(&tiles, &ok)));
    let _ = &mut none;
}

#[test]
fn uniforms_folgen_dem_profil_und_dem_ufer() {
    super::super::register_look();
    let c = cell();
    let u = with_profile("k2c", || mirror_uniforms(&c, 0.0, 2.0));
    assert_eq!(u.emissive, 1.0, "auf 0 … 1 begrenzt");
    assert!((u.brightness - 0.62).abs() < 1e-6 && (u.squash - 0.85).abs() < 1e-6);
    assert_eq!(u.src_top, 0.0);
    assert!((u.ground - 626.4).abs() < 1.0, "Gehlinie bei 58 %: {}", u.ground);
    assert!(
        (u.axis - u.ground - 75.6).abs() < 1.0,
        "Ufer 7 %: {}",
        u.axis - u.ground
    );
    assert!(u.wobble > 0.0 && u.wobble < 5.0, "Schwanken in px: {}", u.wobble);
}

#[test]
fn zeichnet_in_das_rechteck_unter_dem_ufer() {
    super::super::register_look();
    let (w, c) = (World::new(None), cell());
    let mut b = Batch::new();
    b.begin();
    assert!(with_profile("k2c", || w.mirror_cell(&mut b, &c, 1.0, 0.0)));
    b.finish();
    let call = b.mirror_calls()[0];
    assert!((call.dst.y - 702.0).abs() < 1.0 && (call.dst.y + call.dst.h - 1080.0).abs() < 1e-3);
    assert_eq!((call.dst.x, call.dst.w), (0.0, 1920.0));
}
