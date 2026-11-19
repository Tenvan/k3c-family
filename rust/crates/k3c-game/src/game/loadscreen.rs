//! Port von `native/k3c-game/game/loadscreen.go`: das Ladebild im Papierschnitt-Stil (B-949): Papier-Hintergrund mit zwei
//! Hügel-Streifen, der Krieger aus den Menü-Illustrationen, Titel, Fortschrittsbalken und Prozent. Ohne Schrift oder ohne
//! Krieger zeichnet `draw_loading` die schlichte Meldung (`notice.rs`). Nach dem ersten Bild entsteht nichts mehr, nur der
//! Prozent-Text je Prozentschritt.

use k3c_platform::{Affine, Canvas, Rect};

use crate::art::Library;

/// Die Papierfigur im Ladebild (Menü-Illustration, Animation idle).
pub const LOAD_FIGURE: &str = "truppen/krieger";

/// Titel über dem Ladebild.
pub const LOAD_TITLE: &str = "Family Three Crowns";

const fn rgba(r: u8, g: u8, b: u8, a: u8) -> [u8; 4] {
    [r, g, b, a]
}

const PAPER: [u8; 4] = rgba(238, 226, 198, 255);
const HILL_A: [u8; 4] = rgba(136, 160, 112, 255);
const HILL_B: [u8; 4] = rgba(88, 121, 84, 255);
const SHADE: [u8; 4] = rgba(0, 0, 0, 56);
const INK: [u8; 4] = rgba(74, 56, 36, 255);
const TRACK: [u8; 4] = rgba(214, 196, 160, 255);
const GOLD: [u8; 4] = rgba(226, 176, 52, 255);
const LIGHTS: [u8; 4] = rgba(255, 255, 255, 90);

/// Farbe für die Zeichenfläche (gerade, nicht vormultipliziert).
fn col(c: [u8; 4]) -> [f32; 4] {
    c.map(|v| f32::from(v) / 255.0)
}

/// Rundet den Anteil `p` auf ganze Prozent (0 … 100).
pub fn percent(p: f64) -> i64 {
    #[allow(clippy::cast_possible_truncation)]
    {
        (p.clamp(0.0, 1.0) * 100.0).round() as i64
    }
}

/// Hält, was das Ladebild über die Bilder merkt.
#[derive(Debug, Default)]
pub struct LoadScreen {
    tick: i64,
    pct: i64,
    /// „NN %“, neu bei jedem Prozentschritt; leer = noch keins
    pub label: String,
}

impl LoadScreen {
    /// Zeichnet das Ladebild mit Fortschritt `p` (0 … 1); `false` = nichts gezeichnet (keine Schrift oder keine Figur),
    /// der Aufrufer nimmt die Meldung.
    pub fn draw(&mut self, c: &mut dyn Canvas, lib: &mut Library, p: f64) -> bool {
        if lib.font.fallback {
            return false;
        }
        let Some((iw, ih)) = lib
            .sheet(LOAD_FIGURE, "idle")
            .and_then(|s| s.frame(0))
            .map(|f| (f.src.w, f.src.h))
        else {
            return false;
        };
        let n = percent(p);
        if self.label.is_empty() || self.pct != n {
            self.pct = n;
            self.label = format!("{n} %");
        }
        self.tick += 1;
        let (w, h) = c.size();
        c.rect(Rect::new(0.0, 0.0, w, h), col(PAPER));
        // Hügel als Papierstreifen mit Schatten, hinten hell, vorn dunkel
        c.rect(Rect::new(0.0, h * 0.66, w, h * 0.34), col(HILL_A));
        c.rect(Rect::new(0.0, h * 0.66, w, h * 0.012), col(LIGHTS));
        c.rect(Rect::new(0.0, h * 0.80 - h * 0.012, w, h * 0.012), col(SHADE));
        c.rect(Rect::new(0.0, h * 0.80, w, h * 0.20), col(HILL_B));
        // Figur: Fuß auf dem hinteren Hügel, wippt leicht
        let k = h * 0.34 / ih;
        let bob = (self.tick as f32 / 9.0).sin() * h * 0.008;
        let mut g = Affine::IDENTITY;
        g.scale(k, k);
        g.translate((w - iw * k) / 2.0, h * 0.72 - ih * k + bob);
        if let Some(sh) = lib.sheet(LOAD_FIGURE, "idle") {
            sh.draw(c, 0, g);
        }
        self.draw_bar(c, lib, (w, h));
        true
    }

    /// Titel, Balken und Prozent-Text.
    fn draw_bar(&self, c: &mut dyn Canvas, lib: &mut Library, (w, h): (f32, f32)) {
        let size = f64::from(h) * 0.07;
        let (tw, _) = lib.font.measure(LOAD_TITLE, size);
        let (tx, ty) = ((f64::from(w) - tw) / 2.0, f64::from(h) * 0.07);
        lib.font.draw(c, LOAD_TITLE, tx + 3.0, ty + 3.0, size, SHADE);
        lib.font.draw(c, LOAD_TITLE, tx, ty, size, INK);
        let (bw, bh) = (w * 0.5, h * 0.04);
        let (bx, by) = ((w - bw) / 2.0, h * 0.86);
        c.rect(Rect::new(bx + 4.0, by + 4.0, bw, bh), col(SHADE));
        c.rect(Rect::new(bx - 4.0, by - 4.0, bw + 8.0, bh + 8.0), col(INK));
        c.rect(Rect::new(bx, by, bw, bh), col(TRACK));
        c.rect(Rect::new(bx, by, bw * self.pct as f32 / 100.0, bh), col(GOLD));
        let ps = f64::from(h) * 0.035;
        let (lw, _) = lib.font.measure(&self.label, ps);
        let lx = f64::from(bx) + (f64::from(bw) - lw) / 2.0;
        lib.font
            .draw(c, &self.label, lx, f64::from(by + bh) + f64::from(h) * 0.015, ps, PAPER);
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadScreen, percent};
    use crate::art::{Config, Library};
    use k3c_platform::NullCanvas;

    /// B-949/AC-01 (RU61.1): Ohne Schrift (Ersatz-Schrift) und ohne Krieger zeichnet das Ladebild nichts, meldet
    /// `false` (der Aufrufer nimmt die Textmeldung) und zählt keinen Takt.
    #[test]
    fn draw_ohne_schrift_und_figur_zeichnet_nichts() {
        let mut lib = Library::new(Config {
            font_path: std::path::PathBuf::from("gibt-es-nicht.ttf"),
            strict_font: false,
            ..Config::default()
        })
        .unwrap_or_else(|e| unreachable!("{e}"));
        assert!(lib.font.fallback, "Ersatz-Schrift erwartet");
        let mut ls = LoadScreen::default();
        assert!(!ls.draw(&mut NullCanvas::default(), &mut lib, 0.5), "Schrift fehlt");
        assert!(ls.label.is_empty(), "kein Prozent-Text ohne Bild");
        lib.font.fallback = false;
        assert!(!ls.draw(&mut NullCanvas::default(), &mut lib, 0.5), "Krieger fehlt");
        assert!(ls.label.is_empty(), "kein Prozent-Text ohne Krieger");
    }

    /// Go: `TestLoadPercent` (`loading_test.go`).
    #[test]
    fn percent_rundet_und_begrenzt() {
        for (p, want) in [(-1.0, 0), (0.0, 0), (0.334, 33), (0.5, 50), (1.0, 100), (3.0, 100)] {
            assert_eq!(percent(p), want, "percent({p})");
        }
    }
}
