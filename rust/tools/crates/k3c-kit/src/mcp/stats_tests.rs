//! Port von `mcpkit/stats_test.go`.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local, TimeDelta, TimeZone, Utc};

use super::*;
use crate::mcp::TEXT_RUNES;

/// Gestellte Uhr, die bei jedem Ablesen um 100 ms weiterläuft.
fn clock() -> Clock {
    let start: DateTime<Local> = Utc
        .with_ymd_and_hms(2026, 9, 30, 12, 0, 0)
        .unwrap()
        .with_timezone(&Local);
    let now = Mutex::new(start);
    Arc::new(move || {
        let mut t = now.lock().unwrap();
        *t += TimeDelta::milliseconds(100);
        *t
    })
}

fn registered(st: Stats) -> Stats {
    st.register("a", "Tool A");
    st.register("b", "Tool B");
    st
}

fn test_stats() -> Stats {
    registered(Stats::new(clock(), None, None, None))
}

fn ok() -> Outcome {
    Outcome {
        ok: true,
        ..Outcome::default()
    }
}

fn failed(err: &str) -> Outcome {
    Outcome {
        err: err.to_owned(),
        ..Outcome::default()
    }
}

// TestZaehlerJeTool
#[test]
fn zaehler_je_tool() {
    let st = test_stats();
    st.end(st.begin("a", "{}"), ok());
    st.end(st.begin("a", "{}"), failed("kaputt"));
    st.end(st.begin("fremd", "{}"), ok());
    let snap = st.snapshot();
    assert_eq!((snap.total_calls, snap.errors), (3, 1));
    assert_eq!(snap.tools.len(), 2);
    let (a, b) = (&snap.tools[0], &snap.tools[1]);
    assert_eq!((a.calls, a.errors, a.avg_ms, b.calls), (2, 1, 100.0, 0));
    assert!(!a.last_call.is_empty());
}

// TestLetzterFehlerJeTool
#[test]
fn letzter_fehler_je_tool() {
    let st = test_stats();
    st.end(st.begin("a", "{}"), failed("erster"));
    st.end(st.begin("a", "{}"), ok());
    st.end(st.begin("b", "{}"), ok());
    let first = st.snapshot().tools[0].last_error.clone();
    st.end(st.begin("a", "{}"), failed("zweiter"));
    let snap = st.snapshot();
    assert!(!first.is_empty() && snap.tools[0].last_error > first);
    assert_eq!(snap.tools[1].last_error, "");
}

// TestParallelUndFolge
#[test]
fn parallel_und_folge() {
    let st = test_stats();
    let first = st.begin("a", "");
    let second = st.begin("b", "");
    assert_eq!(st.snapshot().peak_in_flight, 2);
    let running = st.calls();
    assert_eq!(running.len(), 2);
    assert!(running[0].running && running[0].tool == "b");
    st.end(first, ok());
    st.end(second, ok());
    let calls = st.calls();
    let (b, a) = (&calls[0], &calls[1]);
    // Folge: a startet (1), b startet (2), a endet (3), b endet (4).
    assert_eq!((a.start_seq, b.start_seq, a.end_seq, b.end_seq), (1, 2, 3, 4));
    assert_eq!(st.snapshot().in_flight, 0);
}

// TestRingHaelt200NeuesteZuerst
#[test]
fn ring_haelt_200_neueste_zuerst() {
    let st = test_stats();
    for _ in 0..CALL_RING_SIZE + 5 {
        st.end(st.begin("a", ""), ok());
    }
    let calls = st.calls();
    assert_eq!(calls.len(), CALL_RING_SIZE);
    assert_eq!(calls[0].id, CALL_RING_SIZE as i64 + 5);
    assert_eq!(calls[calls.len() - 1].id, 6);
    let mut seen = HashSet::new();
    for c in &calls {
        assert!(seen.insert(&c.reference) && c.reference.len() == 7, "{}", c.reference);
    }
}

fn collect(into: &Arc<Mutex<Vec<Call>>>) -> CallHook {
    let into = Arc::clone(into);
    Arc::new(move |c| into.lock().unwrap().push(c.clone()))
}

// TestRueckrufeUndKuerzen
#[test]
fn rueckrufe_und_kuerzen() {
    let started = Arc::new(Mutex::new(Vec::new()));
    let ended = Arc::new(Mutex::new(Vec::new()));
    let target: TargetFn = Arc::new(|tool, args| format!("{tool}:{}", args.len()));
    let st = registered(Stats::new(
        clock(),
        Some(collect(&started)),
        Some(collect(&ended)),
        Some(target),
    ));
    let summary = Outcome {
        summary: "fertig".to_owned(),
        ..ok()
    };
    st.end(st.begin("a", &"x".repeat(500)), summary);
    {
        let (started, ended) = (started.lock().unwrap(), ended.lock().unwrap());
        assert_eq!((started.len(), ended.len()), (1, 1));
        assert!(started[0].running);
        assert_eq!(ended[0].summary, "fertig");
        assert_eq!(ended[0].args.chars().count(), TEXT_RUNES);
        assert_eq!(ended[0].target, "a:500");
    }
    assert_eq!(st.end(99, ok()), None);
    assert_eq!(st.snapshot().total_calls, 1);
}

#[test]
fn json_namen_wie_go() {
    let st = test_stats();
    st.observe_clients(3);
    st.observe_clients(1);
    st.end(st.begin("a", "{}"), ok());
    let call = serde_json::to_value(&st.calls()[0]).unwrap();
    let mut keys: Vec<&str> = call.as_object().unwrap().keys().map(String::as_str).collect();
    keys.sort_unstable();
    let want = "args durationMs endSeq id ok ref running startSeq startedAt summary tool ts";
    assert_eq!(keys.join(" "), want);
    let snap = serde_json::to_value(st.snapshot()).unwrap();
    assert_eq!((&snap["clients"], &snap["peakClients"]), (&1.into(), &3.into()));
    assert!(snap["tools"][0]["avgMs"].is_number() && snap["tools"][0]["lastError"] == "");
    assert!(st.uptime() > TimeDelta::zero());
}
