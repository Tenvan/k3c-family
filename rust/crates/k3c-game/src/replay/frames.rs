//! Messung je Bild der Wiedergabe (RU39.1) im Zeilenformat des Go-Spiels (`native/k3c-game/game/measure_frames.go`):
//! je gezeichnetem Bild der Messung die Zeit seit Messbeginn, die Bilddauer, die Rechenzeit von `draw` bis unmittelbar
//! vor `present`, die Takte seit dem Bild davor und alpha. `gc` ist immer 0 (Rust hat keinen GC).
//!
//! Der Puffer entsteht vor der Messung (Kapazität `FRAME_CAP` je Sekunde wie `frameCap`), geschrieben wird erst danach;
//! ein voller Puffer zählt verworfene Bilder weiter, ohne anzufordern.

use std::io::Write;
use std::path::{Path, PathBuf};

/// Reserve des Puffers je Sekunde Messdauer (Go: `frameCap`).
pub const FRAME_CAP: usize = 1000;
/// Obergrenze des Puffers (Go: `frameCapMax`).
pub const FRAME_CAP_MAX: usize = 2_000_000;

/// Ein gemessenes Bild (Go: `frameSample`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    /// Sekunden seit Beginn der Messung
    pub t: f64,
    /// Bilddauer in ms
    pub dt: f64,
    /// Rechenzeit von `draw` in ms
    pub draw: f64,
    /// Takte seit dem Bild davor
    pub ticks: u32,
    /// Anteil der Zwischenbilder
    pub alpha: f32,
}

/// Sammelt die Bilder der Messung (Go: `frameLog`).
#[derive(Debug)]
pub struct FrameLog {
    pub path: PathBuf,
    pub buf: Vec<Sample>,
    pub dropped: usize,
}

impl FrameLog {
    /// Legt den Puffer für `seconds` Sekunden an.
    pub fn new(path: &Path, seconds: f64) -> Self {
        let cap = ((seconds.max(0.0) as usize) * FRAME_CAP + FRAME_CAP).min(FRAME_CAP_MAX);
        Self {
            path: path.to_path_buf(),
            buf: Vec::with_capacity(cap),
            dropped: 0,
        }
    }

    /// Merkt ein Bild; ohne Platz im Puffer zählt es nur `dropped` (keine Allokation in der Messung).
    pub fn add(&mut self, s: Sample) {
        if self.buf.len() == self.buf.capacity() {
            self.dropped += 1;
            return;
        }
        self.buf.push(s);
    }

    /// Schreibt je Bild eine Zeile nach `path`.
    pub fn write(&self) -> std::io::Result<()> {
        if let Some(dir) = self.path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir)?;
        }
        let mut w = std::io::BufWriter::new(std::fs::File::create(&self.path)?);
        for s in &self.buf {
            writeln!(w, "{}", line(s))?;
        }
        w.flush()
    }
}

/// Eine Zeile wie Go: `{"t":%.6f,"dt_ms":%.4f,"draw_ms":%.4f,"takte":%d,"alpha":%.4f,"gc":%d}`.
pub fn line(s: &Sample) -> String {
    format!(
        "{{\"t\":{:.6},\"dt_ms\":{:.4},\"draw_ms\":{:.4},\"takte\":{},\"alpha\":{:.4},\"gc\":0}}",
        s.t, s.dt, s.draw, s.ticks, s.alpha
    )
}

#[cfg(test)]
#[path = "frames_tests.rs"]
mod tests;
