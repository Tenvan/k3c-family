//! Gestellte Prozesse und Prüfungen (aus `controller_test.go`) und der Attrappen-Prozess für echte Läufe. Alle Nähte
//! sind gestellt: ein Test beendet nie einen fremden Prozess.

use std::collections::BTreeMap;
use std::net::{Ipv4Addr, TcpListener};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{Local, TimeZone};
use k3c_kit::logger::{Level, Logger, Record, Sink, value_string};
use tokio::sync::watch;

use super::config::Service;
use super::options::{Options, OutFn, Spawned};
use super::system::Metrics;

/// Gestellter Prozess: `exit` beendet ihn von selbst, `kill` von außen.
pub(crate) struct FakeProc {
    done: watch::Sender<bool>,
    pub killed: AtomicBool,
}

impl FakeProc {
    pub fn exit(&self) {
        self.done.send_replace(true);
    }
}

/// Hält gestellten Start und gestellte Prüfung.
#[derive(Default)]
pub(crate) struct Fake {
    procs: Mutex<Vec<Arc<FakeProc>>>,
    pub healthy: AtomicBool,
    /// Fremder Prozess am Port.
    pub busy: AtomicBool,
    pub listen_pid: AtomicU32,
    /// Aufrufe von `kill_pid` (übernommene Prozesse).
    pub killed: Mutex<Vec<u32>>,
    /// `kill_pid` wirkt nicht.
    pub immortal: AtomicBool,
}

impl Fake {
    pub fn new(healthy: bool) -> Arc<Fake> {
        let f = Fake::default();
        f.healthy.store(healthy, Ordering::SeqCst);
        Arc::new(f)
    }

    pub fn with_pid(pid: u32) -> Arc<Fake> {
        let f = Fake::new(true);
        f.listen_pid.store(pid, Ordering::SeqCst);
        f
    }

    pub fn last(&self) -> Arc<FakeProc> {
        Arc::clone(self.procs.lock().unwrap().last().expect("noch kein Prozess"))
    }

    pub fn count(&self) -> usize {
        self.procs.lock().unwrap().len()
    }

    pub fn killed(&self) -> Vec<u32> {
        self.killed.lock().unwrap().clone()
    }

    fn start(&self, svc: &Service, out: &OutFn) -> Result<Spawned, String> {
        if svc.command[0] == "kaputt" {
            return Err("gibt es nicht".to_owned());
        }
        out("stdout", &format!("hallo von {}", svc.name));
        let mut procs = self.procs.lock().unwrap();
        let pid = 1000 + u32::try_from(procs.len()).unwrap();
        let (done, mut ended) = watch::channel(false);
        let proc = Arc::new(FakeProc {
            done,
            killed: AtomicBool::new(false),
        });
        procs.push(Arc::clone(&proc));
        Ok(Spawned {
            pid,
            kill: Box::new(move || {
                proc.killed.store(true, Ordering::SeqCst);
                proc.exit();
                Ok(())
            }),
            wait: Box::pin(async move {
                let _ = ended.wait_for(|done| *done).await;
                Ok(())
            }),
        })
    }
}

pub(crate) fn metrics() -> Metrics {
    Metrics {
        cpu: 3.1,
        memory: 480 << 20,
        started: Some(Local.with_ymd_and_hms(2026, 9, 30, 9, 0, 0).unwrap()),
    }
}

/// Optionen mit gestellten Nähten und kurzen Zeiten.
pub(crate) fn options(f: &Arc<Fake>) -> Options {
    let (start, check, listen, kill) = (Arc::clone(f), Arc::clone(f), Arc::clone(f), Arc::clone(f));
    Options {
        start: Arc::new(move |svc: &Service, _: &Path, out: OutFn| start.start(svc, &out)),
        check: Arc::new(move |_: &Service| {
            if check.healthy.load(Ordering::SeqCst) {
                Ok(())
            } else {
                Err("antwortet nicht".to_owned())
            }
        }),
        listen: Arc::new(move |_| {
            (
                listen.listen_pid.load(Ordering::SeqCst),
                listen.busy.load(Ordering::SeqCst),
            )
        }),
        sample: Arc::new(|_| Ok(metrics())),
        kill_pid: Arc::new(move |pid| {
            kill.killed.lock().unwrap().push(pid);
            if !kill.immortal.load(Ordering::SeqCst) {
                kill.busy.store(false, Ordering::SeqCst); // der Port wird frei
            }
            Ok(())
        }),
        supervisor: Arc::new(|_| 0),
        start_poll: Duration::from_millis(2),
        start_timeout: Duration::from_millis(500),
        watch_every: Duration::from_millis(5),
        stop_timeout: Duration::from_secs(1),
        ..Options::default()
    }
}

pub(crate) fn svc(name: &str, auto_restart: bool) -> Service {
    Service {
        name: name.to_owned(),
        command: vec!["x".to_owned()],
        port: 1,
        health: "http".to_owned(),
        auto_restart,
        ..Service::default()
    }
}

/// Wartet, bis `cond` gilt.
pub(crate) async fn wait_for(what: &str, mut cond: impl FnMut() -> bool) {
    for _ in 0..1000 {
        if cond() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    panic!("wartete vergeblich auf: {what}");
}

/// Senke, die Einträge wie slogs Textform sammelt.
#[derive(Clone, Default)]
pub(crate) struct Captured(Arc<Mutex<String>>);

impl Captured {
    pub fn logger(&self) -> Logger {
        Logger::new(vec![Box::new(self.clone())])
    }

    pub fn text(&self) -> String {
        self.0.lock().unwrap().clone()
    }
}

impl Sink for Captured {
    fn enabled(&self, _: Level) -> bool {
        true
    }

    fn handle(&self, r: &Record) -> std::io::Result<()> {
        let mut text = self.0.lock().unwrap();
        text.push_str(&r.msg);
        for (key, value) in &r.attrs {
            text.push_str(&format!(" {key}={}", value_string(value)));
        }
        text.push('\n');
        Ok(())
    }
}

/// Ein freier Port auf 127.0.0.1.
pub(crate) fn free_port() -> u16 {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    listener.local_addr().unwrap().port()
}

/// Dienst, der dieses Test-Binary als Attrappe startet ([`helper_process`]); `port` 0: sie lauscht auf irgendeinem.
pub(crate) fn helper(name: &str, port: u16) -> Service {
    let exe = std::env::current_exe().unwrap().to_string_lossy().into_owned();
    let args = ["services::testutil::helper_process", "--exact", "--nocapture"];
    let mut command = vec![exe];
    command.extend(args.map(str::to_owned));
    Service {
        name: name.to_owned(),
        command,
        cwd: ".".to_owned(),
        port: i64::from(port),
        health: "tcp".to_owned(),
        env: BTreeMap::from([
            ("K3C_HELPER".to_owned(), "listen".to_owned()),
            ("K3C_HELPER_PORT".to_owned(), port.to_string()),
        ]),
        ..Service::default()
    }
}

/// Kein Test: mit `K3C_HELPER=listen` öffnet das Test-Binary einen Port, meldet sich und wartet.
#[test]
fn helper_process() {
    if std::env::var("K3C_HELPER").as_deref() != Ok("listen") {
        return;
    }
    let port = std::env::var("K3C_HELPER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(0);
    let _listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port)).unwrap();
    println!("helfer bereit");
    // Nie endlos, falls ein Test ihn verliert.
    std::thread::sleep(Duration::from_secs(60));
}
