//! Plattform-Kern: Fenster, Zeichnen, Schleife (winit, wgpu); gebaut in RU2.1.
//!
//! Nur `backend/` kennt winit und wgpu. Alles andere (Takt, Sprite-Sammlung, Schrift, Messung,
//! headless) ist reiner Rust-Code ohne Grafikabhängigkeit und lässt sich ohne Fenster testen.
#![forbid(unsafe_code)]

mod backend;
mod batch;
mod clock;
mod font;
mod headless;
mod input;
mod measure;
mod runner;

pub use backend::{Config, run_window};
pub use batch::{Frame, INST_BYTES, Inst};
pub use clock::{Clock, MAX_STEPS, STEP};
pub use headless::{NullCanvas, run_headless};
pub use input::{InputScript, InputState, Key, KeyState, PadAxis, PadButton, PadState, ScriptEntry, ScriptPad};
pub use measure::{Measure, Summary};
pub use runner::Runner;

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

/// Zeichenfläche in logischen 1920×1080.
pub trait Canvas {
    fn size(&self) -> (f32, f32);
    /// Setzt die Ebene der folgenden Aufrufe (Standard 0 je Bild): niedrige Ebenen werden zuerst gezeichnet;
    /// innerhalb einer Ebene wird nach Textur gebündelt, Vollfarbe und Schrift zuletzt.
    fn layer(&mut self, _z: u32) {}
    /// Lädt RGBA8 mit vormultipliziertem Alpha hoch.
    fn texture(&mut self, rgba: &[u8], w: u32, h: u32) -> TexId;
    /// `src` in Textur-Pixeln, `dst` in logischen Einheiten.
    fn sprite(&mut self, t: TexId, src: Rect, dst: Rect, tint: [f32; 4]);
    fn rect(&mut self, dst: Rect, color: [f32; 4]);
    /// Pixel-Schrift (5×7, nur Großbuchstaben, Ziffern, Satzzeichen); `px` = Zeichenhöhe.
    fn text(&mut self, s: &str, x: f32, y: f32, px: f32, color: [f32; 4]);
    /// Beschneidet die folgenden Aufrufe auf `area` (Scissor, logische Einheiten); `None` hebt das auf.
    /// Koordinaten bleiben Fensterkoordinaten (wie `SubImage` bei Ebitengine). Standard je Bild: `None`.
    fn clip(&mut self, _area: Option<Rect>) {}
}
