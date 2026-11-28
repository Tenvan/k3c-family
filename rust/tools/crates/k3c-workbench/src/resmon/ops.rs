//! Dauer und Allokation je Vorgang (Port von `ops.go`): Tool-Aufruf oder Aufruf der Oberfläche.

use std::sync::Arc;
use std::time::Duration;

use super::alloc::allocated_bytes;
use super::{Monitor, lock};

/// Verschiedene Vorgangsnamen, danach zählt [`OVERFLOW_OP`].
pub(super) const MAX_OPS: usize = 256;
pub(super) const OVERFLOW_OP: &str = "(weitere)";

/// Ein Vorgang mit seinen Summen.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Op {
    pub name: String,
    pub calls: i64,
    pub total: Duration,
    pub max: Duration,
    /// Angeforderte Bytes insgesamt (prozessweit gemessen, bei Parallelität nach oben ungenau).
    pub alloc: u64,
    pub max_alloc: u64,
}

impl Monitor {
    fn record(&self, name: &str, duration: Duration, alloc: u64) {
        let mut state = lock(&self.state);
        let key = if state.ops.contains_key(name) || state.ops.len() < MAX_OPS {
            name
        } else {
            OVERFLOW_OP
        };
        let op = state.ops.entry(key.to_owned()).or_insert_with(|| Op {
            name: key.to_owned(),
            ..Op::default()
        });
        op.calls += 1;
        op.total += duration;
        op.alloc += alloc;
        op.max = op.max.max(duration);
        op.max_alloc = op.max_alloc.max(alloc);
    }

    /// Misst einen Vorgang bis zum Aufruf des Ergebnisses. Die Allokation wird prozessweit gelesen; laufen Vorgänge
    /// parallel, gehört ein Teil davon den anderen. Über viele Aufrufe zeigt die Summe trotzdem den Verursacher.
    pub fn track(self: &Arc<Self>, name: &str) -> Box<dyn FnOnce() + Send> {
        let monitor = Arc::clone(self);
        let name = name.to_owned();
        let (start, before) = ((monitor.now)(), allocated_bytes());
        Box::new(move || {
            let duration = ((monitor.now)() - start).to_std().unwrap_or_default();
            monitor.record(&name, duration, allocated_bytes().saturating_sub(before));
        })
    }

    /// Die Vorgänge, absteigend nach angeforderten Bytes.
    pub fn ops(&self) -> Vec<Op> {
        let mut out: Vec<Op> = lock(&self.state).ops.values().cloned().collect();
        out.sort_by(|a, b| b.alloc.cmp(&a.alloc).then_with(|| a.name.cmp(&b.name)));
        out
    }
}
