//! Port von `adopt_test.go` (die Prüfungen gegen echte Prozesse stehen in `system_tests.rs` und `runtime_tests.rs`).

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use chrono::{Local, SecondsFormat, Timelike};

use super::controller_tests::controller;
use super::options::Exit;
use super::testutil::{Fake, options, svc, wait_for};
use super::*;
use crate::testutil::tmp_dir;

const LOST: &str = super::adopt::LOST_ADOPTED;

fn first(c: &Controller) -> Status {
    c.statuses().remove(0)
}

#[tokio::test(flavor = "multi_thread")]
async fn uebernahme_und_ablehnungen() {
    let f = Fake::with_pid(4711);
    let c = controller(&f, vec![svc("Spielserver", true)]);
    c.adopt().await;
    let st = first(&c);
    assert_eq!((st.state, st.pid), (State::Adopted, 4711), "{st:?}");
    assert_eq!(st.started_at.map(|t| t.hour()), Some(9));
    let ctx = Cancel::default();
    let error = c.start(&ctx, "Spielserver").await.unwrap_err();
    assert!(error.message.contains("läuft bereits"), "{}", error.message);
    assert!(c.restart(&ctx, "Spielserver").await.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn uebernommen_stopp_nur_mit_force() {
    let f = Fake::with_pid(4711);
    f.busy.store(true, Ordering::SeqCst);
    let c = controller(&f, vec![svc("Spielserver", true)]);
    c.adopt().await;
    c.stop_all().await; // übernommene bleiben
    let error = c.stop("Spielserver", false).await.unwrap_err();
    assert!(error.message.contains("nur mit force"), "{}", error.message);
    assert_eq!(first(&c).state, State::Adopted);
    assert!(f.killed().is_empty());
    let st = c.stop("Spielserver", true).await.unwrap();
    assert_eq!(st.state, State::Stopped);
    assert_eq!((f.killed(), f.count()), (vec![4711], 0));
}

#[tokio::test(flavor = "multi_thread")]
async fn uebernommener_dienst_verschwindet() {
    let f = Fake::with_pid(4711);
    let c = controller(&f, vec![svc("Spielserver", true)]);
    c.adopt().await;
    f.healthy.store(false, Ordering::SeqCst);
    wait_for("gestoppt", || first(&c).state == State::Stopped).await;
    let st = first(&c);
    assert_eq!((st.last_error.as_str(), st.pid, f.count()), (LOST, 0, 0));
}

#[tokio::test(flavor = "multi_thread")]
async fn port_belegt_startet_nicht() {
    for (pid, want) in [
        (8812, "Port 1 bereits belegt (PID 8812)"),
        (0, "Port 1 bereits belegt (PID unbekannt)"),
    ] {
        let f = Fake::new(false);
        f.listen_pid.store(pid, Ordering::SeqCst);
        f.busy.store(true, Ordering::SeqCst);
        let c = controller(&f, vec![svc("Vite", true)]);
        let error = c.start(&Cancel::default(), "Vite").await.unwrap_err();
        let st = error.status.unwrap();
        assert_eq!((st.state, st.last_error.as_str(), f.count()), (State::Failed, want, 0));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn metriken_fuer_laufende_dienste() {
    let f = Fake::new(true);
    let sampled = Arc::new(AtomicU32::new(0));
    let seen = Arc::clone(&sampled);
    let mut opts = options(&f);
    opts.sample = Arc::new(move |pid| {
        seen.store(pid, Ordering::SeqCst);
        Ok(super::testutil::metrics())
    });
    let off = Service {
        port: 2,
        health: "tcp".to_owned(),
        ..svc("Aus", false)
    };
    let c = Controller::new(vec![svc("Vite", false), off], opts);
    c.start(&Cancel::default(), "Vite").await.unwrap();
    // Der lauschende Prozess (node) statt der Hülle (cmd.exe).
    f.listen_pid.store(9999, Ordering::SeqCst);
    f.busy.store(true, Ordering::SeqCst);
    c.sample_all().await;
    let statuses = c.statuses();
    assert_eq!(sampled.load(Ordering::SeqCst), 9999);
    assert_eq!((statuses[0].cpu, statuses[0].memory), (3.1, 480 << 20));
    assert_eq!(statuses[1].memory, 0);
    let st = c.stop("Vite", false).await.unwrap();
    assert_eq!((st.cpu, st.memory), (0.0, 0));
}

#[tokio::test(flavor = "multi_thread")]
async fn log_levels_letzte_stunde() {
    let root = tmp_dir();
    let line = |ago_min: i64, level: &str| {
        let time = (Local::now() - chrono::TimeDelta::minutes(ago_min)).to_rfc3339_opts(SecondsFormat::Secs, false);
        format!("{{\"time\":\"{time}\",\"level\":\"{level}\",\"msg\":\"m\"}}\n")
    };
    let data = line(120, "ERROR") + &line(50, "WARN") + &line(10, "ERROR") + "kein JSON\n" + &line(1, "info");
    std::fs::create_dir_all(root.path().join("logs")).unwrap();
    std::fs::write(root.path().join("logs").join("k3c-server.jsonl"), data).unwrap();
    let server = Service {
        health: "tcp".to_owned(),
        log: "k3c-server".to_owned(),
        ..svc("Server", false)
    };
    let mut opts = options(&Fake::new(true));
    opts.root = root.path().to_path_buf();
    let c = Controller::new(vec![server, svc("Vite", false)], opts);
    let counts = c.log_levels("Server").unwrap().unwrap();
    assert_eq!((counts.total, counts.budget_hit), (3, false), "{counts:?}");
    for level in ["ERROR", "WARN", "INFO"] {
        assert_eq!(counts.counts.get(level), Some(&1), "{counts:?}");
    }
    assert!(c.log_levels("Vite").unwrap().is_none());
    assert!(c.log_levels("Nichts").is_err());
}

/// Das Byte-Budget schneidet ältere Einträge ab und sagt es.
#[test]
fn log_levels_budget() {
    let root = tmp_dir();
    let path = root.path().join("x.jsonl");
    let now = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
    let line = format!("{{\"time\":\"{now}\",\"level\":\"INFO\"}}\n");
    std::fs::write(&path, line.repeat(10)).unwrap();
    let since = Local::now() - chrono::TimeDelta::hours(1);
    let counts = super::levels::count_since(&path, since, line.len() as u64 * 3 + 5).unwrap();
    assert_eq!((counts.total, counts.budget_hit), (3, true));
    let none = super::levels::count_since(&root.path().join("fehlt"), since, 100).unwrap();
    assert_eq!(none.total, 0);
}

fn adopted_options(f: &Arc<Fake>) -> Options {
    f.healthy.store(true, Ordering::SeqCst);
    f.busy.store(true, Ordering::SeqCst);
    let mut opts = options(f);
    opts.stop_timeout = Duration::from_millis(100);
    opts
}

#[tokio::test(flavor = "multi_thread")]
async fn uebernommen_veraltete_pid_wird_nicht_beendet() {
    let f = Fake::with_pid(4711);
    let c = Controller::new(vec![svc("Spielserver", true)], adopted_options(&f));
    c.adopt().await;
    f.listen_pid.store(5000, Ordering::SeqCst); // von Hand neu gestartet
    let error = c.stop("Spielserver", true).await.unwrap_err();
    assert!(error.message.contains("PID 5000 statt 4711"), "{}", error.message);
    assert!(f.killed().is_empty());
    f.busy.store(false, Ordering::SeqCst); // schon weg: nichts zu beenden
    let st = c.stop("Spielserver", true).await.unwrap();
    assert_eq!(st.state, State::Stopped);
    assert!(f.killed().is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn uebernommen_lebt_nach_kill_weiter() {
    let f = Fake::with_pid(4711);
    let c = Controller::new(vec![svc("Spielserver", true)], adopted_options(&f));
    c.adopt().await;
    f.immortal.store(true, Ordering::SeqCst);
    let error = c.stop("Spielserver", true).await.unwrap_err();
    assert!(
        error.message.contains("läuft nach dem Beenden weiter"),
        "{}",
        error.message
    );
    let st = error.status.unwrap();
    assert_eq!((st.state, st.pid), (State::Adopted, 4711));
}

#[tokio::test(flavor = "multi_thread")]
async fn eigener_prozess_endet_nicht() {
    let mut opts = options(&Fake::new(true));
    opts.stop_timeout = Duration::from_millis(50);
    // Lässt sich nicht beenden.
    opts.start = Arc::new(|_, _, _| {
        Ok(Spawned {
            pid: 7,
            kill: Box::new(|| Ok(())),
            wait: Box::pin(std::future::pending::<Exit>()),
        })
    });
    let c = Controller::new(vec![svc("Vite", false)], opts);
    c.start(&Cancel::default(), "Vite").await.unwrap();
    let error = c.stop("Vite", false).await.unwrap_err();
    let st = error.status.unwrap();
    assert_eq!(st.state, State::Failed);
    assert_eq!(st.last_error, "prozess (PID 7) endet nach 50ms nicht");
}

/// Startet der übernommene Server sich selbst neu (neue PID des Kindes, derselbe Supervisor), beendet der Stopp mit
/// force den Supervisor samt Kind; ein fremder Prozess am Port wird nie getroffen.
#[tokio::test(flavor = "multi_thread")]
async fn uebernommener_server_nach_selbst_neustart() {
    let foreign = Some("am Port lauscht jetzt PID 4712 statt 4711");
    let cases = [
        ("derselbe Supervisor", 100, vec![100], None),
        ("anderer Supervisor", 200, vec![], foreign),
        ("Supervisor unbekannt", 0, vec![], foreign),
    ];
    for (name, supervisor_after, want_killed, want_error) in cases {
        let f = Fake::with_pid(4711);
        let supervisor = Arc::new(AtomicU32::new(100));
        let seen = Arc::clone(&supervisor);
        let mut opts = adopted_options(&f);
        opts.supervisor = Arc::new(move |pid| match pid {
            4711 | 4712 => seen.load(Ordering::SeqCst),
            _ => 0,
        });
        let c = Controller::new(vec![svc("Spielserver", true)], opts);
        c.adopt().await;
        f.listen_pid.store(4712, Ordering::SeqCst); // Selbst-Neustart: neues Kind
        supervisor.store(supervisor_after, Ordering::SeqCst);
        let result = c.stop("Spielserver", true).await;
        match want_error {
            None => assert_eq!(result.unwrap().state, State::Stopped, "{name}"),
            Some(want) => {
                let error = result.unwrap_err().message;
                assert!(error.contains(want), "{name}: {error}");
            }
        }
        assert_eq!(f.killed(), want_killed, "{name}");
    }
}
