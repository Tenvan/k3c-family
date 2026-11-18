//! Plattform-Kern: Fenster, Zeichnen, Schleife (winit, wgpu); gebaut in RU2.1.
//!
//! Nur `backend/` kennt winit und wgpu. Alles andere (Takt, Sprite-Sammlung, Schrift, Messung,
//! headless) ist reiner Rust-Code ohne Grafikabhängigkeit und lässt sich ohne Fenster testen.
#![forbid(unsafe_code)]

mod affine;
mod audio;
mod backend;
mod batch;
mod clock;
mod font;
mod frame_canvas;
mod headless;
mod input;
pub mod log;
mod measure;
mod runner;
mod shape;
mod ttf;

pub use affine::Affine;
pub use audio::{AudioOut, NullAudio, SoundId, VoiceId, open_audio, open_audio_with};
pub use backend::{Config, run_window};
pub use batch::{Frame, INST_BYTES, Inst, Kind, Run, VERT_BYTES, Vertex};
pub use clock::{Clock, MAX_STEPS, STEP};
pub use headless::{NullCanvas, run_headless};
pub use input::{InputScript, InputState, Key, KeyState, PadAxis, PadButton, PadState, ScriptEntry, ScriptPad};
pub use measure::{Measure, Summary};
pub use runner::Runner;
pub use ttf::FontId;

/// Logische Auflösung; das Fenster skaliert darauf.
pub const LOGICAL: (f32, f32) = (1920.0, 1080.0);

/// Rechteck in Pixeln (Textur) oder logischen Einheiten (Bildschirm).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

/// Filter beim Abtasten einer Textur (wie `ebiten.FilterLinear`/`FilterNearest`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    Linear,
    Nearest,
}

/// Mischung der Quelle mit dem Ziel (wie `ebiten.Blend`, soweit das Spiel sie braucht).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Blend {
    /// Quelle (vormultipliziert) über das Ziel.
    #[default]
    Over,
    /// Ziel × Quelle je Farbkanal, die Deckkraft des Ziels bleibt (Lichtkarte, `lightdraw.go`).
    Multiply,
}

/// Handle einer hochgeladenen Textur.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TexId(pub u32);

/// Spiel-Seite der Schleife: Takt-Schritte und Zeichnen.
pub trait App {
    /// Ein Simulationsschritt von `dt` Sekunden (fester Takt, siehe [`STEP`]) mit der Eingabe dieses Takts.
    fn step(&mut self, input: &InputState, dt: f64);
    /// Zeichnet das Bild; `alpha` in [0, 1) ist der Rest zum nächsten Takt.
    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64);
    /// Wahr, sobald die Schleife enden soll.
    fn done(&self) -> bool;
    /// Gewünschter Vollbild-Zustand (`None` = egal); die Schleife setzt ihn nach den Takten, wenn er abweicht.
    fn fullscreen(&self) -> Option<bool> {
        None
    }
}

/// Zeichenfläche in logischen 1920×1080. Farben und Tönungen sind gerade (nicht vormultipliziert, die Plattform
/// rechnet um); Texturen sind RGBA8 mit vormultipliziertem Alpha. Zeichenreihenfolge: Ebene → Ausschnitt → Aufruf (wie Ebitengine, nie nach Textur umsortiert).
pub trait Canvas {
    fn size(&self) -> (f32, f32);
    /// Setzt die Ebene der folgenden Aufrufe (Standard 0 je Bild): niedrige Ebenen werden zuerst gezeichnet;
    /// innerhalb einer Ebene gilt nach Ausschnitt die Aufrufreihenfolge (hinten nach vorn, auch über Texturen).
    fn layer(&mut self, _z: u32) {}
    /// Beschneidet die folgenden Aufrufe auf `area` (Scissor, logische Einheiten); `None` hebt das auf.
    /// Koordinaten bleiben Fensterkoordinaten (wie `SubImage` bei Ebitengine). Standard je Bild: `None`.
    fn clip(&mut self, _area: Option<Rect>) {}
    /// Setzt die Mischung der folgenden Aufrufe (Standard je Bild: [`Blend::Over`]). Aufrufe mit anderer Mischung
    /// bilden eigene Draw-Calls; die Reihenfolge bleibt die der Aufrufe.
    fn blend(&mut self, _mode: Blend) {}
    /// Lädt RGBA8 mit vormultipliziertem Alpha hoch (Filter linear).
    fn texture(&mut self, rgba: &[u8], w: u32, h: u32) -> TexId {
        self.texture_with(rgba, w, h, Filter::Linear)
    }
    /// Lädt RGBA8 mit vormultipliziertem Alpha und dem Filter `filter` hoch.
    fn texture_with(&mut self, rgba: &[u8], w: u32, h: u32, filter: Filter) -> TexId;
    /// Gibt die Textur am Bildende frei; spätere Aufrufe mit ihr zeichnen nichts.
    fn free_texture(&mut self, t: TexId);
    /// `src` in Textur-Pixeln, `dst` in logischen Einheiten.
    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]);
    /// `src` in Textur-Pixeln, abgebildet mit `tf` (wie `DrawImage` eines `SubImage` mit `GeoM`).
    fn sprite_tf(&mut self, t: TexId, src: Rect, tf: Affine, tint: [f32; 4]);
    /// Dreiecksnetz wie `DrawTriangles`: `verts` mit Quelle in Textur-Pixeln, je drei `idx` ein Dreieck.
    fn mesh(&mut self, t: TexId, verts: &[Vertex], idx: &[u16]);
    /// Gefülltes Rechteck (Kurzform von [`Canvas::fill_rect`]).
    fn rect(&mut self, dst: Rect, color: [f32; 4]);
    fn fill_rect(&mut self, dst: Rect, color: [f32; 4]) {
        self.rect(dst, color);
    }
    /// Rahmen der Breite `width`, mittig auf dem Rand (wie `vector.StrokeRect`).
    fn stroke_rect(&mut self, r: Rect, width: f32, color: [f32; 4]);
    fn fill_circle(&mut self, cx: f32, cy: f32, r: f32, color: [f32; 4]);
    /// Kreislinie der Breite `width`, mittig auf dem Radius.
    fn stroke_circle(&mut self, cx: f32, cy: f32, r: f32, width: f32, color: [f32; 4]);
    /// Strecke der Breite `width` mit glatten Enden (wie `vector.StrokeLine`).
    fn stroke_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, width: f32, color: [f32; 4]);
    /// Gefülltes einfaches Polygon aus Geraden (konvex oder konkav).
    fn fill_path(&mut self, points: &[[f32; 2]], color: [f32; 4]);
    /// Pixel-Schrift (5×7, nur Großbuchstaben, Ziffern, Satzzeichen); `px` = Zeichenhöhe. Rückfall, wenn die
    /// TTF-Schrift fehlt.
    fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]);
    /// Lädt eine TTF-Schrift (z. B. `public/schrift/k3c.ttf`).
    fn font(&mut self, ttf: &[u8]) -> Result<FontId, String>;
    /// Text in der Schrift `font` mit Pixelgröße `px` (auf ganze Pixel gerundet); `(x, y)` ist die linke obere
    /// Ecke der Zeile, die Grundlinie liegt bei `y` + Oberlänge (wie `text/v2`).
    fn text_font(&mut self, font: FontId, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]);
    /// Breite von `s` in Pixeln (Vorschub plus Unterschneidung), im Fenster und headless gleich.
    fn text_width(&self, font: FontId, s: &str, px: f32) -> f32;
}
