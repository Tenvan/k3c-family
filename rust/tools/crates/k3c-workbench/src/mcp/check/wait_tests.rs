//! Port von `check_wait_test.go` (B-520).

use std::sync::{Arc, Mutex};
use std::time::Duration;

use k3c_kit::mcp::Progress;
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

use super::run::{RunResult, RunSpec};
use super::testutil::{Fake, Gate, hooks, run_id};

fn fast(run: impl Fn(RunSpec) -> RunResult + Send + Sync + 'static) -> Fake {
    let (mut hooks, started) = hooks(run);
    hooks.wait = Duration::from_millis(5);
    hooks.progress_every = Duration::from_millis(2);
    Fake::with(hooks, started, None)
}

/// Ein Aufruf mit `progressToken`: jede Meldung landet im Empfänger.
fn listening() -> (Progress, UnboundedReceiver<String>) {
    let (tx, rx) = unbounded_channel();
    let tx = Mutex::new(tx);
    let progress = Progress::from_fn(Arc::new(move |msg: &str, _| {
        let _ = tx.lock().unwrap().send(msg.to_owned());
    }));
    (progress, rx)
}

#[tokio::test(flavor = "multi_thread")]
async fn mit_fortschritt_wartet_bis_zum_ende() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = fast(move |spec| {
        held.wait();
        (spec.out)("stderr", "--- FAIL: TestKaputt (0.00s)");
        RunResult {
            exit: 1,
            dur: Duration::from_secs(180),
            ..RunResult::default()
        }
    });
    let (progress, mut got) = listening();
    let listener = tokio::spawn(async move {
        for _ in 0..2 {
            let msg = got.recv().await.unwrap();
            assert!(msg.contains("task:check:diff läuft seit"), "{msg}");
        }
        gate.open();
    });
    let text = fake.run_with("task:check:diff", &progress).await.unwrap();
    assert!(text.contains("task:check:diff · exit 1"), "{text}");
    assert!(text.contains("FAIL: TestKaputt"), "{text}");
    listener.await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn run_wartet_auf_das_ende() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = fast(move |_| {
        held.wait();
        RunResult {
            dur: Duration::from_secs(300),
            ..RunResult::default()
        }
    });
    let text = fake.run("task:check", "").await.unwrap();
    let id = run_id(&text);
    let (progress, mut got) = listening();
    let listener = tokio::spawn(async move {
        got.recv().await.unwrap();
        gate.open();
    });
    let text = fake.resume(&id, &progress).await.unwrap();
    assert!(text.starts_with("task:check · exit 0"), "{text}");
    listener.await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn ohne_fortschritt_nach_wartezeit_lauf_id() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = fast(move |_| {
        held.wait();
        RunResult::default()
    });
    let text = fake.run("task:test", "").await.unwrap();
    gate.open();
    assert!(text.contains("läuft noch"), "{text}");
    assert!(text.contains("weiter warten mit check_run {run:"), "{text}");
}

#[test]
fn ohne_token_kein_melder() {
    assert!(!Progress::none().is_active());
}

/// B-372: Bricht der Aufruf ab, läuft der Lauf zu Ende und bleibt über seine ID abfragbar.
#[tokio::test(flavor = "multi_thread")]
async fn abgebrochener_aufruf_laesst_den_lauf_weiterlaufen() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = Fake::new(move |_| {
        held.wait();
        RunResult::default()
    });
    let cut = tokio::time::timeout(Duration::from_millis(50), fake.run("task:test", "")).await;
    assert!(cut.is_err());
    gate.open();
    let text = fake.resume("L1", &Progress::none()).await.unwrap();
    assert!(text.starts_with("task:test · exit 0"), "{text}");
}
