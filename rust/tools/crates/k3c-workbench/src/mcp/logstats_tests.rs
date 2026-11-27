//! Port von `tools_logstats_test.go` (`TestConsoleTailSince` steht in `console_tests.rs`).

use serde_json::json;

use super::logs_tests::{Fixture, local, size};

const DAY: &str = "2026-09-30T00:00:00Z";

/// `TestLogsStats`.
#[tokio::test(flavor = "multi_thread")]
async fn stats() {
    let mut f = Fixture::new().await;
    let arguments = json!({"service": "k3c-workbench", "since": DAY});
    let foot = format!("{} gelesen · 1 unlesbare Zeilen übersprungen", size());
    let want = format!("7 Einträge seit {DAY} je level · {foot}\nINFO 3, WARN 2, DEBUG 1, ERROR 1");
    assert_eq!(f.ok("logs_stats", arguments).await, want);
    let text = f.ok("logs_stats", json!({"since": DAY, "groupBy": "ns"})).await;
    let want = format!("7 Einträge seit {DAY} je ns · {foot}\nmcp 4, main 2, check 1");
    assert_eq!(text, want);
    // Alle Testdaten liegen vor `since`: nichts zu zählen.
    let text = f.ok("logs_stats", json!({"since": "2099-01-01T00:00:00Z"})).await;
    assert!(
        text.starts_with("0 Einträge seit ") && text.ends_with("\nnoch keine"),
        "{text}"
    );
    let (text, is_error) = f.call("logs_stats", json!({"groupBy": "x"})).await;
    assert!(
        is_error && text == "groupBy \"x\" ungültig: erlaubt level oder ns",
        "{text}"
    );
    f.end().await;
}

/// `TestLogsContext`.
#[tokio::test(flavor = "multi_thread")]
async fn context() {
    let mut f = Fixture::new().await;
    let ts = "2026-09-30T09:04:00Z";
    let arguments = json!({"service": "k3c-workbench", "ts": ts, "before": 1, "after": 1});
    let text = f.ok("logs_context", arguments).await;
    let want = format!(
        "  {} INFO mcp - datei C:\\WORKSPACE\\k3c\\logs\\a.jsonl gelesen\n\
         > {} WARN mcp - check_run: Ziel \"go:y\" unbekannt\n\
         \x20 {} ERROR mcp Repo-Wurzel check_run: Ziel \"task:x\" unbekannt\n3 Zeilen um {ts}",
        local(3, "%H:%M:%S"),
        local(4, "%H:%M:%S"),
        local(5, "%H:%M:%S")
    );
    // `\` am Zeilenende schluckt führende Leerzeichen, darum `\x20` für die Einrückung.
    assert_eq!(text, want);
    let arguments = json!({"service": "k3c-workbench", "ts": ts, "limit": 2});
    let text = f.ok("logs_context", arguments).await;
    assert!(text.ends_with(&format!("\n2 Zeilen um {ts}")), "{text}");
    let arguments = json!({"service": "k3c-workbench", "ts": "gestern"});
    let (text, is_error) = f.call("logs_context", arguments).await;
    assert!(
        is_error && text == "ts \"gestern\" ist kein Zeitpunkt nach RFC 3339",
        "{text}"
    );
    f.end().await;
}
