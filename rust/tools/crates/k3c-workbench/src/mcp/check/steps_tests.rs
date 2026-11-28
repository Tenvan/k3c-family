//! Port von `check_steps_test.go` (B-739).

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::steps::{HEAVY, StepLog, acquire_heavy, is_heavy_target, parse_step_end};
use super::testutil::Records;

#[test]
fn zeit_und_exit() {
    let records = Records::default();
    let base = Instant::now();
    let offset = Arc::new(Mutex::new(Duration::ZERO));
    let seen = Arc::clone(&offset);
    let clock = Arc::new(move || base + *seen.lock().unwrap());
    let log = StepLog::with_clock(records.logger(), "task:check:diff", clock);
    let advance = |ms| *offset.lock().unwrap() += Duration::from_millis(ms);
    log.line("⏱▶ go-test");
    advance(2500);
    log.line("irgendeine Ausgabe");
    log.line("⏱■ go-test exit=0");
    log.line("⏱▶ lint");
    advance(1000);
    log.line("⏱■ lint exit=3");
    let out = records.text();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 4, "{out}");
    assert_eq!(
        lines[1],
        "INFO ✅ schritt beendet ns=\"check\" target=\"task:check:diff\" step=\"go-test\" exit=0 ms=2500"
    );
    assert_eq!(
        lines[3],
        "WARN ❌ schritt beendet ns=\"check\" target=\"task:check:diff\" step=\"lint\" exit=3 ms=1000"
    );
}

#[test]
fn ende_ohne_start() {
    let records = Records::default();
    StepLog::new(records.logger(), "x").line("⏱■ ohne-start exit=1");
    let out = records.text();
    assert!(!out.contains("ms=") && out.contains("step=\"ohne-start\""), "{out}");
}

#[test]
fn zerlegt_das_ende() {
    assert_eq!(parse_step_end("grafik-golden exit=7"), ("grafik-golden", 7));
    assert_eq!(parse_step_end("x"), ("x", -1));
}

#[test]
fn schwere_ziele() {
    assert!(is_heavy_target("task:check:diff") && is_heavy_target("task:check"));
    assert!(!is_heavy_target("task:test"));
}

/// AC-03: Zwei schwere Läufe aus verschiedenen Checkouts laufen nacheinander, der zweite loggt das Warten.
#[tokio::test(flavor = "multi_thread")]
async fn schwere_laeufe_nacheinander() {
    let records = Records::default();
    let log = records.logger();
    let first = acquire_heavy(&log, "task:check:diff", "wurzel").await;
    assert!(first.is_some());
    let second_log = log.clone();
    let second = tokio::spawn(async move {
        acquire_heavy(&second_log, "task:check:diff", "worktree")
            .await
            .is_some()
    });
    for _ in 0..1000 {
        if records.text().contains("wartet auf freien Prüf-Slot") && records.text().contains("worktree") {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert!(
        !second.is_finished(),
        "zweiter Lauf bekam den Slot, obwohl der erste ihn hält"
    );
    // Leichte Ziele warten nie.
    assert!(acquire_heavy(&log, "task:test", "leicht").await.is_none());
    drop(first);
    let got = tokio::time::timeout(Duration::from_secs(20), second).await;
    assert!(got.unwrap().unwrap());
    let out = records.text();
    assert!(
        out.contains("Prüf-Slot frei ns=\"check\" target=\"worktree\" ms="),
        "{out}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn abgebrochenes_warten_belegt_nichts() {
    let log = Records::default().logger();
    let first = acquire_heavy(&log, "task:check", "erster").await;
    let cut = tokio::time::timeout(Duration::from_millis(20), acquire_heavy(&log, "task:check", "zweiter")).await;
    assert!(cut.is_err());
    assert_eq!(
        HEAVY.available_permits(),
        0,
        "Slot frei, obwohl der erste Lauf ihn hält"
    );
    drop(first);
}
