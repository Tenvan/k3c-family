//! Port von `check_taskversion_test.go` (B-868, B-871). `task --version` ist immer gestellt.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use super::run::RunResult;
use super::taskversion::{MIN_TASK_VERSION, parse_version, version_text};
use super::testutil::{Fake, hooks};

/// Fake mit gestellter Antwort auf `task --version`; `reads` zählt die Abfragen.
fn with_version(answer: Result<&str, &str>) -> (Fake, Arc<AtomicUsize>) {
    let answer = answer.map(str::to_owned).map_err(str::to_owned);
    let reads = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&reads);
    let (mut hooks, started) = hooks(|_| RunResult::default());
    hooks.task_version = Arc::new(move || {
        count.fetch_add(1, Ordering::SeqCst);
        let answer = answer.clone();
        Box::pin(async move { answer })
    });
    let fake = Fake::with(hooks, started, None);
    fake.add_frontend_deps();
    (fake, reads)
}

#[derive(PartialEq)]
enum Want {
    Rejected,
    Runs,
    Warns,
    /// Läuft, ohne die Version zu lesen.
    Unread,
}

#[tokio::test(flavor = "multi_thread")]
async fn mindestversion() {
    let cases = [
        (
            "task:test",
            "planning",
            Ok("Task version: v3.40.0 (h1:abc)"),
            Want::Rejected,
        ),
        ("task:test", "planning", Ok("3.54.0"), Want::Runs),
        ("task:test", "planning", Ok("4.0"), Want::Runs),
        ("task:test", "planning", Err("nicht gefunden"), Want::Warns),
        ("dev:test", "TestX", Ok("Task"), Want::Warns),
        ("task:test", "", Ok("3.1.0"), Want::Unread),
        ("go:test", "TestX", Ok("3.1.0"), Want::Unread),
    ];
    for (target, pattern, version, want) in cases {
        let (fake, reads) = with_version(version);
        let got = fake.run(target, pattern).await;
        let case = format!("{target} {pattern:?} {version:?}: {got:?}");
        if want == Want::Rejected {
            let error = got.unwrap_err();
            assert!(
                error.contains("ab 3.54.0") && error.contains("gefunden 3.40.0"),
                "{case}"
            );
            assert_eq!(fake.started(), 0, "{case}");
            continue;
        }
        let text = got.unwrap();
        assert_eq!(fake.started(), 1, "{case}");
        assert_eq!(reads.load(Ordering::SeqCst) == 1, want != Want::Unread, "{case}");
        let warned = text.starts_with(
            "⚠ Task-Version nicht lesbar (task --version); Testmuster brauchen mindestens 3.54.0 (requirements.md)\n",
        );
        assert_eq!(warned, want == Want::Warns, "{case}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn fragt_die_version_einmal() {
    for answer in [Ok("3.54.0"), Err("nicht gefunden")] {
        let (fake, reads) = with_version(answer);
        for _ in 0..2 {
            let text = fake.run("task:test", "planning").await.unwrap();
            assert_eq!(text.contains("Task-Version nicht lesbar"), answer.is_err(), "{text}");
        }
        assert_eq!(reads.load(Ordering::SeqCst), 1, "{answer:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn abgebrochener_aufruf_wird_nicht_gemerkt() {
    let reads = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&reads);
    let (mut hooks, started) = hooks(|_| RunResult::default());
    hooks.task_version = Arc::new(move || {
        let first = count.fetch_add(1, Ordering::SeqCst) == 0;
        Box::pin(async move {
            if first {
                std::future::pending::<()>().await;
            }
            Ok("3.54.0".to_owned())
        })
    });
    let fake = Fake::with(hooks, started, None);
    let cut = tokio::time::timeout(Duration::from_millis(50), fake.run("task:test", "planning")).await;
    assert!(cut.is_err());
    fake.run("task:test", "planning").await.unwrap();
    fake.run("task:test", "planning").await.unwrap();
    assert_eq!(reads.load(Ordering::SeqCst), 2);
}

#[test]
fn liest_versionen() {
    assert_eq!(parse_version("Task version: v3.54.0 (h1:x)"), Some([3, 54, 0]));
    assert_eq!(parse_version("4.0"), Some([4, 0, 0]));
    assert_eq!(parse_version("Task"), None);
}

#[test]
fn requirements_nennt_die_mindestversion() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap();
    let text = std::fs::read_to_string(repo.join("requirements.md")).unwrap();
    let row = text
        .lines()
        .find_map(|l| l.strip_prefix("| Go Task | "))
        .and_then(|rest| rest.split('|').next())
        .expect("Zeile Go Task");
    let want = format!("ab {}", version_text(MIN_TASK_VERSION));
    assert!(row.contains(&want), "{row}");
}
