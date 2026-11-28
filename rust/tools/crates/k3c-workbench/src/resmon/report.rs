//! Bericht und Trend (Port von `report.go`).

use std::time::Duration;

use super::{Monitor, Op, Sample, lock};
use crate::services::go_duration;

/// Wachstum des Live-Heaps je Stunde, ab dem ein Warnhinweis ins Log kommt.
pub(super) const GROWTH_WARN: f64 = (200_u32 << 20) as f64;
/// Kürzeste Spanne, über die ein Wachstum gilt (kürzer ist es Rauschen).
const GROWTH_MIN: Duration = Duration::from_secs(30 * 60);
/// Spanne, über die der Trend gerechnet wird.
const GROWTH_WINDOW: Duration = Duration::from_secs(2 * 60 * 60);

#[derive(Debug)]
pub(super) struct Trend {
    pub per_hour: f64,
    pub window: Duration,
}

/// Eine Leitgröße des Trends (Bericht und Ansicht).
pub(super) struct TrendSpec {
    pub name: &'static str,
    pub bytes: bool,
    pub value: fn(&Sample) -> f64,
}

/// Die Namen sind die Schlüssel der Seite Monitoring; sie nennt die Threads noch `Goroutinen`.
pub(super) const TREND_SPECS: &[TrendSpec] = &[
    TrendSpec {
        name: "Live-Heap",
        bytes: true,
        value: |s| s.heap_live as f64,
    },
    TrendSpec {
        name: "Privat",
        bytes: true,
        value: |s| s.private as f64,
    },
    TrendSpec {
        name: "Kinder-RSS",
        bytes: true,
        value: |s| s.children_rss as f64,
    },
    TrendSpec {
        name: "Goroutinen",
        bytes: false,
        value: |s| s.threads as f64,
    },
];

fn between(later: &Sample, earlier: &Sample) -> Duration {
    (later.at - earlier.at).to_std().unwrap_or_default()
}

/// Steigung der Ausgleichsgeraden über die letzten [`GROWTH_WINDOW`] (mindestens [`GROWTH_MIN`]); ein Sägezahn
/// verfälscht sie weniger als der Vergleich von Anfang und Ende.
pub(super) fn trend_of(history: &[Sample], value: fn(&Sample) -> f64) -> Option<Trend> {
    if history.len() < 3 {
        return None;
    }
    let last = history.last()?;
    let recent = history.get(history.partition_point(|s| between(last, s) > GROWTH_WINDOW)..)?;
    let first = recent.first()?;
    let span = between(last, first);
    if recent.len() < 3 || span < GROWTH_MIN {
        return None;
    }
    let (mut sx, mut sy, mut sxx, mut sxy) = (0.0, 0.0, 0.0, 0.0);
    for s in recent {
        let (x, y) = (between(s, first).as_secs_f64() / 3600.0, value(s));
        (sx, sy, sxx, sxy) = (sx + x, sy + y, sxx + x * x, sxy + x * y);
    }
    let n = recent.len() as f64;
    let denominator = n * sxx - sx * sx;
    if denominator == 0.0 {
        return None;
    }
    // Auf Minuten gerundet wie Go `Round`.
    let minutes = (span.as_secs() + 30) / 60;
    Some(Trend {
        per_hour: (n * sxy - sx * sy) / denominator,
        window: Duration::from_secs(minutes * 60),
    })
}

pub(super) fn format_bytes(n: u64) -> String {
    match n {
        n if n >= 1 << 30 => format!("{:.2} GiB", n as f64 / f64::from(1_u32 << 30)),
        n if n >= 1 << 20 => format!("{:.1} MiB", n as f64 / f64::from(1_u32 << 20)),
        n if n >= 1 << 10 => format!("{:.0} KiB", n as f64 / 1024.0),
        n => format!("{n} B"),
    }
}

fn signed_bytes(v: f64) -> String {
    let sign = if v < 0.0 { '-' } else { '+' };
    format!("{sign}{}", format_bytes(v.abs() as u64))
}

pub(super) fn summary(s: &Sample) -> String {
    let mut out = format!(
        "heap live {} · RSS {} · privat {} · Kinder {} ({}) · Threads {} · Handles {}",
        format_bytes(s.heap_live),
        format_bytes(s.rss),
        format_bytes(s.private),
        s.children,
        format_bytes(s.children_rss),
        s.threads,
        s.handles
    );
    if s.web_view > 0 {
        out.push_str(&format!(" · Renderer {}", format_bytes(s.web_view)));
    }
    out
}

fn op_lines(ops: &[Op], limit: usize) -> Vec<String> {
    if ops.is_empty() {
        return Vec::new();
    }
    let round = |d: Duration| go_duration(Duration::from_millis(((d.as_micros() + 500) / 1000) as u64));
    let mut out = vec!["Vorgänge nach Allokation (Aufrufe · Σ Allokation · Ø · max · Σ Dauer · max Dauer):".to_owned()];
    out.extend(ops.iter().take(limit).map(|o| {
        format!(
            "  {} · {} · {} · {} · {} · {} · {}",
            o.name,
            o.calls,
            format_bytes(o.alloc),
            format_bytes(o.alloc / o.calls.max(1) as u64),
            format_bytes(o.max_alloc),
            round(o.total),
            round(o.max)
        )
    }));
    out
}

impl Monitor {
    fn trend_lines(&self) -> Vec<String> {
        let out: Vec<String> = self
            .trends()
            .iter()
            .map(|t| {
                // Im Bericht heißt die Größe, was sie zählt.
                let name = if t.name == "Goroutinen" { "Threads" } else { t.name };
                let per_hour = if t.bytes {
                    signed_bytes(t.per_hour)
                } else {
                    format!("{:+.1}", t.per_hour)
                };
                format!("Trend {name}: {per_hour}/h über {}", go_duration(t.window))
            })
            .collect();
        if out.is_empty() {
            return vec![format!("Trend: erst ab {} Messzeit", go_duration(GROWTH_MIN))];
        }
        out
    }

    fn gauge_line(&self, current: &Sample) -> Option<String> {
        let parts: Vec<String> = self
            .gauges()
            .iter()
            .map(|g| {
                let value = current.gauges.get(&g.name).copied().unwrap_or_default();
                if g.bytes {
                    format!("{} {}", g.name, format_bytes(value.max(0) as u64))
                } else {
                    format!("{} {value}", g.name)
                }
            })
            .collect();
        (!parts.is_empty()).then(|| format!("Puffer: {}", parts.join(" · ")))
    }

    /// Bericht für `workbench_resources`: aktuelle Werte, Trend, Messwerte der Puffer und die größten Vorgänge nach
    /// Allokation. Ohne frühere Probe misst er selbst (blockiert wie [`Monitor::sample`]).
    pub fn report(&self) -> String {
        let current = self.latest().unwrap_or_else(|| self.sample());
        let (count, top) = {
            let state = lock(&self.state);
            (state.history.len(), state.top.clone())
        };
        let mut out = vec![
            format!(
                "Ressourcen · Messgerät seit {} · {count} Proben (alle {})",
                self.started().format("%d.%m. %H:%M:%S"),
                go_duration(self.interval())
            ),
            summary(&current),
        ];
        out.extend(self.trend_lines());
        if !top.is_empty() {
            let parts: Vec<String> = top
                .iter()
                .map(|p| format!("{}({}) {}", p.name, p.pid, format_bytes(p.rss)))
                .collect();
            out.push(format!("Kinder nach RSS: {}", parts.join(" · ")));
        }
        out.extend(self.gauge_line(&current));
        out.extend(op_lines(&self.ops(), 8));
        out.join("\n")
    }
}
