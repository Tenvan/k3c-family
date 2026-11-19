//! Spiegel im Software-Rasterer (RU55.3): rechnet `mirror_fragment` über das Bild bis zum Aufruf.

use k3c_platform::{MirrorCall, mirror_fragment};

use super::raster::{Image, blend};

/// Führt den Spiegel-Aufruf aus (fehlt er, geschieht nichts).
pub(super) fn run(img: &mut Image, clip: (i64, i64, i64, i64), call: Option<&MirrorCall>) {
    if let Some(c) = call {
        draw_mirror(img, clip, c);
    }
}

/// Spiegel-Aufruf (RU55.3): liest das Bild bis hierher (Streifen über der Gehlinie, nächster Texel) und schreibt die
/// Spiegelung unter die Achse; Lese- und Schreibbereich überlappen nicht (Quelle bis `ground`, Ziel ab `axis`).
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn draw_mirror(img: &mut Image, clip: (i64, i64, i64, i64), c: &MirrorCall) {
    let r = c.dst;
    let x0 = clip.0.max((r.x - 0.5).ceil() as i64);
    let x1 = clip.2.min((r.x + r.w - 0.5).ceil() as i64);
    let y0 = clip.1.max((r.y - 0.5).ceil() as i64);
    let y1 = clip.3.min((r.y + r.h - 0.5).ceil() as i64);
    let rect = [r.x, r.y, r.w, r.h];
    for y in y0..y1 {
        for x in x0..x1 {
            let s = {
                let src = |sx: f32, sy: f32| -> [f32; 3] {
                    let (ix, iy) = (
                        (sx as i64).clamp(0, i64::from(img.width) - 1),
                        (sy as i64).clamp(0, i64::from(img.height) - 1),
                    );
                    let i = (iy as usize * img.width as usize + ix as usize) * 4;
                    [0, 1, 2].map(|k| f32::from(img.pix[i + k]) / 255.0)
                };
                mirror_fragment(&c.u, rect, [x as f32 + 0.5, y as f32 + 0.5], &src)
            };
            blend(img, x, y, s);
        }
    }
}
