//! Port von `workkit/usage/usage_test.go`.

use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local, TimeDelta, TimeZone};

use super::hist::{DurAgg, MAX_VALUES, OVERFLOW_KEY, norm_args};
use super::scope::SLOW_N;
use super::snapshot::TOP_N;
use super::*;

/// Gestellte Uhr.
#[derive(Clone)]
pub(super) struct TestClock(Arc<Mutex<DateTime<Local>>>);

pub(super) fn start_time() -> DateTime<Local> {
    Local.with_ymd_and_hms(2026, 9, 30, 12, 0, 0).unwrap()
}

impl TestClock {
    pub(super) fn new() -> TestClock {
        TestClock(Arc::new(Mutex::new(start_time())))
    }

    pub(super) fn time(&self) -> DateTime<Local> {
        *self.0.lock().unwrap()
    }

    pub(super) fn advance(&self, by: TimeDelta) {
        *self.0.lock().unwrap() += by;
    }

    pub(super) fn clock(&self) -> Clock {
        let me = self.clone();
        Arc::new(move || me.time())
    }

    pub(super) fn event(&self, tool: &str, args: &str, ms: f64) -> Event {
        Event {
            at: self.time(),
            tool: tool.to_owned(),
            args: args.to_owned(),
            duration_ms: ms,
            ok: true,
            error: String::new(),
        }
    }
}

fn new_tracker() -> (Tracker, TestClock) {
    let c = TestClock::new();
    (Tracker::new(c.clock()), c)
}

pub(super) fn repeat(tr: &Tracker, e: &Event, n: usize) {
    for _ in 0..n {
        tr.record(e.clone());
    }
}

#[test]
fn perzentile_aus_histogramm() {
    let mut d = DurAgg::default();
    for i in 1..=1000 {
        d.add(f64::from(i));
    }
    for (q, want) in [(0.5, 500.0), (0.95, 950.0)] {
        let got = d.p(q);
        assert!((got - want).abs() / want <= 0.12, "p{q} = {got}, echter Wert {want}");
    }
    let mut one = DurAgg::default();
    one.add(1234.0);
    assert!(one.p(0.95) <= 1234.0, "Perzentil {} über dem Maximum", one.p(0.95));
    let mut tiny = DurAgg::default();
    tiny.add(0.3);
    assert_eq!(tiny.p(0.5), 0.3, "unter 1 ms");
}

/// Ausreißer eines Tools in der Gesamtzeit, -1 ohne das Tool.
fn outliers(tr: &Tracker, tool: &str) -> i64 {
    let tools = tr.snapshot().all_time.tools;
    tools.iter().find(|u| u.name == tool).map_or(-1, |u| u.outliers)
}

#[test]
fn ausreisser_regel() {
    /// Name, Vorlauf (Tool, Argumente, Dauer, Anzahl), letzter Aufruf (Tool, Argumente, Dauer), erwartete Ausreißer.
    type Case<'a> = (
        &'a str,
        &'a [(&'a str, &'a str, f64, usize)],
        (&'a str, &'a str, f64),
        i64,
    );
    let a = r#"{"target":"a"}"#;
    let b = r#"{"target":"b"}"#;
    let check = r#"{"target":"task:check"}"#;
    let new = r#"{"target":"neu"}"#;
    let cases: [Case; 7] = [
        ("Beispiel logs_query", &[("q", "{}", 100.0, 20)], ("q", "{}", 1500.0), 1),
        (
            "Beispiel check_run unter 2x p95",
            &[("c", check, 40_000.0, 20)],
            ("c", check, 50_000.0),
            0,
        ),
        ("unter 1 s", &[("q", "{}", 100.0, 20)], ("q", "{}", 900.0), 0),
        ("erst ab 8 Aufrufen", &[("q", "{}", 100.0, 7)], ("q", "{}", 5000.0), 0),
        (
            "Vergleichswert vor dem Eintragen",
            &[("q", "{}", 100.0, 8)],
            ("q", "{}", 5000.0),
            1,
        ),
        (
            "Argumentgruppe vor Toolgruppe",
            &[("c", a, 100.0, 200), ("c", b, 2000.0, 8)],
            ("c", b, 3000.0),
            0,
        ),
        (
            "Toolgruppe ohne Argumentgruppe",
            &[("c", a, 100.0, 200)],
            ("c", new, 3000.0),
            1,
        ),
    ];
    for (name, fill, (tool, args, ms), want) in cases {
        let (tr, c) = new_tracker();
        for &(tool, args, ms, n) in fill {
            repeat(&tr, &c.event(tool, args, ms), n);
        }
        let before = outliers(&tr, tool);
        tr.record(c.event(tool, args, ms));
        assert_eq!(outliers(&tr, tool) - before.max(0), want, "{name}");
    }
}

#[test]
fn ranglisten_und_deckel() {
    let (tr, c) = new_tracker();
    repeat(&tr, &c.event("q", r#"{"b":1,"a":2}"#, 10.0), 8); // Schlüssel werden sortiert
    let mut fail = c.event("q", "{}", 5.0);
    (fail.ok, fail.error) = (false, "Ziel unbekannt".to_owned());
    repeat(&tr, &fail, 2);
    // 48 passen noch, 7 zählen unter (weitere)
    for i in 0..MAX_VALUES + 5 {
        tr.record(c.event("q", &format!(r#"{{"n":{i}}}"#), (i + 1) as f64));
    }
    let s = tr.snapshot().session;
    let q = &s.tools[0];
    assert_eq!(q.args[0].value, r#"{"a":2,"b":1}"#);
    assert_eq!((q.args[1].value.as_str(), q.args[1].count), (OVERFLOW_KEY, 7));
    assert_eq!(q.args.len(), TOP_N);
    assert_eq!(
        (q.top_errors.len(), q.top_errors[0].count),
        (1, 2),
        "{:?}",
        q.top_errors
    );
    assert_eq!(
        (s.top_errors[0].tool.as_str(), s.errors),
        ("q", 2),
        "{:?}",
        s.top_errors
    );
    assert_eq!(s.slowest.len(), SLOW_N);
    assert_eq!((s.slowest[0].duration_ms, s.slowest[1].duration_ms), (55.0, 54.0));
}

#[test]
fn letzte_ausreisser_neueste_zuerst() {
    let (tr, c) = new_tracker();
    repeat(&tr, &c.event("q", "{}", 100.0), 300); // groß genug, dass 12 langsame den p95 nicht heben
    for i in 0..SLOW_N + 2 {
        tr.record(c.event("q", "{}", (5000 + i) as f64));
    }
    let got = tr.snapshot().all_time.recent_outliers;
    assert_eq!((got.len(), got[0].duration_ms), (SLOW_N, 5011.0));
}

#[test]
fn zeitreihe_sieben_tage() {
    let (tr, c) = new_tracker();
    tr.record(c.event("q", "{}", 10.0));
    c.advance(TimeDelta::seconds(30));
    tr.record(c.event("q", "{}", 30.0));
    c.advance(TimeDelta::minutes(2));
    tr.record(c.event("c", "{}", 5.0));
    let m = tr.snapshot().minutes;
    assert_eq!(m.len(), 2, "{m:?}");
    assert_eq!((m[0].calls["q"], m[0].ms["q"], m[0].max_ms["q"]), (2, 40.0, 30.0));
    assert_eq!(m[1].calls["c"], 1);
    assert_eq!(m[0].ts, start_time().timestamp_millis());
    c.advance(SERIES_SPAN - TimeDelta::minutes(1));
    let m = tr.snapshot().minutes;
    assert_eq!(m.len(), 1, "nach 7 Tagen: {m:?}");
    assert_eq!(m[0].calls["c"], 1);
}

#[test]
fn schnappschuss_als_json() {
    let (tr, _) = new_tracker();
    let text = serde_json::to_string(&tr.snapshot()).unwrap();
    let keys = [
        r#""session":"#,
        r#""allTime":"#,
        r#""minutes":[]"#,
        r#""tools":[]"#,
        r#""topErrors":[]"#,
        r#""recentOutliers":[]"#,
        r#""slowest":[]"#,
        r#""p95Ms":0"#,
    ];
    for key in keys {
        assert!(text.contains(key), "fehlt {key} in {text}");
    }
    assert!(!text.contains("null"), "null in {text}");
}

#[test]
fn minutenwende_in_beliebiger_reihenfolge() {
    let (tr, c) = new_tracker();
    let mut later = c.event("q", "{}", 5.0);
    later.at = c.time() + TimeDelta::minutes(1);
    tr.record(later.clone());
    tr.record(c.event("q", "{}", 5.0)); // früher, kommt aber später an
    tr.record(later);
    let m = tr.snapshot().minutes;
    assert_eq!(m.len(), 2, "{m:?}");
    assert!(m[0].ts < m[1].ts);
    assert_eq!((m[0].calls["q"], m[1].calls["q"]), (1, 2));
}

/// Ergänzung zum Port: die Erwartungen entsprechen Go `json.Unmarshal` und `json.Marshal` eines `any`.
#[test]
fn argumente_wie_go() {
    assert_eq!(norm_args(""), "{}");
    assert_eq!(norm_args("null"), "{}");
    assert_eq!(norm_args("kein json"), "kein json");
    // `<`, `>`, `&` maskiert Go als Unicode-Folge.
    let esc = |code: &str| format!("\\u{code}");
    let escaped = [esc("003c"), esc("0026"), esc("003e")].concat();
    assert_eq!(
        norm_args(r#"{ "b": 1.0, "a": ["<&>", 1e21, 1e-7, 1.5, -0.0, true, null] }"#),
        format!(r#"{{"a":["{escaped}",1e+21,1e-7,1.5,-0,true,null],"b":1}}"#)
    );
    let long = norm_args(&"ä".repeat(200));
    assert_eq!((long.chars().count(), long.ends_with('…')), (120, true));
}
