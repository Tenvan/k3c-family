//! Port von `native/k3c-game/game/measure_frames.go` (RU0.1, B-1019/AC-01): Messung je Bild. Mit `--measure-frames
//! <datei>` merkt sich das Spiel in der Messung je gezeichnetem Bild die Zeit, die Bilddauer, die Rechenzeit von Draw,
//! die Takte der Simulation seit dem Bild davor, den Anteil `alpha` der Zwischenbilder und die GC-Läufe seit dem Bild
//! davor. Der Puffer wird vor der Messung angelegt und wächst nicht; geschrieben wird erst danach (eine JSON-Zeile je
//! Bild).
//!
//! Abweichung von Go: Rust hat keinen GC. `gc_cycles` liefert immer 0, damit bleiben `gc` je Bild und die GC-Felder des
//! Berichts 0 (wie in `replay/frames.rs`, RU38.3).

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::Path;

use serde::Serialize;

use super::measure::percentile;

/// Reserve des Puffers je Sekunde Messdauer (Bilder je Sekunde ohne VSync liegen weit darüber; ein voller Puffer zählt
/// weiter, merkt sich aber keine Bilder mehr).
pub const FRAME_CAP: usize = 1000;
/// Obergrenze des Puffers.
pub const FRAME_CAP_MAX: usize = 2_000_000;

/// Ein Bild heißt lang, wenn es länger dauert als das 1,5-Fache des Medians.
pub const LONG_FACTOR: f64 = 1.5;

/// Ein gemessenes Bild (Go: `frameSample`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FrameSample {
    /// Sekunden seit Beginn der Messung
    pub t: f64,
    /// Bilddauer in ms
    pub dt: f64,
    /// Rechenzeit von Draw in ms
    pub draw: f64,
    /// Takte der Simulation seit dem Bild davor
    pub ticks: u32,
    /// Anteil der Zwischenbilder (`Blend`)
    pub alpha: f32,
    /// GC-Läufe seit dem Bild davor (Rust: immer 0)
    pub gc: u32,
}

/// Kennwerte der Bilder (Bericht, Feld `bilder`); Namen und Reihenfolge wie Go.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct FrameReport {
    pub datei: String,
    pub bilder: usize,
    /// Bilder ohne Platz im Puffer
    pub verworfen: usize,
    pub median_ms: f64,
    pub p99_ms: f64,
    pub p999_ms: f64,
    pub max_ms: f64,
    /// Bilder über dem 1,5-Fachen des Medians
    pub lang: usize,
    /// davon mit GC-Lauf im selben Bild
    pub lang_mit_gc: usize,
    /// davon ohne neuen Takt
    pub lang_ohne_takt: usize,
    /// davon mit zwei oder mehr Takten
    pub lang_mehr_takt: usize,
    /// alle Bilder ohne neuen Takt
    pub ohne_takt: usize,
    /// alle Bilder mit zwei oder mehr Takten
    pub mehr_takt: usize,
    /// alle Bilder mit GC-Lauf
    pub mit_gc: usize,
    /// GC-Läufe in der Messung
    pub gc_laeufe: usize,
}

/// Sammelt die Bilder der Messung (Go: `frameLog`).
#[derive(Debug)]
pub struct FrameLog {
    pub path: String,
    pub buf: Vec<FrameSample>,
    /// Platz im Puffer (`buf` wächst nie darüber)
    pub cap: usize,
    pub dropped: usize,
    gc_last: u64,
}

impl FrameLog {
    /// Legt den Puffer für `seconds` Sekunden an; ohne Datei gibt es keine Messung je Bild (`None`).
    pub fn new(path: &str, seconds: f64) -> Option<FrameLog> {
        if path.is_empty() {
            return None;
        }
        let want = (seconds.max(0.0) * FRAME_CAP as f64) as usize + FRAME_CAP;
        let cap = want.min(FRAME_CAP_MAX);
        Some(FrameLog {
            path: path.to_string(),
            buf: Vec::with_capacity(cap),
            cap,
            dropped: 0,
            gc_last: 0,
        })
    }

    /// GC-Läufe seit Programmstart (Rust: ohne GC immer 0).
    fn gc_cycles(&self) -> u64 {
        0
    }

    /// Merkt den GC-Stand zu Beginn der Messung.
    pub fn start(&mut self) {
        self.gc_last = self.gc_cycles();
    }

    /// Merkt ein Bild: Zeit `t` seit Beginn der Messung, Bilddauer und Draw in ms, Takte seit dem Bild davor, Anteil
    /// `alpha` der Zwischenbilder (Go setzte ihn vorab als Feld).
    pub fn add(&mut self, t: f64, dt_ms: f64, draw_ms: f64, ticks: u64, alpha: f64) {
        let gc = self.gc_cycles();
        let n = gc - self.gc_last;
        self.gc_last = gc;
        if self.buf.len() >= self.cap {
            self.dropped += 1;
            return;
        }
        self.buf.push(FrameSample {
            t,
            dt: dt_ms,
            draw: draw_ms,
            ticks: u32::try_from(ticks).unwrap_or(u32::MAX),
            alpha: alpha as f32,
            gc: u32::try_from(n).unwrap_or(u32::MAX),
        });
    }

    /// Schreibt je Bild eine JSON-Zeile nach `path` (Go: `write`).
    pub fn write(&self) -> std::io::Result<()> {
        let mut w = std::io::BufWriter::new(std::fs::File::create(Path::new(&self.path))?);
        let mut line = String::with_capacity(96);
        for s in &self.buf {
            line.clear();
            let _ = writeln!(
                line,
                "{{\"t\":{:.6},\"dt_ms\":{:.4},\"draw_ms\":{:.4},\"takte\":{},\"alpha\":{:.4},\"gc\":{}}}",
                s.t, s.dt, s.draw, s.ticks, s.alpha, s.gc
            );
            w.write_all(line.as_bytes())?;
        }
        w.flush()
    }

    /// Schreibt die Datei und liefert die Kennwerte für den Bericht.
    pub fn report(&self) -> (FrameReport, std::io::Result<()>) {
        let mut r = frame_report(&self.buf);
        r.datei.clone_from(&self.path);
        r.verworfen = self.dropped;
        (r, self.write())
    }
}

/// Rechnet die Kennwerte aus den Bildern (Go: `frameReport`).
pub fn frame_report(buf: &[FrameSample]) -> FrameReport {
    let mut r = FrameReport {
        bilder: buf.len(),
        ..FrameReport::default()
    };
    if buf.is_empty() {
        return r;
    }
    let mut sorted: Vec<f64> = buf.iter().map(|s| s.dt).collect();
    sorted.sort_unstable_by(f64::total_cmp);
    r.median_ms = percentile(&sorted, 0.5);
    r.p99_ms = percentile(&sorted, 0.99);
    r.p999_ms = percentile(&sorted, 0.999);
    r.max_ms = sorted[sorted.len() - 1];
    let limit = r.median_ms * LONG_FACTOR;
    for s in buf {
        let long = s.dt > limit;
        r.gc_laeufe += s.gc as usize;
        count_if(s.gc > 0, long, &mut r.mit_gc, &mut r.lang_mit_gc);
        count_if(s.ticks == 0, long, &mut r.ohne_takt, &mut r.lang_ohne_takt);
        count_if(s.ticks >= 2, long, &mut r.mehr_takt, &mut r.lang_mehr_takt);
        if long {
            r.lang += 1;
        }
    }
    r
}

/// Zählt `all`, wenn `hit` gilt, und zusätzlich `long`, wenn das Bild lang ist.
fn count_if(hit: bool, is_long: bool, all: &mut usize, long: &mut usize) {
    if !hit {
        return;
    }
    *all += 1;
    if is_long {
        *long += 1;
    }
}

#[cfg(test)]
#[path = "measure_frames_tests.rs"]
mod tests;
