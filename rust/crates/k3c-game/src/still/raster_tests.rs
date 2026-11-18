//! Tests des Rasterers: Rechteck, Sprite (nächster Nachbar und linear), Netz, Ausschnitt, Aufrufreihenfolge.

use k3c_platform::{Blend, Canvas, Filter, Rect, Vertex};

use super::raster::SoftCanvas;

fn px(img: &super::Image, x: u32, y: u32) -> [u8; 4] {
    let i = ((y * img.width + x) * 4) as usize;
    [img.pix[i], img.pix[i + 1], img.pix[i + 2], img.pix[i + 3]]
}

#[test]
fn rechteck_deckt_pixel_mit_mitte_darin() {
    let mut c = SoftCanvas::new(8, 8);
    c.rect(Rect::new(2.0, 2.0, 3.0, 2.0), [1.0, 0.0, 0.0, 1.0]);
    let img = c.finish();
    assert_eq!(px(&img, 2, 2), [255, 0, 0, 255]);
    assert_eq!(px(&img, 4, 3), [255, 0, 0, 255]);
    assert_eq!(px(&img, 5, 3), [0, 0, 0, 255]);
    assert_eq!(px(&img, 2, 4), [0, 0, 0, 255]);
}

#[test]
fn halbe_deckkraft_mischt_ueber_schwarz() {
    let mut c = SoftCanvas::new(2, 2);
    c.rect(Rect::new(0.0, 0.0, 2.0, 2.0), [1.0, 1.0, 1.0, 0.5]);
    assert_eq!(px(&c.finish(), 0, 0), [128, 128, 128, 255]);
}

#[test]
fn sprite_naechster_nachbar_und_reihenfolge() {
    let mut c = SoftCanvas::new(4, 2);
    // 2×1: rot, grün (vormultipliziert, deckend)
    let t = c.texture_with(&[255, 0, 0, 255, 0, 255, 0, 255], 2, 1, Filter::Nearest);
    c.sprite(
        t,
        Rect::new(0.0, 0.0, 2.0, 1.0),
        Rect::new(0.0, 0.0, 4.0, 2.0),
        [1.0; 4],
    );
    // später gezeichnet liegt obenauf, auch wenn es Vollfarbe ist
    c.rect(Rect::new(3.0, 0.0, 1.0, 1.0), [0.0, 0.0, 1.0, 1.0]);
    let img = c.finish();
    assert_eq!(px(&img, 0, 0), [255, 0, 0, 255]);
    assert_eq!(px(&img, 1, 1), [255, 0, 0, 255]);
    assert_eq!(px(&img, 2, 1), [0, 255, 0, 255]);
    assert_eq!(px(&img, 3, 0), [0, 0, 255, 255]);
}

#[test]
fn sprite_linear_mischt_zwischen_texeln() {
    let mut c = SoftCanvas::new(4, 1);
    let t = c.texture_with(&[0, 0, 0, 255, 255, 255, 255, 255], 2, 1, Filter::Linear);
    c.sprite(
        t,
        Rect::new(0.0, 0.0, 2.0, 1.0),
        Rect::new(0.0, 0.0, 4.0, 1.0),
        [1.0; 4],
    );
    let img = c.finish();
    // Pixelmitte 1.5 → Textur-x 0.75 → zwischen Texel 0 (Mitte 0.5) und 1 (Mitte 1.5): 25 % Weiß
    assert_eq!(px(&img, 1, 0)[0], 64);
    assert_eq!(px(&img, 0, 0)[0], 0, "links der ersten Texelmitte: Rand klemmt");
}

#[test]
fn netz_und_ausschnitt() {
    let mut c = SoftCanvas::new(8, 8);
    let t = c.texture_with(&[255; 4], 1, 1, Filter::Nearest);
    let v = |x: f32, y: f32| Vertex {
        pos: [x, y],
        uv: [0.5, 0.5],
        color: [0.0, 1.0, 0.0, 1.0],
    };
    c.clip(Some(Rect::new(0.0, 0.0, 4.0, 8.0)));
    c.mesh(t, &[v(0.0, 0.0), v(8.0, 0.0), v(0.0, 8.0)], &[0, 1, 2]);
    let img = c.finish();
    assert_eq!(px(&img, 1, 1), [0, 255, 0, 255]);
    assert_eq!(px(&img, 6, 0), [0, 0, 0, 255], "rechts des Ausschnitts");
    assert_eq!(px(&img, 1, 7), [0, 0, 0, 255], "außerhalb des Dreiecks");
}

#[test]
fn formen_zeichnen() {
    let mut c = SoftCanvas::new(16, 16);
    c.fill_circle(8.0, 8.0, 5.0, [1.0, 1.0, 0.0, 1.0]);
    let img = c.finish();
    assert_eq!(px(&img, 8, 8), [255, 255, 0, 255]);
    assert_eq!(px(&img, 0, 0), [0, 0, 0, 255]);
}

/// B-1056: Multiply rechnet Ziel × Quelle je Farbkanal, die Deckkraft des Ziels bleibt; danach gilt wieder Over.
#[test]
fn multiply_blend_rechnet_ziel_mal_quelle() {
    let mut c = SoftCanvas::new(4, 2);
    c.rect(Rect::new(0.0, 0.0, 4.0, 2.0), [1.0, 0.8, 0.4, 1.0]);
    let t = c.texture(&[255; 4], 1, 1);
    let v = |x: f32, y: f32, col: [f32; 4]| Vertex {
        pos: [x, y],
        uv: [0.5, 0.5],
        color: col,
    };
    let half = [0.5, 0.5, 0.5, 1.0];
    c.blend(Blend::Multiply);
    c.mesh(
        t,
        &[
            v(0.0, 0.0, half),
            v(2.0, 0.0, half),
            v(2.0, 2.0, half),
            v(0.0, 2.0, half),
        ],
        &[0, 1, 2, 0, 2, 3],
    );
    c.blend(Blend::Over);
    c.rect(Rect::new(3.0, 0.0, 1.0, 1.0), [0.0, 0.0, 1.0, 1.0]);
    let img = c.finish();
    // Bildpunkte auf der gemeinsamen Diagonale beider Dreiecke zählt der Rasterer doppelt, darum (1, 0) und (0, 1)
    assert_eq!(px(&img, 1, 0), [128, 102, 51, 255], "Ziel (255, 204, 102) mal 0,5");
    assert_eq!(px(&img, 0, 1), [128, 102, 51, 255]);
    assert_eq!(px(&img, 2, 1), [255, 204, 102, 255], "außerhalb des Netzes unverändert");
    assert_eq!(px(&img, 3, 0), [0, 0, 255, 255], "danach wieder Over");
}
