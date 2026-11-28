//! Port von `check_targets_test.go` (B-788) und `check_targets_dev_test.go` (B-831).

use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::run::{RunResult, RunSpec};
use super::targets::{CHECK_RUN_DESCRIPTION, find_target};
use super::testutil::Fake;

/// Fake, der den letzten Lauf festhält.
fn recording(exit: i64) -> (Fake, Arc<Mutex<Option<RunSpec>>>) {
    let got = Arc::new(Mutex::new(None));
    let seen = Arc::clone(&got);
    let fake = Fake::new(move |spec| {
        *seen.lock().unwrap() = Some(spec);
        RunResult {
            exit,
            dur: Duration::from_millis(2300),
            ..RunResult::default()
        }
    });
    (fake, got)
}

fn command(got: &Mutex<Option<RunSpec>>) -> String {
    got.lock().unwrap().as_ref().unwrap().args.join(" ")
}

#[tokio::test(flavor = "multi_thread")]
async fn teilaufgaben_von_check_diff() {
    for (target, cmd, exit) in [
        ("task:check:diff:plan", "task check:diff:plan", 0),
        ("task:check:diff:js", "task check:diff:js", 1),
        ("task:check:diff:go", "task check:diff:go", 0),
    ] {
        let (fake, got) = recording(exit);
        let text = fake.run(target, "").await.unwrap();
        assert_eq!(command(&got), cmd);
        let want = format!("{target} · exit {exit} · 2,3 s");
        assert!(text.starts_with(&want), "{text}");
    }
    for part in [
        "Basis HEAD",
        "zwei Stufen",
        "task:check:diff:plan",
        "task:check:diff:js",
        "task:check:diff:go",
        "task:check:dev",
    ] {
        assert!(CHECK_RUN_DESCRIPTION.contains(part), "{part}");
    }
    assert!(!CHECK_RUN_DESCRIPTION.contains("origin/main"));
    let error = find_target("task:check:diff:rust").unwrap_err();
    assert!(error.contains("task:check:diff:go"), "{error}");
}

#[tokio::test(flavor = "multi_thread")]
async fn workbench_ziele() {
    for (target, pattern, cmd) in [
        ("dev:test", "", "task k3c-workbench:gotest"),
        (
            "dev:test",
            "TestX",
            "task k3c-workbench:gotest GO_TEST_FLAGS=-run='TestX'",
        ),
        ("task:check:dev", "", "task check:dev"),
    ] {
        let (fake, got) = recording(0);
        let error = fake.run(target, pattern).await.unwrap_err();
        assert!(error.contains("task install"), "{target} ohne node_modules: {error}");
        assert_eq!(fake.started(), 0);
        fake.add_frontend_deps();
        fake.run(target, pattern).await.unwrap();
        assert_eq!(command(&got), cmd);
        let spec = got.lock().unwrap().clone().unwrap();
        assert!(spec.timeout >= Duration::from_secs(20 * 60), "{:?}", spec.timeout);
        assert_eq!(spec.dir, fake.ws.root);
    }
}
