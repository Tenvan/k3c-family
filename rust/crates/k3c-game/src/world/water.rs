//! Port von `native/k3c-game/world/water.go`.
//!
//! Fluss mit Kage-Shader (B-790, NG11.1): Lage, Kachelbreite und Fließen der Reihen rechnet `river_layout` wie für die
//! Kacheln; der Shader bekommt sie mit den Phasen der Animation als Uniforms, die Phasen folgen nur aus Frame.Clock.
//! Die Fassung wählt die Umgebungsvariable K3C_RIVER (tiles oder shader, Standard shader) als Startwert; im Spiel
//! schaltet F3 um (view.Settings.River, B-816).
//!
//! Abweichung: Es gibt kein `embed`, kein Übersetzen und kein `DrawRectShader`. `river.kage` steht als WGSL im
//! GPU-Backend der Plattform (B-1072) und als Referenz in Rust (`k3c_platform::river_fragment`, B-1071); hier ruft
//! `RiverWater::draw` den Canvas-Aufruf `river` (B-1070). Meldet der Canvas `false` (headless, ohne Shader), zeichnet
//! `draw_river` die Kacheln wie in Go.

use std::f64::consts::PI;

use super::backdrop::Rgba;
use super::river::{RIVER_BASE, RIVER_ROWS, RIVER_WAVE, RiverRow, river_layout};
use crate::view::cell::Cell;
use crate::view::settings::{RIVER_SHADER, RIVER_TILES};
use k3c_platform::{Canvas, Rect, RiverUniforms};

/// RIVER_ENV wählt die Fassung des Flusses (tiles oder shader).
pub const RIVER_ENV: &str = "K3C_RIVER";

// Tempo der Shader-Animation in Umläufen je Sekunde: Verzerrung, Heben des Bands, Glanz.
pub const WATER_WOBBLE_HZ: f64 = 0.35;
pub const WATER_LIFT_HZ: f64 = 0.5;
pub const WATER_GLINT_HZ: f64 = 0.8;

pub const RIVER_SHADE: Rgba = [50, 104, 150, 255];
pub const RIVER_GLINT: Rgba = [150, 196, 226, 255];

/// RiverMode ist die Fassung des Flusses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RiverMode {
    Shader,
    Tiles,
}

impl RiverMode {
    /// String ist der Name der Fassung (Go: `riverMode.String`).
    pub fn as_str(self) -> &'static str {
        if self == RiverMode::Tiles { "tiles" } else { "shader" }
    }
}

/// RiverModeOf liest den Wert von K3C_RIVER; alles außer "tiles" ist die Shader-Fassung.
pub fn river_mode_of(v: &str) -> RiverMode {
    if v.trim().eq_ignore_ascii_case("tiles") {
        RiverMode::Tiles
    } else {
        RiverMode::Shader
    }
}

/// RiverStart ist der Startwert der Fluss-Fassung aus K3C_RIVER als `RIVER_SHADER` oder `RIVER_TILES` (game schaltet
/// mit `Settings::next_river` von diesem Wert aus um, solange keine Fassung gewählt ist).
pub fn river_start() -> &'static str {
    river_mode_of(&std::env::var(RIVER_ENV).unwrap_or_default()).as_str()
}

/// RiverModeFor ist die Fassung nach den Einstellungen: "" (nichts gewählt) gilt der Startwert start.
pub fn river_mode_for(setting: &str, start: RiverMode) -> RiverMode {
    match setting {
        RIVER_TILES => RiverMode::Tiles,
        RIVER_SHADER => RiverMode::Shader,
        _ => start,
    }
}

/// WaterParams sind die Uniforms einer Zelle zur Zeit t: Rechteck (Fenster-px), Reihen und Phasen (0 … 2π).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WaterParams {
    pub x: i64,
    pub top: i64,
    pub w: i64,
    pub h: i64,
    pub rows: [RiverRow; RIVER_ROWS],
    pub phase: [f64; 3],
}

/// PhaseAt ist die Phase einer Schwingung mit hz Umläufen je Sekunde zur Zeit t, auf 0 … 2π gefaltet (float32 im
/// Shader bleibt so auch nach Stunden genau).
pub fn phase_at(t: f64, hz: f64) -> f64 {
    let mut p = (t * hz) % 1.0;
    if p < 0.0 {
        p += 1.0;
    }
    p * 2.0 * PI
}

/// WaterLayout rechnet die Uniforms der Zelle zur Zeit t (Sekunden aus Frame.Clock); reine Rechnung.
#[allow(clippy::cast_possible_truncation)]
pub fn water_layout(c: &Cell, t: f64) -> WaterParams {
    let rows = river_layout(c, t);
    let mut p = WaterParams {
        rows,
        x: c.rect.min.x,
        w: c.rect.dx(),
        ..WaterParams::default()
    };
    p.top = rows[0].y.floor() as i64;
    p.h = c.rect.max.y - p.top;
    p.phase = [
        phase_at(t, WATER_WOBBLE_HZ),
        phase_at(t, WATER_LIFT_HZ),
        phase_at(t, WATER_GLINT_HZ),
    ];
    p
}

/// RiverWater hält die Uniform-Puffer und die vier Farben; je Zelle werden nur die Werte der Puffer überschrieben.
/// Den Shader selbst besitzt die Plattform: `ready` ist wahr, sobald die Fassung gewählt ist, ob er zeichnet, sagt
/// erst `Canvas::river`.
#[derive(Clone, Debug, Default)]
pub struct RiverWater {
    /// Vorbereitet (einmal)
    pub tried: bool,
    pub origin: [f32; 2],
    pub rows: [f32; 4 * RIVER_ROWS],
    pub phase: [f32; 3],
    /// Grund und Wellen-Band wie die Kacheln, Papier-Schatten und Glanz als Töne derselben Farbe
    pub colors: [[f32; 3]; 4],
}

impl RiverWater {
    /// Init merkt die gewählte Fassung und meldet sie im Log (aktiv ist, was gewählt wurde; fällt der Canvas später
    /// auf Kacheln zurück, steht das nicht im Log).
    pub fn init(&mut self, mode: RiverMode) {
        if mode == RiverMode::Shader {
            self.build();
        }
        k3c_platform::log::info(
            "🚦 Fluss-Fassung",
            &[("gewählt", &mode.as_str()), ("aktiv", &mode.as_str())],
        );
    }

    /// Build legt die Farben an (einmal).
    pub fn build(&mut self) {
        self.tried = true;
        self.colors = [RIVER_BASE, RIVER_WAVE, RIVER_SHADE, RIVER_GLINT].map(rgb);
    }

    /// Ready meldet, ob die Shader-Fassung zur Verfügung steht; bereitet beim ersten Aufruf vor, falls noch nicht
    /// geschehen. Der Rückfall auf Kacheln entscheidet das Ergebnis von `Canvas::river`.
    pub fn ready(&mut self) -> bool {
        if !self.tried {
            self.build();
        }
        true
    }

    /// Set schreibt die Parameter p in die Uniform-Puffer.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn set(&mut self, p: &WaterParams) {
        self.origin = [p.x as f32, p.top as f32];
        for (i, row) in p.rows.iter().enumerate() {
            self.rows[4 * i] = row.y as f32;
            self.rows[4 * i + 1] = row.h as f32;
            self.rows[4 * i + 2] = row.tile as f32;
            self.rows[4 * i + 3] = row.x0 as f32;
        }
        for (i, v) in p.phase.iter().enumerate() {
            self.phase[i] = *v as f32;
        }
    }

    /// Uniforms ist der Inhalt der Puffer als Aufruf-Parameter des Canvas.
    pub fn uniforms(&self) -> RiverUniforms {
        let [base, wave, shade, glint] = self.colors;
        RiverUniforms {
            origin: self.origin,
            rows: self.rows,
            phase: self.phase,
            base,
            wave,
            shade,
            glint,
        }
    }

    /// Draw zeichnet den Fluss der Zelle zur Zeit t mit einem Canvas-Aufruf; `false`, wenn der Canvas den Shader nicht
    /// kennt (der Aufrufer zeichnet dann die Kacheln). Eine Zelle ohne sichtbare Fläche zeichnet nichts (`true`).
    #[allow(clippy::cast_precision_loss)]
    pub fn draw(&mut self, dst: &mut dyn Canvas, c: &Cell, t: f64) -> bool {
        let p = water_layout(c, t);
        if p.w <= 0 || p.h <= 0 || p.rows[0].tile <= 0.0 {
            return true;
        }
        self.set(&p);
        let r = Rect::new(p.x as f32, p.top as f32, p.w as f32, p.h as f32);
        dst.river(r, &self.uniforms())
    }
}

/// Rgb ist die Farbe als Uniform (Kanäle 0 … 1).
pub fn rgb(c: Rgba) -> [f32; 3] {
    [
        f32::from(c[0]) / 255.0,
        f32::from(c[1]) / 255.0,
        f32::from(c[2]) / 255.0,
    ]
}

#[cfg(test)]
#[path = "water_tests.rs"]
mod water_tests;
