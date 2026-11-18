//! Port von `native/k3c-game/menu/hero.go`: das große Weltbild der Weltauswahl (B-938): Papier-Rahmen mit dem Titelbild
//! der Welt (Querformat), darüber auf einem dunklen Band der Weltname als Schrift, links und rechts die Blätter-Pfeile.
//! Fehlt das Bild, zeigt der Rahmen nur den Namen.

use k3c_platform::Canvas;

use super::draw::{Hero, Painter};
use super::layout::{COL_DIM, COL_TITLE, Geom, MAT_LINEN};

impl Geom {
    /// Der Rahmen des Weltbilds `(x, y, w, h)`: Breite der Karten, von deren Oberkante bis kurz über die Meldungszeile.
    pub fn hero_rect(&self) -> (f64, f64, f64, f64) {
        (
            self.row_x,
            self.row_top,
            self.row_w,
            self.note_y - self.row_top - 2.0 * self.row_gap,
        )
    }
}

impl Painter {
    /// Zeichnet das große Weltbild von `h`.
    pub fn hero(&mut self, dst: &mut dyn Canvas, g: &Geom, h: &Hero) {
        let (x, y, w, ht) = g.hero_rect();
        self.paper(dst, g, (x, y, w, ht), (MAT_LINEN, false), true);
        let inset = g.strip_inset();
        self.art_box(dst, &h.art, x + inset, y + inset, w - 2.0 * inset, ht - 2.0 * inset);
        let band = 1.9 * g.row_px;
        self.rect(dst, x + inset, y + ht - inset - band, w - 2.0 * inset, band, COL_DIM);
        let name_y = y + ht - inset - band + (band - 1.3 * g.row_px) / 2.0;
        self.centered(dst, &h.name, x + w / 2.0, name_y, 1.3 * g.row_px, COL_TITLE);
        let (_, ah) = self.measure(">", g.title_px);
        let ay = y + (ht - ah) / 2.0;
        if h.left {
            let lx = x - 2.0 * inset - self.char_w(g.title_px);
            self.text(dst, "<", lx, ay, g.title_px, COL_TITLE);
        }
        if h.right {
            self.text(dst, ">", x + w + 2.0 * inset, ay, g.title_px, COL_TITLE);
        }
    }
}
