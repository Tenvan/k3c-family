//! Messung je Bild der Wiedergabe (RU39.1) im Zeilenformat des Go-Spiels (`native/k3c-game/game/measure_frames.go`).
//!
//! Seit B-1116 gibt es nur noch eine Fassung: Puffer, Zeilen und Kennwerte (`FrameReport`, Feld `bilder` des Berichts)
//! liegen in `game/measure_frames.rs` und gelten für Spiel und Wiedergabe gleich.

pub use crate::game::measure_frames::{FRAME_CAP, FRAME_CAP_MAX, FrameLog, FrameReport, FrameSample, frame_report};
