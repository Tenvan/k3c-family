//! Messmodus (B-776/AC-04): warten, bis alle Bilder geladen sind, aufwärmen, dann Frame-Zeiten sammeln und einen
//! JSON-Bericht schreiben. GPU-Speicher misst `native/measure.ps1` von außen (Windows-Zähler) und ergänzt den Bericht.

use crate::paper::Paper;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use serde::Serialize;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Resource)]
pub struct Measure {
    pub mode: String,
    pub warmup_s: f32,
    pub measure_s: f32,
    pub report: Option<PathBuf>,
    pub vsync: bool,
    pub started: Instant,
    phase: Phase,
    loaded_after_s: f32,
    frames_ms: Vec<f32>,
}

#[derive(PartialEq)]
enum Phase {
    Loading,
    Warmup(f32),
    Measuring(f32),
    Done,
}

impl Measure {
    pub fn new(mode: String, warmup_s: f32, measure_s: f32, report: Option<PathBuf>, vsync: bool) -> Self {
        Self { mode, warmup_s, measure_s, report, vsync, started: Instant::now(), phase: Phase::Loading, loaded_after_s: 0.0, frames_ms: Vec::new() }
    }
}

#[derive(Serialize)]
struct FrameStats {
    p50: f32,
    p95: f32,
    p99: f32,
    max: f32,
}

#[derive(Serialize)]
struct Report {
    modus: String,
    bevy: &'static str,
    adapter: String,
    backend: String,
    treiber: String,
    fenster: [u32; 2],
    vsync: bool,
    sheets: usize,
    textur_mib_rgba: f64,
    ladezeit_s: f32,
    messdauer_s: f32,
    frames: usize,
    fps_mittel: f32,
    fps_1pct_low: f32,
    frametime_ms: FrameStats,
}

/// Ablauf: Laden → Aufwärmen → Messen → Bericht → Ende.
pub fn measure_step(
    time: Res<Time>,
    server: Res<AssetServer>,
    paper: Res<Paper>,
    mut m: ResMut<Measure>,
    adapter: Option<Res<RenderAdapterInfo>>,
    windows: Query<&Window>,
    mut exit: MessageWriter<AppExit>,
) {
    let dt = time.delta_secs();
    match m.phase {
        Phase::Loading => {
            if paper.handles.iter().all(|h| server.is_loaded_with_dependencies(h)) {
                m.loaded_after_s = m.started.elapsed().as_secs_f32();
                info!("📂 {} Sheets geladen in {:.1} s ({:.0} MiB RGBA)", paper.sheet_count(), m.loaded_after_s, mib(paper.texture_bytes()));
                m.phase = Phase::Warmup(0.0);
            }
        }
        Phase::Warmup(t) => m.phase = if t + dt >= m.warmup_s { Phase::Measuring(0.0) } else { Phase::Warmup(t + dt) },
        Phase::Measuring(t) => {
            m.frames_ms.push(dt * 1000.0);
            if m.measure_s > 0.0 && t + dt >= m.measure_s {
                let size = windows.iter().next().map_or([0, 0], |w| [w.physical_width(), w.physical_height()]);
                let report = build_report(&m, &paper, adapter.as_deref(), size);
                finish(&m, &report);
                m.phase = Phase::Done;
                exit.write(AppExit::Success);
            } else {
                m.phase = Phase::Measuring(t + dt);
            }
        }
        Phase::Done => {}
    }
}

fn mib(b: u64) -> f64 {
    b as f64 / (1024.0 * 1024.0)
}

fn percentile(sorted: &[f32], p: f32) -> f32 {
    if sorted.is_empty() {
        return 0.0;
    }
    let i = ((sorted.len() - 1) as f32 * p).round() as usize;
    sorted[i]
}

fn build_report(m: &Measure, paper: &Paper, adapter: Option<&RenderAdapterInfo>, size: [u32; 2]) -> Report {
    let mut sorted = m.frames_ms.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let total_ms: f32 = sorted.iter().sum();
    let n = sorted.len();
    // 1-%-Low: mittlere FPS der langsamsten 1 % der Frames
    let worst = &sorted[n - (n / 100).max(1)..];
    let worst_ms = worst.iter().sum::<f32>() / worst.len() as f32;
    Report {
        modus: m.mode.clone(),
        bevy: "0.20.1",
        adapter: adapter.map_or("?".into(), |a| a.name.clone()),
        backend: adapter.map_or("?".into(), |a| format!("{:?}", a.backend)),
        treiber: adapter.map_or("?".into(), |a| format!("{} {}", a.driver, a.driver_info)),
        fenster: size,
        vsync: m.vsync,
        sheets: paper.sheet_count(),
        textur_mib_rgba: (mib(paper.texture_bytes()) * 10.0).round() / 10.0,
        ladezeit_s: m.loaded_after_s,
        messdauer_s: total_ms / 1000.0,
        frames: n,
        fps_mittel: n as f32 * 1000.0 / total_ms,
        fps_1pct_low: 1000.0 / worst_ms,
        frametime_ms: FrameStats { p50: percentile(&sorted, 0.5), p95: percentile(&sorted, 0.95), p99: percentile(&sorted, 0.99), max: *sorted.last().unwrap_or(&0.0) },
    }
}

fn finish(m: &Measure, r: &Report) {
    let json = serde_json::to_string_pretty(r).expect("Bericht serialisieren");
    info!("✅ Messung {}: {:.0} FPS Mittel, {:.0} FPS 1-%-Low, p99 {:.1} ms", r.modus, r.fps_mittel, r.fps_1pct_low, r.frametime_ms.p99);
    match &m.report {
        Some(path) => match std::fs::write(path, &json) {
            Ok(()) => info!("💾 Bericht: {}", path.display()),
            Err(e) => error!("❌ Bericht nicht geschrieben: {} ({e})", path.display()),
        },
        None => println!("{json}"),
    }
}
