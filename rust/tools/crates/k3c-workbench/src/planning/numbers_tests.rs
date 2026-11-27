//! Go `numbers_test.go` (B-387, B-360). Wegwerf-Repos unter `.work/`, git abgeschottet wie in den Tests von `gitops`:
//! keine geerbten `GIT_*`-Ziele, keine globale oder System-Config, kein Aufstieg ins äußere Repo, nur lokale Protokolle.

use std::path::{Path, PathBuf};

use super::create_fixtures::new_ticket;
use super::edit::set;
use super::fixtures::{doc, populate, repo, values};
use super::testhooks::isolate_git;
use super::testsupport::write;
use super::ticketnr::next_ticket;
use crate::gitops::fixture_tests::Fixture;

/// Planung als Git-Repo mit einem Commit; die Planung dieses Threads ruft git danach abgeschottet auf.
async fn git_repo() -> (Fixture, PathBuf) {
    let f = Fixture::empty();
    let root = f.tmp.path().join("work");
    std::fs::create_dir(&root).expect("Ordner");
    populate(&root);
    f.sh(&root, &["init", "-q"]).await;
    // Sicherung: alles Folgende trifft wirklich das Wegwerf-Repo.
    let top = f.sh(&root, &["rev-parse", "--show-toplevel"]).await;
    assert!(top.ends_with("/work"), "{top}");
    f.sh(&root, &["add", "-A"]).await;
    f.sh(&root, &["commit", "-q", "-m", "init"]).await;
    isolate_git(f.env.clone());
    (f, root)
}

async fn add_worktree(f: &Fixture, root: &Path, name: &str, branch: &str) -> PathBuf {
    let path = f.tmp.path().join(name);
    let target = path.display().to_string();
    f.sh(root, &["worktree", "add", "-q", &target, "-b", branch]).await;
    path
}

fn add_ticket(root: &Path, rel: &str) {
    write(root, &format!("docs/backlog/{rel}"), "# Fremd\n");
}

fn create_next(root: &Path) -> String {
    new_ticket(root, "neu", "Neu", "DEV", "hoch")
}

/// B-387/AC-01 und AC-03: höhere Nummer in einem anderen Worktree, die Antwort nennt den Checkout.
#[tokio::test]
async fn nummer_aus_anderem_worktree() {
    let (f, root) = git_repo().await;
    let other = add_worktree(&f, &root, "wt-zwei", "zwei").await;
    add_ticket(&other, "B-005-fremd.md");
    let out = create_next(&root);
    assert!(out.contains("B-006 angelegt"), "{out}");
    assert!(out.contains("höchste bekannte Nummer B-005: Worktree wt-zwei"), "{out}");
    assert!(!out.contains("Warnung"), "{out}");
}

/// Auch das Archiv eines anderen Worktrees zählt.
#[tokio::test]
async fn nummer_aus_dem_archiv_eines_worktrees() {
    let (f, root) = git_repo().await;
    let other = add_worktree(&f, &root, "wt-archiv", "archiv").await;
    add_ticket(&other, "archiv/B-1009-alt.md");
    let out = create_next(&root);
    assert!(out.contains("B-1010 angelegt"), "{out}");
}

/// B-387/AC-02: ein nicht lesbarer Worktree wird übersprungen und genannt, die Anlage gelingt.
#[tokio::test]
async fn nicht_lesbarer_worktree() {
    let (f, root) = git_repo().await;
    let broken = add_worktree(&f, &root, "wt-kaputt", "kaputt").await;
    std::fs::remove_dir_all(broken.join("docs").join("backlog")).expect("entfernen");
    let out = create_next(&root);
    assert!(out.contains("B-002 angelegt"), "{out}");
    assert!(out.contains("übersprungen: Worktree wt-kaputt nicht lesbar"), "{out}");
}

/// B-387/AC-04: ein Remote-Ref ohne Worktree zählt mit, ohne Fetch; andere Refs zählen nicht.
#[tokio::test]
async fn nummer_aus_remote_ref() {
    let (f, root) = git_repo().await;
    let other = add_worktree(&f, &root, "wt-ref", "nurref").await;
    add_ticket(&other, "B-007-remote.md");
    f.sh(&other, &["add", "-A"]).await;
    f.sh(&other, &["commit", "-q", "-m", "ticket"]).await;
    let head = f.sh(&other, &["rev-parse", "HEAD"]).await;
    f.sh(&root, &["update-ref", "refs/remotes/origin/sprint/k9", &head])
        .await;
    add_ticket(&other, "B-050-fremder-ref.md");
    f.sh(&other, &["add", "-A"]).await;
    f.sh(&other, &["commit", "-q", "-m", "fremd"]).await;
    let head = f.sh(&other, &["rev-parse", "HEAD"]).await;
    f.sh(&root, &["update-ref", "refs/remotes/origin/feature", &head]).await;
    let target = other.display().to_string();
    f.sh(&root, &["worktree", "remove", "--force", &target]).await;
    let out = create_next(&root);
    assert!(out.contains("B-008 angelegt"), "{out}");
    assert!(out.contains("origin/sprint/k9"), "{out}");
}

/// B-403/N1: liegt der Ordner unterhalb eines Repos (keine Wurzel), sagt die Antwort, dass Worktrees und Refs fehlen.
#[tokio::test]
async fn keine_repo_wurzel_wird_genannt() {
    let (_f, root) = git_repo().await;
    let below = root.join("unterordner");
    std::fs::create_dir_all(below.join("docs").join("backlog")).expect("Ordner");
    let (nr, notes) = next_ticket(&below).expect("Nummer");
    assert_eq!(nr, "B-001");
    assert!(notes.join("\n").contains("keine Repo-Wurzel"), "{notes:?}");
}

/// B-360/AC-01: ein Feld, das in der Datei fehlt, wird abgelehnt, die Datei bleibt unverändert.
#[test]
fn fehlendes_feld_wird_abgelehnt() {
    let tmp = repo();
    let root = tmp.path();
    let before = doc(root, "backlog/B-001-alt.md");
    let error = set(root, "B-001", &values(&[("Prio", "hoch")])).expect_err("Feld fehlt");
    assert!(error.contains("Prio"), "{error}");
    assert_eq!(doc(root, "backlog/B-001-alt.md"), before);
}
