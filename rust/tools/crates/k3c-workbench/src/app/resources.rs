//! Seite Monitoring (Port von `app_resources.go` und `resmon_start.go`): Messwerte des Messgeräts für die Diagramme.
//! Die Seite fragt im Takt der Proben neu, solange sie sichtbar ist.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use k3c_kit::logger::Logger;
use serde::Serialize;

use super::{App, lock};
use crate::resmon::{Config, GaugeInfo, Monitor, Sample, Ticker};

#[cfg(test)]
#[path = "resources_tests.rs"]
mod tests;

/// Takt der Proben; die Seite Monitoring fragt im selben Takt.
const INTERVAL: Duration = Duration::from_secs(10);
/// 24 h bei 10 s (etwa 5 MB).
const HISTORY: usize = 8640;
/// Alle 5 min eine Zeile im Log.
const LOG_EVERY: usize = 30;
/// Punkte je Diagramm (verdichtet mit dem Maximum je Fenster).
const MAX_POINTS: usize = 240;
/// Gewählter Zeitraum in Minuten.
const MIN_WINDOW: i64 = 5;
const MAX_WINDOW: i64 = 24 * 60;
const TOP_OPS: usize = 10;

/// Eine (verdichtete) Probe; `gaugeValues` folgt der Reihenfolge von [`ResourceView::gauges`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResPoint {
    /// Unix-Millisekunden.
    t: i64,
    heap_live: u64,
    /// Go „von der Laufzeit gehalten“; ohne Go-Laufzeit immer 0.
    total: u64,
    private: u64,
    rss: u64,
    children_rss: u64,
    children: usize,
    /// Thread-Zahl des Prozesses (die Seite nennt sie noch Goroutinen).
    goroutines: usize,
    handles: usize,
    gauge_values: Vec<i64>,
    /// Ohne Speicherbereinigung immer 0.
    gc_cycles: u64,
}

#[derive(Debug, Serialize)]
pub struct ResGauge {
    name: String,
    bytes: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResOp {
    name: String,
    calls: i64,
    total_ms: f64,
    max_ms: f64,
    alloc: u64,
    max_alloc: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResTrend {
    name: &'static str,
    bytes: bool,
    per_hour: f64,
    window_min: u64,
}

#[derive(Debug, Serialize)]
pub struct ResProc {
    name: String,
    pid: u32,
    rss: u64,
}

/// Alles, was die Seite Monitoring zeichnet. Ohne Messgerät ist `enabled` falsch und die Listen sind `null` (wie Go).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceView {
    enabled: bool,
    started: i64,
    interval_s: u64,
    window_min: i64,
    points: Option<Vec<ResPoint>>,
    gauges: Option<Vec<ResGauge>>,
    ops: Option<Vec<ResOp>>,
    trends: Option<Vec<ResTrend>>,
    children: Option<Vec<ResProc>>,
    /// Heap-Profile gibt es nicht; die Liste bleibt leer.
    profiles: Option<Vec<String>>,
}

fn point(s: &Sample, gauges: &[GaugeInfo]) -> ResPoint {
    ResPoint {
        t: s.at.timestamp_millis(),
        heap_live: s.heap_live,
        total: 0,
        private: s.private,
        rss: s.rss,
        children_rss: s.children_rss,
        children: s.children,
        goroutines: s.threads,
        handles: s.handles,
        gauge_values: gauges
            .iter()
            .map(|g| s.gauges.get(&g.name).copied().unwrap_or_default())
            .collect(),
        gc_cycles: 0,
    }
}

fn millis(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// Die Messwerte der letzten `window_min` Minuten (zwischen 5 min und 24 h).
fn view(monitor: &Monitor, window_min: i64) -> ResourceView {
    let window_min = window_min.clamp(MIN_WINDOW, MAX_WINDOW);
    let window = Duration::from_secs(window_min.unsigned_abs() * 60);
    let gauges = monitor.gauges();
    let points = monitor
        .series(window, MAX_POINTS)
        .iter()
        .map(|s| point(s, &gauges))
        .collect();
    let ops = monitor.ops().into_iter().take(TOP_OPS).map(|o| ResOp {
        name: o.name,
        calls: o.calls,
        total_ms: millis(o.total),
        max_ms: millis(o.max),
        alloc: o.alloc,
        max_alloc: o.max_alloc,
    });
    let trends = monitor.trends().into_iter().map(|t| ResTrend {
        name: t.name,
        bytes: t.bytes,
        per_hour: t.per_hour,
        window_min: t.window.as_secs() / 60,
    });
    let children = monitor.top().into_iter().map(|p| ResProc {
        name: p.name,
        pid: p.pid,
        rss: p.rss,
    });
    ResourceView {
        enabled: true,
        started: monitor.started().timestamp_millis(),
        interval_s: monitor.interval().as_secs(),
        window_min,
        points: Some(points),
        gauges: Some(
            gauges
                .into_iter()
                .map(|g| ResGauge {
                    name: g.name,
                    bytes: g.bytes,
                })
                .collect(),
        ),
        ops: Some(ops.collect()),
        trends: Some(trends.collect()),
        children: Some(children.collect()),
        profiles: Some(Vec::new()),
    }
}

/// Summe der Dateien eines Ordners (ohne Unterordner); ein Fehler zählt als 0.
fn dir_bytes(dir: &Path) -> i64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let sizes = entries
        .flatten()
        .filter_map(|e| e.metadata().ok())
        .filter(std::fs::Metadata::is_file)
        .map(|m| m.len());
    i64::try_from(sizes.sum::<u64>()).unwrap_or(i64::MAX)
}

impl App {
    /// Startet das Messgerät: Proben alle 10 s (Verlauf 24 h), eine Zeile je 5 min im Log (`ns=res`). Muss nach dem
    /// Öffnen des Logs und vor dem MCP-Server laufen.
    pub(super) fn start_resources(&self, logger: &Logger) -> (Arc<Monitor>, Option<Ticker>) {
        let monitor = Monitor::new(Config {
            interval: INTERVAL,
            history: HISTORY,
            log_every: LOG_EVERY,
            log: logger.clone(),
            ..Config::default()
        });
        let logs = self.root.join("logs");
        monitor.gauge("Log-Dateien", true, move || dir_bytes(&logs));
        let ticker = monitor.run();
        if let Err(e) = &ticker {
            logger.warn(
                &format!("❌ ressourcen-takt nicht gestartet: {e}"),
                &[("ns", serde_json::json!("res"))],
            );
        }
        (monitor, ticker.ok())
    }

    fn monitor(&self) -> Option<Arc<Monitor>> {
        lock(&self.running).as_ref().map(|r| Arc::clone(&r.resources))
    }

    /// Misst einen Aufruf der Oberfläche; ohne Messgerät ohne Wirkung.
    pub fn track(&self, name: &str) -> Box<dyn FnOnce() + Send> {
        match self.monitor() {
            Some(monitor) => monitor.track(name),
            None => Box::new(|| {}),
        }
    }

    /// Die Messwerte für die Seite Monitoring; ohne Messgerät ist `enabled` falsch.
    pub fn resources(&self, window_min: i64) -> ResourceView {
        self.wait();
        self.monitor()
            .map_or_else(ResourceView::default, |monitor| view(&monitor, window_min))
    }
}
