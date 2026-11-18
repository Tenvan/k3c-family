//! Port von `native/k3c-game/devtools/chronicle.go`.
//!
//! Chronik des laufenden Spiels (B-889) als `view::layer::Listener`: knappe Logs nur bei Wechseln und als
//! Zusammenfassung, nie je Takt. Tageszeit, Welle, Bau fertig, Burgfall, Stufenwechsel je Spieler; Kampf je Nacht und
//! Stufe; Gold je Spieler und Tag; Takt über Budget (`data/native.json`, höchstens alle `slowLogEverySeconds`). Am Ende
//! schreibt `report.rs` die Sitzungszusammenfassung. Die Zahlen kommen aus Welt und Ereignissen des Frames; nicht im
//! Aufnahme-Modus.
//!
//! Abweichungen von Go: `*slog.Logger` und `now func() time.Time` sind `Box<dyn LogSink>` und `Box<dyn Fn() -> Zeit>`
//! (Tests ersetzen beide); die Zeit ist UTC (Go: Ortszeit) und kommt aus `std::time::SystemTime`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::rc::Rc;

use serde_json::Value;

use super::chronicle_stage::NightStats;
use super::report::Zeit;
use crate::art::Library;
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Frame, Outcome};
use crate::view::layer::Listener;
use crate::view::roles::Finisher;

/// Wert bei fehlendem oder unzulässigem `stepBudgetMs` (ms).
const DEFAULT_STEP_BUDGET_MS: f64 = 4.0;
/// Wert bei fehlendem `slowLogEverySeconds` (s).
const DEFAULT_SLOW_LOG_EVERY_SECONDS: f64 = 5.0;

/// LogSink nimmt die Meldungen der Chronik (Go: `*slog.Logger`). Jede Meldung beginnt mit einem Emoji.
pub trait LogSink {
    fn info(&self, msg: &str, fields: &[(&str, String)]);
    fn warn(&self, msg: &str, fields: &[(&str, String)]);
}

/// PlatformSink schreibt über `k3c_platform::log` (Standard, Go: `slog.Default`).
pub struct PlatformSink;

fn as_display<'a>(fields: &'a [(&'a str, String)]) -> Vec<(&'a str, &'a dyn Display)> {
    fields.iter().map(|(k, v)| (*k, v as &dyn Display)).collect()
}

impl LogSink for PlatformSink {
    fn info(&self, msg: &str, fields: &[(&str, String)]) {
        k3c_platform::log::info(msg, &as_display(fields));
    }

    fn warn(&self, msg: &str, fields: &[(&str, String)]) {
        k3c_platform::log::warn(msg, &as_display(fields));
    }
}

/// NativeValues sind die Werte aus `data/native.json`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeValues {
    pub step_budget_ms: f64,
    pub slow_log_every_seconds: f64,
}

/// load_native liest `data/native.json`; fehlt ein Wert: 4 ms bzw. 5 s. Eine nicht lesbare Datei bleibt still (wie Go),
/// eine ungültige loggt `💥` und gibt die Standardwerte.
pub fn load_native(log: &dyn LogSink) -> NativeValues {
    let mut v = NativeValues {
        step_budget_ms: DEFAULT_STEP_BUDGET_MS,
        slow_log_every_seconds: DEFAULT_SLOW_LOG_EVERY_SECONDS,
    };
    if let Ok(raw) = k3c_data::Source::from_env().read(k3c_data::native::FILE) {
        match k3c_data::native::load(&raw) {
            Ok(n) => {
                v.step_budget_ms = to_f64(n.step_budget_ms);
                v.slow_log_every_seconds = to_f64(n.slow_log_every_seconds);
            }
            Err(e) => log.warn("💥 data/native.json unlesbar, Standardwerte", &[("err", e.to_string())]),
        }
    }
    if v.step_budget_ms <= 0.0 {
        v.step_budget_ms = DEFAULT_STEP_BUDGET_MS;
    }
    if v.slow_log_every_seconds <= 0.0 {
        v.slow_log_every_seconds = DEFAULT_SLOW_LOG_EVERY_SECONDS;
    }
    v
}

#[allow(clippy::cast_precision_loss)] // Werte aus der Datei sind klein
fn to_f64(n: i64) -> f64 {
    n as f64
}

/// Run ist der Zustand einer Sitzung; `Chronicle::begin` setzt ihn mit dem ersten Frame eines Spiels zurück.
#[derive(Default)]
pub(super) struct Run {
    pub(super) active: bool,
    pub(super) start: Zeit,
    pub(super) seed: String,
    pub(super) start_tick: u64,
    pub(super) tick: u64,
    pub(super) first_day: i64,
    pub(super) day: i64,
    /// je Stufe: letzte Tageszeit bzw. Welle
    pub(super) phase: BTreeMap<i64, Value>,
    pub(super) wave: BTreeMap<i64, Value>,
    pub(super) stage: [i64; MAX_PLAYERS],
    pub(super) gold: [i64; MAX_PLAYERS],
    pub(super) income: [i64; MAX_PLAYERS],
    pub(super) spending: [i64; MAX_PLAYERS],
    pub(super) seen: [bool; MAX_PLAYERS],
    pub(super) nights: BTreeMap<i64, NightStats>,
    pub(super) slow_n: u64,
    pub(super) slow_at: u64,
    pub(super) written: bool,
}

/// Chronicle ist die Chronik: Listener und Finisher.
pub struct Chronicle {
    pub(super) log: Box<dyn LogSink>,
    pub(super) now: Box<dyn Fn() -> Zeit>,
    pub(super) dir: String,
    pub(super) vals: NativeValues,
    pub(super) run: Run,
}

impl Chronicle {
    /// with legt die Chronik an; Berichte gehen nach `dir` (leer: keine), Log und Uhr sind ersetzbar (Tests).
    pub fn with(log: Box<dyn LogSink>, now: Box<dyn Fn() -> Zeit>, dir: &str) -> Chronicle {
        let vals = load_native(log.as_ref());
        Chronicle {
            log,
            now,
            dir: dir.to_string(),
            vals,
            run: Run::default(),
        }
    }

    /// begin startet eine Sitzung mit dem ersten Frame eines Spiels.
    fn begin(&mut self, f: &Frame) {
        let mut run = Run {
            active: true,
            start: (self.now)(),
            start_tick: f.tick,
            stage: [-1; MAX_PLAYERS],
            ..Run::default()
        };
        if let Some(w) = f.stages.first().and_then(|s| s.world.as_deref()) {
            run.seed.clone_from(&w.seed);
            run.first_day = w.cycle.day;
            run.day = w.cycle.day;
        }
        self.run = run;
    }
}

/// new_chronicle liefert die Chronik (Log über `k3c_platform::log`, Berichte nach `dir`). Ohne Grafik-Bibliothek (Tests,
/// `--assets`) schreibt sie keinen Bericht.
pub fn new_chronicle(lib: Option<&Library>, dir: &str) -> Chronicle {
    let dir = if lib.is_some() { dir } else { "" };
    Chronicle::with(Box::new(PlatformSink), Box::new(Zeit::now), dir)
}

impl Listener for Chronicle {
    /// Listen schreibt die Chronik mit dem Frame f fort.
    fn listen(&mut self, f: &Frame) {
        if f.recording {
            return;
        }
        if f.stages.is_empty() {
            if self.run.active {
                self.finish_with(outcome_name(f.outcome));
                self.run.active = false;
            }
            return;
        }
        if !self.run.active {
            self.begin(f);
        }
        self.run.start_tick = self.run.start_tick.min(f.tick); // F6 lädt einen älteren Stand
        self.run.tick = f.tick;
        for st in &f.stages {
            self.stage_changes(f, st);
        }
        self.seats(f);
        self.budget(f);
        if f.outcome != Outcome::None && !self.run.written {
            self.finish_with(outcome_name(f.outcome));
        }
    }
}

impl Finisher for Chronicle {
    /// Finish beendet die Sitzung beim Programmende.
    fn finish(&mut self) {
        if self.run.active {
            self.finish_with("beendet");
        }
        self.run.active = false;
    }
}

/// ChronicleHandle teilt die Chronik zwischen den Rollen Listener und Finisher (game meldet beide an).
#[derive(Clone)]
pub struct ChronicleHandle(pub Rc<RefCell<Chronicle>>);

impl Listener for ChronicleHandle {
    fn listen(&mut self, f: &Frame) {
        self.0.borrow_mut().listen(f);
    }
}

impl Finisher for ChronicleHandle {
    fn finish(&mut self) {
        self.0.borrow_mut().finish();
    }
}

/// changed merkt v unter key in m und meldet einen Wechsel (nicht beim ersten Wert).
pub(super) fn changed(m: &mut BTreeMap<i64, Value>, key: i64, v: Value) -> bool {
    let old = m.insert(key, v.clone());
    old.is_some_and(|o| o != v)
}

/// outcome_name ist die Ende-Ursache.
pub fn outcome_name(o: Outcome) -> &'static str {
    match o {
        Outcome::Won => "gewonnen",
        Outcome::Lost => "verloren",
        Outcome::None => "verlassen",
    }
}
