//! Port von `native/k3c-game/world/cull.go`.
//!
//! Culling (B-795/AC-01, Vorbild src/scenes/render/cameraCull.ts): Gezeichnet wird nur, was in den Ausschnitt einer
//! Kamera ragt. Der Ausschnitt einer Ebene mit Scroll-Faktor factor ist um Kamera-x · factor verschoben (Parallaxe);
//! Boden-Ebene = Faktor 1. Reine Rechnung auf `view::Camera`, für alle Ebenen der Welt.

use super::camera::half_units;
use crate::view::cell::Camera;

/// CULL_MARGIN ist der Rand in Units links und rechts vom Ausschnitt (400 px im Browser): breite Objekte (Burg,
/// Gerüste) verschwinden nicht, solange sie noch hineinragen können.
pub const CULL_MARGIN: f64 = 400.0 / 32.0;

/// Span liefert den sichtbaren Bereich (Units) einer Ebene mit Scroll-Faktor factor: links, rechts. Ohne Maßstab ist
/// er leer (links > rechts).
pub fn span(k: &Camera, factor: f64) -> (f64, f64) {
    let h = half_units(k);
    if h <= 0.0 {
        return (1.0, 0.0);
    }
    let mid = k.x * factor;
    (mid - h, mid + h)
}

/// Visible meldet, ob ein Objekt auf der Boden-Ebene bei x mit Rand margin (Units) in den Ausschnitt ragt.
pub fn visible(k: &Camera, x: f64, margin: f64) -> bool {
    let (l, r) = span(k, 1.0);
    l <= r && x >= l - margin && x <= r + margin
}

/// CullMask liefert für ein Objekt bei x die Kameras (Bit i = cams[i]), in deren Ausschnitt samt Rand margin es nicht
/// liegt; je Kamera getrennt, ohne Kamera 0.
pub fn cull_mask(x: f64, cams: &[Camera], margin: f64) -> u64 {
    let mut mask: u64 = 0;
    for (i, k) in cams.iter().enumerate() {
        if !visible(k, x, margin) {
            mask |= 1 << i;
        }
    }
    mask
}

/// TileRange liefert die Kacheln (Index k, Kachel k liegt bei k·tile … (k+1)·tile) einer Ebene mit Scroll-Faktor
/// factor, die in den Ausschnitt ragen: erste und letzte; leer = first > last.
pub fn tile_range(k: &Camera, factor: f64, tile: f64) -> (i64, i64) {
    let (l, r) = span(k, factor);
    if l > r || tile <= 0.0 {
        return (0, -1);
    }
    ((l / tile).floor() as i64, (r / tile).floor() as i64)
}

#[cfg(test)]
#[path = "cull_tests.rs"]
mod cull_tests;
