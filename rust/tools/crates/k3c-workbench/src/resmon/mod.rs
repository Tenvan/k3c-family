//! Ressourcenverbrauch der Workbench über Stunden (Port von `internal/resmon`): Speicher (Heap, Prozess,
//! Kindprozesse), Threads, Handles und die Größe der eigenen Puffer. Ein Messgerät je Prozess nimmt in festem Takt
//! Proben, hält einen begrenzten Verlauf und schreibt in großen Abständen eine Zeile ins Log. Je Vorgang merkt es Dauer
//! und Allokation, damit ein Dauerläufer auffällt, bevor der Speicher voll ist.
//!
//! Anders als Go: eigener Zähler im Allocator statt der Go-Laufzeit, Threads statt Goroutinen, keine GC-Läufe und
//! keine Heap-Profile (darum auch keine Schwelle für den Live-Heap).

mod alloc;
mod ops;
mod proc;
mod report;
mod series;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, mpsc};
use std::time::Duration;

use chrono::{DateTime, Local};
use k3c_kit::logger::Logger;
use serde_json::json;

pub use ops::Op;
pub use proc::ProcInfo;
pub use series::{GaugeInfo, TrendInfo};

use report::{GROWTH_WARN, format_bytes, summary, trend_of};

/// Renderer-Arbeitssatz, ab dem eine Warnung erscheint (B-895); danach je 50 % mehr. Darüber wird die Oberfläche träge
/// und bleibt nach einiger Zeit leer.
const DEFAULT_WEB_VIEW_WARN: u64 = 500 << 20;
const WEB_VIEW_PROCESS: &str = "msedgewebview2.exe";
/// Threads, ab denen die erste Warnung erscheint; danach beim Doppelten.
const THREADS_WARN: usize = 2000;
/// So viele Kindprozesse nennt der Bericht.
const TOP: usize = 5;

/// Uhr des Messgeräts.
pub type Clock = Arc<dyn Fn() -> DateTime<Local> + Send + Sync>;

/// Liest einen Messwert; muss schnell sein.
type GaugeFn = Arc<dyn Fn() -> i64 + Send + Sync>;

/// Beschreibung eines Messgeräts.
#[derive(Clone)]
pub struct Config {
    pub interval: Duration,
    /// Proben im Verlauf.
    pub history: usize,
    /// Jede n-te Probe steht im Log.
    pub log_every: usize,
    pub web_view_warn: u64,
    pub log: Logger,
    pub now: Clock,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            interval: Duration::from_secs(30),
            history: 2880,
            log_every: 10,
            web_view_warn: DEFAULT_WEB_VIEW_WARN,
            log: Logger::discard(),
            now: Arc::new(Local::now),
        }
    }
}

/// Eine Probe.
#[derive(Clone, Debug, Default)]
pub struct Sample {
    pub at: DateTime<Local>,
    /// Belegte Heap-Bytes.
    pub heap_live: u64,
    /// Privater Speicher des Prozesses (Windows: Commit).
    pub private: u64,
    /// Arbeitssatz des Prozesses.
    pub rss: u64,
    pub children: usize,
    pub children_rss: u64,
    /// Größter WebView2-Prozess unter den Kindern: der Renderer der Oberfläche (0 ohne Fenster).
    pub web_view: u64,
    pub threads: usize,
    pub handles: usize,
    pub gauges: BTreeMap<String, i64>,
}

#[derive(Clone)]
struct Gauge {
    name: String,
    bytes: bool,
    read: GaugeFn,
}

struct State {
    gauges: Vec<Gauge>,
    history: VecDeque<Sample>,
    ops: HashMap<String, Op>,
    next_threads: usize,
    next_web_view: u64,
    samples: usize,
    top: Vec<ProcInfo>,
}

/// Das Messgerät. Alle Methoden sind nebenläufig sicher.
pub struct Monitor {
    config: Config,
    now: Clock,
    started: DateTime<Local>,
    state: Mutex<State>,
}

/// Der laufende Takt; er endet beim Fallenlassen.
pub struct Ticker {
    _stop: mpsc::Sender<()>,
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Monitor {
    /// Legt ein Messgerät an; den Takt startet [`Monitor::run`].
    pub fn new(config: Config) -> Arc<Monitor> {
        let state = State {
            gauges: Vec::new(),
            history: VecDeque::new(),
            ops: HashMap::new(),
            next_threads: THREADS_WARN,
            next_web_view: config.web_view_warn,
            samples: 0,
            top: Vec::new(),
        };
        Arc::new(Monitor {
            now: Arc::clone(&config.now),
            started: (config.now)(),
            config,
            state: Mutex::new(state),
        })
    }

    /// Meldet einen Messwert an, der bei jeder Probe gelesen wird (Größe eines Puffers, Zahl der Läufe); `bytes`
    /// zeigt ihn als Speichergröße.
    pub fn gauge(&self, name: &str, bytes: bool, read: impl Fn() -> i64 + Send + Sync + 'static) {
        lock(&self.state).gauges.push(Gauge {
            name: name.to_owned(),
            bytes,
            read: Arc::new(read),
        });
    }

    /// Abstand der Proben.
    pub fn interval(&self) -> Duration {
        self.config.interval
    }

    /// Beginn der Messung.
    pub fn started(&self) -> DateTime<Local> {
        self.started
    }

    /// Nimmt in einem eigenen Thread Proben, die erste sofort.
    pub fn run(self: &Arc<Self>) -> std::io::Result<Ticker> {
        let (stop, stopped) = mpsc::channel::<()>();
        let monitor = Arc::clone(self);
        std::thread::Builder::new()
            .name("ressourcen".to_owned())
            .spawn(move || {
                loop {
                    monitor.sample();
                    if stopped.recv_timeout(monitor.config.interval) != Err(mpsc::RecvTimeoutError::Timeout) {
                        return;
                    }
                }
            })?;
        Ok(Ticker { _stop: stop })
    }

    /// Liest Prozess und Messwerte. Die Messwerte laufen außerhalb der Sperre: sie nehmen eigene Sperren.
    fn read(&self) -> Sample {
        let at = (self.now)();
        let heap_live = alloc::live_bytes();
        let process = proc::read();
        let web_view = process
            .children
            .iter()
            .filter(|c| c.name.eq_ignore_ascii_case(WEB_VIEW_PROCESS))
            .map(|c| c.rss)
            .max()
            .unwrap_or_default();
        let mut top = process.children.clone();
        top.sort_by_key(|p| std::cmp::Reverse(p.rss));
        top.truncate(TOP);
        let gauges = {
            let mut state = lock(&self.state);
            state.top = top;
            state.gauges.clone()
        };
        Sample {
            at,
            heap_live,
            private: process.private,
            rss: process.rss,
            children: process.children.len(),
            children_rss: process.children.iter().map(|c| c.rss).sum(),
            web_view,
            threads: process.threads,
            handles: process.handles,
            gauges: gauges.iter().map(|g| (g.name.clone(), (g.read)())).collect(),
        }
    }

    fn keep(&self, sample: Sample) -> bool {
        let mut state = lock(&self.state);
        state.history.push_back(sample);
        while state.history.len() > self.config.history.max(1) {
            state.history.pop_front();
        }
        state.samples += 1;
        state.samples == 1 || state.samples.is_multiple_of(self.config.log_every.max(1))
    }

    /// Nimmt eine Probe, hängt sie an den Verlauf und prüft die Schwellen.
    pub fn sample(&self) -> Sample {
        let sample = self.read();
        if self.keep(sample.clone()) {
            let fields = [
                ("ns", json!("res")),
                ("heap_live", json!(sample.heap_live)),
                ("rss", json!(sample.rss)),
                ("private", json!(sample.private)),
                ("children_rss", json!(sample.children_rss)),
                ("threads", json!(sample.threads)),
                ("handles", json!(sample.handles)),
            ];
            let message = format!("💓 ressourcen: {}", summary(&sample));
            self.config.log.info(&message, &fields);
        }
        self.check_thresholds(&sample);
        sample
    }

    /// Meldet große Renderer, viele Threads und stetiges Wachstum. Jede Schwelle gilt einmal und steigt danach.
    fn check_thresholds(&self, s: &Sample) {
        let (web_view_limit, threads_hit) = {
            let mut state = lock(&self.state);
            let limit = state.next_web_view;
            let web_view_limit = (s.web_view >= limit).then_some(limit);
            if web_view_limit.is_some() {
                state.next_web_view = s.web_view + s.web_view / 2;
            }
            let threads_hit = s.threads >= state.next_threads;
            if threads_hit {
                state.next_threads = s.threads * 2;
            }
            (web_view_limit, threads_hit)
        };
        let log = &self.config.log;
        if let Some(limit) = web_view_limit {
            let message = format!(
                "🤒 webview-renderer über der grenze: {} (grenze {})",
                format_bytes(s.web_view),
                format_bytes(limit)
            );
            let fields = [
                ("ns", json!("res")),
                ("webview_rss", json!(s.web_view)),
                ("limit", json!(limit)),
            ];
            log.warn(&message, &fields);
        }
        if threads_hit {
            let fields = [("ns", json!("res")), ("threads", json!(s.threads))];
            log.warn(&format!("🤒 {} threads", s.threads), &fields);
        }
        let growth = trend_of(&self.history(), |s| s.heap_live as f64);
        if let Some(g) = growth.filter(|g| g.per_hour >= GROWTH_WARN) {
            let message = format!(
                "🤒 live-heap wächst stetig: {}/h über {}",
                format_bytes(g.per_hour as u64),
                crate::services::go_duration(g.window)
            );
            let fields = [
                ("ns", json!("res")),
                ("heap_live", json!(s.heap_live)),
                ("per_hour", json!(g.per_hour as i64)),
            ];
            log.warn(&message, &fields);
        }
    }

    /// Der Verlauf, älteste zuerst.
    pub fn history(&self) -> Vec<Sample> {
        lock(&self.state).history.iter().cloned().collect()
    }

    /// Die letzte Probe.
    pub fn latest(&self) -> Option<Sample> {
        lock(&self.state).history.back().cloned()
    }
}
