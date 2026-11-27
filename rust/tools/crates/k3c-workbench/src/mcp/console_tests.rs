//! Port von `tools_console_checkout_test.go` und dem Konsolen-Teil von `TestWorktreeTrenntKonsoleUndDateien`.

use std::path::Path;
use std::sync::Arc;

use chrono::{Local, TimeZone};
use k3c_kit::console::Store;
use serde_json::json;

use super::test_client::Client;
use super::test_helpers::write;
use super::workspace_tests::repo_with_worktree;
use super::{CheckState, Config, Server, checkout_policy};
use crate::testutil::tmp_dir;

fn finished(name: &str) -> CheckState {
    CheckState {
        name: name.to_owned(),
        running: false,
        at: Local.with_ymd_and_hms(2026, 11, 14, 9, 30, 15).unwrap(),
        ms: 0.0,
        exit: 0,
        timed_out: false,
        error: String::new(),
    }
}

async fn started(root: &Path, store: &Arc<Store>) -> Server {
    let server = Server::new(Config {
        root: root.to_path_buf(),
        console: Some(Arc::clone(store)),
        ..Config::default()
    });
    server.start().await.unwrap();
    server
}

async fn from(server: &Server, dir: &Path) -> Client {
    Client::connect_from(server.url(), Some(dir.display().to_string())).await
}

/// B-855/AC-01: je Checkout nur der eigene Lauf, die Kopfzeile nennt Checkout und Startzeit, ohne Lauf eine klare
/// Meldung statt fremder Ausgabe.
#[tokio::test(flavor = "multi_thread")]
async fn console_tail_je_checkout() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    let store = Arc::new(Store::new(0, None));
    store.add("check:task:check:go", "stdout", "lauf der Wurzel");
    store.add("check:wt1/task:check:go", "stdout", "lauf des Worktrees");
    let server = started(&main, &store).await;
    server.record_check(finished("wt1/task:check:go"));
    let (mut own, mut root) = (from(&server, &wt).await, from(&server, &main).await);
    let arguments = json!({"service": "check:task:check:go"});
    let (text, is_error) = own.call_text("console_tail", arguments.clone()).await;
    assert!(!is_error && text.contains("lauf des Worktrees"), "{text}");
    assert!(!text.contains("lauf der Wurzel"), "{text}");
    let head = text.lines().next().unwrap();
    assert!(head.contains("Checkout Worktree wt1"), "{head}");
    assert!(head.contains("Lauf ab 09:30:15"), "{head}");
    let (text, _) = root.call_text("console_tail", arguments).await;
    assert!(text.contains("lauf der Wurzel"), "{text}");
    assert!(!text.contains("lauf des Worktrees"), "{text}");
    let arguments = json!({"service": "check:task:check"});
    let (text, is_error) = own.call_text("console_tail", arguments).await;
    assert!(is_error, "{text}");
    assert!(
        text.starts_with("kein Lauf von check:task:check im Checkout Worktree wt1 ("),
        "{text}"
    );
    assert!(checkout_policy::takes_checkout("console_tail"));
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn quellen_cursor_farben_und_fehler() {
    let tmp = tmp_dir();
    let main = tmp.path().join("k3c");
    write(&main.join("go.mod"), "module k3c\n");
    let store = Arc::new(Store::new(0, None));
    let server = started(&main, &store).await;
    let mut client = Client::connect(server.url()).await;
    let (text, is_error) = client.call_text("console_tail", json!({"service": "x"})).await;
    assert!(is_error);
    assert_eq!(text, "unbekannte Quelle \"x\"; bekannt: noch keine");
    store.add("Vite", "stdout", "\u{1b}[32mgrün\u{1b}[0m");
    store.add("Vite", "stderr", "kaputt");
    store.add("Vite", "stdout", "drei");
    store.add("wt9/Vite", "stdout", "fremd");
    let (text, _) = client.call_text("console_tail", json!({"service": "x"})).await;
    assert_eq!(text, "unbekannte Quelle \"x\"; bekannt: Vite");
    let (text, _) = client.call_text("console_tail", json!({"service": "Vite"})).await;
    assert_eq!(text, "Vite · letzte 3 Zeilen (bis Nr. 3)\ngrün\n! kaputt\ndrei");
    let arguments = json!({"service": "Vite", "limit": 1});
    let (text, _) = client.call_text("console_tail", arguments).await;
    assert_eq!(text, "Vite · letzte 1 Zeilen (bis Nr. 3)\ndrei");
    let arguments = json!({"service": "Vite", "since": 1, "limit": 1});
    let (text, _) = client.call_text("console_tail", arguments).await;
    assert_eq!(text, "Vite · letzte 1 Zeilen (bis Nr. 2)\n! kaputt");
    let arguments = json!({"service": "Vite", "since": 3});
    let (text, _) = client.call_text("console_tail", arguments).await;
    assert_eq!(text, "Vite · keine Ausgabe");
    server.stop().await.unwrap();
}

/// Konsolen-Teil von `TestWorktreeTrenntKonsoleUndDateien` (`logs_services` kommt mit RU69.5).
#[tokio::test(flavor = "multi_thread")]
async fn worktree_trennt_konsole() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    let store = Arc::new(Store::new(0, None));
    store.add("check:wt1/task:test", "stdout", "aus dem Worktree");
    store.add("check:task:test", "stdout", "aus der Wurzel");
    let server = started(&main, &store).await;
    let arguments = json!({"service": "check:task:test"});
    let (text, _) = from(&server, &wt)
        .await
        .call_text("console_tail", arguments.clone())
        .await;
    assert!(text.contains("aus dem Worktree"), "{text}");
    let (text, _) = from(&server, &main).await.call_text("console_tail", arguments).await;
    assert!(text.contains("aus der Wurzel"), "{text}");
    server.stop().await.unwrap();
}
