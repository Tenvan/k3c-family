//! Port von `workspace_test.go` (`TestFindRoot`, `TestWorkspaceAusHeader`, `TestSchreibendesToolNenntCheckout` mit der
//! Attrappe `plan_section`). Die Worktree-Tests mit Diensten und Konsole stehen in `worktree_tests.rs` und
//! `console_tests.rs`.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use k3c_kit::applog;
use k3c_kit::console::Store;
use k3c_kit::mcp::{find_root, same_path};
use serde_json::json;

use super::Config;
use super::test_client::Client;
use super::test_helpers::{WRITTEN, started, write};
use crate::testutil::tmp_dir;

/// Repo-Wurzel mit einem Worktree wt1, wie `git worktree add` sie schreibt, und ein fremdes Repo daneben.
pub(super) fn repo_with_worktree(base: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let main = base.join("k3c");
    let foreign = base.join("anders");
    let wt = main.join(".claude").join("worktrees").join("wt1");
    for dir in [&main, &wt, &foreign] {
        write(&dir.join("go.mod"), "module k3c\n");
    }
    let gitdir = |p: PathBuf| format!("gitdir: {}\n", p.display().to_string().replace('\\', "/"));
    write(
        &wt.join(".git"),
        &gitdir(main.join(".git").join("worktrees").join("wt1")),
    );
    let other = foreign.join("x").join(".git").join("worktrees").join("y");
    write(&foreign.join(".git"), &gitdir(other));
    (main, wt, foreign)
}

async fn status_from(url: &str, dir: Option<&Path>) -> (String, bool) {
    let root = dir.map(|d| d.display().to_string());
    let mut client = Client::connect_from(url.to_owned(), root).await;
    client.call_text("workbench_status", json!({})).await
}

#[test]
fn find_root_sucht_das_go_mod_des_spiels() {
    let tmp = tmp_dir();
    let root = tmp.path().join("k3c");
    let sub = root.join("tools").join("k3c-workbench");
    write(&root.join("go.mod"), "module k3c\n\ngo 1.27.0\n");
    write(&sub.join("go.mod"), "module k3c/tools/k3c-workbench\n");
    assert!(same_path(find_root(&sub).unwrap(), &root));
    // Der Temp-Ordner liegt im Repo; die Laufwerkswurzel hat sicher kein go.mod.
    let drive: PathBuf = std::env::current_dir().unwrap().components().take(2).collect();
    assert!(find_root(drive).is_err(), "ohne go.mod kein Fehler");
}

#[tokio::test(flavor = "multi_thread")]
async fn workspace_aus_header() {
    let tmp = tmp_dir();
    let (main, wt, foreign) = repo_with_worktree(tmp.path());
    let server = started(Config {
        root: main.clone(),
        ..Config::default()
    })
    .await;
    let url = server.url();
    let label = format!("Repo-Wurzel {}", main.display());
    let (text, _) = status_from(&url, Some(&wt.join("tools"))).await;
    assert!(text.contains("Worktree wt1"), "Header aus Unterordner: {text}");
    let (text, _) = status_from(&url, Some(&main)).await;
    assert!(text.contains(&label), "Header mit der Wurzel: {text}");
    let (text, is_error) = status_from(&url, Some(&foreign)).await;
    assert!(is_error && text.contains("kein Worktree von"), "fremdes Repo: {text}");
    let (text, is_error) = status_from(&url, main.parent()).await;
    assert!(is_error, "Ordner ohne go.mod nicht abgelehnt: {text}");
    // Ohne Header bleibt es bei der Repo-Wurzel.
    let (text, _) = status_from(&url, None).await;
    assert!(text.contains(&label), "ohne Zuordnung: {text}");
    server.stop().await.unwrap();
}

/// B-275/AC-03: Ein schreibendes Tool ohne Header schreibt in die Wurzel und nennt sie; mit Header auf einen Worktree
/// nur dort. Das Log vermerkt je Aufruf, ob der Header da war.
#[tokio::test(flavor = "multi_thread")]
async fn schreibendes_tool_nennt_checkout() {
    let tmp = tmp_dir();
    let (main, wt, _) = repo_with_worktree(tmp.path());
    let logs = tmp.path().join("logs");
    let (log, error) = applog::open_named(&logs, "test", Arc::new(Store::new(10, None)));
    assert!(error.is_none(), "{error:?}");
    let server = started(Config {
        root: main.clone(),
        log: log.logger().clone(),
        ..Config::default()
    })
    .await;
    let url = server.url();
    let mut plain = Client::connect(url.clone()).await;
    let (text, is_error) = plain.call_text("plan_section", json!({"text": "aus der Wurzel"})).await;
    let label = format!("Checkout: Repo-Wurzel {}", main.display());
    assert!(!is_error && text.contains(&label), "ohne Header: {text}");
    let mut headed = Client::connect_from(url, Some(wt.display().to_string())).await;
    let (text, is_error) = headed
        .call_text("plan_section", json!({"text": "aus dem Worktree"}))
        .await;
    assert!(
        !is_error && text.contains("Checkout: Worktree wt1"),
        "mit Header: {text}"
    );
    let read = |dir: &Path| std::fs::read_to_string(dir.join(WRITTEN)).unwrap();
    assert_eq!(
        (read(&main), read(&wt)),
        ("aus der Wurzel".to_owned(), "aus dem Worktree".to_owned())
    );
    server.stop().await.unwrap();
    log.close().unwrap();
    let written = std::fs::read_to_string(logs.join("test.jsonl")).unwrap();
    let lines: Vec<&str> = written
        .lines()
        .filter(|l| l.contains(r#""tool":"plan_section""#) && l.contains(r#""header""#))
        .collect();
    assert_eq!(lines.len(), 2, "{written}");
    assert!(
        lines[0].contains(r#""header":false"#) && lines[0].contains(r#""checkout":"Repo-Wurzel "#),
        "{written}"
    );
    assert!(
        lines[1].contains(r#""header":true"#) && lines[1].contains(r#""checkout":"Worktree wt1 ("#),
        "{written}"
    );
}
