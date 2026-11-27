//! Port von `offline_exe_test.go` (RU40.3): der Aufruf der EXE mit Attrappe statt `gameexe::stream`. Kein Test startet
//! die Spiel-EXE; jedes Register bekommt vor dem Start eine Attrappe oder eigene Runner.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use k3c_kit::logger::Logger;

use super::offline::{OnOutput, Proc, ProcResult};
use super::offline_run::{EXIT_ABORTED, Stop, exe_args, run_with};
use super::registry::{Registry, RegistryError};
use super::run::{CANCELED, CancelToken, Runner, lock};
use super::spec::{Spec, Tune};
use super::test_helpers::{registry, repo_root, temp_root, wait_for};
use crate::gameexe::{Error, Event, Options};

type End = Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
type Seen = Arc<Mutex<Vec<String>>>;

fn texts(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

fn proc(dir: &Path, args: &[&str], timeout: Duration) -> (Proc, Seen) {
    let seen = Seen::default();
    let lines = Arc::clone(&seen);
    let out: OnOutput = Arc::new(move |stream, text| lock(&lines).push(format!("{stream}: {text}")));
    let proc = Proc {
        dir: dir.to_path_buf(),
        args: texts(args),
        head: false,
        speed: 0,
        timeout,
        out,
    };
    (proc, seen)
}

fn run(p: &Proc, cancel: &CancelToken, end: impl FnOnce(Options, Stop) -> End + Send) -> ProcResult {
    run_with(end, cancel, p, Logger::discard())
}

/// Wartet auf Stop oder Zeitlimit wie `gameexe::stream`.
fn until_stopped(_: Options, stop: Stop) -> End {
    Box::pin(async move { Err(Error::Canceled(stop.await)) })
}

#[test]
fn exe_args_ohne_unterbefehl_mit_absoluten_pfaden() {
    let (_dir, root) = temp_root();
    let args = texts(&[
        "balance",
        "--targets",
        "--report-dir",
        "reports/simtest-run-1",
        "--seeds",
        "2",
        "--baseline",
        "reports/b.json",
    ]);
    let path = |parts: &[&str]| {
        let joined: PathBuf = parts.iter().fold(root.clone(), |p, part| p.join(part));
        joined.to_string_lossy().into_owned()
    };
    let want = vec![
        "--targets".to_owned(),
        "--report-dir".to_owned(),
        path(&["reports", "simtest-run-1"]),
        "--seeds".to_owned(),
        "2".to_owned(),
        "--baseline".to_owned(),
        path(&["reports", "b.json"]),
    ];
    assert_eq!(exe_args(&root, &args), want);
    // ein absoluter Pfad bleibt
    let absolute = path(&["x.json"]);
    let args = texts(&["--out", &absolute]);
    assert_eq!(exe_args(&root, &args), args);
}

/// Härtung gegenüber Go: ein Wert ist nie ein Schalter und schreibt das folgende Argument nicht um.
#[test]
fn exe_args_werte_sind_nie_schalter() {
    let (_dir, root) = temp_root();
    let args = texts(&[
        "balance",
        "--targets",
        "--seed-list",
        "--baseline",
        "--depth",
        "3",
        "--grade",
        "--input",
        "--bot",
        "--out",
        "--write-baseline",
        "--progress",
        "--label",
    ]);
    let want = texts(&[
        "--targets",
        "--seed-list=--baseline",
        "--depth",
        "3",
        "--grade=--input",
        "--bot=--out",
        "--write-baseline",
        "--progress",
        "--label",
    ]);
    assert_eq!(exe_args(&root, &args), want);
}

#[test]
fn exe_args_eines_auftrags_mit_fuehrendem_strich() {
    let (_dir, root) = temp_root();
    let spec = Spec {
        seed_list: texts(&["--out"]),
        depth: Some(1),
        vary: texts(&["-x.json:a=1"]),
        ..Spec::default()
    };
    spec.check(&repo_root()).expect("wie Go angenommen");
    let args = exe_args(&root, &spec.args("reports/x"));
    let tail = texts(&["--seed-list=--out", "--depth", "1", "--vary=-x.json:a=1"]);
    assert!(args.ends_with(&tail), "{args:?}");
}

#[test]
fn run_exe_headless_und_mit_fenster() {
    let (_dir, root) = temp_root();
    let got: Arc<Mutex<Option<Options>>> = Arc::default();
    let fake = |got: &Arc<Mutex<Option<Options>>>| {
        let got = Arc::clone(got);
        move |o: Options, _: Stop| -> End {
            if let Some(on) = &o.on_event {
                on(&Event {
                    event: "progress".to_owned(),
                    done: 1,
                    total: 2,
                    ..Event::default()
                });
            }
            if let Some(on) = &o.on_stderr {
                on("hallo");
            }
            *lock(&got) = Some(o);
            Box::pin(async { Ok(()) })
        }
    };
    let (p, lines) = proc(&root, &["balance", "--targets"], Duration::from_secs(60));
    let res = run(&p, &CancelToken::new(), fake(&got));
    assert!(res.exit == 0 && !res.timed_out && res.err.is_none(), "{res:?}");
    let o = lock(&got).take().expect("Optionen");
    assert!(o.sub == "balance" && !o.window && o.speed == 0 && o.root == root);
    assert_eq!(o.args, ["--targets"]);
    assert_eq!(o.input, b"{}");
    assert_eq!(
        lock(&lines).join("|"),
        r#"stdout: {"event":"progress","done":1,"total":2}|stderr: hallo"#
    );
    let (mut p, _) = proc(&root, &["balance"], Duration::from_secs(60));
    (p.head, p.speed) = (true, 8);
    run(&p, &CancelToken::new(), fake(&got));
    let o = lock(&got).take().expect("Optionen");
    assert!(o.window && o.speed == 8);
}

#[test]
fn run_exe_fehler_abbruch_und_zeitlimit() {
    let (_dir, root) = temp_root();
    let (p, lines) = proc(&root, &[], Duration::from_secs(60));
    let res = run(&p, &CancelToken::new(), |_, _| {
        Box::pin(async { Err(Error::Msg("welt fehlt".to_owned())) })
    });
    assert_eq!(res.exit, 1, "{res:?}");
    assert!(
        lock(&lines).iter().any(|l| l == "stderr: ❌ k3c-game sim: welt fehlt"),
        "{:?}",
        lock(&lines)
    );
    let canceled = CancelToken::new();
    canceled.cancel();
    let res = run(&p, &canceled, until_stopped);
    assert!(res.exit == EXIT_ABORTED && !res.timed_out, "{res:?}");
    let (p, _) = proc(&root, &[], Duration::from_millis(1));
    let res = run(&p, &CancelToken::new(), until_stopped);
    assert!(res.timed_out && res.exit == -1, "{res:?}");
}

/// AC-03: Stop eines Laufs bricht den Aufruf der EXE ab (stdin zu, danach Job-Objekt: `gameexe`).
#[test]
fn stop_beendet_lauf_der_exe() {
    let (_dir, root) = temp_root();
    let (started, wait_started) = mpsc::channel::<(bool, i64, Vec<String>)>();
    let g = registry();
    g.set_exec(Arc::new(move |cancel: CancelToken, p: Proc| {
        let started = started.clone();
        run(&p, &cancel, move |o: Options, stop: Stop| {
            started.send((o.window, o.speed, o.args.clone())).expect("melden");
            until_stopped(o, stop)
        })
    }));
    let spec = Spec {
        seeds: 2,
        depth: Some(3),
        head: true,
        speed: 4,
        ..Spec::default()
    };
    let info = g.start_offline(&root, "", &spec).expect("Start");
    let (window, speed, args) = wait_started.recv_timeout(Duration::from_secs(5)).expect("Aufruf");
    assert!(window && speed == 4, "{window} {speed}");
    assert!(args.iter().any(|a| a == "--depth"), "{args:?}");
    g.stop(&root, &info.id).expect("Stop");
    wait_for(&g, &root, &info.id, |i| i.state == CANCELED && !i.report.is_empty());
}

/// Runner, der bis zum Abbruch oder bis `release` wartet und nichts meldet.
fn idle(release: &CancelToken) -> Runner {
    let release = release.clone();
    Arc::new(
        move |cancel: CancelToken, _| {
            while !cancel.is_canceled() && !release.wait_timeout(Duration::from_millis(5)) {}
        },
    )
}

/// Zweiter Lauf ohne Warteschlange neben einem ersten.
fn second_is_refused(g: &Registry, root: &Path, first_head: bool) {
    let release = CancelToken::new();
    let spec = |head| {
        Some(Spec {
            head,
            ..Spec::default()
        })
    };
    g.start(root, "", "erster", spec(first_head), idle(&release), false)
        .expect("erster Lauf");
    let second = g.start(root, "", "zweiter", spec(!first_head), idle(&release), false);
    assert_eq!(second.err(), Some(RegistryError::Full), "Fenster zuerst: {first_head}");
    release.cancel();
}

#[test]
fn fenster_lauf_laeuft_allein() {
    let (_dir, root) = temp_root();
    second_is_refused(&registry(), &root, false);
}

#[test]
fn nach_fenster_lauf_startet_kein_zweiter() {
    let (_dir, root) = temp_root();
    second_is_refused(&registry(), &root, true);
}

#[test]
fn spec_head_speed_pruefung() {
    let root = repo_root();
    let spec = |head, speed| Spec {
        head,
        speed,
        ..Spec::default()
    };
    let tune = Spec {
        head: true,
        tune: Some(Tune {
            mode: "sweep".to_owned(),
            params: texts(&["a.json:b=1..2"]),
            ..Tune::default()
        }),
        ..Spec::default()
    };
    for bad in [spec(false, 2), spec(true, 65), spec(true, -1), tune] {
        assert!(bad.check(&root).is_err(), "{bad:?} angenommen");
    }
    spec(true, 64).check(&root).expect("64 erlaubt");
    // --window gehört in die Optionen, nicht in die Argumente
    assert!(!spec(true, 4).args("r").iter().any(|a| a == "--window"));
}
