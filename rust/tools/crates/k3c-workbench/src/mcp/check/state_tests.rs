//! Port von `check_state_test.go`: Jeder Lauf meldet Beginn und Ende, auch bei gescheitertem Start und Zeitlimit.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{Local, TimeDelta};

use super::CheckState;
use super::run::RunResult;
use super::testutil::{Fake, Gate, hooks};

#[tokio::test(flavor = "multi_thread")]
async fn meldet_beginn_und_ende() {
    let results = [
        RunResult {
            exit: 1,
            dur: Duration::from_millis(12),
            ..RunResult::default()
        },
        RunResult {
            err: Some("task fehlt".to_owned()),
            ..RunResult::default()
        },
        RunResult {
            timed_out: true,
            exit: -1,
            ..RunResult::default()
        },
    ];
    let n = AtomicUsize::new(0);
    let (hooks, started) = hooks(move |_| results[n.fetch_add(1, Ordering::SeqCst)].clone());
    let got = Arc::new(Mutex::new(Vec::<CheckState>::new()));
    let seen = Arc::clone(&got);
    let on_check = Arc::new(move |st: &CheckState| seen.lock().unwrap().push(st.clone()));
    let fake = Fake::with(hooks, started, Some(on_check));
    for target in ["task:test", "task:lint", "task:check"] {
        let _ = fake.run(target, "").await;
    }
    let got = got.lock().unwrap();
    assert_eq!(got.len(), 6, "{got:?}");
    for pair in got.chunks(2) {
        assert!(pair[0].running && !pair[1].running, "{pair:?}");
        assert_eq!(pair[0].name, pair[1].name);
    }
    assert_eq!(
        (got[1].exit, got[3].error.as_str(), got[5].timed_out),
        (1, "task fehlt", true)
    );
    let checks = fake.checks.all();
    assert_eq!(checks.len(), 3, "{checks:?}");
    assert!(checks[0].name == "task:check" && !checks[0].running, "{checks:?}");
}

/// Ein Wiederholungslauf meldet seinen eigenen Beginn, nicht den des vorigen Laufs; `workbench_status` nennt so lange
/// den letzten beendeten.
#[tokio::test(flavor = "multi_thread")]
async fn zeigt_beginn_des_laufenden_laufs() {
    let gate = Gate::default();
    let held = gate.clone();
    let n = AtomicUsize::new(0);
    let fake = Arc::new(Fake::new(move |_| {
        if n.fetch_add(1, Ordering::SeqCst) == 1 {
            held.wait();
        }
        RunResult {
            at: Some(Local::now() - TimeDelta::hours(1)),
            ..RunResult::default()
        }
    }));
    fake.run("task:test", "").await.unwrap();
    let second = Arc::clone(&fake);
    let second = tokio::spawn(async move { second.run("task:test", "").await });
    fake.wait_started(2).await;
    let checks = fake.checks.all();
    let lines = fake.checks.run_lines(&fake.ws);
    gate.open();
    second.await.unwrap().unwrap();
    assert!(checks.len() == 1 && checks[0].running, "{checks:?}");
    assert!(Local::now() - checks[0].at < TimeDelta::minutes(1), "{checks:?}");
    assert!(lines[0].starts_with("Lauf task:test · exit 0 · "), "{lines:?}");
}
