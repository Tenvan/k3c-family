//! Port von `app_resources_test.go`.

use std::sync::Arc;

use serde_json::json;

use super::*;
use crate::testutil::tmp_dir;

#[test]
fn ohne_messgeraet_ist_die_ansicht_aus() {
    let app = App::new("unbenutzt".into(), 1, None);
    app.mark_ready();
    let view = serde_json::to_value(app.resources(60)).unwrap();
    assert_eq!(view["enabled"], json!(false));
    assert_eq!(view["points"], json!(null));
    app.track("ui:x")();
}

#[test]
fn liefert_punkte_und_klemmt_den_zeitraum() {
    let monitor = Monitor::new(Config {
        interval: Duration::from_secs(1),
        ..Config::default()
    });
    monitor.gauge("Konsole-Text", true, || 42);
    monitor.track("tool:x")();
    monitor.sample();
    monitor.sample();
    let got = serde_json::to_value(view(&monitor, 1_000_000)).unwrap();
    assert_eq!(got["enabled"], json!(true));
    assert_eq!((&got["windowMin"], &got["intervalS"]), (&json!(MAX_WINDOW), &json!(1)));
    assert_eq!(got["gauges"], json!([{"name": "Konsole-Text", "bytes": true}]));
    let last = got["points"].as_array().unwrap().last().unwrap();
    assert_eq!(last["gaugeValues"], json!([42]));
    assert!(last["goroutines"].as_u64().unwrap() >= 1, "{last}");
    let mut keys: Vec<&String> = last.as_object().unwrap().keys().collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "children",
            "childrenRss",
            "gaugeValues",
            "gcCycles",
            "goroutines",
            "handles",
            "heapLive",
            "private",
            "rss",
            "t",
            "total"
        ]
    );
    assert_eq!(got["ops"].as_array().unwrap().len(), 1);
    assert_eq!(got["ops"][0]["name"], json!("tool:x"));
    assert_eq!(got["profiles"], json!([]));
    assert!(got["children"].is_array());
    assert_eq!(view(&monitor, -3).window_min, MIN_WINDOW);
}

/// Verdrahtung nach `startup`: Puffer wie in Go, Messung je Aufruf der Oberfläche, Bildbeschreibung ohne Auftrag.
#[tokio::test(flavor = "multi_thread")]
async fn startup_verdrahtet_messung_und_bildbeschreibung() {
    let root = tmp_dir();
    let app = App::new(root.path().to_path_buf(), 0, None).with_usage_file(root.path().join("usage.json"));
    let app = Arc::new(app);
    app.startup().await;
    let inner = Arc::clone(&app);
    let got = tokio::task::spawn_blocking(move || {
        crate::rpc::call(&inner, "PlaytestVisionInfo", &[]).unwrap();
        let idle = crate::rpc::call(&inner, "PlaytestDescribeStatus", &[]).unwrap();
        assert_eq!(
            idle,
            json!({"running": false, "id": "", "done": 0, "total": 0, "error": ""})
        );
        let cancel = crate::rpc::call(&inner, "PlaytestDescribeCancel", &[]).unwrap_err();
        assert_eq!(cancel.0, "keine Bildbeschreibung aktiv");
        crate::rpc::call(&inner, "Resources", &[json!(60)]).unwrap()
    })
    .await
    .unwrap();
    app.shutdown().await;
    assert_eq!(got["enabled"], json!(true));
    assert_eq!(got["intervalS"], json!(10));
    let gauges: Vec<&str> = got["gauges"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        gauges,
        [
            "Log-Dateien",
            "Konsole-Quellen",
            "Konsole-Zeilen",
            "Konsole-Text",
            "Prüfläufe",
            "Prüflauf-Text",
            "MCP-Sessions"
        ]
    );
    let ops = got["ops"].to_string();
    assert!(ops.contains("ui:PlaytestDescribeStatus"), "{ops}");
}

#[test]
fn ordner_zaehlt_nur_dateien() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("a.jsonl"), "1234").unwrap();
    std::fs::create_dir(dir.path().join("unter")).unwrap();
    std::fs::write(dir.path().join("unter").join("b"), "99").unwrap();
    assert_eq!(dir_bytes(dir.path()), 4);
    assert_eq!(dir_bytes(&dir.path().join("fehlt")), 0);
}
