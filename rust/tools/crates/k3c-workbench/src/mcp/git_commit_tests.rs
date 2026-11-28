//! Port von `tools_git_commit_test.go`, dazu `gh_status` ohne `gh` (aus `roundtrip_test.go`) und die Tools über MCP.
//! Repos sind Wegwerf-Repos unter `.work/` mit abgeschottetem git; das echte `gh` läuft nie.

use std::path::PathBuf;
use std::sync::Arc;

use serde_json::json;

use super::*;
use crate::github;
use crate::gitops::fixture_tests::Fixture;
use crate::mcp::test_client::Client;
use crate::mcp::{Config, Server};

/// Test-Repo mit dem Branch `branch`, ohne Commit.
async fn repo(branch: &str, files: &[&str]) -> (Fixture, PathBuf) {
    let f = Fixture::empty();
    let root = f.tmp.path().join("repo");
    std::fs::create_dir(&root).unwrap();
    f.sh(&root, &["init", "-q", "-b", branch]).await;
    let top = f.sh(&root, &["rev-parse", "--show-toplevel"]).await;
    assert!(top.ends_with("/repo"), "{top}");
    for file in files {
        std::fs::write(root.join(file), "x").unwrap();
    }
    (f, root)
}

fn input(paths: &[&str], scope: &str) -> CommitIn {
    CommitIn {
        paths: paths.iter().map(|p| (*p).to_owned()).collect(),
        kind: "docs".to_owned(),
        scope: scope.to_owned(),
        subject: "Test".to_owned(),
        body: String::new(),
    }
}

/// Auf main wird nicht committet: die Ablehnung nennt den Grund und lässt den Index leer.
#[tokio::test]
async fn lehnt_main_ab() {
    let (f, root) = repo("main", &["a.md"]).await;
    let git = Git::with_env(root, f.env.clone());
    let error = commit_files(&git, input(&["a.md"], "")).await.unwrap_err();
    assert_eq!(error, "auf main wird nicht committet: Feature-Branch anlegen");
    assert!(git.read().await.unwrap().staged.is_empty());
}

/// Eine fremde gestagte Datei im Index lässt den Commit scheitern, damit keine fremde Arbeit mitgeht.
#[tokio::test]
async fn lehnt_fremde_gestagte_datei_ab() {
    let (f, root) = repo("sprint/test", &["fremd.md", "mein.md"]).await;
    f.sh(&root, &["add", "fremd.md"]).await;
    let git = Git::with_env(root, f.env.clone());
    let error = commit_files(&git, input(&["mein.md"], "")).await.unwrap_err();
    assert_eq!(
        error,
        "fremde gestagte Datei im Index: fremd.md (erst entfernen oder mit committen)"
    );
    assert_eq!(git.read().await.unwrap().staged.len(), 1, "Index bleibt");
}

/// Auf einem Feature-Branch committet das Werkzeug genau die genannte Datei im Format `typ(domäne): Betreff`.
#[tokio::test]
async fn committet_auf_feature_branch() {
    let (f, root) = repo("sprint/test", &["mein.md", "andere.md"]).await;
    let git = Git::with_env(root.clone(), f.env.clone());
    let out = commit_files(&git, input(&["mein.md"], "planung")).await.unwrap();
    assert!(out.starts_with("Commit "), "{out}");
    assert!(out.ends_with(" auf sprint/test: docs(planung): Test\n"), "{out}");
    assert_eq!(f.sh(&root, &["log", "-1", "--format=%s"]).await, "docs(planung): Test");
    assert_eq!(
        f.sh(&root, &["show", "--name-only", "--format=", "HEAD"]).await,
        "mein.md"
    );
}

/// Über MCP: der Checkout des Servers, die Argumente nach Schema und die Zeile `Checkout:` eines schreibenden Tools.
#[tokio::test(flavor = "multi_thread")]
async fn git_commit_ueber_mcp() {
    let (f, root) = repo("sprint/test", &["mein.md"]).await;
    let server = Server::new(Config {
        root: root.clone(),
        git_env: f.env.clone(),
        ..Config::default()
    });
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let args = json!({"paths": ["mein.md"], "type": "docs", "subject": "Über MCP", "body": "Rumpf"});
    let (text, failed) = client.call_text("git_commit", args).await;
    assert!(!failed && text.contains(" auf sprint/test: docs: Über MCP"), "{text}");
    assert!(text.contains("Checkout:"), "{text}");
    assert_eq!(
        f.sh(&root, &["log", "-1", "--format=%B"]).await,
        "docs: Über MCP\n\nRumpf"
    );
    server.stop().await.unwrap();
}

/// `TestGhStatusOhneGh`: ohne `gh` ein Hinweis statt eines Fehlers (B-212). Statt den `PATH` des Testprozesses zu
/// leeren, ruft der echte Aufruf ein Programm, das es nicht gibt; so erreicht der Test nie das Netz.
#[tokio::test(flavor = "multi_thread")]
async fn gh_status_ohne_gh() {
    let f = Fixture::empty();
    let run = github::exec_runner(f.tmp.path().to_owned(), "k3c-gibt-es-nicht");
    let server = Server::new(Config {
        root: f.tmp.path().to_owned(),
        github: Some(Arc::new(github::Client::with_runner(run))),
        ..Config::default()
    });
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let (text, failed) = client.call_text("gh_status", json!({"force": true})).await;
    assert!(!failed, "{text}");
    assert_eq!(
        text,
        "Hinweis: gh nicht gefunden: GitHub CLI installieren und `gh auth login` ausführen"
    );
    server.stop().await.unwrap();
}
