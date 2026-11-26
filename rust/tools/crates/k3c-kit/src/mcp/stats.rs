//! Zähler je Tool und Aufruf-Log (Port von `mcpkit/stats.go`). Die serde-Namen sind die Form für die Oberfläche.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use chrono::{DateTime, Local, TimeDelta};
use serde::Serialize;

use super::{CALL_RING_SIZE, clip, ref_name};
use crate::logger::time_millis;
use crate::usage::Clock;

/// Zähler eines Tools seit dem Programmstart.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStats {
    pub name: String,
    pub description: String,
    pub calls: u64,
    pub errors: u64,
    pub avg_ms: f64,
    pub last_call: String,
    /// Zeitpunkt des letzten fehlgeschlagenen Aufrufs; leer = noch keiner.
    pub last_error: String,
}

/// Eintrag im Aufruf-Log. `start_seq` und `end_seq` stammen aus einer gemeinsamen Folge über alle Start- und
/// End-Ereignisse; damit verbindet die Oberfläche Start- und Endzeile eines Aufrufs.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub id: i64,
    #[serde(rename = "ref")]
    pub reference: String,
    pub started_at: String,
    pub ts: String,
    pub start_seq: i64,
    pub end_seq: i64,
    pub running: bool,
    pub tool: String,
    pub args: String,
    pub duration_ms: f64,
    pub ok: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    pub summary: String,
    /// Ziel eines Aufrufs aus den ungekürzten Argumenten (Workbench: `check_run`, B-876); sonst leer.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub target: String,
}

/// Alle Zähler für die Oberfläche; Tools in Katalog-Reihenfolge.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub started_at: String,
    pub clients: usize,
    pub peak_clients: usize,
    pub in_flight: usize,
    pub peak_in_flight: usize,
    pub total_calls: u64,
    pub errors: u64,
    pub tools: Vec<ToolStats>,
}

/// Ergebnis eines beendeten Aufrufs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Outcome {
    pub ok: bool,
    pub err: String,
    pub summary: String,
}

/// Meldet Beginn oder Ende eines Aufrufs.
pub type CallHook = Arc<dyn Fn(&Call) + Send + Sync>;
/// Liest das Ziel eines Aufrufs aus Tool und rohen Argumenten.
pub type TargetFn = Arc<dyn Fn(&str, &str) -> String + Send + Sync>;

#[derive(Default)]
struct Counter {
    desc: String,
    calls: u64,
    errors: u64,
    sum_ms: f64,
    last_call: String,
    last_err: String,
}

#[derive(Default)]
struct Inner {
    order: Vec<String>,
    tools: HashMap<String, Counter>,
    ring: Vec<Call>,
    next: usize,
    running: HashMap<i64, (Call, DateTime<Local>)>,
    next_id: i64,
    seq: i64,
    total: u64,
    errors: u64,
    in_flight: usize,
    peak_in_flight: usize,
    clients: usize,
    peak_clients: usize,
}

impl Inner {
    /// Zählt einen beendeten Aufruf; ein Tool außerhalb des Katalogs zählt nur in den Summen.
    fn count(&mut self, c: &Call) {
        self.in_flight = self.in_flight.saturating_sub(1);
        self.total += 1;
        if !c.ok {
            self.errors += 1;
        }
        let Some(t) = self.tools.get_mut(&c.tool) else {
            return;
        };
        t.calls += 1;
        t.sum_ms += c.duration_ms;
        t.last_call.clone_from(&c.ts);
        if !c.ok {
            t.errors += 1;
            t.last_err.clone_from(&c.ts);
        }
    }

    fn push(&mut self, c: Call) {
        if self.ring.len() < CALL_RING_SIZE {
            self.ring.push(c);
            return;
        }
        self.ring[self.next] = c;
        self.next = (self.next + 1) % CALL_RING_SIZE;
    }
}

/// Zählt Aufrufe und hält das Aufruf-Log; alle Methoden sind nebenläufig sicher.
pub struct Stats {
    now: Clock,
    started_at: DateTime<Local>,
    on_start: Option<CallHook>,
    on_call: Option<CallHook>,
    target: Option<TargetFn>,
    inner: Mutex<Inner>,
}

impl Stats {
    /// `on_start` und `on_call` melden Beginn und Ende eines Aufrufs, `target` füllt [`Call::target`].
    pub fn new(now: Clock, on_start: Option<CallHook>, on_call: Option<CallHook>, target: Option<TargetFn>) -> Stats {
        let started_at = now();
        Stats {
            now,
            started_at,
            on_start,
            on_call,
            target,
            inner: Mutex::default(),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Nimmt ein Tool aus dem Katalog auf, damit es auch ohne Aufruf in den Zählern steht.
    pub fn register(&self, name: &str, desc: &str) {
        let mut st = self.lock();
        st.order.push(name.to_owned());
        let counter = Counter {
            desc: desc.to_owned(),
            ..Counter::default()
        };
        st.tools.insert(name.to_owned(), counter);
    }

    /// Trägt einen laufenden Aufruf ein und liefert seine ID.
    pub fn begin(&self, tool: &str, args: &str) -> i64 {
        let call = {
            let mut st = self.lock();
            let now = (self.now)();
            st.next_id += 1;
            st.seq += 1;
            let call = Call {
                id: st.next_id,
                reference: ref_name(st.next_id),
                started_at: time_millis(&now),
                start_seq: st.seq,
                running: true,
                tool: tool.to_owned(),
                args: clip(args),
                target: self.target.as_ref().map_or_else(String::new, |f| f(tool, args)),
                ..Call::default()
            };
            st.running.insert(call.id, (call.clone(), now));
            st.in_flight += 1;
            st.peak_in_flight = st.peak_in_flight.max(st.in_flight);
            call
        };
        if let Some(f) = &self.on_start {
            f(&call);
        }
        call.id
    }

    /// Schließt einen Aufruf ab, zählt ihn, schiebt ihn ins Aufruf-Log und liefert ihn; `None` bei unbekannter ID.
    pub fn end(&self, id: i64, o: Outcome) -> Option<Call> {
        let call = {
            let mut st = self.lock();
            let (mut c, start) = st.running.remove(&id)?;
            let now = (self.now)();
            st.seq += 1;
            c.ts = time_millis(&now);
            c.end_seq = st.seq;
            c.running = false;
            let took = now.signed_duration_since(start);
            c.duration_ms = took.num_microseconds().unwrap_or(i64::MAX) as f64 / 1000.0;
            c.ok = o.ok;
            c.error = clip(&o.err);
            c.summary = clip(&o.summary);
            st.count(&c);
            st.push(c.clone());
            c
        };
        if let Some(f) = &self.on_call {
            f(&call);
        }
        Some(call)
    }

    /// Merkt die aktuelle Zahl der Clients und ihren Höchststand.
    pub fn observe_clients(&self, n: usize) {
        let mut st = self.lock();
        st.clients = n;
        st.peak_clients = st.peak_clients.max(n);
    }

    /// Beendete und laufende Aufrufe, neueste zuerst (nach `start_seq`).
    pub fn calls(&self) -> Vec<Call> {
        let mut out = {
            let st = self.lock();
            let running = st.running.values().map(|(c, _)| c.clone());
            st.ring.iter().cloned().chain(running).collect::<Vec<_>>()
        };
        out.sort_by_key(|c| std::cmp::Reverse(c.start_seq));
        out
    }

    /// Alle Zähler, Tools in Katalog-Reihenfolge.
    pub fn snapshot(&self) -> Snapshot {
        let st = self.lock();
        let tools = st.order.iter().filter_map(|name| {
            let t = st.tools.get(name)?;
            Some(ToolStats {
                name: name.clone(),
                description: t.desc.clone(),
                calls: t.calls,
                errors: t.errors,
                avg_ms: if t.calls > 0 { t.sum_ms / t.calls as f64 } else { 0.0 },
                last_call: t.last_call.clone(),
                last_error: t.last_err.clone(),
            })
        });
        Snapshot {
            started_at: time_millis(&self.started_at),
            clients: st.clients,
            peak_clients: st.peak_clients,
            in_flight: st.in_flight,
            peak_in_flight: st.peak_in_flight,
            total_calls: st.total,
            errors: st.errors,
            tools: tools.collect(),
        }
    }

    /// Zeit seit dem Anlegen der Zähler.
    pub fn uptime(&self) -> TimeDelta {
        (self.now)().signed_duration_since(self.started_at)
    }
}

#[cfg(test)]
#[path = "stats_tests.rs"]
mod tests;
