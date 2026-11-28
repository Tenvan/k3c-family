//! Port von `tools_logs_test.go` (`TestEreignisseImEigenenLog` steht in `check`): je Tool ein Aufruf über MCP auf den
//! Testdaten, mit dem vollen Antworttext wie Go. Die Wurzel liegt immer in einem Wegwerf-Ordner.

use std::path::Path;
use std::sync::Arc;

use chrono::{Local, TimeDelta};
use k3c_kit::applog;
use k3c_kit::console::Store;
use k3c_kit::mcp::ARGS_RUNES;
use serde_json::{Value, json};

use super::logtext::parse_since;
use super::services::format_bytes;
use super::test_client::Client;
use super::{Config, Server};
use crate::logs::parse_rfc3339;
use crate::testutil::tmp_dir;

const SAMPLE: &[u8] = include_bytes!("../../tests/data/logs/k3c-workbench.jsonl");

/// Server, dessen Repo-Wurzel `logs/k3c-workbench.jsonl` aus den Testdaten enthält.
pub(super) struct Fixture {
    pub client: Client,
    server: Server,
    _tmp: tempfile::TempDir,
}

impl Fixture {
    pub async fn new() -> Fixture {
        let tmp = tmp_dir();
        let logs = tmp.path().join("logs");
        std::fs::create_dir_all(&logs).unwrap();
        std::fs::write(logs.join("k3c-workbench.jsonl"), SAMPLE).unwrap();
        Fixture::at(tmp, Store::new(0, None)).await
    }

    async fn at(tmp: tempfile::TempDir, store: Store) -> Fixture {
        store.add("check:task:test", "stdout", "x");
        let server = Server::new(Config {
            root: tmp.path().to_path_buf(),
            console: Some(Arc::new(store)),
            ..Config::default()
        });
        server.start().await.unwrap();
        let client = Client::connect(server.url()).await;
        Fixture {
            client,
            server,
            _tmp: tmp,
        }
    }

    pub async fn call(&mut self, name: &str, arguments: Value) -> (String, bool) {
        self.client.call_text(name, arguments).await
    }

    pub async fn ok(&mut self, name: &str, arguments: Value) -> String {
        let (text, is_error) = self.call(name, arguments).await;
        assert!(!is_error, "{name}: {text}");
        text
    }

    pub async fn end(self) {
        self.server.stop().await.unwrap();
    }
}

/// Ortszeit eines Zeitpunkts der Testdaten.
pub(super) fn local(minute: u32, format: &str) -> String {
    let at = parse_rfc3339(&format!("2026-09-30T09:{minute:02}:00Z")).unwrap();
    at.with_timezone(&Local).format(format).to_string()
}

pub(super) fn size() -> String {
    format_bytes(SAMPLE.len() as u64)
}

/// `TestLogsSourcesUndUnbekannteQuelle`.
#[tokio::test(flavor = "multi_thread")]
async fn services_und_unbekannte_quelle() {
    let mut f = Fixture::new().await;
    let text = f.ok("logs_services", json!({})).await;
    let want = format!(
        "Log k3c-workbench · {} · zuletzt {} WARN\nKonsole check:task:test · 1 Zeilen",
        size(),
        local(6, "%d.%m. %H:%M:%S")
    );
    assert_eq!(text, want);
    for bad in ["../k3c-workbench", "gibtsnicht", ""] {
        let (text, is_error) = f.call("logs_query", json!({"service": bad})).await;
        let want = format!("unbekannte Log-Quelle {bad:?}; gültig: k3c-workbench");
        assert!(is_error && text == want, "{bad}: {text}");
    }
    f.end().await;
}

/// `TestLogsQuery`.
#[tokio::test(flavor = "multi_thread")]
async fn query() {
    let mut f = Fixture::new().await;
    let arguments = json!({"service": "k3c-workbench", "level": "WARN", "limit": 2});
    let text = f.ok("logs_query", arguments).await;
    let lines: Vec<&str> = text.lines().collect();
    let first = format!("{} WARN main - server gestoppt", local(6, "%H:%M:%S"));
    let second = format!(
        "{} ERROR mcp Repo-Wurzel check_run: Ziel \"task:x\" unbekannt",
        local(5, "%H:%M:%S")
    );
    assert_eq!(lines[..2], [first, second]);
    assert!(
        lines.len() == 3 && lines[2].starts_with("2 Einträge · ") && lines[2].ends_with(" gelesen"),
        "{text}"
    );
    let arguments = json!({"service": "k3c-workbench", "ns": "check"});
    let want = format!(
        "{} INFO check - lauf beendet {{\"exit\":0,\"target\":\"task:test\"}}\n\
         1 Einträge · {} gelesen · 1 unlesbare Zeilen übersprungen",
        local(2, "%H:%M:%S"),
        size()
    );
    assert_eq!(f.ok("logs_query", arguments).await, want);
    let arguments = json!({"service": "k3c-workbench", "pattern": "["});
    let (text, is_error) = f.call("logs_query", arguments).await;
    assert!(
        is_error && text.starts_with("pattern \"[\" ist kein gültiger regulärer Ausdruck: "),
        "{text}"
    );
    let arguments = json!({"service": "k3c-workbench", "since": "gestern"});
    let (text, is_error) = f.call("logs_query", arguments).await;
    let want = "since \"gestern\": erwartet eine Dauer wie 30m, 24h, 7d oder einen Zeitpunkt RFC 3339";
    assert!(is_error && text == want, "{text}");
    f.end().await;
}

/// `TestLogsErrorsUndSince`.
#[tokio::test(flavor = "multi_thread")]
async fn errors_und_since() {
    let mut f = Fixture::new().await;
    let arguments = json!({"service": "k3c-workbench", "since": "2026-09-30T00:00:00Z"});
    let text = f.ok("logs_errors", arguments).await;
    let want = format!(
        "2× ERROR mcp check_run: Ziel \"task:x\" unbekannt · {}–{}\n1× WARN main server gestoppt · {}\n\
         2 Gruppen aus 3 Einträgen ab WARN seit 2026-09-30T00:00:00Z · {} gelesen · 1 unlesbare Zeilen übersprungen",
        local(4, "%H:%M"),
        local(5, "%H:%M"),
        local(6, "%H:%M"),
        size()
    );
    assert_eq!(text, want);
    let text = f.ok("logs_since", json!({"service": "k3c-workbench"})).await;
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 8, "{text}");
    let first = "k3c-workbench gestartet {\"url\":\"http://127.0.0.1:5180/mcp\"}";
    assert!(lines[0].ends_with(first), "{text}");
    assert_eq!(lines[7], format!("7 neue Einträge · cursor={}", SAMPLE.len()));
    let arguments = json!({"service": "k3c-workbench", "cursor": SAMPLE.len()});
    let again = f.ok("logs_since", arguments).await;
    assert_eq!(again, format!("0 neue Einträge · cursor={}", SAMPLE.len()));
    let arguments = json!({"service": "k3c-workbench", "limit": 1, "cursor": -1});
    let text = f.ok("logs_since", arguments).await;
    assert!(
        text.ends_with(" · gekürzt (Datei neu begonnen oder mehr als das Limit)"),
        "{text}"
    );
    f.end().await;
}

/// `TestParseSinceUndFormatBytes`.
#[test]
fn parse_since_und_format_bytes() {
    let at = |text: &str| parse_rfc3339(text).unwrap();
    let now = at("2026-09-30T12:00:00Z");
    let cases = [
        ("30m", Some(now - TimeDelta::minutes(30))),
        ("24h", Some(now - TimeDelta::hours(24))),
        ("7d", Some(now - TimeDelta::days(7))),
        ("1h30m", Some(now - TimeDelta::minutes(90))),
        ("1.5h", Some(now - TimeDelta::minutes(90))),
        ("-1h", Some(now + TimeDelta::hours(1))),
        ("0", Some(now)),
        ("2026-09-29T08:00:00Z", Some(at("2026-09-29T08:00:00Z"))),
        ("", None),
    ];
    for (raw, want) in cases {
        assert_eq!(parse_since(raw, now), Ok(want), "{raw}");
    }
    for raw in ["gestern", "5", "h", "-", "d", "1x", "1.2.3h"] {
        assert!(parse_since(raw, now).is_err(), "{raw}");
    }
    for (n, want) in [(512, "512 B"), (4200, "4,1 KB"), (2_411_724, "2,3 MB")] {
        assert_eq!(format_bytes(n), want);
    }
}

/// `TestToolAufrufeImLog`: Debug beim Start mit gekürzten Argumenten, Info oder Warn am Ende mit Dauer.
#[tokio::test(flavor = "multi_thread")]
async fn tool_aufrufe_im_log() {
    let tmp = tmp_dir();
    let dir = tmp.path().join("logs");
    let (log, error) = applog::open_named(&dir, "test", Arc::new(Store::new(10, None)));
    assert!(error.is_none(), "{error:?}");
    let server = Server::new(Config {
        root: tmp.path().to_path_buf(),
        log: log.logger().clone(),
        ..Config::default()
    });
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    client.call_text("workbench_status", json!({})).await;
    let long = json!({"service": "x".repeat(400)});
    client.call_text("logs_query", long).await;
    server.stop().await.unwrap();
    log.close().unwrap();
    let (starts, ends) = calls_in(&dir.join("test.jsonl"));
    assert_eq!((starts.len(), ends.len()), (2, 2), "{starts:?} {ends:?}");
    // Die Datei schreibt Debug-Einträge nur auf der Stufe DEBUG; ohne `K3C_DEV_LOG_LEVEL` ist das der Standard.
    let args = starts[1]["args"].as_str().unwrap();
    assert!(args.chars().count() == ARGS_RUNES && args.ends_with('…'), "{args}");
    for (end, (level, ok)) in ends.iter().zip([("INFO", true), ("WARN", false)]) {
        assert!(
            end["level"] == level && end["ok"] == ok && !end["ms"].is_null(),
            "{end}"
        );
    }
}

/// Start- und Ende-Einträge der Tool-Aufrufe.
fn calls_in(path: &Path) -> (Vec<Value>, Vec<Value>) {
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|m| m["ns"] == "mcp" && !m["tool"].is_null())
        .partition(|m| m["level"] == "DEBUG")
}
