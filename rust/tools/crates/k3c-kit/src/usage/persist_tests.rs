//! Port von `workkit/usage/persist_test.go`, dazu das Laden einer Datei im Go-Format.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::{Local, TimeDelta, TimeZone, Utc};

use super::usage_tests::{TestClock, repeat};
use super::*;
use crate::testutil::tmp_dir;

fn system_clock() -> Clock {
    Arc::new(Local::now)
}

/// Sammelt die Warnungen.
#[derive(Clone, Default)]
struct Warnings(Arc<Mutex<Vec<String>>>);

impl Warnings {
    fn hook(&self) -> Arc<dyn Fn(&str) + Send + Sync> {
        let list = Arc::clone(&self.0);
        Arc::new(move |msg| list.lock().unwrap().push(msg.to_owned()))
    }

    fn all(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }
}

fn open(path: &Path, now: Clock) -> (Arc<Tracker>, Warnings) {
    let warnings = Warnings::default();
    (Tracker::open(path, now, warnings.hook()), warnings)
}

fn event_now(tool: &str, args: &str, ok: bool) -> Event {
    Event {
        at: Local::now(),
        tool: tool.to_owned(),
        args: args.to_owned(),
        duration_ms: 5.0,
        ok,
        error: if ok { String::new() } else { "kaputt".to_owned() },
    }
}

#[test]
fn neustart_behaelt_gesamtzeit_und_zeitreihe() {
    let tmp = tmp_dir();
    let path = tmp.path().join("k3c").join("mcp-usage.json");
    let c = TestClock::new();
    let (tr, _) = open(&path, c.clock());
    repeat(&tr, &c.event("q", r#"{"a":1}"#, 100.0), 20);
    tr.record(c.event("q", r#"{"a":1}"#, 1500.0));
    tr.flush().unwrap();
    assert!(
        !tmp.path().join("k3c").join("mcp-usage.json.tmp").exists(),
        "temporäre Datei liegt noch da"
    );
    c.advance(TimeDelta::hours(1));
    let (again, warnings) = open(&path, c.clock());
    assert_eq!(warnings.all(), Vec::<String>::new());
    let snap = again.snapshot();
    assert_eq!(
        (
            snap.session.calls,
            snap.all_time.calls,
            snap.all_time.outliers,
            snap.minutes.len()
        ),
        (0, 21, 1, 1)
    );
    assert!(
        snap.all_time.since.starts_with("2026-09-30T12:00"),
        "{}",
        snap.all_time.since
    );
    assert!(
        snap.session.since.starts_with("2026-09-30T13:00"),
        "{}",
        snap.session.since
    );
    again.record(c.event("q", r#"{"a":1}"#, 100.0)); // Vergleichsgruppe aus der Datei
    let u = &again.snapshot().all_time.tools[0];
    assert_eq!((u.calls, u.args[0].count), (22, 22));
    again.flush().unwrap();
}

#[test]
fn kaputte_oder_fremde_datei_wird_bak() {
    for (name, content) in [
        ("kaputt", "{nicht"),
        ("fremd", r#"{"version":99,"allTime":{"tools":{}}}"#),
    ] {
        let tmp = tmp_dir();
        let path = tmp.path().join("mcp-usage.json");
        std::fs::write(&path, content).unwrap();
        let (tr, warnings) = open(&path, system_clock());
        let warned = warnings.all();
        assert!(warned.len() == 1 && warned[0].contains(".bak"), "{name}: {warned:?}");
        assert_eq!(tr.snapshot().all_time.calls, 0, "{name}");
        let bak = std::fs::read_to_string(tmp.path().join("mcp-usage.json.bak")).unwrap();
        assert_eq!(bak, content, "{name}: .bak verändert");
    }
}

#[test]
fn nicht_schreibbar_warnt_einmal() {
    let tmp = tmp_dir();
    let blocker = tmp.path().join("datei");
    std::fs::write(&blocker, "").unwrap();
    let (tr, warnings) = open(&blocker.join("mcp-usage.json"), system_clock());
    tr.record(event_now("q", "", true));
    for _ in 0..3 {
        assert!(tr.flush().is_err(), "Schreiben in eine Datei als Ordner ohne Fehler");
    }
    assert_eq!(warnings.all().len(), 1, "{:?}", warnings.all());
    assert_eq!(tr.snapshot().session.calls, 1);
}

#[test]
fn record_plant_speichern_gebuendelt() {
    let tmp = tmp_dir();
    let path = tmp.path().join("mcp-usage.json");
    let (tr, _) = open(&path, system_clock());
    tr.record(event_now("q", "", true));
    tr.record(event_now("q", "", true));
    assert!(!path.exists(), "sofort geschrieben statt gebündelt");
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if path.exists() {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("nach SAVE_DELAY nicht geschrieben");
}

#[test]
fn zweite_sicherung_ueberschreibt_erste_nicht() {
    let tmp = tmp_dir();
    let path = tmp.path().join("mcp-usage.json");
    for content in ["{erste", "{zweite"] {
        std::fs::write(&path, content).unwrap();
        open(&path, system_clock());
    }
    let contents: Vec<String> = std::fs::read_dir(tmp.path())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "bak"))
        .map(|p| std::fs::read_to_string(p).unwrap())
        .collect();
    assert_eq!(contents.len(), 2, "Sicherungen: {contents:?}");
    assert!(contents.iter().any(|c| c == "{erste"), "Sicherungen: {contents:?}");
}

#[test]
fn fremde_datei_mit_luecken_loest_keine_panik_aus() {
    let tmp = tmp_dir();
    let path = tmp.path().join("mcp-usage.json");
    let content = format!(
        concat!(
            r#"{{"version":1,"allTime":{{"since":"2026-09-30T10:00:00Z","#,
            r#""tools":{{"x":{{}},"y":null,"z":{{"argDur":{{"{{}}":null}}}}}}}},"#,
            r#""minutes":[{{"t":{},"calls":null}}]}}"#
        ),
        Local::now().timestamp() / 60
    );
    std::fs::write(&path, content).unwrap();
    let (tr, warnings) = open(&path, system_clock());
    assert_eq!(warnings.all(), Vec::<String>::new());
    for tool in ["x", "y", "z"] {
        tr.record(event_now(tool, "{}", false));
    }
    let snap = tr.snapshot();
    // Liegt zwischen Datei und Aufruf eine Minutenwende, gibt es zwei Minuten.
    let calls_x: i64 = snap.minutes.iter().filter_map(|m| m.calls.get("x")).sum();
    assert_eq!((snap.all_time.calls, calls_x), (3, 1), "{:?}", snap.minutes);
    assert!((1..=2).contains(&snap.minutes.len()));
    tr.flush().unwrap();
}

#[test]
fn lesefehler_laesst_datei_unangetastet() {
    let tmp = tmp_dir();
    let path = tmp.path().join("mcp-usage.json");
    // Ein Ordner statt einer Datei: Lesen scheitert, aber nicht mit NotFound.
    std::fs::create_dir(&path).unwrap();
    let (tr, warnings) = open(&path, system_clock());
    tr.record(event_now("q", "", true));
    tr.flush().unwrap();
    let warned = warnings.all();
    assert!(warned.len() == 1 && warned[0].contains("nur im Speicher"), "{warned:?}");
    assert!(path.is_dir(), "Pfad verändert");
    assert!(
        !tmp.path().join("mcp-usage.json.bak").exists(),
        "gültiger Pfad nach .bak verschoben"
    );
}

/// Eine von Go geschriebene Datei (`fileData`, Zeiten RFC3339Nano, `nil`-Listen als `null`) lädt und zählt weiter;
/// was Rust schreibt, hat dieselben Schlüssel.
#[test]
fn laedt_go_datei() {
    let fixture = include_str!("../../testdata/usage-go.json");
    let tmp = tmp_dir();
    let path = tmp.path().join("mcp-usage.json");
    std::fs::write(&path, fixture).unwrap();
    let now = Utc
        .with_ymd_and_hms(2026, 9, 30, 11, 0, 0)
        .unwrap()
        .with_timezone(&Local);
    let (tr, warnings) = open(&path, Arc::new(move || now));
    assert_eq!(warnings.all(), Vec::<String>::new());
    let snap = tr.snapshot();
    let all = &snap.all_time;
    assert_eq!(all.since, "2026-09-28T08:15:30.123+02:00");
    assert_eq!((all.calls, all.errors, all.outliers, all.max_ms), (10, 1, 1, 1500.0));
    let names: Vec<_> = all.tools.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(names, ["logs_query", "plan_get"]);
    assert_eq!(all.tools[0].args[0].value, r#"{"level":"WARN"}"#);
    assert_eq!(all.top_errors[0].value, "Ziel unbekannt");
    assert_eq!(all.recent_outliers[0].at, "2026-09-30T12:00:05.000+02:00");
    assert_eq!(snap.minutes.len(), 1);
    assert_eq!(snap.minutes[0].calls["logs_query"], 9);
    tr.record(Event {
        at: now,
        ..event_now("logs_query", r#"{ "level": "WARN" }"#, true)
    });
    assert_eq!(tr.snapshot().all_time.tools[0].args[0].count, 10);
    tr.flush().unwrap();

    let go: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let rust: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let keys = |v: &serde_json::Value, at: &str| -> Vec<String> {
        let mut keys: Vec<String> = v.pointer(at).unwrap().as_object().unwrap().keys().cloned().collect();
        keys.sort();
        keys
    };
    for at in [
        "",
        "/allTime",
        "/allTime/tools/logs_query",
        "/allTime/slowest/0",
        "/minutes/0",
    ] {
        assert_eq!(keys(&rust, at), keys(&go, at), "Schlüssel unter {at:?}");
    }
    assert_eq!(rust["allTime"]["since"], "2026-09-28T08:15:30.123456700+02:00");
}
