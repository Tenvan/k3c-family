//! Port von `repo_test.go`: echte Repos, damit Squash-Erkennung und Löschen gegen git selbst geprüft sind, nicht gegen
//! erfundene Ausgaben.

use std::collections::HashMap;
use std::sync::Arc;

use super::fixture_tests::Fixture;
use super::*;

fn by_name(list: Vec<Branch>) -> HashMap<String, Branch> {
    list.into_iter().map(|b| (b.name.clone(), b)).collect()
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|n| (*n).to_owned()).collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn branches_erkennt_squash_und_gone() {
    let f = Fixture::setup().await;
    let r = f.repo();
    let before = by_name(r.branches().await.unwrap());
    let sq = &before["squash"];
    // Die teure Prüfung läuft nur auf Knopfdruck.
    assert!(
        sq.state == STATE_OPEN && !sq.checked && !sq.deletable,
        "ohne Prüfung: {sq:?}"
    );
    r.check(&[], &[]).await.unwrap();
    let b = by_name(r.branches().await.unwrap());
    let checks = [
        ("main", STATE_BASE, TRACK_SYNC, false), // ausgecheckt, aber Basis geht vor
        ("squash", STATE_SQUASH, TRACK_GONE, true),
        ("merged", STATE_MERGED, TRACK_NONE, true),
        ("offen", STATE_OPEN, TRACK_NONE, false),
    ];
    for (name, state, track, deletable) in checks {
        let got = &b[name];
        assert_eq!(
            (got.state, got.track, got.deletable),
            (state, track, deletable),
            "{name}"
        );
    }
    assert!(
        b["offen"].base_ahead == 1 && b["offen"].checked,
        "offen: {:?}",
        b["offen"]
    );
}

/// Sammel-PR: main bekam den Branch zusammen mit anderem in einem Squash und änderte die Datei danach weiter. git allein
/// erkennt das nicht (merge-tree Konflikt, Squash-Patch anders), der gemergte PR mit gleichem Head schon.
#[tokio::test(flavor = "multi_thread")]
async fn check_erkennt_sammel_pr() {
    let f = Fixture::setup().await;
    let (r, work) = (f.repo(), f.work());
    f.sh(&work, &["checkout", "-q", "-b", "viapr"]).await;
    f.commit(&work, "a.txt", "2").await;
    let sha = f.sh(&work, &["rev-parse", "HEAD"]).await;
    f.sh(&work, &["checkout", "-q", "main"]).await;
    f.sh(&work, &["merge", "-q", "--squash", "viapr"]).await;
    std::fs::write(work.join("anderes.txt"), "x").unwrap();
    f.sh(&work, &["add", "anderes.txt"]).await;
    f.sh(&work, &["commit", "-q", "-m", "Sammel-PR (#9)"]).await;
    f.commit(&work, "a.txt", "3").await;
    f.sh(&work, &["push", "-q", "origin", "main"]).await;
    f.sh(&work, &["fetch", "-q"]).await;

    r.check(&names(&["viapr"]), &[]).await.unwrap();
    let v = by_name(r.branches().await.unwrap()).remove("viapr").unwrap();
    assert!(v.state == STATE_OPEN && v.checked && !v.deletable, "nur git: {v:?}");
    let pr = |number, head: &str| MergedPr {
        number,
        head: head.to_owned(),
        sha: sha.clone(),
    };
    r.check(&names(&["viapr"]), &[pr(3, "anderer"), pr(9, "viapr")])
        .await
        .unwrap();
    let v = by_name(r.branches().await.unwrap()).remove("viapr").unwrap();
    assert!(v.state == STATE_SQUASH && v.pr == 9 && v.deletable, "mit PR: {v:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn delete_nur_gemergte() {
    let f = Fixture::setup().await;
    let (r, work) = (f.repo(), f.work());
    assert!(
        r.delete(&names(&["squash"]), false).await.is_err(),
        "ungeprüfter Squash-Branch darf nicht löschbar sein"
    );
    r.check(&[], &[]).await.unwrap();
    for bad in [
        &["offen"][..],
        &["main"],
        &["gibtsnicht"],
        &["-D"],
        &["squash", "offen"],
        &[],
    ] {
        assert!(r.delete(&names(bad), false).await.is_err(), "delete({bad:?})");
    }
    let left = f.sh(&work, &["branch", "--list", "squash"]).await;
    assert!(left.contains("squash"), "abgelehnter Aufruf darf nichts löschen");
    let message = r.delete(&names(&["squash", "merged"]), true).await.unwrap();
    assert!(message.starts_with("2 Branches gelöscht: squash ("), "{message}");
    assert!(message.contains("merged ("), "{message}");
    assert_eq!(f.sh(&work, &["branch", "--list", "squash", "merged"]).await, "");
}

/// Review RU69.6: `update-ref -d` prüft keinen Checkout; ausgecheckte Branches schützt nur `delete` selbst.
#[tokio::test(flavor = "multi_thread")]
async fn delete_nie_ausgecheckte_branches() {
    let f = Fixture::setup().await;
    let (r, work) = (f.repo(), f.work());
    r.check(&[], &[]).await.unwrap();
    f.sh(&work, &["checkout", "-q", "merged"]).await;
    assert!(r.delete(&names(&["merged"]), false).await.is_err(), "aktueller Branch");
    f.sh(&work, &["checkout", "-q", "main"]).await;
    let other = work.with_file_name("zweiter");
    let other_arg = other.display().to_string();
    f.sh(&work, &["worktree", "add", "-q", &other_arg, "merged"]).await;
    assert!(
        r.delete(&names(&["merged"]), false).await.is_err(),
        "Branch im Worktree"
    );
    let left = f.sh(&work, &["branch", "--list", "merged"]).await;
    assert!(left.contains("merged"), "{left}");
}

/// Rev. 5: Der Fingerabdruck bleibt ohne Änderung gleich und ändert sich bei Commit, Stage und Fetch.
#[tokio::test(flavor = "multi_thread")]
async fn stamp_folgt_dem_repo() {
    let f = Fixture::setup().await;
    let (r, work) = (f.repo(), f.work());
    let a = r.stamp().await.unwrap();
    assert_eq!(a.len(), 16, "{a}");
    assert_eq!(a, r.stamp().await.unwrap(), "ohne Änderung");

    let before = r.stamp().await.unwrap();
    f.commit(&work, "neu.txt", "n").await;
    assert_ne!(r.stamp().await.unwrap(), before, "commit");

    let before = r.stamp().await.unwrap();
    std::fs::write(work.join("x.txt"), "x").unwrap();
    f.sh(&work, &["add", "x.txt"]).await;
    assert_ne!(r.stamp().await.unwrap(), before, "stage");

    let before = r.stamp().await.unwrap();
    f.sh(
        &work,
        &["fetch", "-q", "--force", "origin", "main:refs/remotes/origin/probe"],
    )
    .await;
    assert_ne!(r.stamp().await.unwrap(), before, "fetch");
}

/// Review B-580 (1): Scheitert die Messung zu main, darf nichts als gemergt und löschbar gelten.
#[tokio::test(flavor = "multi_thread")]
async fn messfehler_gilt_nie_als_gemergt() {
    let f = Fixture::setup().await;
    let real = f.runner(f.work());
    let broken: Runner = Arc::new(move |args: Vec<String>| {
        if args[0] == "rev-list" {
            return Box::pin(async { Err("index.lock".to_owned()) });
        }
        real(args)
    });
    let r = Repo::with_runner(f.work(), broken);
    for b in r.branches().await.unwrap() {
        assert!(
            !b.deletable && b.state != STATE_MERGED && b.state != STATE_SQUASH,
            "{} trotz Messfehler: {b:?}",
            b.name
        );
    }
    assert!(
        r.delete(&names(&["merged"]), false).await.is_err(),
        "delete trotz Messfehler"
    );
}

/// Review B-580 (2): Liegen auf origin Commits, die main fehlen, bleibt der Remote-Branch und auch der lokale.
#[tokio::test(flavor = "multi_thread")]
async fn remote_mit_fremden_commits_bleibt() {
    let f = Fixture::setup().await;
    let (r, work) = (f.repo(), f.work());
    f.sh(&work, &["push", "-q", "-u", "origin", "merged"]).await;
    let base = f.tmp.path().to_owned();
    let other = base.join("other");
    let (origin, clone) = (f.origin().display().to_string(), other.display().to_string());
    f.sh(&base, &["clone", "-q", "-b", "merged", &origin, &clone]).await;
    f.commit(&other, "fremd.txt", "nur auf origin").await;
    f.sh(&other, &["push", "-q", "origin", "merged"]).await;
    f.sh(&work, &["fetch", "-q"]).await;
    let error = r.delete(&names(&["merged"]), true).await.unwrap_err();
    assert!(error.contains("origin hat Commits"), "{error}");
    let remote = f.sh(&work, &["ls-remote", "--heads", "origin", "merged"]).await;
    let local = f.sh(&work, &["branch", "--list", "merged"]).await;
    assert!(
        !remote.is_empty() && !local.is_empty(),
        "Remote- und lokaler Branch müssen bleiben"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn status_und_aktionen() {
    let f = Fixture::setup().await;
    let (r, work) = (f.repo(), f.work());
    let error = r.act(ACT_PUSH).await.unwrap_err();
    assert!(error.contains("main"), "Push auf main muss scheitern: {error}");
    f.sh(&work, &["checkout", "-q", "offen"]).await;
    std::fs::write(work.join("neu.txt"), "n").unwrap();
    let st = r.status().await.unwrap();
    assert!(
        st.branch == "offen" && st.changes.len() == 1 && st.base_ahead == 1 && st.upstream.is_empty(),
        "{st:?}"
    );
    assert!(st.fetched_at > 0, "{st:?}");
    assert_eq!(r.act(ACT_STASH).await.unwrap(), "Änderungen gestasht");
    let st = r.status().await.unwrap();
    assert!(st.changes.is_empty() && st.stashes.len() == 1, "nach Stash: {st:?}");
    assert!(st.stashes[0].subject.contains("workbench "), "{st:?}");
    assert_eq!(r.act(ACT_STASH_POP).await.unwrap(), "Letzter Stash zurückgeholt");

    let message = r.act(ACT_PUSH).await.unwrap();
    assert_eq!(message, "Push fertig, Upstream origin/offen gesetzt");
    let st = r.status().await.unwrap();
    assert!(st.upstream == "origin/offen" && st.ahead == 0, "nach Push: {st:?}");
    assert_eq!(r.act(ACT_PUSH).await.unwrap(), "Push fertig");
    assert_eq!(r.act(ACT_FETCH).await.unwrap(), "Fetch fertig");
    assert_eq!(r.act(ACT_PULL).await.unwrap(), "Pull fertig (Fast-Forward)");
    let files = r.branch_files().await.unwrap();
    assert!(files.files.len() == 1 && files.files[0].path == "o.txt", "{files:?}");
    assert_eq!(r.act("rebase").await.unwrap_err(), "unbekannte Aktion \"rebase\"");
}
