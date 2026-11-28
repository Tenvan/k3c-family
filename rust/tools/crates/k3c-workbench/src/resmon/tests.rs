//! Port von `resmon_test.go` und `webview_test.go`. Statt des Heap-Profil-Tests prüft `zaehler_folgt_dem_heap` den
//! eigenen Zähler (Profile entfallen).

use chrono::TimeDelta;
use k3c_kit::applog;
use k3c_kit::console::Store;

use super::ops::{MAX_OPS, OVERFLOW_OP};
use super::*;
use crate::testutil::tmp_dir;

/// Eine Uhr, die der Test vorstellt.
#[derive(Clone)]
struct TestClock(Arc<Mutex<DateTime<Local>>>);

impl TestClock {
    fn new() -> TestClock {
        let start = DateTime::from_timestamp(1_000_000, 0).unwrap().with_timezone(&Local);
        TestClock(Arc::new(Mutex::new(start)))
    }

    fn advance(&self, seconds: i64) {
        *lock(&self.0) += TimeDelta::seconds(seconds);
    }

    fn config(&self, history: usize) -> Config {
        let time = Arc::clone(&self.0);
        Config {
            history,
            now: Arc::new(move || *lock(&time)),
            ..Config::default()
        }
    }
}

#[test]
fn probe_liest_messwerte_und_haelt_den_verlauf_begrenzt() {
    let clock = TestClock::new();
    let monitor = Monitor::new(clock.config(3));
    monitor.gauge("puffer", true, || 7);
    for _ in 0..5 {
        clock.advance(60);
        monitor.sample();
    }
    assert_eq!(monitor.history().len(), 3);
    let s = monitor.latest().unwrap();
    assert_eq!(s.gauges["puffer"], 7);
    assert!(s.heap_live > 0 && s.threads >= 1 && s.rss > 0 && s.handles > 0, "{s:?}");
}

#[test]
fn verlauf_bleibt_begrenzt() {
    let clock = TestClock::new();
    let monitor = Monitor::new(clock.config(4));
    for _ in 0..50 {
        clock.advance(1);
        monitor.sample();
    }
    assert_eq!(monitor.history().len(), 4);
    assert_eq!(lock(&monitor.state).history.len(), 4);
}

fn heap_at(minutes: i64, live: u64) -> Sample {
    let start = DateTime::from_timestamp(0, 0).unwrap().with_timezone(&Local);
    Sample {
        at: start + TimeDelta::minutes(minutes),
        heap_live: live,
        ..Sample::default()
    }
}

#[test]
fn trend_erst_nach_der_mindestspanne() {
    let live: fn(&Sample) -> f64 = |s| s.heap_live as f64;
    let short = [heap_at(0, 1), heap_at(5, 2), heap_at(10, 3)];
    assert!(trend_of(&short, live).is_none(), "10 min sind zu kurz");
    // +10 MiB je 5 min = +120 MiB/h.
    let long: Vec<Sample> = (0..=12)
        .map(|i| heap_at(i * 5, (100 << 20) + u64::try_from(i).unwrap() * (10 << 20)))
        .collect();
    let trend = trend_of(&long, live).unwrap();
    let per_hour = trend.per_hour / f64::from(1 << 20);
    assert!((119.0..=121.0).contains(&per_hour), "{per_hour}");
    assert_eq!(crate::services::go_duration(trend.window), "1h0m0s");
}

#[test]
fn vorgaenge_werden_summiert_und_begrenzt() {
    let monitor = Monitor::new(Config::default());
    for i in 0..MAX_OPS + 10 {
        monitor.track(&format!("op{i}"))();
    }
    monitor.track("op0")();
    let ops = monitor.ops();
    assert!(ops.len() <= MAX_OPS + 1, "{}", ops.len());
    assert!(ops.iter().any(|o| o.name == OVERFLOW_OP && o.calls == 10));
    assert!(ops.iter().any(|o| o.name == "op0" && o.calls == 2));
}

#[test]
fn zaehler_folgt_dem_heap() {
    let monitor = Monitor::new(Config::default());
    let done = monitor.track("tool:viel");
    let block = std::hint::black_box(vec![1_u8; 8 << 20]);
    // Andere Tests allokieren parallel, darum nur Untergrenzen.
    assert!(monitor.sample().heap_live >= 8 << 20);
    done();
    drop(block);
    let op = &monitor.ops()[0];
    assert!(op.alloc >= 8 << 20 && op.max_alloc == op.alloc, "{op:?}");
}

#[test]
fn bericht_nennt_werte_und_puffer() {
    let monitor = Monitor::new(Config::default());
    monitor.gauge("konsole", true, || 2048);
    monitor.gauge("läufe", false, || 3);
    monitor.track("tool:check_run")();
    let report = monitor.report();
    for want in [
        "heap live",
        "konsole 2 KiB",
        "läufe 3",
        "tool:check_run",
        "Trend: erst ab 30m0s",
        "Threads",
        "alle 30s",
    ] {
        assert!(report.contains(want), "Bericht ohne {want:?}:\n{report}");
    }
    for gone in ["Goroutinen", "go gesamt", "pprof", "Profile"] {
        assert!(!report.contains(gone), "{gone}:\n{report}");
    }
}

#[test]
fn reihe_verdichtet_mit_maximum() {
    let clock = TestClock::new();
    let monitor = Monitor::new(clock.config(1000));
    for i in 0..100_u64 {
        clock.advance(10);
        monitor.sample();
        lock(&monitor.state).history.back_mut().unwrap().heap_live = i;
    }
    let got = monitor.series(Duration::from_secs(3600), 10);
    assert!((5..=10).contains(&got.len()), "{}", got.len());
    assert_eq!(got.last().unwrap().heap_live, 99);
    let narrow = monitor.series(Duration::from_secs(60), 50);
    assert!((1..=7).contains(&narrow.len()), "{}", narrow.len());
}

#[test]
fn webview_ueber_der_grenze_warnt_je_stufe() {
    let dir = tmp_dir();
    let (log, error) = applog::open_named(dir.path(), "res", Arc::new(Store::new(64, None)));
    assert!(error.is_none());
    let monitor = Monitor::new(Config {
        log: log.logger().clone(),
        ..Config::default()
    });
    let warnings = || {
        let text = std::fs::read_to_string(dir.path().join("res.jsonl")).unwrap_or_default();
        text.matches("🤒 webview-renderer").count()
    };
    let at = |web_view: u64| Sample {
        web_view,
        ..Sample::default()
    };
    monitor.check_thresholds(&at(400 << 20));
    assert_eq!(warnings(), 0, "unter der Grenze gewarnt");
    monitor.check_thresholds(&at(600 << 20));
    assert_eq!(warnings(), 1);
    // Nächste Stufe erst bei 900 MiB.
    monitor.check_thresholds(&at(700 << 20));
    assert_eq!(warnings(), 1);
    monitor.check_thresholds(&at(950 << 20));
    assert_eq!(warnings(), 2);
    let text = std::fs::read_to_string(dir.path().join("res.jsonl")).unwrap();
    assert!(text.contains(&format_bytes(600 << 20)), "{text}");
    let _ = log.close();
}

#[test]
fn webview_vorgabe_und_zusammenfassung() {
    assert_eq!(Config::default().web_view_warn, 500 << 20);
    let with = Sample {
        web_view: 600 << 20,
        ..Sample::default()
    };
    assert!(summary(&with).contains("Renderer 600.0 MiB"), "{}", summary(&with));
    assert!(!summary(&Sample::default()).contains("Renderer"));
}
