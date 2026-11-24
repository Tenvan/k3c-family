//! Port von `native/k3c-game/devtools/drawdebug.go`.
//!
//! Zeichnen des Debug-Overlays (F10): Formen aus `debuggeo` je Zelle, beschnitten auf deren Rechteck, und eine Zeile
//! mit der Kamera unten links. Auf der Seite Werte bleiben die Formen stehen, dazu kommt die Werte-Tafel.

use k3c_platform::{Canvas, Rect};

use super::debuggeo::{
    GEO_CAMERA, GEO_CHUNK, GEO_COIN, GEO_EDGE, GEO_ENEMY, GEO_KINDS, GEO_MONARCH, GEO_SITE, GEO_STAND, GEO_STAND_TREE,
    GEO_TROOP, geometry,
};
use super::debugstands::{HATCH_GAP, hatch};
use super::values::value_lines;
use super::{DevOverlay, Rgba, TEXT_COLOR, boxed, straight};
use crate::art::Font;
use crate::view::cell::{Cell, stats_rect};
use crate::view::devkeys::DEBUG_VALUES;
use crate::view::frame::Frame;

/// GEO_COLORS sind die Farben je Art (vormultipliziert wie in Go).
const GEO_COLORS: [Rgba; GEO_KINDS] = {
    let mut c = [[0u8; 4]; GEO_KINDS];
    c[GEO_MONARCH] = [250, 210, 60, 255];
    c[GEO_TROOP] = [90, 200, 250, 255];
    c[GEO_ENEMY] = [250, 80, 70, 255];
    c[GEO_COIN] = [250, 240, 140, 255];
    c[GEO_SITE] = [140, 250, 140, 255];
    c[GEO_CHUNK] = [200, 120, 250, 200];
    c[GEO_EDGE] = [250, 250, 250, 255];
    c[GEO_CAMERA] = [80, 160, 255, 160];
    c[GEO_STAND] = [70, 190, 90, 150];
    c[GEO_STAND_TREE] = [150, 250, 120, 255];
    c
};

impl DevOverlay {
    /// draw_debug zeichnet die Geometrie der Zelle `c`.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub(super) fn draw_debug(&mut self, dst: &mut dyn Canvas, font: Option<&mut Font>, f: &Frame, c: &Cell) {
        let Some(w) = f.world(c) else { return };
        self.shapes.clear();
        geometry(&mut self.shapes, w, c);
        dst.clip(Some(c.rect.to_rect()));
        for s in &self.shapes {
            let clr = straight(GEO_COLORS[s.kind.min(GEO_KINDS - 1)]);
            if s.x0 == s.x1 {
                dst.stroke_line(s.x0 as f32, s.y0 as f32, s.x1 as f32, s.y1 as f32, 1.0, clr);
                continue;
            }
            if s.kind == GEO_STAND {
                for l in hatch(s.x0, s.y0, s.x1, s.y1, HATCH_GAP) {
                    dst.stroke_line(l[0] as f32, l[1] as f32, l[2] as f32, l[3] as f32, 1.0, clr);
                }
            }
            let r = Rect::new(s.x0 as f32, s.y0 as f32, (s.x1 - s.x0) as f32, (s.y1 - s.y0) as f32);
            dst.stroke_rect(r, 1.5, clr);
        }
        if let Some(font) = font {
            let (l, r) = c.camera.span();
            let px = (c.rect.dy() as f64 / 40.0).max(12.0);
            let text = format!(
                "Kamera x {:.1} · {:.0} px/Unit · sichtbar {:.1}–{:.1} · Welt {:.0} Units",
                c.camera.x, c.camera.scale, l, r, w.width_units
            );
            let (tw, th) = font.measure(&text, px);
            let (ax, ay) = (
                c.rect.min.x as f64 + f64::from(px as i32),
                c.rect.max.y as f64 - f64::from((th + px) as i32),
            );
            boxed(dst, ax - px / 2.0, ay - px / 4.0, tw + px, th + px / 2.0);
            font.draw(dst, &text, ax, ay, px, TEXT_COLOR);
            if f.dev.debug == DEBUG_VALUES {
                self.draw_values(dst, font, f, c, px);
            }
        }
        dst.clip(None);
    }

    /// draw_values zeichnet die Werte-Tafel oben rechts in die Zelle `c` (unter der Anzeige oben rechts).
    #[allow(clippy::cast_precision_loss)]
    fn draw_values(&mut self, dst: &mut dyn Canvas, font: &mut Font, f: &Frame, c: &Cell, px: f64) {
        self.lines.clear();
        value_lines(&mut self.lines, f, c);
        let text = self.lines.join("\n");
        let (tw, th) = font.measure(&text, px);
        let x = c.rect.max.x as f64 - tw - 2.0 * px;
        let y = c.rect.min.y as f64 + stats_rect(c.rect).dy() as f64 + px;
        boxed(dst, x - px / 2.0, y - px / 2.0, tw + px, th + px);
        font.draw(dst, &text, x, y, px, TEXT_COLOR);
    }
}
