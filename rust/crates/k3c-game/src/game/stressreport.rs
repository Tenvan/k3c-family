//! Port von `native/k3c-game/game/stressreport.go`: Messwerte und Bericht des Stresstests (B-896): je Takt Rechenzeit der
//! Simulation (`Frame::step_ms`), Bildrate und Objektzahlen; am Ende `reports/native-stress-<Zeit>.json` (lesbar über
//! `report_read` der Workbench) mit Takten je Sekunde, Taktdauer p50/p95/max, FPS, Objektzahlen und Speicher.
//!
//! Das Format ist das des Go-Spiels (Feldnamen und Reihenfolge, `stress_tests` prüft die Feldliste). `goHeapMib` und
//! `goSysMib` haben in Rust kein Gegenstück und bleiben 0 (wie in `measure.rs`).

use std::path::{Path, PathBuf};

use serde::Serialize;

use super::measure::{FrameStats, percentile, round1};
use crate::devtools::report::Zeit;
use crate::view::frame::{Frame, Outcome};
use crate::view::stress::StressTest;

/// Ein laufender Stresstest (Go: `stressRun`).
pub struct StressRun {
    pub(super) cfg: StressTest,
    pub(super) start: Zeit,
    /// Vorspann (Ausbau, Truppen, Zeitraffer) ausgeführt
    pub(super) prelude: bool,
    /// Takt beim Vorspann
    pub(super) start_tick: u64,
    /// gezählte Wellen (Ereignis `wave` der Stufe von Spieler 1)
    pub(super) waves: i64,
    /// seit der letzten Welle waren Gegner da (ihre Gegner liegen erst in der `spawn_queue`)
    pub(super) fought: bool,
    steps: Vec<f64>,
    fps_sum: f64,
    fps_min: f64,
    fps_n: usize,
    peak: StressObjects,
}

/// Die Objektzahlen aller Stufen des Frames (Go: `stressObjects`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct StressObjects {
    #[serde(rename = "gegner")]
    pub enemies: usize,
    #[serde(rename = "truppen")]
    pub troops: usize,
    #[serde(rename = "geschosse")]
    pub projectiles: usize,
    #[serde(rename = "muenzen")]
    pub coins: usize,
    /// Gegner in der `spawn_queue` (noch nicht erschienen)
    #[serde(rename = "wartend")]
    pub queued: usize,
}

/// Der Bericht eines Stresstests (Go: `StressReport`; Namen und Reihenfolge wie dort).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StressReport {
    /// immer "native-stress"
    pub kind: String,
    #[serde(rename = "einstellungen")]
    pub config: StressTest,
    pub start: String,
    pub end: String,
    #[serde(rename = "dauerSekunden")]
    pub seconds: f64,
    /// wellen, minuten, gewonnen, verloren, verlassen
    #[serde(rename = "endeUrsache")]
    pub cause: String,
    #[serde(rename = "wellen")]
    pub waves: i64,
    #[serde(rename = "takte")]
    pub ticks: u64,
    #[serde(rename = "takteJeSekunde")]
    pub ticks_per_second: f64,
    #[serde(rename = "taktdauerMs")]
    pub step_ms: FrameStats,
    #[serde(rename = "fpsMittel")]
    pub fps_mean: f64,
    #[serde(rename = "fpsMin")]
    pub fps_min: f64,
    #[serde(rename = "hoechstwerte")]
    pub peak: StressObjects,
    #[serde(rename = "amEnde")]
    pub at_end: StressObjects,
    #[serde(rename = "goHeapMib")]
    pub heap_mib: f64,
    #[serde(rename = "goSysMib")]
    pub sys_mib: f64,
}

/// Zählt die Objekte aller Stufen des Frames (Go: `objectsOf`).
pub fn objects_of(f: &Frame) -> StressObjects {
    let mut o = StressObjects::default();
    for w in f.stages.iter().filter_map(|st| st.world.as_ref()) {
        o.enemies += w.enemies.len();
        o.troops += w.troops.len();
        o.projectiles += w.projectiles.len();
        o.coins += w.coins.len();
        o.queued += w.spawn_queue.len();
    }
    o
}

/// Auf zwei Nachkommastellen gerundet (Go: `float64(int64(v*100+0.5)) / 100`, wörtlich).
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn round2(v: f64) -> f64 {
    ((v * 100.0 + 0.5) as i64) as f64 / 100.0
}

impl StressRun {
    pub fn new(cfg: StressTest, now: Zeit) -> StressRun {
        StressRun {
            cfg,
            start: now,
            prelude: false,
            start_tick: 0,
            waves: 0,
            fought: false,
            steps: Vec::with_capacity(4096),
            fps_sum: 0.0,
            fps_min: 0.0,
            fps_n: 0,
            peak: StressObjects::default(),
        }
    }

    /// Nimmt die Messwerte eines Takts auf (`fps <= 0`: keine Bildrate, z. B. Tests).
    #[allow(clippy::float_cmp)]
    pub fn sample(&mut self, f: &Frame, fps: f64) {
        if f.step_ms > 0.0 {
            self.steps.push(f.step_ms);
        }
        if fps > 0.0 {
            self.fps_sum += fps;
            self.fps_n += 1;
            if self.fps_min == 0.0 || fps < self.fps_min {
                self.fps_min = fps;
            }
        }
        let o = objects_of(f);
        self.peak = StressObjects {
            enemies: self.peak.enemies.max(o.enemies),
            troops: self.peak.troops.max(o.troops),
            projectiles: self.peak.projectiles.max(o.projectiles),
            coins: self.peak.coins.max(o.coins),
            queued: self.peak.queued.max(o.queued),
        };
        self.fought = self.fought || o.enemies > 0;
    }

    /// Die Ursache, aus der der Test endet (leer = läuft weiter): Spielende, Minuten (Wanduhr samt Pausen), oder die
    /// letzte eingestellte Welle ist geschlagen: ihre Gegner waren da, und keiner lebt oder wartet noch. `sample` läuft
    /// vorher (`fought`).
    #[allow(clippy::cast_precision_loss)]
    pub fn over(&self, f: &Frame, now: &Zeit) -> String {
        let c = &self.cfg;
        let o = objects_of(f);
        let cause = if f.outcome == Outcome::Won {
            "gewonnen"
        } else if f.outcome == Outcome::Lost {
            "verloren"
        } else if c.minutes > 0 && now.since(&self.start) >= (c.minutes * 60) as f64 {
            "minuten"
        } else if c.waves > 0 && self.waves >= c.waves && self.fought && o.enemies + o.queued == 0 {
            "wellen"
        } else {
            ""
        };
        cause.to_string()
    }

    /// Baut den Bericht zum Frame `f` am Ende.
    #[allow(clippy::cast_precision_loss)]
    pub fn report(&self, f: &Frame, now: &Zeit, cause: &str) -> StressReport {
        let secs = now.since(&self.start);
        let ticks = f.tick.saturating_sub(self.start_tick);
        let mut sorted = self.steps.clone();
        sorted.sort_unstable_by(f64::total_cmp);
        let mut rep = StressReport {
            kind: "native-stress".to_string(),
            config: self.cfg.clone(),
            start: self.start.format(),
            end: now.format(),
            seconds: round1(secs),
            cause: cause.to_string(),
            waves: self.waves,
            ticks,
            ticks_per_second: if secs > 0.0 { round1(ticks as f64 / secs) } else { 0.0 },
            step_ms: FrameStats::default(),
            fps_mean: 0.0,
            fps_min: 0.0,
            peak: self.peak,
            at_end: objects_of(f),
            heap_mib: 0.0,
            sys_mib: 0.0,
        };
        if let Some(max) = sorted.last() {
            rep.step_ms = FrameStats {
                p50: round2(percentile(&sorted, 0.50)),
                p95: round2(percentile(&sorted, 0.95)),
                p99: round2(percentile(&sorted, 0.99)),
                max: round2(*max),
            };
        }
        if self.fps_n > 0 {
            rep.fps_mean = round1(self.fps_sum / self.fps_n as f64);
            rep.fps_min = round1(self.fps_min);
        }
        rep
    }
}

/// Schreibt den Bericht nach `dir` und liefert den Pfad (Doppelpunkte der Uhrzeit als Bindestrich).
pub fn write_stress_report(dir: &Path, r: &StressReport) -> Result<PathBuf, String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let raw = serde_json::to_string_pretty(r).map_err(|e| e.to_string())?;
    let path = dir.join(format!("native-stress-{}.json", r.end.replace(':', "-")));
    std::fs::write(&path, raw).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
#[path = "stressreport_tests.rs"]
mod tests;
