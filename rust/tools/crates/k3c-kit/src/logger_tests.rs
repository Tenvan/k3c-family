use std::sync::{Arc, Mutex};

use chrono::{FixedOffset, TimeZone};
use serde_json::json;

use super::*;

/// Senke, die alle Einträge ab einer Stufe sammelt.
struct Collect {
    min: Level,
    got: Arc<Mutex<Vec<Record>>>,
}

impl Sink for Collect {
    fn enabled(&self, level: Level) -> bool {
        level >= self.min
    }
    fn handle(&self, r: &Record) -> std::io::Result<()> {
        self.got.lock().unwrap().push(r.clone());
        Ok(())
    }
}

fn collect(min: Level) -> (Logger, Arc<Mutex<Vec<Record>>>) {
    let got = Arc::new(Mutex::new(Vec::new()));
    (
        Logger::new(vec![Box::new(Collect {
            min,
            got: Arc::clone(&got),
        })]),
        got,
    )
}

#[test]
fn level_reihenfolge_und_namen_wie_slog() {
    assert!(Level::Debug < Level::Info && Level::Info < Level::Warn && Level::Warn < Level::Error);
    let vals: Vec<i32> = [Level::Debug, Level::Info, Level::Warn, Level::Error]
        .iter()
        .map(|l| l.value())
        .collect();
    assert_eq!(vals, vec![-4, 0, 4, 8]);
    assert_eq!(format!("{} {}", Level::Warn, Level::Error.name()), "WARN ERROR");
}

#[test]
fn with_steht_vor_den_aufruf_attributen_und_filtert_nach_stufe() {
    let (log, got) = collect(Level::Info);
    let sub = log.with(&[("ns", json!("mcp")), ("a", json!(1))]);
    sub.debug("versteckt", &[]);
    sub.warn("laut", &[("b", json!("x")), ("ns", json!("svc"))]);
    log.info("ohne", &[]);
    let got = got.lock().unwrap();
    assert_eq!(got.len(), 2, "{got:?}");
    let keys: Vec<&str> = got[0].attrs.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, vec!["ns", "a", "b", "ns"]);
    assert_eq!((got[0].level, got[0].msg.as_str()), (Level::Warn, "laut"));
    assert!(got[1].attrs.is_empty(), "with ändert den Ursprung nicht");
}

#[test]
fn discard_schreibt_nichts() {
    let log = Logger::discard();
    assert!(!log.enabled(Level::Error));
    log.error("weg", &[("x", json!(1))]);
}

#[test]
fn zeitformate_wie_go() {
    let utc = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2026, 1, 2, 3, 4, 5)
        .unwrap();
    let t = utc.with_nanosecond(120_000_000).unwrap().with_timezone(&Local);
    // Lokale Zeitzone des Rechners: nur Muster prüfen, Offset ist `Z` oder `±hh:mm`.
    let millis = time_millis(&t);
    assert_eq!(
        millis.len(),
        "2026-01-02T03:04:05.120".len() + offset(&t).len(),
        "{millis}"
    );
    assert!(millis.contains(":05.120"), "{millis}");
    let nanos = time_nanos(&t);
    assert!(nanos.contains(":05.12") && !nanos.contains(":05.120"), "{nanos}");
    let whole = t.with_nanosecond(0).unwrap();
    assert!(time_nanos(&whole).contains(":05") && !time_nanos(&whole).contains(":05."));
}

#[test]
fn werte_als_text_wie_slog() {
    let cases = [
        (json!("a b"), "a b"),
        (json!(42), "42"),
        (json!(-7), "-7"),
        (json!(1.5), "1.5"),
        (json!(2.0), "2"),
        (json!(1_000_000.0), "1e+06"),
        (json!(0.00001), "1e-05"),
        (json!(true), "true"),
        (Value::Null, "<nil>"),
    ];
    for (v, want) in cases {
        assert_eq!(value_string(&v), want, "{v:?}");
    }
}
