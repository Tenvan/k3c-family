//! Schrift für HUD und Menüs: eine freie OFL-Schrift aus festem Pfad (Datei kommt über die MCP-Tools `assets_*` ins
//! Repo). Fehlt die Datei, fällt die Schrift nicht still zurück, sondern meldet es mit Pfad: im strengen Modus als
//! Fehler, sonst mit der eingebauten Pixel-Schrift der Plattform und Warnung im Log. Vorlage: `native/k3c-game/art/font.go`.
//!
//! Abweichungen: Die Schrift wird bei der ersten Zeichnung auf der Zeichenfläche angemeldet (`Canvas::font`); messen
//! geht ohne Fenster über eine eigene `NullCanvas`. Faces je Größe gibt es nicht (die Plattform rundet die Größe und
//! cacht Glyphen selbst). Die Höhe einer Zeile ist die Schriftgröße (Ober- plus Unterlänge stellt `k3c-platform` nicht
//! bereit; Go nimmt die Metrik der Schrift).

use std::path::{Path, PathBuf};

use k3c_platform::{Canvas, FontId, NullCanvas};

use super::ArtError;

/// Debug-Schrift von ebitenutil: 6 × 16 Pixel je Zeichen (Zeilenabstand 16, `\n` bricht dort selbst um).
const DEBUG_CHAR_W: i64 = 6;
const DEBUG_CHAR_H: i64 = 16;
/// Zeichenhöhe der Pixel-Schrift im Rückfall (5×7-Raster in der 16er-Zeile).
const DEBUG_PX: f32 = 12.0;

/// Abstand zweier Grundlinien als Vielfaches der Schriftgröße (Text mit `\n`).
pub const LINE_SPACING: f64 = 1.25;

/// Font zeichnet Text in einer Schrift; Fallback = Pixel-Schrift (Größe wirkt dann nicht).
pub struct Font {
    pub path: PathBuf,
    pub fallback: bool,

    ttf: Vec<u8>,
    meter: NullCanvas,
    meter_id: Option<FontId>,
    id: Option<FontId>,
}

impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font")
            .field("path", &self.path)
            .field("fallback", &self.fallback)
            .finish_non_exhaustive()
    }
}

/// Lädt die Schrift aus path. Fehlt die Datei: strict → Fehler (`Missing`, mit Pfad), sonst Pixel-Schrift.
/// Eine vorhandene, aber kaputte Datei ist immer ein Fehler.
pub fn load_font(path: &Path, strict: bool) -> Result<Font, ArtError> {
    let mut f = Font {
        path: path.to_path_buf(),
        fallback: false,
        ttf: Vec::new(),
        meter: NullCanvas::default(),
        meter_id: None,
        id: None,
    };
    match std::fs::read(path) {
        Err(e) => {
            if strict {
                return Err(ArtError::Missing(format!("Schrift {} ({e})", path.display())));
            }
            let (p, err) = (path.display().to_string(), e.to_string());
            k3c_platform::log::warn(
                "📄 Schrift fehlt, Debug-Schrift als Ersatz",
                &[("pfad", &p), ("fehler", &err)],
            );
            f.fallback = true;
        }
        Ok(data) => {
            let id = f
                .meter
                .font(&data)
                .map_err(|e| ArtError::Broken(format!("{}: Schrift kaputt ({e})", path.display())))?;
            f.meter_id = Some(id);
            f.ttf = data;
        }
    }
    Ok(f)
}

impl Font {
    /// Liefert Breite (längste Zeile) und Höhe von s in Pixeln bei Größe size; `\n` beginnt eine neue Zeile im
    /// Abstand `LINE_SPACING` × size.
    pub fn measure(&self, s: &str, size: f64) -> (f64, f64) {
        let Some(id) = self.meter_id.filter(|_| !self.fallback) else {
            let (lines, widest) = debug_lines(s);
            return ((widest * DEBUG_CHAR_W) as f64, (lines * DEBUG_CHAR_H) as f64);
        };
        let mut w = 0.0f64;
        let mut lines = 0;
        for line in s.split('\n') {
            w = w.max(f64::from(self.meter.text_width(id, line, size as f32)));
            lines += 1;
        }
        (w, LINE_SPACING * size * f64::from(lines - 1) + size)
    }

    /// Zeichnet s mit der linken oberen Ecke bei (x, y); `\n` beginnt eine neue Zeile im Abstand `LINE_SPACING` ×
    /// size. clr ist vormultipliziert (wie `color.RGBA`).
    pub fn draw(&mut self, dst: &mut dyn Canvas, s: &str, x: f64, y: f64, size: f64, clr: [u8; 4]) {
        if self.fallback {
            // wie ebitenutil.DebugPrintAt: ganze Pixel, feste Größe, weiß
            let (x, y) = ((x as i64) as f32, (y as i64) as f32);
            for (i, line) in s.split('\n').enumerate() {
                dst.text(line, x, y + (i as i64 * DEBUG_CHAR_H) as f32, DEBUG_PX, [1.0; 4]);
            }
            return;
        }
        if self.id.is_none() {
            match dst.font(&self.ttf) {
                Ok(id) => self.id = Some(id),
                Err(e) => {
                    let p = self.path.display().to_string();
                    k3c_platform::log::error("❌ Schrift nicht anmeldbar", &[("pfad", &p), ("fehler", &e)]);
                    self.fallback = true;
                    return;
                }
            }
        }
        let Some(id) = self.id else { return };
        let color = clr.map(|c| f32::from(c) / 255.0);
        for (i, line) in s.split('\n').enumerate() {
            let ly = y + LINE_SPACING * size * i as f64;
            dst.text_font(id, line, x as f32, ly as f32, size as f32, color);
        }
    }
}

/// Zählt die Zeilen von s und die Zeichen der längsten (Debug-Schrift, ein Byte je Zeichen wie DebugPrint).
fn debug_lines(s: &str) -> (i64, i64) {
    let (mut n, mut lines, mut widest) = (0i64, 1i64, 0i64);
    for b in s.bytes() {
        if b == b'\n' {
            (lines, widest, n) = (lines + 1, widest.max(n), 0);
            continue;
        }
        n += 1;
    }
    (lines, widest.max(n))
}
