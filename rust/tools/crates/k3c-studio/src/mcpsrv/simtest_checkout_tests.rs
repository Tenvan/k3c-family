//! RU40.3: `sim_test start` rechnet mit der EXE des Checkouts aus `checkout`, auch eines Worktrees (Port von
//! `simtest_checkout_test.go`; Attrappe statt EXE, echtes Git-Repo mit Worktrees).

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::json;

use super::super::test_helpers::{connect_mit, git_repo};
use crate::simrun::{Exec, Proc, ProcResult};

fn exec(seen: &Arc<Mutex<Vec<Proc>>>, started: &Arc<AtomicUsize>) -> Exec {
    let (seen, started) = (Arc::clone(seen), Arc::clone(started));
    Arc::new(move |_, proc| {
        started.fetch_add(1, Ordering::SeqCst);
        seen.lock().expect("Liste").push(proc);
        ProcResult::default()
    })
}

fn gleich(a: &PathBuf, b: &PathBuf) -> bool {
    std::fs::canonicalize(a).expect("a") == std::fs::canonicalize(b).expect("b")
}

#[tokio::test(flavor = "multi_thread")]
async fn startet_im_worktree() {
    let Some((repo, wts)) = git_repo(&[".claude/worktrees/wt-a"]) else {
        return;
    };
    let (seen, started) = (Arc::default(), Arc::default());
    let run = exec(&seen, &started);
    let mut server = connect_mit(&repo.pfad, move |studio| {
        studio.sim.unlocked = || true;
        studio.sim.registry.set_exec(run);
    })
    .await;
    let text = server
        .ok(
            "sim_test",
            json!({"action": "start", "checkout": "wt-a", "head": true, "speed": 4}),
        )
        .await;
    assert!(text.contains("head"), "{text}");
    // Lauf zu Ende, bevor der Ordner aufräumt
    server
        .ok("sim_test", json!({"action": "wait", "id": "run-1", "checkout": "wt-a"}))
        .await;
    {
        let seen = seen.lock().expect("Liste");
        let proc = &seen[0];
        assert!(gleich(&proc.dir, &wts[0]), "{}", proc.dir.display());
        assert!(
            proc.head && proc.speed == 4 && proc.args[0] == "balance",
            "{:?}",
            proc.args
        );
    }
    let text = server
        .abgelehnt("sim_test", json!({"action": "start", "speed": 4}))
        .await;
    assert!(text.contains("speed"), "{text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn unbekannter_checkout() {
    let Some((repo, _)) = git_repo(&[".claude/worktrees/wt-a", ".claude/worktrees/wt-b"]) else {
        return;
    };
    let (seen, started): (Arc<Mutex<Vec<Proc>>>, Arc<AtomicUsize>) = (Arc::default(), Arc::default());
    let run = exec(&seen, &started);
    let mut server = connect_mit(&repo.pfad, move |studio| {
        studio.sim.unlocked = || true;
        studio.sim.registry.set_exec(run);
    })
    .await;
    let text = server
        .abgelehnt("sim_test", json!({"action": "start", "checkout": "gibtsnicht"}))
        .await;
    assert!(text.contains("wt-a") && text.contains("wt-b"), "{text}");
    assert_eq!(started.load(Ordering::SeqCst), 0);
}
