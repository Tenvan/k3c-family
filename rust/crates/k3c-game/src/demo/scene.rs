//! Geladene Sheets auf der Zeichenfläche: Frame wählen, Sprite mit Fußpunkt und Spiegelung setzen.

use k3c_platform::{Canvas, Rect};

use crate::art::{Library, Sheet, SheetFrame};

/// Rahmen `[x, y, b, h]` in Einheiten aus Frame-Größe, Fußpunkt und Maßstab des Sheets (wie das Manifest).
pub fn rahmen(s: &Sheet) -> [f64; 4] {
    let (w, h) = (s.fw as f64 * s.units_per_px, s.fh as f64 * s.units_per_px);
    [-s.origin_x * w, -s.origin_y * h, w, h]
}

/// Frame der Animation `anim` von `asset` zur Zeit `ticks` (Takte) samt Rahmen; unbekannte Animation → `fallback`.
pub fn pick<'a>(
    lib: Option<&'a Library>,
    asset: &str,
    anim: &str,
    fallback: &str,
    ticks: f64,
) -> Option<(&'a SheetFrame, [f64; 4])> {
    let lib = lib?;
    let s = lib.sheet(asset, anim).or_else(|| lib.sheet(asset, fallback))?;
    let f = s.frame_at(ticks.max(0.0) / crate::demo::world::TICKS)?;
    Some((f, rahmen(s)))
}

/// Ziel-Rechteck eines Frames mit Rahmen `[x, y, b, h]` relativ zum Fußpunkt, skaliert und optional gespiegelt.
pub fn target(rahmen: [f64; 4], foot: (f32, f32), s: f32, mirror: bool) -> Rect {
    let [rx, ry, rw, rh] = rahmen.map(|v| v as f32 * s);
    let x = if mirror { foot.0 - rx - rw } else { foot.0 + rx };
    Rect::new(x, foot.1 + ry, rw, rh)
}

/// Zeichnet den Frame mit Fußpunkt `foot`; `s` verkleinert den Rahmen, `mirror` spiegelt um den Fußpunkt.
pub fn sprite_at(c: &mut dyn Canvas, f: &SheetFrame, rahmen: [f64; 4], foot: (f32, f32), s: f32, mirror: bool) {
    let Rect { x, y, w, h } = f.src;
    let src = if mirror { Rect::new(x + w, y, -w, h) } else { f.src };
    c.sprite(f.tex, src, target(rahmen, foot, s, mirror), [1.0; 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiegeln_dreht_um_den_fusspunkt() {
        let rahmen = [-60.0, -100.0, 200.0, 100.0];
        assert_eq!(
            target(rahmen, (500.0, 800.0), 0.5, false),
            Rect::new(470.0, 750.0, 100.0, 50.0)
        );
        assert_eq!(
            target(rahmen, (500.0, 800.0), 0.5, true),
            Rect::new(430.0, 750.0, 100.0, 50.0)
        );
    }

    #[test]
    fn rahmen_aus_sheet() {
        let mut s = Sheet::default();
        (s.fw, s.fh, s.origin_x, s.origin_y, s.units_per_px) = (4, 6, 0.5, 5.0 / 6.0, 1.0);
        assert_eq!(rahmen(&s), [-2.0, -5.0, 4.0, 6.0]);
    }
}
