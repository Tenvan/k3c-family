//! Wiedergabe einer Aufzeichnung aus Go (RU38): Leser, Takt-Steuerung, Zwischenbilder und die Wiedergabe als `App`.
//!
//! Format: `engine/internal/trace` (Kopfzeile mit dem Level, danach je Takt Zustand und Ereignisse).

pub mod app;
pub mod blend;
pub mod cli;
pub mod pending;
pub mod player;
pub mod report;
pub mod run;
pub mod trace;

pub use app::ReplayApp;
pub use blend::Blend;
pub use player::Playback;
pub use report::Report;
pub use trace::{Head, Row, Trace};
