//! Werte für die Diagramme der Seite Monitoring (Port von `series.go`).

use std::time::Duration;

use super::report::TREND_SPECS;
use super::{Monitor, ProcInfo, Sample, lock};

/// Ein angemeldeter Messwert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GaugeInfo {
    pub name: String,
    /// Bytes statt Zähler.
    pub bytes: bool,
}

/// Steigung einer Messgröße je Stunde über `window`.
#[derive(Clone, Debug)]
pub struct TrendInfo {
    pub name: &'static str,
    pub bytes: bool,
    pub per_hour: f64,
    pub window: Duration,
}

/// Fasst Proben zu einer zusammen: Maximum jeder Größe, Zeit der letzten.
fn peak(bucket: &[Sample]) -> Option<Sample> {
    let mut out = Sample {
        at: bucket.last()?.at,
        ..Sample::default()
    };
    for s in bucket {
        out.heap_live = out.heap_live.max(s.heap_live);
        out.private = out.private.max(s.private);
        out.rss = out.rss.max(s.rss);
        out.children_rss = out.children_rss.max(s.children_rss);
        out.children = out.children.max(s.children);
        out.threads = out.threads.max(s.threads);
        out.handles = out.handles.max(s.handles);
        for (name, value) in &s.gauges {
            let kept = out.gauges.entry(name.clone()).or_insert(0);
            *kept = (*kept).max(*value);
        }
    }
    Some(out)
}

impl Monitor {
    /// Die angemeldeten Messwerte in Anmelde-Reihenfolge.
    pub fn gauges(&self) -> Vec<GaugeInfo> {
        lock(&self.state)
            .gauges
            .iter()
            .map(|g| GaugeInfo {
                name: g.name.clone(),
                bytes: g.bytes,
            })
            .collect()
    }

    /// Die Kindprozesse mit dem größten Arbeitssatz der letzten Probe.
    pub fn top(&self) -> Vec<ProcInfo> {
        lock(&self.state).top.clone()
    }

    /// Trend der vier Leitgrößen; eine Größe ohne genug Messzeit fehlt.
    pub fn trends(&self) -> Vec<TrendInfo> {
        let history = self.history();
        TREND_SPECS
            .iter()
            .filter_map(|spec| {
                let trend = super::report::trend_of(&history, spec.value)?;
                Some(TrendInfo {
                    name: spec.name,
                    bytes: spec.bytes,
                    per_hour: trend.per_hour,
                    window: trend.window,
                })
            })
            .collect()
    }

    /// Die Proben der letzten `window`, auf höchstens `max_points` verdichtet: je Zeitfenster der größte Wert jeder
    /// Größe (ein Ausschlag bleibt sichtbar), der Zeitpunkt ist der der letzten Probe im Fenster.
    pub fn series(&self, window: Duration, max_points: usize) -> Vec<Sample> {
        let history = self.history();
        let Some(last) = history.last().map(|s| s.at) else {
            return Vec::new();
        };
        if max_points == 0 {
            return Vec::new();
        }
        let from = last - chrono::TimeDelta::from_std(window).unwrap_or(chrono::TimeDelta::MAX);
        let kept = history
            .get(history.partition_point(|s| s.at < from)..)
            .unwrap_or_default();
        let step = kept.len().div_ceil(max_points);
        if step <= 1 {
            return kept.to_vec();
        }
        kept.chunks(step).filter_map(peak).collect()
    }
}
