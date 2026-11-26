//! Ein Bereich (Sitzung oder Gesamtzeit) und die Ausreißerregel (Port von `scope.go`).

use std::collections::BTreeMap;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize};

use super::hist::{DurAgg, bump, clip};
use super::{Event, nullable};

/// Ausreißer: länger als `OUTLIER_FACTOR` × p95 der Vergleichsgruppe und mindestens `OUTLIER_FLOOR_MS`; eine
/// Vergleichsgruppe zählt ab `BASELINE_CALLS` Aufrufen (B-062). Die Oberfläche zeigt dieselben Werte an.
pub const OUTLIER_FACTOR: f64 = 2.0;
pub const OUTLIER_FLOOR_MS: f64 = 1000.0;
pub const BASELINE_CALLS: i64 = 8;
/// Länge der Listen langsamster Aufrufe und letzter Ausreißer.
pub(super) const SLOW_N: usize = 10;

/// Ein einzelner Aufruf in den Listen langsamster Aufrufe und letzter Ausreißer.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SlowCall {
    #[serde(deserialize_with = "nullable")]
    pub at: String,
    #[serde(deserialize_with = "nullable")]
    pub tool: String,
    #[serde(deserialize_with = "nullable")]
    pub args: String,
    #[serde(rename = "durationMs", deserialize_with = "nullable")]
    pub duration_ms: f64,
    #[serde(deserialize_with = "nullable")]
    pub ok: bool,
    #[serde(rename = "baselineMs", deserialize_with = "nullable")]
    pub baseline_ms: f64,
}

/// Alles zu einem Tool in einem Bereich.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub(super) struct ToolAgg {
    #[serde(flatten)]
    pub dur: DurAgg,
    #[serde(deserialize_with = "nullable")]
    pub errors: i64,
    #[serde(deserialize_with = "nullable")]
    pub outliers: i64,
    #[serde(deserialize_with = "nullable")]
    pub args: BTreeMap<String, i64>,
    #[serde(rename = "argDur", deserialize_with = "without_null")]
    pub arg_dur: BTreeMap<String, DurAgg>,
    #[serde(rename = "errorMsgs", deserialize_with = "nullable")]
    pub error_msgs: BTreeMap<String, i64>,
}

/// Eine Zuordnung, deren `null`-Einträge wegfallen (wie `repair` in Go).
fn without_null<'de, D, T>(de: D) -> Result<BTreeMap<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let raw: Option<BTreeMap<String, Option<T>>> = Option::deserialize(de)?;
    Ok(raw
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect())
}

/// Go-Nullzeit `0001-01-01T00:00:00Z`, wenn eine Datei kein `since` hat.
fn zero_time() -> DateTime<FixedOffset> {
    NaiveDate::from_ymd_opt(1, 1, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .unwrap_or_default()
        .and_utc()
        .fixed_offset()
}

fn since_or_zero<'de, D: Deserializer<'de>>(de: D) -> Result<DateTime<FixedOffset>, D::Error> {
    Ok(Option::deserialize(de)?.unwrap_or_else(zero_time))
}

/// Ein Bereich: die Sitzung oder die Gesamtzeit.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ScopeAgg {
    #[serde(default = "zero_time", deserialize_with = "since_or_zero")]
    pub since: DateTime<FixedOffset>,
    #[serde(default, deserialize_with = "without_null")]
    pub tools: BTreeMap<String, ToolAgg>,
    #[serde(default, deserialize_with = "nullable")]
    pub slowest: Vec<SlowCall>,
    /// Neueste zuerst.
    #[serde(default, deserialize_with = "nullable")]
    pub outliers: Vec<SlowCall>,
}

impl ScopeAgg {
    pub(super) fn new(since: DateTime<FixedOffset>) -> ScopeAgg {
        ScopeAgg {
            since,
            tools: BTreeMap::new(),
            slowest: Vec::new(),
            outliers: Vec::new(),
        }
    }

    /// p95 der Vergleichsgruppe: gleiches Tool mit gleichen Argumenten ab `BASELINE_CALLS` Aufrufen, sonst das Tool
    /// ab `BASELINE_CALLS`, sonst 0 (nicht bewerten).
    pub(super) fn baseline(&self, tool: &str, args: &str) -> f64 {
        let Some(t) = self.tools.get(tool) else {
            return 0.0;
        };
        if let Some(d) = t.arg_dur.get(args).filter(|d| d.calls >= BASELINE_CALLS) {
            return d.p(0.95);
        }
        if t.dur.calls >= BASELINE_CALLS {
            return t.dur.p(0.95);
        }
        0.0
    }

    pub(super) fn add(&mut self, e: &Event, args: &str, slow: &SlowCall, outlier: bool) {
        let t = self.tools.entry(e.tool.clone()).or_default();
        t.dur.add(e.duration_ms);
        let key = bump(&mut t.args, args);
        t.arg_dur.entry(key).or_default().add(e.duration_ms);
        if !e.ok {
            t.errors += 1;
            bump(&mut t.error_msgs, &clip(&e.error));
        }
        if outlier {
            t.outliers += 1;
            self.outliers.insert(0, slow.clone());
            self.outliers.truncate(SLOW_N);
        }
        insert_slowest(&mut self.slowest, slow);
    }
}

/// Hält die `SLOW_N` langsamsten Aufrufe, längste zuerst; bei Gleichstand bleibt der frühere vorn.
fn insert_slowest(list: &mut Vec<SlowCall>, call: &SlowCall) {
    let i = list.partition_point(|c| c.duration_ms >= call.duration_ms);
    if i >= SLOW_N {
        return;
    }
    list.insert(i, call.clone());
    list.truncate(SLOW_N);
}
