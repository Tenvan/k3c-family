//! Lauf starten und abbrechen (Port von `run.go`). Die EXE wird ohne Shell gestartet, jedes Argument einzeln; Eingaben
//! gehen als Datei unter `<Root>/.work/runs/` an sie, nie über die Befehlszeile.

use std::ffi::OsString;
use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::pin::pin;
use std::process::{ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use k3c_kit::logger::Logger;
use k3c_kit::proc::{Cmd, Proc};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};

use super::{Event, find, parse_event};

/// Frist zwischen dem Schließen von stdin (Abbruch) und dem harten Beenden des Prozessbaums.
pub const GRACE: Duration = Duration::from_secs(5);

/// Unterbefehle von `k3c-game sim`, wie in Go: `save` fehlt dort (Fund RU70.2, nicht still behoben).
const VALID_SUB: [&str; 5] = ["run", "level", "balance", "world", "hub"];

/// Längste erwartete stdout-Zeile (Berichte von world und hub).
const MAX_LINE: usize = 16 << 20;

/// Rückruf für jede JSON-Zeile von stdout.
pub type OnEvent = Arc<dyn Fn(&Event) + Send + Sync>;
/// Rückruf für jede Zeile von stderr.
pub type OnLine = Arc<dyn Fn(&str) + Send + Sync>;

/// Ein Lauf `k3c-game sim <sub> …`.
#[derive(Clone, Default)]
pub struct Options {
    /// Checkout: EXE, `data/` und `.work/runs/`.
    pub root: PathBuf,
    /// Absoluter Daten-Ordner statt `<root>/data` (Tests: Fixture-Daten, EXE der Repo-Wurzel).
    pub data: Option<PathBuf>,
    /// run, level, world, hub, balance.
    pub sub: String,
    /// Eingabe-JSON; die Datei dazu wird nach dem Ende gelöscht.
    pub input: Vec<u8>,
    /// `--window` (nur run und balance).
    pub window: bool,
    /// `--speed N` (1 bis 64), nur mit `window`.
    pub speed: i64,
    /// Weitere Schalter der Befehlszeile (balance: `--targets` …), stehen nach dem Eingabe-JSON.
    pub args: Vec<String>,
    pub on_event: Option<OnEvent>,
    pub on_stderr: Option<OnLine>,
    /// 0 heißt [`GRACE`].
    pub grace: Duration,
    pub log: Option<Logger>,
}

/// Warum ein Lauf abgebrochen wird; die Texte sind die von Go `ctx.Err()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cancel {
    Canceled,
    Deadline,
}

/// Ende eines Laufs ohne Erfolg.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Abgebrochen durch den Aufrufer.
    Canceled(Cancel),
    /// Meldung der Zeile `error`, „Exit <n>“ oder Fehler beim Start.
    Msg(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Canceled(Cancel::Canceled) => f.write_str("context canceled"),
            Error::Canceled(Cancel::Deadline) => f.write_str("context deadline exceeded"),
            Error::Msg(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Msg(e.to_string())
    }
}

/// Abbruchsignal, das nie kommt.
pub async fn never() -> Cancel {
    std::future::pending().await
}

/// Die Eingabe-Datei eines Laufs; beim Fallenlassen gelöscht.
struct InputFile(PathBuf);

impl Drop for InputFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Neue Datei unter `<root>/.work/runs/`, eindeutig auch gegen andere Prozesse.
fn input_file(root: &Path, input: &[u8]) -> io::Result<InputFile> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let runs = root.join(".work").join("runs");
    std::fs::create_dir_all(&runs)?;
    loop {
        let now = chrono::Local::now();
        let name = format!(
            "{}-{}-{}-{}.json",
            now.format("%Y%m%d-%H%M%S"),
            now.timestamp_subsec_nanos(),
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        let path = runs.join(name);
        match std::fs::File::create_new(&path) {
            Ok(mut file) => {
                let guard = InputFile(path);
                file.write_all(input)?;
                return Ok(guard);
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
}

/// Die Befehlszeile `k3c-game sim <sub> --input <Datei> --data <data|root/data> --stop-stdin [--window [--speed N]] [args…]`.
pub fn argv_for(exe: &Path, input: &Path, o: &Options) -> Vec<OsString> {
    let data = o.data.clone().unwrap_or_else(|| o.root.join("data"));
    let mut argv: Vec<OsString> = vec![
        exe.into(),
        "sim".into(),
        o.sub.as_str().into(),
        "--input".into(),
        input.into(),
        "--data".into(),
        data.into(),
        "--stop-stdin".into(),
    ];
    if o.window {
        argv.push("--window".into());
        if o.speed > 0 {
            argv.extend(["--speed".into(), o.speed.to_string().into()]);
        }
    }
    argv.extend(o.args.iter().map(OsString::from));
    argv
}

/// Startet die EXE des Checkouts im Job-Objekt und reicht Zeilen an die Rückrufe. Ende:
/// - `Ok`: Exit 0
/// - [`Error::Canceled`]: `cancel` kam. stdin wird geschlossen (die EXE schreibt `aborted`, Exit 130); nach der Frist
///   beendet das Job-Objekt den ganzen Baum. Ein Ende nach eigenem Stop gilt als Abbruch, auch ohne Zeile `aborted`.
/// - sonst der Text der Zeile `error` oder „Exit <n>“.
pub async fn stream(o: Options, cancel: impl Future<Output = Cancel>) -> Result<(), Error> {
    if !VALID_SUB.contains(&o.sub.as_str()) {
        return Err(Error::Msg(format!("unbekannter Unterbefehl {:?}", o.sub)));
    }
    let (exe, dir) = find(&o.root).map_err(Error::Msg)?;
    let input = input_file(&o.root, &o.input)?;
    run(&argv_for(&exe, &input.0, &o), &dir, &o, cancel).await
}

/// Ruft für jede Zeile den Rückruf; ist eine Zeile zu lang, wird der Rest verworfen (die EXE soll nicht an einer vollen
/// Pipe blockieren).
async fn read_lines(r: impl AsyncRead + Unpin, log: &Logger, mut line: impl FnMut(&str)) {
    let mut reader = BufReader::new(r);
    let mut buf = Vec::new();
    let failed = loop {
        buf.clear();
        let mut limited = (&mut reader).take(MAX_LINE as u64 + 1);
        match limited.read_until(b'\n', &mut buf).await {
            Ok(0) => return,
            Ok(_) if buf.len() > MAX_LINE && !buf.ends_with(b"\n") => break "Zeile zu lang".to_owned(),
            Ok(_) => {
                let text = buf.strip_suffix(b"\n").unwrap_or(&buf);
                let text = text.strip_suffix(b"\r").unwrap_or(text);
                line(&String::from_utf8_lossy(text));
            }
            Err(e) => break e.to_string(),
        }
    };
    log.warn(
        "❌ Zeile der Spiel-EXE nicht lesbar",
        &[("ns", json!("sim")), ("err", json!(failed))],
    );
    let _ = tokio::io::copy(&mut reader, &mut tokio::io::sink()).await;
}

/// Liest beide Ausgaben bis zum Ende und wartet dann auf den Prozess; liefert die Meldung der Zeile `error`.
async fn drain(proc: &mut Proc, o: &Options, log: &Logger) -> (io::Result<ExitStatus>, Option<String>) {
    let stdout = proc.child_mut().stdout.take();
    let stderr = proc.child_mut().stderr.take();
    let failure = Mutex::new(None);
    let events = async {
        let Some(out) = stdout else { return };
        read_lines(out, log, |l| {
            let Some(ev) = parse_event(l.trim()) else { return };
            if ev.event == "error" {
                *failure.lock().unwrap_or_else(PoisonError::into_inner) = Some(ev.message.clone());
            }
            if let Some(on) = &o.on_event {
                on(&ev);
            }
        })
        .await;
    };
    let errors = async {
        let Some(err) = stderr else { return };
        read_lines(err, log, |l| {
            if let Some(on) = &o.on_stderr {
                on(l);
            }
        })
        .await;
    };
    tokio::join!(events, errors);
    let status = proc.wait().await;
    (status, failure.into_inner().unwrap_or_else(PoisonError::into_inner))
}

/// Wartet auf das Ende des Prozesses oder auf `cancel`: dann schließt es stdin (die EXE bricht ab) und beendet nach der
/// Frist den ganzen Baum. Ohne Abbruch gibt es keine Frist.
async fn watch(
    mut proc: Proc,
    o: &Options,
    log: &Logger,
    cancel: impl Future<Output = Cancel>,
) -> (io::Result<ExitStatus>, Option<String>, Option<Cancel>) {
    let grace = if o.grace.is_zero() { GRACE } else { o.grace };
    let stdin = proc.child_mut().stdin.take();
    let kill = proc.kill_handle();
    let mut done = pin!(drain(&mut proc, o, log));
    let why = tokio::select! {
        biased;
        (status, failure) = &mut done => return (status, failure, None),
        why = cancel => why,
    };
    drop(stdin);
    let (status, failure) = match tokio::time::timeout(grace, &mut done).await {
        Ok(end) => end,
        Err(_) => {
            let grace = format!("{grace:?}");
            log.warn(
                "⏳ Spiel-EXE nach der Frist hart beendet",
                &[("ns", json!("sim")), ("grace", json!(grace))],
            );
            if let Err(e) = kill.kill() {
                let attrs = [("ns", json!("sim")), ("err", json!(e.to_string()))];
                log.error("💥 Spiel-EXE nicht beendet", &attrs);
            }
            done.await
        }
    };
    (status, failure, Some(why))
}

/// Startet `argv` (Tests ersetzen die EXE durch ein Hilfs-Programm) und wartet auf das Ende; siehe [`stream`]. Wird das
/// Future fallen gelassen, schließt sich der Job und der Prozessbaum endet.
#[doc(hidden)]
pub async fn run(
    argv: &[OsString],
    dir: &Path,
    o: &Options,
    cancel: impl Future<Output = Cancel>,
) -> Result<(), Error> {
    let log = o.log.clone().unwrap_or_else(Logger::discard);
    let mut cmd = Cmd::new(argv);
    cmd.command_mut()
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let sub = ("sub", json!(o.sub));
    let proc = cmd.start().map_err(|e| {
        let attrs = [("ns", json!("sim")), sub.clone(), ("err", json!(e.to_string()))];
        log.error("❌ Spiel-EXE nicht gestartet", &attrs);
        Error::Msg(format!("Spiel-EXE nicht gestartet: {e}"))
    })?;
    let exe = argv.first().map(|a| a.to_string_lossy().into_owned());
    let attrs = [
        ("ns", json!("sim")),
        ("exe", json!(exe)),
        sub.clone(),
        ("head", json!(o.window)),
    ];
    log.info("🚀 Spiel-EXE gestartet", &attrs);

    let (status, failure, stopped) = watch(proc, o, &log, cancel).await;
    // Ein fertiger Lauf zählt auch dann, wenn der Abbruch gleichzeitig kam.
    if status.as_ref().is_ok_and(ExitStatus::success) {
        log.info("✅ Spiel-EXE beendet", &[("ns", json!("sim")), sub]);
        return Ok(());
    }
    if let Some(why) = stopped {
        log.info("🛑 Spiel-EXE abgebrochen", &[("ns", json!("sim")), sub]);
        return Err(Error::Canceled(why));
    }
    let message = failure.filter(|m| !m.is_empty()).unwrap_or_else(|| exit_text(status));
    let attrs = [("ns", json!("sim")), sub, ("err", json!(message))];
    log.error("❌ Spiel-EXE gescheitert", &attrs);
    Err(Error::Msg(message))
}

/// „Exit <n>“, sonst der Fehler des Wartens.
fn exit_text(status: io::Result<ExitStatus>) -> String {
    match status {
        Ok(s) => s.code().map_or_else(|| s.to_string(), |code| format!("Exit {code}")),
        Err(e) => e.to_string(),
    }
}

/// Führt einen kurzen Lauf (run, level, world, hub) aus und liefert das letzte Ergebnis (result oder report).
pub async fn call(mut o: Options, cancel: impl Future<Output = Cancel>) -> Result<Event, Error> {
    let result = Arc::new(Mutex::new(None));
    let seen = Arc::clone(&result);
    let next = o.on_event.take();
    o.on_event = Some(Arc::new(move |ev: &Event| {
        if ev.event == "result" || ev.event == "report" {
            *seen.lock().unwrap_or_else(PoisonError::into_inner) = Some(ev.clone());
        }
        if let Some(on) = &next {
            on(ev);
        }
    }));
    let sub = o.sub.clone();
    stream(o, cancel).await?;
    let last = result.lock().unwrap_or_else(PoisonError::into_inner).take();
    last.ok_or_else(|| Error::Msg(format!("k3c-game sim {sub}: ohne Ergebnis beendet")))
}
