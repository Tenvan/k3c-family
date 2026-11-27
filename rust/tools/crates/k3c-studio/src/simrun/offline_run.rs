//! Aufruf der Spiel-EXE für einen Offline-Lauf (Port von `runExe` und `exeArgs` aus `offline.go`, RU40.3). Jedes Event
//! geht als JSON-Zeile nach `out` („stdout“), jede Zeile von stderr als „stderr“. Stop und Zeitlimit schließen stdin
//! der EXE (Exit 130, `aborted`); bleibt sie hängen, beendet `gameexe` nach der Frist den ganzen Prozessbaum.
//!
//! Härtung gegenüber Go: die Argumente werden als Paare Schalter/Wert gelesen. Ein Wert gilt nie als Schalter (Go
//! sucht die Pfad-Schalter an jeder Stelle), und ein Wert mit führendem `-` geht als `--schalter=wert` an die EXE,
//! deren Vorlauf für `--input` sonst jede Stelle durchsucht.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use k3c_kit::logger::Logger;
use serde::Serialize;
use serde_json::value::RawValue;

use super::offline::{Exec, OnOutput, Proc, ProcResult};
use super::run::CancelToken;
use crate::gameexe::{self, Cancel, Error, Event, Options};

/// Exit-Code eines abgebrochenen Laufs (`docs/standards/k3c-game-laeufe.md`).
pub const EXIT_ABORTED: i32 = 130;

/// Schalter von `balance`, deren Wert ein Pfad ist; relative Werte gelten ab dem Checkout. Go führt auch
/// `--write-baseline`, das in der EXE aber ein Schalter ohne Wert ist.
pub const PATH_FLAGS: [&str; 6] = [
    "--report-dir",
    "--baseline",
    "--out",
    "--replay-dir",
    "--play",
    "--insights",
];

/// Schalter von `balance` ohne Wert (`rust/crates/k3c-game/src/runs/balance_specs.rs`).
const BARE_FLAGS: [&str; 9] = [
    "--targets",
    "--write-baseline",
    "--progress",
    "--sensitivity",
    "--curves",
    "--headless",
    "--window",
    "--stop-stdin",
    "--legacy",
];

/// Wie oft ein laufender Aufruf nach dem Abbruch fragt.
const POLL: Duration = Duration::from_millis(20);

/// Abbruchsignal, das an [`gameexe::stream`] geht.
pub(super) type Stop = Pin<Box<dyn Future<Output = Cancel> + Send>>;

/// `args` ohne den Unterbefehl, Pfadwerte absolut ab `dir` (die EXE läuft im Ordner ihres Pakets, nicht im Checkout).
pub(super) fn exe_args(dir: &Path, args: &[String]) -> Vec<String> {
    let skip = usize::from(args.first().is_some_and(|a| a == "balance"));
    let mut rest = args.iter().skip(skip);
    let mut out = Vec::new();
    while let Some(flag) = rest.next() {
        let bare = !flag.starts_with('-') || flag.contains('=') || BARE_FLAGS.contains(&flag.as_str());
        let value = if bare { None } else { rest.next() };
        let Some(value) = value else {
            out.push(flag.clone());
            continue;
        };
        let value = if PATH_FLAGS.contains(&flag.as_str()) {
            absolute_in(dir, value)
        } else {
            value.clone()
        };
        if value.starts_with('-') {
            out.push(format!("{flag}={value}"));
        } else {
            out.extend([flag.clone(), value]);
        }
    }
    out
}

/// Wie Go `filepath.Join(dir, value)` für relative Werte; absolute bleiben.
fn absolute_in(dir: &Path, value: &str) -> String {
    if Path::new(value).is_absolute() {
        return value.to_owned();
    }
    // über die Teile gesammelt: ein Trenner je Plattform
    let joined: PathBuf = dir.join(value).components().collect();
    joined.to_string_lossy().into_owned()
}

/// Zeile von stdout wie Go `json.Marshal(gameexe.Event)`: leere Felder fehlen.
fn event_line(ev: &Event) -> Option<String> {
    #[derive(Serialize)]
    struct Line<'a> {
        event: &'a str,
        #[serde(skip_serializing_if = "crate::balance::report_types::is_zero")]
        done: i64,
        #[serde(skip_serializing_if = "crate::balance::report_types::is_zero")]
        total: i64,
        #[serde(skip_serializing_if = "str::is_empty")]
        text: &'a str,
        #[serde(skip_serializing_if = "str::is_empty")]
        message: &'a str,
        #[serde(skip_serializing_if = "<[String]>::is_empty")]
        files: &'a [String],
        #[serde(skip_serializing_if = "Option::is_none")]
        report: Option<&'a RawValue>,
    }
    serde_json::to_string(&Line {
        event: &ev.event,
        done: ev.done,
        total: ev.total,
        text: &ev.text,
        message: &ev.message,
        files: &ev.files,
        report: ev.report.as_deref(),
    })
    .ok()
}

/// Die Optionen eines Laufs der Spiel-EXE.
fn options(p: &Proc, log: Logger) -> Options {
    let (events, errors): (OnOutput, OnOutput) = (Arc::clone(&p.out), Arc::clone(&p.out));
    Options {
        root: p.dir.clone(),
        sub: "balance".to_owned(),
        input: b"{}".to_vec(),
        window: p.head,
        speed: p.speed,
        args: exe_args(&p.dir, &p.args),
        on_event: Some(Arc::new(move |ev: &Event| {
            if let Some(line) = event_line(ev) {
                events("stdout", &line);
            }
        })),
        on_stderr: Some(Arc::new(move |line: &str| errors("stderr", line))),
        log: Some(log),
        ..Options::default()
    }
}

/// Wartet auf Stop oder Zeitlimit.
async fn stop_signal(cancel: CancelToken, deadline: Option<Instant>) -> Cancel {
    loop {
        if cancel.is_canceled() {
            return Cancel::Canceled;
        }
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return Cancel::Deadline;
        }
        let left = deadline.map_or(POLL, |d| d.saturating_duration_since(Instant::now()));
        tokio::time::sleep(left.min(POLL)).await;
    }
}

/// Rechnet einen Lauf über `stream` (Test-Naht für [`gameexe::stream`]). Blockiert den Aufrufer: eigener Thread mit
/// eigener Laufzeit, damit es aus jedem Kontext geht.
pub(super) fn run_with<F, Fut>(stream: F, cancel: &CancelToken, p: &Proc, log: Logger) -> ProcResult
where
    F: FnOnce(Options, Stop) -> Fut + Send,
    Fut: Future<Output = Result<(), Error>>,
{
    let options = options(p, log);
    let deadline = Instant::now().checked_add(p.timeout);
    let stop: Stop = Box::pin(stop_signal(cancel.clone(), deadline));
    let end = std::thread::scope(|scope| {
        let run = scope.spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
            Ok::<_, std::io::Error>(runtime.block_on(stream(options, stop)))
        });
        run.join()
    });
    let error = match end {
        Ok(Ok(Ok(()))) => return ProcResult::default(),
        Ok(Ok(Err(e))) => e,
        Ok(Err(e)) => return not_started(e.to_string()),
        Err(_) => return not_started("Aufruf der Spiel-EXE abgestürzt".to_owned()),
    };
    let late = deadline.is_some_and(|d| Instant::now() >= d);
    let canceled = cancel.is_canceled();
    if error == Error::Canceled(Cancel::Deadline) || (late && !canceled) {
        return ProcResult {
            exit: -1,
            timed_out: true,
            err: None,
        };
    }
    if canceled || matches!(error, Error::Canceled(_)) {
        return ProcResult {
            exit: EXIT_ABORTED,
            ..ProcResult::default()
        };
    }
    // Ein Fehler ohne Meldung der EXE steht so auf stderr.
    (p.out)("stderr", &format!("❌ k3c-game sim: {error}"));
    ProcResult {
        exit: 1,
        ..ProcResult::default()
    }
}

fn not_started(why: String) -> ProcResult {
    ProcResult {
        err: Some(why),
        ..ProcResult::default()
    }
}

/// Rechnet mit der Spiel-EXE des Checkouts `p.dir` (`k3c-game sim balance`).
pub fn run_exe(cancel: &CancelToken, p: &Proc, log: Logger) -> ProcResult {
    run_with(gameexe::stream, cancel, p, log)
}

/// [`run_exe`] als Runner des Registers.
pub(super) fn exe_exec(log: Logger) -> Exec {
    Arc::new(move |cancel: CancelToken, p: Proc| run_exe(&cancel, &p, log.clone()))
}
