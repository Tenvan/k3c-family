//! Port von `workkit/proc/proc_test.go`. Statt `go version` läuft das Test-Binary selbst, damit die Tests ohne Go
//! auskommen.

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use tokio::io::AsyncReadExt;

use super::*;
use crate::testutil::tmp_dir;

const HELPER: &str = "K3C_PROC_HELPER";
const PORT_FILE: &str = "K3C_PROC_PORTFILE";
const HELPER_ARGS: [&str; 3] = ["--exact", "proc::tests::helper_process", "--nocapture"];

/// Kein Test: als `parent` startet das Test-Binary einen Enkel und endet sofort (der Enkel ist dann verwaist), als
/// `grandchild` öffnet es einen Port, schreibt ihn in die Adress-Datei und wartet.
#[test]
fn helper_process() {
    match std::env::var(HELPER).as_deref() {
        Ok("parent") => {
            let spawned = std::process::Command::new(std::env::current_exe().unwrap())
                .args(HELPER_ARGS)
                .env(HELPER, "grandchild")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
            std::process::exit(if spawned.is_ok() { 0 } else { 2 });
        }
        Ok("grandchild") => {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let file = std::path::PathBuf::from(std::env::var_os(PORT_FILE).unwrap());
            let part = file.with_extension("part");
            std::fs::write(&part, listener.local_addr().unwrap().to_string()).unwrap();
            std::fs::rename(part, file).unwrap();
            std::thread::sleep(Duration::from_secs(60));
        }
        _ => {}
    }
}

fn helper(role: &str, port_file: &Path) -> Cmd {
    let exe = std::env::current_exe().unwrap();
    let mut args = vec![exe.into_os_string()];
    args.extend(HELPER_ARGS.map(Into::into));
    let mut cmd = Cmd::new(&args);
    cmd.command_mut().env(HELPER, role).env(PORT_FILE, port_file);
    cmd
}

async fn wait_for_addr(file: &Path) -> SocketAddr {
    for _ in 0..200 {
        if let Some(addr) = std::fs::read_to_string(file).ok().and_then(|s| s.trim().parse().ok()) {
            return addr;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("Enkel hat keinen Port gemeldet");
}

async fn assert_gone(addr: SocketAddr) {
    for _ in 0..50 {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_err() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    panic!("Prozess an {addr} läuft noch");
}

#[tokio::test]
async fn kill_trifft_verwaisten_enkel() {
    let tmp = tmp_dir();
    let port_file = tmp.path().join("port");
    let mut parent = helper("parent", &port_file).start().unwrap();
    let addr = wait_for_addr(&port_file).await;
    // Das Kind endet von selbst; wait räumt den Job ab.
    let status = parent.wait().await.unwrap();
    assert!(status.success(), "Kind: {status}");
    assert_gone(addr).await;
}

/// Ersatz für den Context-Abbruch: der Griff beendet einen laufenden Prozess, `wait` kommt zurück.
#[tokio::test]
async fn kill_handle_beendet_laufenden_prozess() {
    fn send_sync<T: Send + Sync>(_: &T) {}
    let tmp = tmp_dir();
    let port_file = tmp.path().join("port");
    let mut running = helper("grandchild", &port_file).start().unwrap();
    let addr = wait_for_addr(&port_file).await;
    let handle = running.kill_handle();
    send_sync(&handle);
    assert!(running.id() > 0);
    handle.kill().unwrap();
    let status = tokio::time::timeout(Duration::from_secs(20), running.wait()).await;
    assert!(!status.unwrap().unwrap().success());
    assert_gone(addr).await;
}

#[tokio::test]
async fn kill_ohne_start_und_executable() {
    // Vor dem Start gibt es hier keinen `Proc`; geprüft wird der leere Zustand dahinter.
    let unstarted = KillHandle { state: Arc::default() };
    assert!(unstarted.kill().is_ok(), "Kill vor Start");
    assert_eq!(executable("go"), "go");
    assert_eq!(executable("npm"), if cfg!(windows) { "npm.cmd" } else { "npm" });
    let mut cmd = helper("", Path::new(""));
    cmd.command_mut().stdout(Stdio::piped());
    let mut run = cmd.start().unwrap();
    let mut out = String::new();
    let pipe = run.child_mut().stdout.as_mut().unwrap();
    pipe.read_to_string(&mut out).await.unwrap();
    assert!(run.wait().await.unwrap().success());
    assert!(out.contains("running 1 test"), "einfacher Lauf: {out:?}");
    let empty: [&str; 0] = [];
    assert!(Cmd::new(&empty).start().is_err(), "leerer Befehl");
}

#[tokio::test]
async fn kill_nach_wait_tut_nichts() {
    let mut run = helper("", Path::new("")).start().unwrap();
    let handle = run.kill_handle();
    assert!(run.wait().await.unwrap().success());
    // Die PID ist frei und kann schon einem fremden Prozess gehören: kein taskkill mehr.
    assert!(handle.kill().is_ok(), "Kill nach Wait");
    assert!(helper("", Path::new("")).run().await.unwrap().success());
}
