//! Echte Prozesse über `k3c_kit::proc` (AC-03): Start, Ausgabe, Neustart, Stopp und Übernahme eines eigenen
//! Attrappen-Prozesses; dazu die Programmsuche. Beendet wird nur, was der Test selbst gestartet hat.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use k3c_kit::proc::Cmd;

use super::{exit_of, look_path};
use crate::services::testutil::{free_port, helper, wait_for};
use crate::services::{Cancel, Controller, Options, State};
use crate::testutil::tmp_dir;

#[test]
fn programm_aus_path_mit_endungen() {
    let dir = tmp_dir();
    std::fs::write(dir.path().join("werkzeug.cmd"), "").unwrap();
    let path = OsString::from(dir.path());
    let found = look_path(Path::new("werkzeug"), &path, ".EXE;.CMD");
    assert_eq!(found, dir.path().join("werkzeug.cmd"));
    for unchanged in ["fehlt", "werkzeug.exe", "ordner/werkzeug"] {
        let got = look_path(Path::new(unchanged), &path, ".EXE;.CMD");
        assert_eq!(got, PathBuf::from(unchanged));
    }
}

#[cfg(windows)]
#[test]
fn exit_wie_go() {
    use std::os::windows::process::ExitStatusExt;
    use std::process::ExitStatus;

    assert_eq!(exit_of(Ok(ExitStatus::from_raw(0))), Ok(()));
    let one = exit_of(Ok(ExitStatus::from_raw(1))).unwrap_err();
    assert_eq!((one.code, one.message.as_str()), (1, "exit status 1"));
    let big = exit_of(Ok(ExitStatus::from_raw(0xc000_013a))).unwrap_err();
    assert_eq!(big.message, "exit status 0xc000013a");
}

/// Optionen mit echten Prozessen. `kill_pid` darf nur `own` treffen, ein Supervisor zählt nicht (der Elternprozess der
/// Attrappe ist dieses Test-Binary und würde sonst mit beendet).
fn real(root: &Path, own: &Arc<AtomicU32>) -> Options {
    let own = Arc::clone(own);
    Options {
        root: root.to_path_buf(),
        supervisor: Arc::new(|_| 0),
        kill_pid: Arc::new(move |pid| {
            assert_eq!(pid, own.load(Ordering::SeqCst), "fremder Prozess");
            k3c_kit::proc::kill_tree(pid).map_err(|e| e.to_string())
        }),
        start_poll: std::time::Duration::from_millis(50),
        ..Options::default()
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn echter_prozess_start_neustart_stopp() {
    let root = tmp_dir();
    let port = free_port();
    let own = Arc::new(AtomicU32::new(0));
    let opts = real(root.path(), &own);
    let store = Arc::clone(&opts.console);
    let c = Controller::new(vec![helper("Helfer", port)], opts);
    let ctx = Cancel::default();
    let first = c.start(&ctx, "Helfer").await.unwrap();
    assert_eq!(first.state, State::Running);
    assert_eq!(c.port_owner(port).await, format!("PID {}", first.pid));
    wait_for("Ausgabe", || {
        let lines = store.tail("Helfer", 0).unwrap_or_default();
        lines
            .iter()
            .any(|l| l.stream == "stdout" && l.text.contains("helfer bereit"))
    })
    .await;
    c.sample_all().await;
    assert!(c.statuses()[0].memory > 0);
    let second = c.restart(&ctx, "Helfer").await.unwrap();
    assert_eq!(second.state, State::Running);
    assert_ne!(second.pid, first.pid);
    let st = c.stop("Helfer", false).await.unwrap();
    assert_eq!((st.state, st.pid), (State::Stopped, 0));
    assert_eq!(c.port_owner(port).await, "frei");
    // Die Senke schließt mit dem Prozess.
    let log = root.path().join("logs").join("helfer.jsonl");
    let text = std::fs::read_to_string(log).unwrap();
    assert!(text.contains("helfer bereit"), "{text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn echter_prozess_uebernahme_und_stopp_mit_force() {
    let root = tmp_dir();
    let port = free_port();
    let service = helper("Helfer", port);
    let mut cmd = Cmd::new(&service.command);
    cmd.command_mut()
        .envs(&service.env)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut proc = cmd.start().unwrap();
    let own = Arc::new(AtomicU32::new(proc.id()));
    let c = Controller::new(vec![service], real(root.path(), &own));
    wait_for("Port", || super::super::system::port_listener(port).1).await;
    c.adopt().await;
    let st = c.statuses().remove(0);
    assert_eq!((st.state, st.pid), (State::Adopted, proc.id()), "{st:?}");
    c.stop_all().await;
    assert_eq!(c.statuses()[0].state, State::Adopted);
    let st = c.stop("Helfer", true).await.unwrap();
    assert_eq!(st.state, State::Stopped);
    let status = tokio::time::timeout(std::time::Duration::from_secs(5), proc.wait()).await;
    assert!(!status.unwrap().unwrap().success());
}

/// Review RU69.2: Das Arbeitsverzeichnis (hier der Crate-Ordner mit `Cargo.toml`) ist kein Suchort.
#[test]
fn look_path_ohne_relative_eintraege() {
    let name = Path::new("Cargo");
    for path in ["", ".", ";."] {
        assert_eq!(look_path(name, std::ffi::OsStr::new(path), ".toml"), name, "{path:?}");
    }
}
