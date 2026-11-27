//! Port der Lauf-Tests aus `check_test.go`, dazu die Temp-Variablen des Kindprozesses, die Einträge im eigenen Log
//! (`tools_logs_test.go`) und das Ziel im Aufruf-Log (`stats_target_test.go`). Kindprozess ist immer das Test-Binary
//! selbst ([`helper_process`]), nie `task`, `go` oder `cargo`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::json;

use super::run::{RunResult, RunSpec, run_process};
use super::targets::TEST_TIMEOUT;
use super::testutil::{Fake, Gate, Records, hooks};
use crate::mcp::test_client::Client;
use crate::mcp::{Config, Server};
use crate::testutil::tmp_dir;

const HELPER: &str = "K3C_HELPER";
const TEMP_NAMES: [&str; 3] = ["TMP", "TEMP", "GOTMPDIR"];

fn helper_args() -> Vec<String> {
    let exe = std::env::current_exe().unwrap().display().to_string();
    let name = "mcp::check::run_tests::helper_process";
    [exe.as_str(), "--exact", name, "--nocapture", "--quiet"]
        .map(str::to_owned)
        .to_vec()
}

/// Kein Test: Die Prozess-Tests starten damit das Test-Binary als Kind- bzw. Enkelprozess.
#[test]
fn helper_process() {
    let sleep = || std::thread::sleep(Duration::from_secs(60));
    match std::env::var(HELPER).as_deref() {
        Ok("exit3") => {
            println!("\nvor dem Fehler");
            eprintln!("fehler");
            std::process::exit(3);
        }
        Ok("parent") => {
            // Ein Enkel, der stdout geerbt hat und offen hält; danach wartet dieser Prozess selbst.
            let args = helper_args();
            let child = std::process::Command::new(&args[0])
                .args(&args[1..])
                .env(HELPER, "sleep")
                .spawn();
            println!("\nenkel {}", child.is_ok());
            sleep();
        }
        Ok("sleep") => sleep(),
        Ok("env") => {
            println!();
            for name in TEMP_NAMES {
                println!("{name}={}", std::env::var(name).unwrap_or_default());
            }
        }
        _ => {}
    }
}

async fn helper(args: Vec<String>, mode: &str, timeout: Duration) -> (RunResult, String) {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&lines);
    let dir = tmp_dir();
    let res = run_process(RunSpec {
        dir: dir.path().to_path_buf(),
        args,
        timeout,
        out: Arc::new(move |stream: &str, text: &str| seen.lock().unwrap().push(format!("{stream}:{text}"))),
        env: vec![(HELPER.to_owned(), mode.to_owned())],
    })
    .await;
    let joined = lines.lock().unwrap().join("|");
    (res, joined)
}

#[tokio::test(flavor = "multi_thread")]
async fn exit_und_ausgabe() {
    let (res, lines) = helper(helper_args(), "exit3", Duration::from_secs(60)).await;
    assert_eq!(
        (res.exit, res.err.as_deref(), res.timed_out),
        (3, None, false),
        "{lines}"
    );
    assert!(lines.contains("stdout:vor dem Fehler"), "{lines}");
    assert!(lines.contains("stderr:fehler"), "{lines}");
    // Ein fehlendes Programm ist ein Fehler, auch wenn das Zeitlimit schon vor dem Start abgelaufen wäre (B-702).
    for limit in [Duration::from_secs(60), Duration::from_nanos(1)] {
        let (res, _) = helper(vec!["gibt-es-nicht-k3c".to_owned()], "", limit).await;
        assert!(res.err.is_some() && !res.timed_out, "{limit:?}: {res:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn zeitlimit_beendet_den_prozessbaum() {
    let start = Instant::now();
    let (res, lines) = helper(helper_args(), "parent", Duration::from_secs(2)).await;
    assert!(lines.contains("stdout:enkel true"), "{lines}");
    // Ohne Baum-Ende hielte der Enkel stdout offen und der Lauf wartete zusätzlich 5 s auf die Ausgabe.
    assert!(res.timed_out && res.exit == -1, "{res:?}");
    assert!(start.elapsed() < Duration::from_millis(6000), "{:?}", start.elapsed());
}

/// AC: der Kindprozess eines Laufs bekommt `TMP`, `TEMP` und `GOTMPDIR` unter `.work/` seines Checkouts.
#[tokio::test(flavor = "multi_thread")]
async fn kindprozess_bekommt_temp_unter_work() {
    let (mut hooks, started) = hooks(|_| RunResult::default());
    // Echter Prozess mit allem, was `check_run` gesetzt hat; nur der Befehl ist die Attrappe.
    hooks.run = Arc::new(|mut spec| {
        spec.args = helper_args();
        spec.env.push((HELPER.to_owned(), "env".to_owned()));
        Box::pin(run_process(spec))
    });
    let fake = Fake::with(hooks, started, None);
    let text = fake.run("task:test", "").await.unwrap();
    assert!(text.starts_with("task:test · exit 0 · "), "{text}");
    let dir = fake.ws.root.join(".work").join("tmp");
    assert!(dir.is_dir());
    let lines = fake.console.tail("check:task:test", 0).unwrap();
    for name in TEMP_NAMES {
        let want = format!("{name}={}", dir.display());
        assert!(lines.iter().any(|l| l.text == want), "{want} fehlt: {lines:?}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn lehnt_ab_ohne_prozessstart() {
    let fake = Fake::new(|_| RunResult::default());
    let mut cases = vec![
        ("task:alles".to_owned(), String::new()),
        ("task:lint".to_owned(), "planning".to_owned()),
        ("task:test".to_owned(), "--watch".to_owned()),
        ("go:test".to_owned(), "-v".to_owned()),
    ];
    let long = "a".repeat(101);
    for bad in [";", "&", "`", "\"", "'", "<", ">", "%", "\\", " ", "a b", long.as_str()] {
        cases.push(("task:test".to_owned(), format!("x{bad}")));
    }
    for (target, pattern) in &cases {
        assert!(fake.run(target, pattern).await.is_err(), "{target} {pattern:?}");
    }
    let error = fake.run("task:alles", "").await.unwrap_err();
    assert_eq!(
        error,
        "unbekanntes Ziel \"task:alles\"; gültige Ziele: task:check, task:test, task:typecheck, task:lint, \
         task:build, task:check:go, task:check:diff, task:check:diff:plan, task:check:diff:js, task:check:diff:go, \
         task:check:race, go:test, go:lint, dev:test, task:check:dev"
    );
    assert_eq!(fake.started(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn ein_lauf_je_ziel() {
    let gate = Gate::default();
    let held = gate.clone();
    let fake = Arc::new(Fake::new(move |_| {
        held.wait();
        RunResult::default()
    }));
    let first = Arc::clone(&fake);
    let first = tokio::spawn(async move { first.run("task:test", "").await });
    fake.wait_started(1).await;
    let error = fake.run("task:test", "").await.unwrap_err();
    assert!(error.contains("läuft bereits"), "{error}");
    gate.open();
    assert!(first.await.unwrap().is_ok());
    assert_eq!(fake.started(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn gibt_sperre_nach_panik_frei() {
    let calls = AtomicUsize::new(0);
    let fake = Fake::new(move |_| {
        assert!(calls.fetch_add(1, Ordering::SeqCst) > 0, "kaputt");
        RunResult::default()
    });
    let error = fake.run("task:test", "").await.unwrap_err();
    assert_eq!(error, "task:test ließ sich nicht starten: abgestürzt: kaputt");
    assert!(fake.run("task:test", "").await.is_ok(), "Ziel nach Panik gesperrt");
}

/// Über MCP: Befehl, Antwort, Konsole und `workbench_status` (Go `TestCheckRunMusterUndKonsole`).
#[tokio::test(flavor = "multi_thread")]
async fn muster_und_konsole_ueber_mcp() {
    let got = Arc::new(Mutex::new(None));
    let seen = Arc::clone(&got);
    let (hooks, _) = hooks(move |spec| {
        (spec.out)("stdout", "läuft");
        (spec.out)("stderr", "warnung");
        *seen.lock().unwrap() = Some(spec);
        RunResult {
            dur: Duration::from_millis(12_400),
            ..RunResult::default()
        }
    });
    let root = tmp_dir();
    let config = Config {
        root: root.path().to_path_buf(),
        ..Config::default()
    };
    let server = Server::with_check_hooks(config, hooks);
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    let arguments = json!({"target": "task:test", "pattern": "planning"});
    let (text, is_error) = client.call_text("check_run", arguments).await;
    assert!(!is_error);
    assert_eq!(text, "task:test · exit 0 · 12,4 s");
    {
        let spec = got.lock().unwrap();
        let spec = spec.as_ref().unwrap();
        assert_eq!(spec.args.join(" "), "task test -- planning");
        assert_eq!(spec.timeout, TEST_TIMEOUT);
    }
    let (tail, _) = client
        .call_text("console_tail", json!({"service": "check:task:test"}))
        .await;
    assert!(tail.contains("läuft\n! warnung") && tail.contains("Lauf ab "), "{tail}");
    let (status, _) = client.call_text("workbench_status", json!({})).await;
    assert!(status.contains("Lauf task:test · exit 0 · 12,4 s"), "{status}");
    server.stop().await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn ereignisse_im_eigenen_log_und_ziel_im_aufruf_log() {
    let records = Records::default();
    let (hooks, _) = hooks(|_| RunResult {
        exit: 1,
        ..RunResult::default()
    });
    let root = tmp_dir();
    let config = Config {
        root: root.path().to_path_buf(),
        log: records.logger(),
        ..Config::default()
    };
    let server = Server::with_check_hooks(config, hooks);
    server.start().await.unwrap();
    let mut client = Client::connect(server.url()).await;
    client.call_text("check_run", json!({"target": "task:lint"})).await;
    // Das Ziel steht im Aufruf-Log, auch wenn die Argumente vor ihm abgeschnitten werden (B-876).
    let long = json!({"pattern": "x".repeat(300), "target": "task:alles"});
    let (_, is_error) = client.call_text("check_run", long).await;
    assert!(is_error);
    let calls = server.calls();
    assert_eq!(calls[0].target, "task:alles");
    assert!(!calls[0].args.contains("task:alles"), "{}", calls[0].args);
    let log = records.text();
    assert!(
        log.contains("WARN ❌ lauf beendet ns=\"check\" target=\"task:lint\" exit=1 ms="),
        "{log}"
    );
    assert!(log.contains("❌ check_run: unbekanntes Ziel"), "{log}");
    server.stop().await.unwrap();
}
