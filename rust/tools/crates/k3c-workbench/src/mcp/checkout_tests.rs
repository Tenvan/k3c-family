//! Port von `checkout_test.go` mit echtem Git-Repo; statt der Planungs-Tools stehen Attrappen gleichen Namens (die
//! echten kommen mit den folgenden Sessions).

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;

use k3c_kit::applog;
use k3c_kit::console::Store;
use k3c_kit::proc::Cmd;
use serde_json::json;

use super::Config;
use super::test_client::Client;
use super::test_helpers::{WRITTEN, started, write};
use crate::testutil::tmp_dir;

/// Variablen, mit denen ein umgebender Git-Aufruf (Hook) die Tests auf ein fremdes Repo lenken könnte.
const GIT_ENV: &[&str] = &[
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_COMMON_DIR",
    "GIT_OBJECT_DIRECTORY",
    "GIT_NAMESPACE",
];

async fn git(repo: &Path, args: &[&str]) {
    let repo = repo.display().to_string();
    let mut all = vec!["git", "-C", &repo, "-c", "user.name=t", "-c", "user.email=t@t"];
    // Ohne die Warnung zu LF und CRLF.
    all.extend_from_slice(&["-c", "core.autocrlf=false"]);
    all.extend_from_slice(args);
    let mut cmd = Cmd::new(&all);
    for name in GIT_ENV {
        cmd.command_mut().env_remove(name);
    }
    cmd.command_mut().stdout(Stdio::null()).stderr(Stdio::inherit());
    let status = cmd.run().await.expect("git starten");
    assert!(status.success(), "git {args:?}: {status}");
}

/// Repo des Spiels mit den Worktrees unter den relativen Pfaden `worktrees` (B-388).
async fn git_repo(base: &Path, worktrees: &[&str]) -> (PathBuf, Vec<PathBuf>) {
    let main = base.join("k3c");
    write(&main.join("go.mod"), "module k3c\n");
    git(&main, &["init", "-q"]).await;
    git(&main, &["add", "."]).await;
    git(&main, &["commit", "-qm", "start"]).await;
    let mut paths = Vec::new();
    for (i, relative) in worktrees.iter().enumerate() {
        let path = relative.split('/').fold(main.clone(), |p, part| p.join(part));
        let branch = format!("b{i}");
        git(
            &main,
            &["worktree", "add", "-q", "-b", &branch, &path.display().to_string()],
        )
        .await;
        paths.push(path);
    }
    (main, paths)
}

fn written(dir: &Path) -> String {
    std::fs::read_to_string(dir.join(WRITTEN)).unwrap_or_default()
}

/// AC-01 bis AC-04: `checkout` (Name oder Pfad) gilt vor dem Header, geschrieben wird nur im genannten Worktree.
#[tokio::test(flavor = "multi_thread")]
async fn checkout_vor_header() {
    let tmp = tmp_dir();
    let (main, worktrees) = git_repo(tmp.path(), &[".claude/worktrees/wt-a"]).await;
    let logs = tmp.path().join("logs");
    let (log, error) = applog::open_named(&logs, "test", Arc::new(Store::new(10, None)));
    assert!(error.is_none(), "{error:?}");
    let server = started(Config {
        root: main.clone(),
        log: log.logger().clone(),
        ..Config::default()
    })
    .await;
    let mut client = Client::connect_from(server.url(), Some(main.display().to_string())).await;
    for argument in ["wt-a".to_owned(), worktrees[0].display().to_string()] {
        let sent = format!("per {argument}");
        let arguments = json!({"text": sent, "checkout": argument});
        let (text, is_error) = client.call_text("plan_section", arguments).await;
        assert!(
            !is_error && text.contains("Checkout: Worktree wt-a"),
            "{argument}: {text}"
        );
        assert_eq!((written(&main), written(&worktrees[0])), (String::new(), sent));
    }
    let (text, is_error) = client.call_text("plan_section", json!({"text": "ohne"})).await;
    assert!(
        !is_error && text.contains("Checkout: Repo-Wurzel"),
        "ohne checkout: {text}"
    );
    server.stop().await.unwrap();
    log.close().unwrap();
    let all = std::fs::read_to_string(logs.join("test.jsonl")).unwrap();
    let lines: Vec<&str> = all
        .lines()
        .filter(|l| l.contains(r#""tool":"plan_section""#) && l.contains(r#""checkout_arg""#))
        .collect();
    assert_eq!(lines.len(), 3, "{all}");
    assert!(lines[0].contains(r#""checkout_arg":true"#) && lines[2].contains(r#""checkout_arg":false"#));
}

/// AC-02: Unbekannt, außerhalb und mehrdeutig wird abgelehnt, ohne zu schreiben; die Meldung nennt die Worktrees.
#[tokio::test(flavor = "multi_thread")]
async fn checkout_ungueltig() {
    let tmp = tmp_dir();
    let (main, worktrees) = git_repo(tmp.path(), &["a/doppelt", "b/doppelt", "c/einzeln"]).await;
    let server = started(Config {
        root: main.clone(),
        ..Config::default()
    })
    .await;
    let mut client = Client::connect(server.url()).await;
    let outside = tmp_dir();
    for (argument, want) in [
        ("gibtsnicht".to_owned(), "einzeln"),
        (outside.path().display().to_string(), "einzeln"),
        ("doppelt".to_owned(), "mehrdeutig"),
        (Path::new("rel").join("x").display().to_string(), "einzeln"),
    ] {
        let arguments = json!({"text": "falsch", "checkout": argument});
        let (text, is_error) = client.call_text("plan_section", arguments).await;
        assert!(is_error && text.contains(want), "{argument}: {text}");
    }
    for dir in worktrees.iter().chain([&main]) {
        assert_eq!(written(dir), "", "geschrieben in {}", dir.display());
    }
    server.stop().await.unwrap();
}

/// `check_run` und `task_start` nehmen `checkout`; Lese-Tools kennen es nicht.
#[tokio::test(flavor = "multi_thread")]
async fn checkout_nur_bei_schreibenden_tools() {
    let tmp = tmp_dir();
    let (main, _) = git_repo(tmp.path(), &[".claude/worktrees/wt-a"]).await;
    let server = started(Config {
        root: main,
        ..Config::default()
    })
    .await;
    let mut client = Client::connect(server.url()).await;
    let arguments = json!({"target": "gibtsnicht", "checkout": "wt-a"});
    let (text, _) = client.call_text("check_run", arguments).await;
    assert!(!text.contains("additional"), "check_run ohne checkout: {text}");
    let arguments = json!({"id": "B-001", "checkout": "wt-a"});
    let (text, is_error) = client.call_text("plan_get", arguments).await;
    assert!(
        is_error && text.contains("gültige Parameter: id"),
        "plan_get mit checkout: {text}"
    );
    // Der Server hat keine Tasks-Seite; wichtig ist, dass `checkout` angenommen wird.
    let arguments = json!({"task": "check", "checkout": "wt-a"});
    let (text, is_error) = client.call_text("task_start", arguments).await;
    assert!(is_error && !text.contains("additional"), "task_start: {text}");
    assert!(text.contains("Checkout: Worktree wt-a"), "task_start: {text}");
    server.stop().await.unwrap();
}
