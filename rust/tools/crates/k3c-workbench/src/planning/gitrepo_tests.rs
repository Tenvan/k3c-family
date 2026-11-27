//! Go `gitrepo_test.go` (B-830).

use super::gitrepo::{has_git_entry, in_repo};
use super::worktree::worktrees;
use crate::testutil::tmp_dir;

/// `.git` (Ordner oder Datei) im Ordner oder darüber. Der Temp-Ordner liegt unter `.work/` im Checkout; „ohne `.git`
/// kein Repo“ lässt sich dort nicht prüfen (Go überspringt in dem Fall den ganzen Test).
#[test]
fn git_eintrag_wird_gefunden() {
    let tmp = tmp_dir();
    let repo = tmp.path().join("repo");
    let sub = repo.join("docs").join("backlog");
    std::fs::create_dir_all(&sub).expect("Ordner");
    assert!(!has_git_entry(&repo), "ohne .git als Repo erkannt");
    // Worktree: .git ist eine Datei.
    std::fs::write(repo.join(".git"), "gitdir: x\n").expect(".git");
    assert!(in_repo(&sub) && has_git_entry(&repo) && !has_git_entry(&sub));
    assert!(worktrees(&repo).is_empty(), "kein Repo, keine Worktrees");
}
