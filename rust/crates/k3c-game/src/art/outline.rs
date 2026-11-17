//! Text mit schwarzer Kontur; Vorlage `native/k3c-game/art/outline.go`.
//!
//! Abweichung (RU7-README › Vorentscheidungen): Rust hat kein Offscreen-Ziel, darum gibt es kein Zwischenbild. Die
//! Kontur wird direkt mit `text_font` an den Versätzen `OUTLINE4`/`OUTLINE8` gezeichnet (wie Gos Pfad bei vollem
//! Zwischenspeicher), die Lage wie im Zwischenbild-Pfad auf ganze Bildpunkte gerundet. `OutlineCache` bleibt als
//! Hülle mit derselben API; `sweep` und `len` haben nichts mehr zu tun.

use k3c_platform::Canvas;

use super::Font;

/// Konturen: Versätze in Vielfachen der Konturstärke (Browser: stroke 6 bei 28 px).
pub const OUTLINE4: [[f64; 2]; 4] = [[-1.0, 0.0], [1.0, 0.0], [0.0, -1.0], [0.0, 1.0]];
pub const OUTLINE8: [[f64; 2]; 8] = [
    [-1.0, 0.0],
    [1.0, 0.0],
    [0.0, -1.0],
    [0.0, 1.0],
    [-1.0, -1.0],
    [1.0, -1.0],
    [-1.0, 1.0],
    [1.0, 1.0],
];

const OUTLINE_BLACK: [u8; 4] = [0, 0, 0, 255];

/// Zwischenspeicher eines Pakets; `Default` ist bereit. Ohne Offscreen-Bilder bleibt er leer.
#[derive(Debug, Default)]
pub struct OutlineCache {
    gen_: u64,
}

/// Konturstärke bei Größe size (mindestens 1 px).
pub fn outline_stroke(size: f64) -> f64 {
    (size / 14.0).max(1.0)
}

/// Rand des Zwischenbilds um die gemessene Fläche: Kontur und Überhang der Glyphen (bleibt für Aufrufer, die
/// Flächen danach bemessen).
pub fn outline_pad(size: f64) -> f64 {
    outline_stroke(size).ceil() + (size * 0.3).ceil()
}

impl OutlineCache {
    /// Zeichnet s mit linker oberer Ecke bei (x, y), Farbe clr (vormultipliziert), schwarzer Kontur ring
    /// (`OUTLINE4`/`OUTLINE8`) und Deckkraft alpha (0 … 1). Pixel-Schrift: ohne Kontur.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go `Draw`
    pub fn draw(
        &mut self,
        dst: &mut dyn Canvas,
        f: Option<&mut Font>,
        s: &str,
        x: f64,
        y: f64,
        size: f64,
        clr: [u8; 4],
        ring: &[[f64; 2]],
        alpha: f64,
    ) {
        let Some(f) = f else { return };
        if s.is_empty() || alpha <= 0.0 {
            return;
        }
        if f.fallback {
            f.draw(dst, s, x, y, size, clr);
            return;
        }
        let (x, y) = (x.round(), y.round());
        let d = outline_stroke(size);
        let black = scale_alpha(OUTLINE_BLACK, alpha);
        for o in ring {
            f.draw(dst, s, x + o[0] * d, y + o[1] * d, size, black);
        }
        f.draw(dst, s, x, y, size, scale_alpha(clr, alpha));
    }

    /// Zählt einen Durchlauf weiter (einmal je Takt); ohne Zwischenbilder gibt es nichts freizugeben.
    pub fn sweep(&mut self) {
        self.gen_ += 1;
    }

    /// Zahl der Zwischenbilder (immer 0).
    pub fn len(&self) -> usize {
        0
    }

    /// Wahr, solange keine Zwischenbilder bestehen (immer).
    pub fn is_empty(&self) -> bool {
        true
    }
}

/// Skaliert eine vormultiplizierte Farbe mit a.
pub fn scale_alpha(c: [u8; 4], a: f64) -> [u8; 4] {
    if a >= 1.0 {
        return c;
    }
    c.map(|v| (f64::from(v) * a) as u8)
}
