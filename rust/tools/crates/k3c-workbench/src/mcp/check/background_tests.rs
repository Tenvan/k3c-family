//! Port von `check_background_test.go` (B-372).

use std::sync::Arc;
use std::time::Duration;

use k3c_kit::mcp::Progress;

use super::compact::FALLBACK_LINES;
use super::job::{Job, MAX_JOBS};
use super::run::RunResult;
use super::targets::find_target;
use super::testutil::{Fake, Gate, hooks, run_id};
use crate::mcp::INSTRUCTIONS;
use crate::mcp::console::TailIn;

/// Fake, der ohne Fortschritt nur `wait` wartet.
fn impatient(wait: Duration, run: impl Fn(super::run::RunSpec) -> RunResult + Send + Sync + 'static) -> Fake {
    let (mut hooks, started) = hooks(run);
    hooks.wait = wait;
    Fake::with(hooks, started, None)
}

async fn job_done(fake: &Fake, id: &str) {
    let job = fake.checks.runs().job(id).unwrap();
    tokio::time::timeout(Duration::from_secs(20), job.done()).await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn langer_lauf_liefert_lauf_id() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = impatient(Duration::from_millis(20), move |spec| {
        held.wait();
        (spec.out)("stderr", "--- FAIL: TestKaputt (0.00s)");
        RunResult {
            exit: 1,
            dur: Duration::from_secs(90),
            ..RunResult::default()
        }
    });
    let text = fake.run("task:check:go", "").await.unwrap();
    assert!(text.contains("läuft noch"), "{text}");
    let id = run_id(&text);
    let want = format!("weiter warten mit check_run {{run: \"{id}\"}}, Ausgabe mit console_tail check:task:check:go");
    assert!(text.ends_with(&want), "{text}");
    let text = fake.resume(&id, &Progress::none()).await.unwrap();
    assert!(text.contains("läuft noch"), "{text}");
    let error = fake.run("task:check:go", "").await.unwrap_err();
    assert!(error.contains("läuft bereits"), "{error}");
    gate.open();
    // Auf das Ende des Laufs warten, nicht auf Zeit.
    job_done(&fake, &id).await;
    let got = fake.resume(&id, &Progress::none()).await.unwrap();
    assert!(got.contains("task:check:go · exit 1"), "{got}");
    assert!(got.contains("FAIL: TestKaputt"), "{got}");
}

#[tokio::test(flavor = "multi_thread")]
async fn unbekannte_lauf_id_und_nur_die_letzten_laeufe() {
    let fake = Fake::new(|_| RunResult::default());
    let error = fake.resume("L99", &Progress::none()).await.unwrap_err();
    assert_eq!(
        error,
        "unbekannte Lauf-ID \"L99\" (gemerkt werden die letzten 20 Läufe seit dem Start von k3c-workbench)"
    );
    for _ in 0..=MAX_JOBS {
        fake.run("task:test", "").await.unwrap();
    }
    assert!(fake.resume("L1", &Progress::none()).await.is_err());
    assert!(fake.resume("L2", &Progress::none()).await.is_ok());
}

#[tokio::test(flavor = "multi_thread")]
async fn kurzer_lauf_antwortet_direkt() {
    let fake = Fake::new(|_| RunResult {
        dur: Duration::from_secs(2),
        ..RunResult::default()
    });
    assert_eq!(fake.run("task:test", "").await.unwrap(), "task:test · exit 0 · 2,0 s");
}

/// B-372/AC-02: Die Server-Anweisungen nennen alle Parameter von `console_tail`.
#[test]
fn instructions_nennen_console_tail_parameter() {
    let line = INSTRUCTIONS
        .lines()
        .find(|l| l.trim().starts_with("- `console_tail {"))
        .expect("keine Zeile zu console_tail in den Anweisungen");
    let schema = serde_json::to_value(schemars::schema_for!(TailIn)).unwrap();
    let properties = schema["properties"].as_object().unwrap();
    assert!(!properties.is_empty());
    for name in properties.keys() {
        assert!(line.contains(name.as_str()), "Parameter {name:?} fehlt: {line:?}");
    }
}

#[test]
fn job_haelt_nur_fehlerzeilen_und_das_ende() {
    let target = find_target("go:test").unwrap();
    let job = Job::new("L1".to_owned(), target, "go:test".to_owned());
    for _ in 0..1000 {
        job.add("ok zeile");
    }
    job.add("--- FAIL: TestX (0.00s)");
    assert_eq!(job.held(), (1, FALLBACK_LINES));
    job.set_result(RunResult {
        exit: 1,
        ..RunResult::default()
    });
    let got = job.answer().unwrap();
    assert!(got.contains("--- FAIL: TestX"), "{got}");
}

#[tokio::test(flavor = "multi_thread")]
async fn laeuft_bereits_nennt_die_lauf_id() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = Arc::new(impatient(Duration::from_millis(10), move |_| {
        held.wait();
        RunResult::default()
    }));
    let text = fake.run("task:test", "").await.unwrap();
    let id = run_id(&text);
    fake.wait_started(1).await;
    let error = fake.run("task:test", "").await.unwrap_err();
    assert_eq!(
        error,
        format!(
            "task:test läuft bereits (Lauf-ID {id}); auf das Ende warten, Ergebnis mit check_run {{run}}, Ausgabe \
             über console_tail check:task:test"
        )
    );
    assert_eq!(fake.started(), 1);
    gate.open();
}
