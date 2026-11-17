//! Platzhalter nach Port-Leitfaden § 7: was `art` aus RU4.2 (`k3c-platform`) braucht, aber noch fehlt.
//! RU4.2 liefert `k3c_platform::Affine` und `Canvas::sprite_tf`; dann entfallen beide hier, `Sheet::draw` ruft
//! `canvas.sprite_tf` direkt.

use k3c_platform::{Canvas, Rect, TexId};

/// Matrix wie `ebiten.GeoM` (Bild-Pixel → Ziel); Felder wie im RU4-Plan (`Affine { a, b, c, d, tx, ty }`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

/// Zeichnet `src` aus Textur `t` mit Matrix `tf`; wird `Canvas::sprite_tf` aus RU4.2.
pub fn sprite_tf(_c: &mut dyn Canvas, _t: TexId, _src: Rect, _tf: Affine, _tint: [f32; 4]) {
    unimplemented!("RU4.2: sprite_tf")
}
