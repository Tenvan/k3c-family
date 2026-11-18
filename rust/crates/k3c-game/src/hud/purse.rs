//! Port von `native/k3c-game/hud/purse.go`: Geldsack der Spielerkarte (B-919, B-922): ein Sack in drei Größen nach
//! `purse_size` (1 bis 3); die Fläche ist immer doppelt so groß wie ein Ressourcen-Icon, der Sack füllt sie je nach
//! Größe. Das Bild ist das Papierschnitt-Icon `icons/geldsack` (B-940), bodenbündig und waagerecht mittig in der Fläche;
//! ohne Grafik (Tests) bleibt nur der Blitz-Ring.

use k3c_platform::Canvas;

use super::drawstock::{ICON_SCALE, Icon, draw_icon};
use super::fonts::rgb;
use super::layout::Rect;
use super::paint::Painter;

/// PURSE_ICON_FACTOR: Kante des Geldsack-Icons als Vielfaches der Kante eines Ressourcen-Icons.
pub const PURSE_ICON_FACTOR: f64 = 2.0;

/// PURSE_FILL ist der Anteil der Fläche, den der Sack je Größe 1 bis 3 füllt.
pub const PURSE_FILL: [f64; 3] = [0.62, 0.8, 1.0];

/// PURSE_ASSET ist das Papierschnitt-Icon des Geldsacks.
pub const PURSE_ASSET: &str = "icons/geldsack";

/// purse_side ist die Kante der Geldsack-Fläche bei der Schriftgröße shared_px der Vorrats-Anzeige.
pub fn purse_side(shared_px: f64) -> f64 {
    PURSE_ICON_FACTOR * ICON_SCALE * shared_px
}

/// purse_scale ist der Füllanteil der Größe size (außerhalb 1 bis 3 begrenzt).
pub fn purse_scale(size: i64) -> f64 {
    PURSE_FILL[(size.clamp(1, PURSE_FILL.len() as i64) - 1) as usize]
}

/// draw_purse zeichnet den Sack der Größe size in die quadratische Fläche r; flash legt einen hellen Ring darum.
pub fn draw_purse(dst: &mut dyn Canvas, paint: &Painter, icon: Option<&Icon>, r: Rect, size: i64, flash: bool, k: f64) {
    let s = r.w * purse_scale(size);
    let (cx, base) = (r.x + r.w / 2.0, r.y + r.h);
    if let Some(ic) = icon {
        draw_icon(dst, ic, cx - s / 2.0, base - s, s);
    }
    if flash {
        paint.ring_at(dst, cx, base - s * 0.4, s * 0.36 + 4.0 * k, rgb(0xffffff, 1.0));
    }
}
