//! Port von `app_logview_test.go`, über die Aufruf-Tabelle wie die Oberfläche.

use std::path::Path;
use std::sync::Arc;

use chrono::{SecondsFormat, TimeDelta, Utc};
use serde_json::{Value, json};

use crate::app::App;
use crate::ports::LOG_NAME;
use crate::rpc::call;
use crate::testutil::tmp_dir;

/// App über einem Repo mit `logs/test.jsonl` (fünf Einträge, der erste vor 30 h), ohne Server und eigenes Log.
fn log_app(root: &Path) -> Arc<App> {
    let now = Utc::now();
    let entry = |ago: TimeDelta, level: &str, ns: &str, msg: &str| {
        let time = (now - ago).to_rfc3339_opts(SecondsFormat::Nanos, true);
        format!(r#"{{"time":"{time}","level":"{level}","ns":"{ns}","msg":"{msg}","port":8080}}"#) + "\n"
    };
    let text = [
        entry(TimeDelta::hours(30), "ERROR", "svc", "port 8079 belegt"),
        entry(TimeDelta::minutes(3), "INFO", "mcp", "aufruf beendet"),
        entry(TimeDelta::minutes(2), "WARN", "svc", "port 8080 belegt"),
        entry(TimeDelta::minutes(1), "WARN", "svc", "port 8081 belegt"),
        entry(TimeDelta::zero(), "ERROR", "check", "lauf abgebrochen"),
    ]
    .concat();
    std::fs::create_dir_all(root.join("logs")).unwrap();
    std::fs::write(root.join("logs").join("test.jsonl"), text).unwrap();
    let app = Arc::new(App::new(root.to_path_buf(), 0, None).with_usage_file(root.join("usage.json")));
    app.mark_ready();
    app
}

fn query(app: &Arc<App>, source: &str, filter: Value) -> Result<Value, String> {
    call(app, "LogsQuery", &[json!(source), filter]).map_err(|e| e.0)
}

fn errors(app: &Arc<App>, source: &str, level: &str) -> Result<Value, String> {
    call(app, "LogsErrors", &[json!(source), json!(level)]).map_err(|e| e.0)
}

fn filter(level: &str, ns: &str, pattern: &str, limit: i64) -> Value {
    json!({"minLevel": level, "ns": ns, "pattern": pattern, "limit": limit})
}

fn len(view: &Value, key: &str) -> usize {
    view[key].as_array().unwrap().len()
}

/// `TestLogsQuery`.
#[test]
fn logs_query() {
    let tmp = tmp_dir();
    let app = log_app(tmp.path());
    let all = query(&app, "test", filter("", "", "", 100)).unwrap();
    assert_eq!(len(&all, "entries"), 5, "{all}");
    assert_eq!(all["entries"][0]["msg"], "lauf abgebrochen");
    assert!(all["missing"] == false && all["bytesRead"].as_u64() > Some(0), "{all}");
    assert_eq!((&all["budgetHit"], &all["skipped"]), (&json!(false), &json!(0)));
    let warn = query(&app, "test", filter("WARN", "", "", 100)).unwrap();
    assert_eq!(len(&warn, "entries"), 4);
    let search = query(&app, "test", filter("", "svc", "808[01]", 100)).unwrap();
    assert_eq!(len(&search, "entries"), 2);
    assert_eq!(all["entries"][1]["data"]["port"], json!(8080));
    let own = query(&app, LOG_NAME, filter("", "", "", 100)).unwrap();
    assert!(own["missing"] == true && len(&own, "entries") == 0, "{own}");
}

/// `TestLogsQueryAbgelehnt`: Eingaben aus dem Frontend prüft das Backend (nie ein Pfad).
#[test]
fn logs_query_abgelehnt() {
    let tmp = tmp_dir();
    let app = log_app(tmp.path());
    let bad = [
        ("ungültige Suche", filter("", "", "(", 100)),
        ("limit 150; gültig: [100 200 500]", filter("", "", "", 150)),
        (
            "unbekanntes Level \"FATAL\"; gültig: [ DEBUG INFO WARN ERROR]",
            filter("FATAL", "", "", 100),
        ),
    ];
    for (want, filter) in bad {
        let error = query(&app, "test", filter).unwrap_err();
        assert!(error.contains(want), "{error}");
    }
    let error = query(&app, "../test", filter("", "", "", 100)).unwrap_err();
    assert!(error.contains("unbekannte Log-Quelle"), "{error}");
}

/// `TestLogsErrors`.
#[test]
fn logs_errors() {
    let tmp = tmp_dir();
    let app = log_app(tmp.path());
    let warn = errors(&app, "test", "WARN").unwrap();
    assert!(warn["entries"] == 3 && len(&warn, "groups") == 2, "{warn}");
    let group = &warn["groups"][0];
    assert_eq!(
        (&group["count"], &group["ns"], &group["fingerprint"], &group["example"]),
        (
            &json!(2),
            &json!("svc"),
            &json!("port <n> belegt"),
            &json!("port 8081 belegt")
        )
    );
    let only = errors(&app, "test", "ERROR").unwrap();
    assert!(
        len(&only, "groups") == 1 && only["groups"][0]["example"] == "lauf abgebrochen",
        "{only}"
    );
    let error = errors(&app, "test", "INFO").unwrap_err();
    assert_eq!(error, "unbekanntes Level \"INFO\"; gültig: [WARN ERROR]");
    let own = errors(&app, LOG_NAME, "WARN").unwrap();
    assert!(own["missing"] == true && own["groups"] == json!([]), "{own}");
}
