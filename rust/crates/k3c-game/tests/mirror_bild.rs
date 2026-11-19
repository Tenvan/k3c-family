//! RU55.3 (B-1044), AC-03: Der Fluss spiegelt die Welt über der Gehlinie (Software-Rasterer, kein Fenster, keine
//! Grafik nötig): Lage unter dem Ufer, Helligkeit 0,6 … 0,7 des Originals, Lichtquellen bleiben nachts hell; Profil
//! `go` und ein Canvas ohne Spiegel zeichnen die Wellen.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use k3c_data::look::with_profile;
use k3c_game::still::raster::SoftCanvas;
use k3c_game::view::cell::{Cell, new_camera, rect};
use k3c_game::world::{World, mirror::mirror_uniforms, register_look};
use k3c_platform::{Canvas, NullCanvas, Rect};

const SKY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

/// Zelle 400 × 300 im Profil k2c: Gehlinie bei 174, Ufer 21 px, Spiegel ab 195.
fn cell() -> Cell {
    register_look();
    let r = rect(0, 0, 400, 300);
    Cell {
        rect: r,
        camera: new_camera(r, 0.0, 20.0),
        ..Cell::default()
    }
}

/// Zeichnet Himmel und ein Objekt der Farbe `obj` über der Gehlinie, dann den Spiegel; liefert Pixel (RGB 0 … 1).
fn render(obj: [f32; 4], emissive: f64) -> impl Fn(usize, usize) -> [f32; 3] {
    let img = with_profile("k2c", || {
        let (w, c) = (World::new(None), cell());
        let mut cv = SoftCanvas::new(400, 300);
        cv.rect(Rect::new(0.0, 0.0, 400.0, 174.0), SKY);
        cv.rect(Rect::new(100.0, 124.0, 50.0, 50.0), obj);
        assert!(w.mirror_cell(&mut cv, &c, 0.0, emissive), "SoftCanvas kann den Spiegel");
        cv.finish()
    });
    move |x, y| {
        let i = (y * 400 + x) * 4;
        [0, 1, 2].map(|k| f32::from(img.pix[i + k]) / 255.0)
    }
}

#[test]
fn objekt_erscheint_unter_dem_ufer_dunkler_und_gespiegelt() {
    let px = render([1.0, 0.1, 0.1, 1.0], 0.0);
    // Unter dem Objekt (x 100 … 150) direkt unter der Achse: rot, mit 0,6 … 0,7 der Helligkeit, nicht mehr hell
    let c = px(125, 200);
    assert!(c[0] > 0.45 && c[0] < 0.75, "Rot der Spiegelung {c:?}");
    assert!(c[0] > c[1] + 0.2, "bleibt rötlich: {c:?}");
    // daneben nur Himmel gespiegelt: grau-bläulich, dunkler als der Himmel (0,5)
    let side = px(300, 200);
    assert!(side[0] < 0.5 && side[2] > side[0], "Himmel gespiegelt {side:?}");
    // Die Spiegelung reicht nur so weit, wie das Objekt hoch ist (50 px · 0,85 = 42 px unter der Achse), darunter Himmel/Wasser
    let low = px(125, 250);
    assert!(low[0] < c[0] - 0.1, "Objekt endet: {low:?} gegen {c:?}");
    // Das Ufer (174 … 195) liegt nicht im Spiegel: dort bleibt das schwarze Bild
    assert_eq!(px(125, 180), [0.0, 0.0, 0.0]);
}

#[test]
fn lichtquelle_bleibt_nachts_hell() {
    let lamp = [1.0, 0.85, 0.4, 1.0];
    let (day, night) = (render(lamp, 0.0)(125, 200), render(lamp, 1.0)(125, 200));
    assert!(day[0] < 0.75, "am Tag gedämpft: {day:?}");
    assert!(night[0] > 0.9 && night[1] > 0.7, "nachts hell: {night:?}");
}

#[test]
fn profil_go_und_canvas_ohne_spiegel_haben_keinen_spiegel() {
    // Ein Canvas ohne Spiegel meldet false: der Aufrufer zeichnet die Wellen.
    let (w, c) = (World::new(None), cell());
    assert!(!with_profile("k2c", || w.mirror_cell(
        &mut NullCanvas::default(),
        &c,
        0.0,
        0.0
    )));
    assert!(!NullCanvas::default().can_mirror());
    // Profil go: Spiegel nicht vorgesehen
    assert!(!with_profile("go", || k3c_data::look::current().mirror.enabled));
    assert!(with_profile("k2c", || k3c_data::look::current().mirror.enabled));
    let u = with_profile("k2c", || mirror_uniforms(&c, 0.0, 0.0));
    assert!((u.brightness - 0.65).abs() < 1e-6 && u.axis > u.ground);
}
