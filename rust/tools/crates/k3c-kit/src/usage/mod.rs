//! Nutzungsstatistik der MCP-Aufrufe (Port von `workkit/usage`, B-062): je Tool Laufzeiten, Perzentile aus einem
//! Histogramm, Ausreißer, häufigste Argumente und Fehler, einmal für die Sitzung und einmal für die Gesamtzeit, dazu
//! eine Minuten-Zeitreihe der letzten sieben Tage für die Live-Monitore. Das Modul kennt den Server nicht; die
//! Middleware gibt jeden beendeten Aufruf mit den rohen Argumenten hinein. Nur `std` (Threads), ohne tokio.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, FixedOffset, Local, SecondsFormat, TimeDelta};
use serde::{Deserialize, Deserializer, Serialize};

mod hist;
mod persist;
mod scope;
mod snapshot;

pub use hist::PERCENTILE_ERROR_PCT;
pub use scope::{BASELINE_CALLS, OUTLIER_FACTOR, OUTLIER_FLOOR_MS, SlowCall};
pub use snapshot::{Count, Scope, ToolUsage, Usage};

use persist::Persist;
use scope::ScopeAgg;

#[cfg(test)]
#[path = "persist_tests.rs"]
mod persist_tests;
#[cfg(test)]
#[path = "usage_tests.rs"]
mod usage_tests;

/// Fenster der Minuten-Zeitreihe.
pub const SERIES_SPAN: TimeDelta = TimeDelta::days(7);

/// Uhr des Trackers (Tests stellen sie).
pub type Clock = Arc<dyn Fn() -> DateTime<Local> + Send + Sync>;

/// Ein beendeter Tool-Aufruf.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub at: DateTime<Local>,
    pub tool: String,
    /// Roh, wie vom Client gesendet.
    pub args: String,
    pub duration_ms: f64,
    #[serde(rename = "OK")]
    pub ok: bool,
    pub error: String,
}

/// Zeitform der Oberfläche: Millisekunden, `Z` bei UTC (Go `2006-01-02T15:04:05.000Z07:00`).
fn format_time(at: DateTime<FixedOffset>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Wie Go `json.Unmarshal`: `null` lässt den Nullwert stehen.
fn nullable<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(de)?.unwrap_or_default())
}

/// Ein Eintrag der Zeitreihe; `t` zählt Minuten seit 1970.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Minute {
    #[serde(deserialize_with = "nullable")]
    t: i64,
    #[serde(deserialize_with = "nullable")]
    calls: BTreeMap<String, i64>,
    #[serde(deserialize_with = "nullable")]
    errors: i64,
    #[serde(deserialize_with = "nullable")]
    ms: BTreeMap<String, f64>,
    #[serde(rename = "maxMs", deserialize_with = "nullable")]
    max_ms: BTreeMap<String, f64>,
    #[serde(deserialize_with = "nullable")]
    outliers: BTreeMap<String, i64>,
}

/// Eine Minute der Zeitreihe im Schnappschuss; `ts` ist der Minutenanfang in ms seit 1970.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    pub ts: i64,
    pub calls: BTreeMap<String, i64>,
    pub errors: i64,
    pub ms: BTreeMap<String, f64>,
    #[serde(rename = "maxMs")]
    pub max_ms: BTreeMap<String, f64>,
    pub outliers: BTreeMap<String, i64>,
}

struct Inner {
    session: ScopeAgg,
    all_time: ScopeAgg,
    /// Aufsteigend nach `t`, ohne leere Minuten.
    minutes: Vec<Minute>,
}

impl Inner {
    /// Liefert die Minute `m` und legt sie an der richtigen Stelle an. Zwei parallele Aufrufe an der Minutenwende
    /// kommen in beliebiger Reihenfolge an, die Liste bleibt trotzdem aufsteigend und ohne Doppel.
    fn minute_for(&mut self, m: i64) -> &mut Minute {
        let mut i = self.minutes.len();
        while i > 0 && self.minutes[i - 1].t > m {
            i -= 1;
        }
        if i > 0 && self.minutes[i - 1].t == m {
            return &mut self.minutes[i - 1];
        }
        self.minutes.insert(
            i,
            Minute {
                t: m,
                ..Minute::default()
            },
        );
        &mut self.minutes[i]
    }

    fn add_minute(&mut self, e: &Event, outlier: bool) {
        let b = self.minute_for(e.at.timestamp() / 60);
        *b.calls.entry(e.tool.clone()).or_default() += 1;
        *b.ms.entry(e.tool.clone()).or_default() += e.duration_ms;
        let max = b.max_ms.entry(e.tool.clone()).or_default();
        *max = max.max(e.duration_ms);
        if !e.ok {
            b.errors += 1;
        }
        if outlier {
            *b.outliers.entry(e.tool.clone()).or_default() += 1;
        }
    }

    /// Verwirft Minuten, die älter als [`SERIES_SPAN`] sind.
    fn prune(&mut self, now: DateTime<Local>) {
        let oldest = (now - SERIES_SPAN).timestamp() / 60;
        let keep = self.minutes.iter().take_while(|m| m.t < oldest).count();
        self.minutes.drain(..keep);
    }
}

/// Hält beide Bereiche und die Zeitreihe; alle Methoden sind nebenläufig sicher.
pub struct Tracker {
    now: Clock,
    inner: Mutex<Inner>,
    /// `None` = nur im Speicher ([`Tracker::new`]).
    persist: Option<Persist>,
}

impl Tracker {
    /// Leerer Tracker nur im Speicher. Mit Datei: [`Tracker::open`].
    pub fn new(now: Clock) -> Tracker {
        let start = now().fixed_offset();
        let inner = Inner {
            session: ScopeAgg::new(start),
            all_time: ScopeAgg::new(start),
            minutes: Vec::new(),
        };
        Tracker {
            now,
            inner: Mutex::new(inner),
            persist: None,
        }
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Zählt einen beendeten Aufruf. Der Vergleichswert für den Ausreißer entsteht vor dem Eintragen und stammt aus
    /// der Gesamtzeit, die mehr Aufrufe kennt als die Sitzung.
    pub fn record(&self, e: Event) {
        let args = hist::norm_args(&e.args);
        let mut inner = self.lock();
        let base = inner.all_time.baseline(&e.tool, &args);
        let outlier = base > 0.0 && e.duration_ms > OUTLIER_FACTOR * base && e.duration_ms >= OUTLIER_FLOOR_MS;
        let slow = SlowCall {
            at: format_time(e.at.fixed_offset()),
            tool: e.tool.clone(),
            args: args.clone(),
            duration_ms: e.duration_ms,
            ok: e.ok,
            baseline_ms: base,
        };
        inner.session.add(&e, &args, &slow, outlier);
        inner.all_time.add(&e, &args, &slow, outlier);
        inner.add_minute(&e, outlier);
        inner.prune((self.now)());
        self.schedule_save();
    }

    /// Beide Bereiche und die Zeitreihe; alle Listen sind leer statt `null`.
    pub fn snapshot(&self) -> Usage {
        let mut inner = self.lock();
        inner.prune((self.now)());
        let minutes = inner
            .minutes
            .iter()
            .map(|m| Bucket {
                ts: m.t * 60_000,
                calls: m.calls.clone(),
                errors: m.errors,
                ms: m.ms.clone(),
                max_ms: m.max_ms.clone(),
                outliers: m.outliers.clone(),
            })
            .collect();
        Usage {
            session: inner.session.snapshot(),
            all_time: inner.all_time.snapshot(),
            minutes,
        }
    }
}
