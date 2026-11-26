//! Auswertung für die Oberfläche (Port von `snapshot.go`; Form: `src/api/types.ts` der Workbench).

use serde::{Deserialize, Serialize};

use super::Bucket;
use super::format_time;
use super::hist::DurAgg;
use super::scope::{ScopeAgg, ToolAgg};

/// Länge der Ranglisten.
pub(super) const TOP_N: usize = 10;

fn is_zero(v: &f64) -> bool {
    *v == 0.0
}

/// Eintrag einer Rangliste: ein Argument (mit Laufzeiten) oder eine Fehlermeldung (mit Tool).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Count {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tool: String,
    pub value: String,
    pub count: i64,
    #[serde(rename = "avgMs", skip_serializing_if = "is_zero")]
    pub avg_ms: f64,
    #[serde(rename = "p95Ms", skip_serializing_if = "is_zero")]
    pub p95_ms: f64,
    #[serde(rename = "maxMs", skip_serializing_if = "is_zero")]
    pub max_ms: f64,
}

/// Auswertung eines Tools in einem Bereich.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolUsage {
    pub name: String,
    pub calls: i64,
    pub errors: i64,
    pub avg_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub max_ms: f64,
    pub sum_ms: f64,
    pub outliers: i64,
    pub args: Vec<Count>,
    pub top_errors: Vec<Count>,
}

/// Auswertung eines Bereichs über alle Tools.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    pub since: String,
    pub calls: i64,
    pub errors: i64,
    pub avg_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub max_ms: f64,
    pub sum_ms: f64,
    pub outliers: i64,
    pub tools: Vec<ToolUsage>,
    pub top_errors: Vec<Count>,
    pub recent_outliers: Vec<super::SlowCall>,
    pub slowest: Vec<super::SlowCall>,
}

/// Schnappschuss für die Oberfläche: beide Bereiche und die Minuten-Zeitreihe.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub session: Scope,
    pub all_time: Scope,
    pub minutes: Vec<Bucket>,
}

impl ScopeAgg {
    pub(super) fn snapshot(&self) -> Scope {
        let mut all = DurAgg::default();
        let (mut errors, mut outliers) = (0, 0);
        let mut tools = Vec::new();
        let mut top_errors = Vec::new();
        for (name, t) in &self.tools {
            all.merge(&t.dur);
            errors += t.errors;
            outliers += t.outliers;
            tools.push(t.usage(name));
            top_errors.extend(t.error_msgs.iter().map(|(msg, n)| Count {
                tool: name.clone(),
                value: msg.clone(),
                count: *n,
                ..Count::default()
            }));
        }
        tools.sort_by(|a, b| b.calls.cmp(&a.calls).then_with(|| a.name.cmp(&b.name)));
        Scope {
            since: format_time(self.since),
            calls: all.calls,
            errors,
            avg_ms: all.avg(),
            p50_ms: all.p(0.5),
            p95_ms: all.p(0.95),
            max_ms: all.max_ms,
            sum_ms: all.sum_ms,
            outliers,
            tools,
            top_errors: top(top_errors),
            recent_outliers: self.outliers.clone(),
            slowest: self.slowest.clone(),
        }
    }
}

impl ToolAgg {
    fn usage(&self, name: &str) -> ToolUsage {
        let args = self.args.iter().map(|(value, n)| {
            let mut c = Count {
                value: value.clone(),
                count: *n,
                ..Count::default()
            };
            if let Some(d) = self.arg_dur.get(value) {
                (c.avg_ms, c.p95_ms, c.max_ms) = (d.avg(), d.p(0.95), d.max_ms);
            }
            c
        });
        let errors = self.error_msgs.iter().map(|(msg, n)| Count {
            value: msg.clone(),
            count: *n,
            ..Count::default()
        });
        ToolUsage {
            name: name.to_owned(),
            calls: self.dur.calls,
            errors: self.errors,
            avg_ms: self.dur.avg(),
            p50_ms: self.dur.p(0.5),
            p95_ms: self.dur.p(0.95),
            max_ms: self.dur.max_ms,
            sum_ms: self.dur.sum_ms,
            outliers: self.outliers,
            args: top(args.collect()),
            top_errors: top(errors.collect()),
        }
    }
}

/// Sortiert absteigend nach Anzahl, bei Gleichstand nach Tool und Wert (wie Go als ein Text, danach nach Tool, damit
/// die Reihenfolge eindeutig ist), und kürzt auf `TOP_N`.
fn top(mut list: Vec<Count>) -> Vec<Count> {
    list.sort_by(|a, b| {
        let joined = |c: &Count| format!("{}{}", c.tool, c.value);
        b.count
            .cmp(&a.count)
            .then_with(|| joined(a).cmp(&joined(b)))
            .then_with(|| a.tool.cmp(&b.tool))
    });
    list.truncate(TOP_N);
    list
}
