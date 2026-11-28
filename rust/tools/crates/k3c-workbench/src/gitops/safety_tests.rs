//! Nur in Rust: Werte mit führendem „-“ erreichen git nie, Fehlertexte und Feldnamen wie Go.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde::Serialize;

use super::fixture_tests::Fixture;
use super::*;

/// Repo, dessen Runner nur mitschreibt.
fn recording(seen: &Arc<Mutex<Vec<Vec<String>>>>) -> Repo {
    let seen = Arc::clone(seen);
    let run: Runner = Arc::new(move |args: Vec<String>| {
        seen.lock().unwrap().push(args);
        Box::pin(async { Ok(Vec::new()) })
    });
    Repo::with_runner(PathBuf::from("unbenutzt"), run)
}

#[tokio::test]
async fn optionen_erreichen_git_nie() {
    let seen = Arc::default();
    let r = recording(&seen);
    for bad in ["-D", "--force", "--upload-pack=x"] {
        let error = r.delete(&[bad.to_owned()], true).await.unwrap_err();
        assert_eq!(error, format!("ungültiger Branch-Name {bad:?}"));
        assert!(r.act(bad).await.is_err());
        assert_eq!(r.squashed(bad).await, (false, false));
        assert_eq!(r.ahead_behind(bad, "origin/main").await, None);
    }
    assert!(seen.lock().unwrap().is_empty(), "{:?}", seen.lock().unwrap());
}

#[tokio::test]
async fn fehlertexte_von_git() {
    let f = Fixture::setup().await;
    let r = f.repo();
    // Ohne Fehlerausgabe der Exit-Code; daran erkennt die Prüfung ein „Nein“.
    let error = r
        .git(&["merge-base", "--is-ancestor", "offen", "merged"])
        .await
        .unwrap_err();
    assert_eq!(error, "git merge-base: exit status 1");
    let error = r.git(&["rev-parse", "--verify", "gibtsnicht"]).await.unwrap_err();
    assert!(error.starts_with("git rev-parse: fatal:"), "{error}");
}

fn keys(value: impl Serialize) -> BTreeSet<String> {
    let value = serde_json::to_value(value).unwrap();
    value.as_object().unwrap().keys().cloned().collect()
}

/// Felder des Interface `name` aus `typesGit.ts` (nur die oberste Ebene).
fn ts_fields(source: &str, name: &str) -> BTreeSet<String> {
    let start = format!("export interface {name} {{");
    let body = source.split_once(&start).unwrap().1.split_once("\n}").unwrap().0;
    body.lines()
        .filter_map(|l| l.strip_prefix("  ")?.split_once(':'))
        .map(|(field, _)| field.trim_end_matches('?').to_owned())
        .filter(|field| field.chars().all(char::is_alphanumeric))
        .collect()
}

/// AC-03: die JSON-Form entspricht `GitView` samt Unterformen in `frontend/src/api/typesGit.ts`.
#[test]
fn feldnamen_wie_types_git_ts() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../tools/k3c-workbench/frontend/src/api/typesGit.ts");
    // Die Datei gehört zum Frontend und bleibt auch ohne Go: Fehlt sie, ist das ein Fehler.
    let source = std::fs::read_to_string(&path)
        .expect("typesGit.ts lesen")
        .replace("\r\n", "\n");
    let view = crate::app::GitView {
        status: Status::default(),
        branches: Vec::new(),
        files: Files::empty(),
    };
    assert_eq!(keys(view), ts_fields(&source, "GitView"));
    assert_eq!(keys(Status::default()), ts_fields(&source, "GitStatus"));
    assert_eq!(keys(Change::default()), ts_fields(&source, "GitChange"));
    assert_eq!(keys(Stash::default()), ts_fields(&source, "GitStash"));
    assert_eq!(keys(Worktree::default()), ts_fields(&source, "GitWorktree"));
    assert_eq!(keys(Branch::default()), ts_fields(&source, "GitBranch"));
    assert_eq!(keys(DiffFile::default()), ts_fields(&source, "GitDiffFile"));
    // `files` steht in TS inline.
    let files: BTreeSet<String> = ["base", "files"].map(str::to_owned).into();
    assert_eq!(keys(Files::empty()), files);
    assert!(source.contains("files: { base: string; files: GitDiffFile[] };"));
}
