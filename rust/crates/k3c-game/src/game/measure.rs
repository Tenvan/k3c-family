//! Port von `native/k3c-game/game/measure.go`: Messmodus (`--measure`, NG9.2, B-802/AC-03). Das Spiel läuft im Fenster in
//! Echtzeit (Szene aus `--seed`, `--tick`, `--input`), wärmt `--warmup` Sekunden auf und misst dann `--measure` Sekunden
//! die echte Zeit je gezeichnetem Bild (`clock::clock`), die Rechenzeit von Draw (CPU, Zeichenbefehle) und von Update
//! (Eingabe, Simulation, Listener; 30 Hz). Danach schreibt es den Bericht als JSON und beendet sich (`Meter::done`).
//!
//! Der Bericht hat dieselben Namen und dieselbe Reihenfolge wie der von Go (`Report`, `measure.rs` sammelt ihn im Spiel;
//! `replay/report.rs` baut ihn für die Wiedergabe, ohne das Feld `bilder`). Felder ohne Rust-Gegenstück bleiben 0:
//! `go_heap_mib`, `go_sys_mib`, `gc_laeufe`, `gc_pause_ms`. `ebitengine` nennt die Plattform samt wgpu-Version, `backend`
//! das Grafik-Backend (`wgpu`; die Plattform nennt den Treiber nicht). `--novsync` kann die Plattform nicht (Fifo): Das
//! Feld `vsync` bleibt `true`, das Log meldet es.
//!
//! Die Zeit kommt nur hier und nur im Messmodus aus der Wanduhr; `Frame::tick` zählt weiter nur Takte. Die Zeitsammlung
//! der Plattform rechnet ein Bild mit 0, 1 oder 2 Takten; `bilder_ohne_takt` zählt die ohne neuen Takt (bei 60 Hz Bild
//! und 30 Hz Takt rund die Hälfte, der Normalfall).

use k3c_platform::log;
use serde::Serialize;

use super::measure_frames::{FRAME_CAP, FRAME_CAP_MAX, FrameLog, FrameReport};
use super::{Game, Measure, Scene, TPS};
use crate::view::frame::Frame;

/// Version der wgpu-Abhängigkeit der Plattform (`rust/Cargo.toml`, fest mit `=`); `measure_tests` prüft sie dort.
pub const WGPU_VERSION: &str = "30.0.2";

/// Kennwerte der Zeiten in Millisekunden (Go: `FrameStats`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct FrameStats {
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub max: f64,
}

/// Der Messbericht (Go: `Report`; Namen und Reihenfolge wie dort).
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Report {
    pub szene: String,
    pub spieler: usize,
    pub ebitengine: String,
    pub backend: String,
    pub fenster: [i64; 2],
    pub vsync: bool,
    pub sheets: i64,
    pub textur_mib_rgba: f64,
    pub rig_mib_rgba: f64,
    pub messdauer_s: f64,
    pub frames: usize,
    pub fps_mittel: f64,
    pub fps_1pct_low: f64,
    pub frametime_ms: FrameStats,
    /// Mittel je Bild: Draw (CPU, Zeichenbefehle)
    pub rechenzeit_ms: f64,
    /// Mittel je Update: Eingabe, Simulation, Listener (30 Hz)
    pub takt_ms: f64,
    pub takt_max_ms: f64,
    pub takte: usize,
    /// `Frame::tick` am Ende der Messung
    pub takt_ende: u64,
    /// Tageszeit der Stufe von Spieler 1 am Ende
    pub phase: String,
    /// lebende Gegner aller Stufen am Ende
    pub gegner: usize,
    pub go_heap_mib: f64,
    pub go_sys_mib: f64,
    /// Bilder, in denen sich der Takt seit dem Bild davor nicht änderte
    pub bilder_ohne_takt: usize,
    /// Zeit zwischen zwei Updates (Soll 33,3 ms)
    pub takt_abstand_ms: FrameStats,
    pub gc_laeufe: u32,
    pub gc_pause_ms: f64,
    /// Kennwerte je Bild (`--measure-frames`), sonst nicht im Bericht
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bilder: Option<FrameReport>,
}

/// Sammelt die Zeiten; Phase 0 = Aufwärmen, 1 = Messen, 2 = fertig (Go: `meter`).
pub struct Meter {
    cfg: Measure,
    phase: u8,
    /// Sekunden in der Phase
    t: f64,
    /// Zeitpunkt des letzten Bilds
    last: f64,
    frames: Vec<f64>,
    draw_ms: f64,
    ticks: Vec<f64>,
    /// Takt des letzten Bilds
    last_tick: u64,
    /// Bilder ohne neuen Takt
    repeated: usize,
    /// Zeitpunkt des letzten Updates
    last_start: f64,
    gaps: Vec<f64>,
    /// Messung je Bild (`--measure-frames`), sonst `None`
    log: Option<FrameLog>,
}

impl Meter {
    /// Legt die Puffer vor der Messung an (kein Wachsen im Lauf).
    pub fn new(cfg: &Measure) -> Meter {
        if cfg.no_vsync {
            log::info("🚦 --novsync wirkt in Rust nicht (Fifo)", &[]); // die Plattform kennt nur Fifo
        }
        let seconds = cfg.seconds.max(0.0);
        let frames = ((seconds * FRAME_CAP as f64) as usize + FRAME_CAP).min(FRAME_CAP_MAX);
        let takes = (seconds * (TPS * 4) as f64) as usize + 64;
        Meter {
            cfg: cfg.clone(),
            phase: 0,
            t: 0.0,
            last: 0.0,
            frames: Vec::with_capacity(frames),
            draw_ms: 0.0,
            ticks: Vec::with_capacity(takes),
            last_tick: 0,
            repeated: 0,
            last_start: 0.0,
            gaps: Vec::with_capacity(takes),
            log: FrameLog::new(&cfg.frames, cfg.seconds),
        }
    }

    /// Meldet, dass der Bericht geschrieben ist (`Game::update` beendet das Programm).
    pub fn done(&self) -> bool {
        self.phase == 2
    }

    /// Zählt die Rechenzeit eines Updates in Sekunden (nur in der Messung).
    pub fn tick(&mut self, sec: f64) {
        if self.phase == 1 {
            self.ticks.push(sec * 1000.0);
        }
    }

    /// Merkt den Beginn eines Updates (Uhrzeit in Sekunden) und sammelt den Abstand zum Update davor.
    pub fn started(&mut self, now: f64) {
        if self.phase == 1 && self.last_start > 0.0 {
            self.gaps.push((now - self.last_start) * 1000.0);
        }
        self.last_start = now;
    }

    /// Läuft am Ende jedes Draw mit der Uhrzeit `now`, der Rechenzeit `work` dieses Draw in Sekunden, dem Anteil
    /// `alpha` der Zwischenbilder und dem Takt des Frames. Meldet `true`, wenn die Messung in diesem Bild endet: Dann
    /// baut der Aufrufer mit `Game::report` den Bericht und übergibt ihn an `finish`.
    pub fn frame(&mut self, now: f64, work: f64, alpha: f64, tick: u64) -> bool {
        let dt = if self.last > 0.0 { now - self.last } else { 0.0 };
        self.last = now;
        let mut ended = false;
        match self.phase {
            0 => {
                self.t += dt;
                if self.t >= self.cfg.warmup {
                    (self.phase, self.t) = (1, 0.0);
                    if let Some(l) = self.log.as_mut() {
                        l.start();
                    }
                    log::info(
                        "⏳ Messung beginnt",
                        &[("takt", &tick), ("sekunden", &self.cfg.seconds)],
                    );
                }
            }
            1 => {
                self.frames.push(dt * 1000.0);
                if tick == self.last_tick {
                    self.repeated += 1;
                }
                self.draw_ms += work * 1000.0;
                let ticks = tick.saturating_sub(self.last_tick);
                if let Some(l) = self.log.as_mut() {
                    l.add(self.t, dt * 1000.0, work * 1000.0, ticks, alpha);
                }
                self.t += dt;
                if self.t >= self.cfg.seconds {
                    self.phase = 2;
                    ended = true;
                }
            }
            _ => {}
        }
        self.last_tick = tick;
        ended
    }

    /// Schreibt die Bilder (`--measure-frames`) und den Bericht nach `cfg.report` bzw. auf die Standardausgabe.
    pub fn finish(&self, mut r: Report) {
        if let Some(l) = self.log.as_ref() {
            let (b, res) = l.report();
            r.bilder = Some(b);
            if let Err(e) = res {
                log::error(
                    "❌ Bilder der Messung nicht geschrieben",
                    &[("datei", &self.cfg.frames), ("err", &e)],
                );
            }
        }
        log::info(
            "✅ Messung fertig",
            &[
                ("fps", &r.fps_mittel.round()),
                ("low1", &r.fps_1pct_low.round()),
                ("p99_ms", &r.frametime_ms.p99),
                ("draw_ms", &r.rechenzeit_ms),
                ("takt_ms", &r.takt_ms),
                ("gegner", &r.gegner),
            ],
        );
        match write_report(&r, &self.cfg.report) {
            Ok(()) => log::info("💾 Messbericht", &[("datei", &self.cfg.report)]),
            Err(e) => log::error(
                "❌ Messbericht nicht geschrieben",
                &[("datei", &self.cfg.report), ("err", &e)],
            ),
        }
    }
}

/// Schreibt den Bericht als JSON (zwei Leerzeichen Einrückung) nach `path`; leer = Standardausgabe.
fn write_report(r: &Report, path: &str) -> Result<(), String> {
    let text = serde_json::to_string_pretty(r).map_err(|e| e.to_string())?;
    if path.is_empty() {
        println!("{text}");
        return Ok(());
    }
    std::fs::write(path, text).map_err(|e| e.to_string())
}

impl Game {
    /// Baut den Bericht aus den gesammelten Zeiten und dem Stand des Spiels (Go: `report`).
    pub(super) fn report(&self, m: &Meter) -> Report {
        let mut sorted = m.frames.clone();
        sorted.sort_unstable_by(f64::total_cmp);
        let n = sorted.len().max(1);
        let total = sum(&sorted);
        let worst = &sorted[sorted.len() - (sorted.len() / 100).max(1).min(sorted.len())..];
        let worst_mean = if worst.is_empty() {
            0.0
        } else {
            sum(worst) / worst.len() as f64
        };
        let mut r = Report {
            szene: self.args.scene.name(),
            spieler: self.frame.seats.len(),
            ebitengine: engine_name(),
            backend: BACKEND.to_string(),
            fenster: [self.win.dx(), self.win.dy()],
            vsync: true, // --novsync wirkt in Rust nicht (Fifo)
            messdauer_s: total / 1000.0,
            frames: sorted.len(),
            fps_mittel: ratio(sorted.len() as f64 * 1000.0, total),
            fps_1pct_low: ratio(1000.0, worst_mean),
            frametime_ms: stats(&sorted),
            rechenzeit_ms: m.draw_ms / n as f64,
            takte: m.ticks.len(),
            takt_ende: self.frame.tick,
            bilder_ohne_takt: m.repeated,
            ..Report::default()
        };
        if !m.ticks.is_empty() {
            r.takt_ms = sum(&m.ticks) / m.ticks.len() as f64;
            r.takt_max_ms = m.ticks.iter().copied().fold(0.0, f64::max);
        }
        if let Some(lib) = self.parts.art.as_ref().and_then(|l| l.library()) {
            let lib = lib.borrow();
            let b = lib.budget();
            (r.sheets, r.textur_mib_rgba, r.rig_mib_rgba) = (b.images, round1(b.mib()), round1(lib.rig_mib()));
        }
        (r.phase, r.gegner) = world_state(&self.frame);
        let mut gaps = m.gaps.clone();
        gaps.sort_unstable_by(f64::total_cmp);
        r.takt_abstand_ms = stats(&gaps);
        r
    }
}

impl Scene {
    /// Beschreibt die gemessene Szene (Go: `sceneName`).
    pub fn name(&self) -> String {
        format!("seed={} tick={} spieler={}", self.seed, self.tick, self.players)
    }
}

/// Liefert die Tageszeit der Stufe von Spieler 1 und die lebenden Gegner aller Stufen (Go: `worldState`).
pub fn world_state(f: &Frame) -> (String, usize) {
    let (mut phase, mut n) = (String::new(), 0);
    for w in f.stages.iter().filter_map(|st| st.world.as_ref()) {
        if phase.is_empty() {
            phase.clone_from(&w.cycle.phase);
        }
        n += w.enemies.iter().filter(|e| e.hp > 0.0).count();
    }
    (phase, n)
}

/// Name des Grafik-Backends im Bericht.
pub const BACKEND: &str = "wgpu";

/// Name der Plattform samt wgpu-Version (Feld `ebitengine`).
pub fn engine_name() -> String {
    format!("k3c-platform {} wgpu {WGPU_VERSION}", env!("CARGO_PKG_VERSION"))
}

/// Kennwerte aus sortierten Werten (leer = alles 0).
fn stats(sorted: &[f64]) -> FrameStats {
    FrameStats {
        p50: percentile(sorted, 0.5),
        p95: percentile(sorted, 0.95),
        p99: percentile(sorted, 0.99),
        max: sorted.last().copied().unwrap_or(0.0),
    }
}

/// Wert am Rang `round((n-1)*p)` der sortierten Folge (wie Go); leer = 0.
pub fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    sorted[(((sorted.len() - 1) as f64) * p).round() as usize]
}

/// Summe der Werte.
pub fn sum(v: &[f64]) -> f64 {
    v.iter().sum()
}

/// Auf eine Nachkommastelle gerundet (Go: `round1`).
pub fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

/// `a / b`, aber 0 statt unendlich oder NaN (Go bricht dort ab; JSON kennt beides nicht).
fn ratio(a: f64, b: f64) -> f64 {
    if b > 0.0 { a / b } else { 0.0 }
}

#[cfg(test)]
#[path = "measure_tests.rs"]
mod tests;
