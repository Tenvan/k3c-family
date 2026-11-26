//! Port von `run_test.go`: Prozess-Tests mit `gameexe-helper` als Spiel-EXE (V8); `echte_exe` rechnet nur mit frischer
//! EXE im Checkout und baut nie eine.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::ffi::OsString;
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use k3c_studio::gameexe::{BUILD_COMMAND, Cancel, Error, Event, OnEvent, Options, call, find, never, run, stream};
use tokio::sync::Notify;

#[path = "support/tmp.rs"]
mod tmp;
use tmp::tmp_dir;

const HELPER: &str = env!("CARGO_BIN_EXE_gameexe-helper");

fn argv(mode: &str, extra: &[&Path]) -> Vec<OsString> {
    let mut argv = vec![OsString::from(HELPER), OsString::from(mode)];
    argv.extend(extra.iter().map(|p| p.as_os_str().to_owned()));
    argv
}

type Seen = Arc<Mutex<Vec<Event>>>;

/// Sammelt die Events und meldet bei `progress` den Abbruchwunsch.
fn collect() -> (Seen, Arc<Notify>, OnEvent) {
    let seen: Seen = Arc::default();
    let progress = Arc::new(Notify::new());
    let (list, notify) = (Arc::clone(&seen), Arc::clone(&progress));
    let on: OnEvent = Arc::new(move |ev: &Event| {
        list.lock().unwrap().push(ev.clone());
        if ev.event == "progress" {
            notify.notify_one();
        }
    });
    (seen, progress, on)
}

async fn on_progress(progress: Arc<Notify>) -> Cancel {
    progress.notified().await;
    Cancel::Canceled
}

#[tokio::test]
async fn lines_und_input_datei() {
    let root = tmp_dir();
    let input = root.path().join("in.json");
    std::fs::write(&input, r#"{"seed":1}"#).unwrap();
    let (seen, _, on) = collect();
    let errors: Arc<Mutex<Vec<String>>> = Arc::default();
    let list = Arc::clone(&errors);
    let o = Options {
        on_event: Some(on),
        on_stderr: Some(Arc::new(move |l: &str| list.lock().unwrap().push(l.to_owned()))),
        ..Options::default()
    };
    let mut args = argv("lines", &[]);
    args.extend(["--input".into(), input.into_os_string()]);
    run(&args, root.path(), &o, never()).await.unwrap();
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 2, "{seen:?}");
    assert_eq!((seen[0].event.as_str(), seen[0].done), ("progress", 1));
    assert_eq!(seen[1].text, r#"{"seed":1}"#);
    assert_eq!(*errors.lock().unwrap(), ["❌ Meldung"]);
}

#[tokio::test]
async fn fehlerzeile_und_exit_code() {
    let root = tmp_dir();
    let o = Options::default();
    let error = run(&argv("fail", &[]), root.path(), &o, never()).await;
    assert_eq!(error, Err(Error::Msg("kaputt".into())));
    let error = run(&argv("exit", &[]), root.path(), &o, never()).await;
    assert_eq!(error, Err(Error::Msg("Exit 3".into())));
}

#[tokio::test]
async fn zu_lange_zeile_blockiert_nicht() {
    let root = tmp_dir();
    let (seen, _, on) = collect();
    let o = Options {
        on_event: Some(on),
        ..Options::default()
    };
    run(&argv("long", &[]), root.path(), &o, never()).await.unwrap();
    assert!(seen.lock().unwrap().is_empty(), "nach der zu langen Zeile zählt nichts");
}

#[tokio::test]
async fn start_ohne_programm() {
    let root = tmp_dir();
    let missing = [root.path().join("fehlt.exe").into_os_string()];
    let error = run(&missing, root.path(), &Options::default(), never()).await;
    let text = error.unwrap_err().to_string();
    assert!(text.starts_with("Spiel-EXE nicht gestartet: "), "{text}");
}

#[tokio::test]
async fn abbruch_ueber_stdin() {
    let root = tmp_dir();
    let (seen, progress, on) = collect();
    let o = Options {
        on_event: Some(on),
        ..Options::default()
    };
    let start = Instant::now();
    let error = run(&argv("wait", &[]), root.path(), &o, on_progress(progress)).await;
    assert_eq!(error, Err(Error::Canceled(Cancel::Canceled)));
    assert!(start.elapsed() < Duration::from_secs(4), "{:?}", start.elapsed());
    let names: Vec<String> = seen.lock().unwrap().iter().map(|e| e.event.clone()).collect();
    assert_eq!(names, ["progress", "aborted"]);
}

/// Läuft der Enkel hinter `address` nicht mehr?
fn gone(address: SocketAddr) -> bool {
    for _ in 0..50 {
        match TcpStream::connect_timeout(&address, Duration::from_millis(200)) {
            Err(_) => return true,
            Ok(connection) => drop(connection),
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

fn grandchild(port_file: &Path) -> SocketAddr {
    let text = std::fs::read_to_string(port_file).expect("Enkel hat keinen Port gemeldet");
    text.trim().parse().expect("Adresse des Enkels")
}

#[tokio::test]
async fn harter_abbruch_nach_frist_raeumt_baum() {
    let root = tmp_dir();
    let port_file = root.path().join("port");
    let (_, progress, on) = collect();
    let o = Options {
        grace: Duration::from_millis(300),
        on_event: Some(on),
        ..Options::default()
    };
    let args = argv("hang", &[&port_file]);
    let error = run(&args, root.path(), &o, on_progress(progress)).await;
    assert_eq!(error, Err(Error::Canceled(Cancel::Canceled)));
    let address = grandchild(&port_file);
    assert!(gone(address), "Enkel an {address} läuft nach dem Abbruch noch");
}

/// Auch ein fallen gelassener Lauf (Server-Stopp) hinterlässt keinen Prozess.
#[tokio::test]
async fn fallen_gelassener_lauf_raeumt_baum() {
    let root = tmp_dir();
    let port_file = root.path().join("port");
    let (_, progress, on) = collect();
    let o = Options {
        on_event: Some(on),
        ..Options::default()
    };
    let args = argv("hang", &[&port_file]);
    tokio::select! {
        end = run(&args, root.path(), &o, never()) => panic!("beendet: {end:?}"),
        () = progress.notified() => {}
    }
    let address = grandchild(&port_file);
    assert!(gone(address), "Enkel an {address} läuft noch");
}

#[tokio::test]
async fn stream_ohne_exe_nennt_bau_befehl() {
    let root = tmp_dir();
    let o = Options {
        root: root.path().to_path_buf(),
        sub: "level".into(),
        input: b"{}".to_vec(),
        ..Options::default()
    };
    let error = stream(o.clone(), never()).await.unwrap_err().to_string();
    assert!(error.contains(BUILD_COMMAND), "{error}");
    let unknown = Options { sub: "x".into(), ..o };
    let error = stream(unknown, never()).await.unwrap_err().to_string();
    assert_eq!(error, r#"unbekannter Unterbefehl "x""#);
}

/// Rechnet `sim level` mit der echten EXE, wenn sie frisch im Checkout liegt.
#[tokio::test]
async fn echte_exe() {
    let root: PathBuf =
        std::path::absolute(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")).expect("Repo-Wurzel");
    if let Err(e) = find(&root) {
        eprintln!("übersprungen: keine frische Spiel-EXE: {e}");
        return;
    }
    let o = Options {
        root,
        sub: "level".into(),
        input: br#"{"biome":"forest","seed":"1"}"#.to_vec(),
        ..Options::default()
    };
    let ev = call(o, never()).await.expect("sim level");
    assert!(ev.event == "result" && !ev.text.is_empty(), "{ev:?}");
}
