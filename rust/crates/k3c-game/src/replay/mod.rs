//! Wiedergabe einer Aufzeichnung aus Go (RU38): Leser, Takt-Steuerung, Zwischenbilder und die Wiedergabe als `App`.
//! Seit RU39.1 zeichnen die Zeichner des Spiels (`scene`) statt Platzhaltern; `frames` misst je Bild im Go-Format.
//!
//! Format: `engine/internal/trace` (Kopfzeile mit dem Level, danach je Takt Zustand und Ereignisse).

pub mod app;
pub mod blend;
pub mod cli;
pub mod frames;
pub mod layout;
pub mod neutral;
pub mod player;
pub mod report;
pub mod run;
pub mod scene;
#[cfg(test)]
mod scene_tests;
pub mod trace;

pub use app::ReplayApp;
pub use blend::Blend;
pub use player::Playback;
pub use report::Report;
pub use trace::{Head, Row, Trace};
