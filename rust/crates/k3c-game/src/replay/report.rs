//! Messbericht der Wiedergabe: dieselben JSON-Namen und Reihenfolge wie `Report` in `native/k3c-game/game/measure.go`.
//!
//! Felder ohne Rust-Gegenstück (`go_heap_mib`, `go_sys_mib`, `gc_*`, `ebitengine`) bleiben 0 bzw. leer;
//! `backend` nennt das Rust-Backend.

use std::time::Instant;

use k3c_platform::{App, Canvas, InputState};
use serde::Serialize;

use super::app::ReplayApp;

/// Kennwerte der Zeiten in Millisekunden.
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct FrameStats {
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub max: f64,
}

/// Messbericht (Feldreihenfolge wie Go).
#[derive(Debug, Clone, Default, Serialize)]
pub struct Report {
    pub szene: String,
    pub spieler: u32,
    pub ebitengine: String,
    pub backend: String,
    pub fenster: [u32; 2],
    pub vsync: bool,
    pub sheets: u32,
    pub textur_mib_rgba: f64,
    pub rig_mib_rgba: f64,
    pub messdauer_s: f64,
    pub frames: usize,
    pub fps_mittel: f64,
    pub fps_1pct_low: f64,
    pub frametime_ms: FrameStats,
    pub rechenzeit_ms: f64,
    pub takt_ms: f64,
    pub takt_max_ms: f64,
    pub takte: usize,
    pub takt_ende: u64,
    pub phase: String,
    pub gegner: usize,
    pub go_heap_mib: f64,
    pub go_sys_mib: f64,
    pub bilder_ohne_takt: usize,
    pub takt_abstand_ms: FrameStats,
    pub gc_laeufe: u32,
    pub gc_pause_ms: f64,
}

fn sum(v: &[f64]) -> f64 {
    v.iter().sum()
}

fn percentile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    sorted[((sorted.len() as f64 * q).ceil() as usize).clamp(1, sorted.len()) - 1]
}

fn stats(mut v: Vec<f64>) -> FrameStats {
    v.sort_unstable_by(f64::total_cmp);
    FrameStats {
        p50: percentile(&v, 0.5),
        p95: percentile(&v, 0.95),
        p99: percentile(&v, 0.99),
        max: v.last().copied().unwrap_or(0.0),
    }
}

/// Misst eine Wiedergabe: `warmup` Takte aufwärmen, dann `measure` Takte messen, danach `done`.
pub struct Metered {
    pub app: ReplayApp,
    warmup: u64,
    measure: u64,
    steps: u64,
    last_draw: Option<Instant>,
    last_step: Option<Instant>,
    last_tick: usize,
    frames: Vec<f64>,
    draw_ms: Vec<f64>,
    ticks: Vec<f64>,
    gaps: Vec<f64>,
    repeated: usize,
}

impl Metered {
    pub fn new(app: ReplayApp, warmup: u64, measure: u64) -> Self {
        Self {
            app,
            warmup,
            measure,
            steps: 0,
            last_draw: None,
            last_step: None,
            last_tick: 0,
            frames: Vec::new(),
            draw_ms: Vec::new(),
            ticks: Vec::new(),
            gaps: Vec::new(),
            repeated: 0,
        }
    }

    fn measuring(&self) -> bool {
        self.steps >= self.warmup
    }

    /// Bericht aus den gesammelten Zeiten und dem Stand am Ende.
    pub fn report(&self, backend: &str) -> Report {
        let mut sorted = self.frames.clone();
        sorted.sort_unstable_by(f64::total_cmp);
        let total = sum(&sorted);
        let worst = &sorted[sorted.len() - (sorted.len() / 100).max(1).min(sorted.len())..];
        let (frames_n, worst_n) = (sorted.len(), worst.len());
        let mut r = Report {
            szene: format!(
                "szene={} seed={} spieler={}",
                self.app.playback().head.szene,
                self.app.playback().head.seed,
                self.app.playback().head.spieler
            ),
            spieler: self.app.playback().head.spieler,
            backend: backend.into(),
            fenster: [1920, 1080],
            vsync: true,
            messdauer_s: total / 1000.0,
            frames: frames_n,
            frametime_ms: stats(self.frames.clone()),
            rechenzeit_ms: sum(&self.draw_ms) / self.draw_ms.len().max(1) as f64,
            takte: self.ticks.len(),
            takt_ende: self.app.playback().pos() as u64,
            bilder_ohne_takt: self.repeated,
            takt_abstand_ms: stats(self.gaps.clone()),
            ..Report::default()
        };
        if total > 0.0 && frames_n > 0 {
            r.fps_mittel = frames_n as f64 * 1000.0 / total;
            r.fps_1pct_low = 1000.0 / (sum(worst) / worst_n as f64);
        }
        if !self.ticks.is_empty() {
            r.takt_ms = sum(&self.ticks) / self.ticks.len() as f64;
            r.takt_max_ms = self.ticks.iter().copied().fold(0.0, f64::max);
        }
        if let Some(w) = self.app.playback().world() {
            r.phase = w.cycle.phase.clone();
            r.gegner = w.enemies.iter().filter(|e| e.hp > 0.0).count();
        }
        r
    }
}

impl App for Metered {
    fn step(&mut self, input: &InputState, dt: f64) {
        let t0 = Instant::now();
        self.app.step(input, dt);
        if self.measuring() {
            self.ticks.push(t0.elapsed().as_secs_f64() * 1000.0);
            if let Some(l) = self.last_step {
                self.gaps.push(t0.duration_since(l).as_secs_f64() * 1000.0);
            }
            self.last_step = Some(t0);
        }
        self.steps += 1;
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        let t0 = Instant::now();
        self.app.draw(c, alpha);
        let work = t0.elapsed().as_secs_f64() * 1000.0;
        let dt = self
            .last_draw
            .map_or(0.0, |l| t0.duration_since(l).as_secs_f64() * 1000.0);
        self.last_draw = Some(t0);
        let tick = self.app.playback().pos();
        if self.measuring() && self.steps < self.warmup.saturating_add(self.measure) {
            self.frames.push(dt);
            self.draw_ms.push(work);
            if tick == self.last_tick {
                self.repeated += 1;
            }
        }
        self.last_tick = tick;
    }

    fn done(&self) -> bool {
        self.app.done() || self.steps >= self.warmup.saturating_add(self.measure)
    }
}

#[cfg(test)]
#[path = "report_tests.rs"]
mod tests;
