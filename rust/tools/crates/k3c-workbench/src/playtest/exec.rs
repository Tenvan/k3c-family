//! Echte Prozesse und ihre Abstraktion (Port von `exec.go`). Alles läuft über `k3c_kit::proc` (Job-Objekt: stirbt die
//! Workbench, enden ffmpeg und Spiel mit; ohne Konsolenfenster). Die Aufnahme sieht nur [`Spawn`] und [`Proc`].

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, LazyLock, Mutex, mpsc};
use std::time::Duration;

use k3c_kit::proc::{Cmd, KillHandle};
use regex::Regex;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::ChildStdin;
use tokio::runtime::Handle;

use super::devices::{Monitor, Probe, list_monitors, strings};
use super::encoders::PickCache;
use super::ffmpeg_find::LookPath;
use super::{Error, Signal, lock, no_ffmpeg};
use crate::services::exit_of;

/// So lange darf nach dem Ende noch Ausgabe kommen, die ein Enkel offen hält (Go `WaitDelay`).
const WAIT_DELAY: Duration = Duration::from_secs(5);

/// Ein laufender Prozess.
pub trait Proc: Send + Sync {
    /// Blockiert bis zum Ende; `Err` nennt den Exit-Code wie Go.
    fn wait(&self) -> Result<(), String>;
    /// Sauber beenden (ffmpeg: `q` auf stdin).
    fn quit(&self);
    fn kill(&self);
    /// Letzte nichtleere Zeile von stderr.
    fn tail(&self) -> String;
}

/// Startet `args` (`args[0]` = Programm) im Ordner `dir`.
pub type Spawn = Arc<dyn Fn(&[String], &Path) -> Result<Arc<dyn Proc>, Error> + Send + Sync>;

/// Das Spiel: EXE und Arbeitsordner.
pub type GameFinder = Arc<dyn Fn() -> Result<(String, PathBuf), Error> + Send + Sync>;

/// Hält die letzte nichtleere Zeile der Ausgabe.
#[derive(Default)]
struct TailWriter {
    last: String,
    part: Vec<u8>,
}

impl TailWriter {
    fn write(&mut self, bytes: &[u8]) {
        self.part.extend_from_slice(bytes);
        while let Some(end) = self.part.iter().position(|b| *b == b'\n') {
            let line: Vec<u8> = self.part.drain(..=end).collect();
            self.keep(&line);
        }
        if self.part.len() < 4096 {
            let part = std::mem::take(&mut self.part);
            self.keep(&part);
            self.part = part;
        }
    }

    fn keep(&mut self, line: &[u8]) {
        let line = String::from_utf8_lossy(line);
        let line = line.trim();
        if !line.is_empty() {
            line.clone_into(&mut self.last);
        }
    }
}

struct RealProc {
    runtime: Handle,
    kill: KillHandle,
    stdin: Mutex<Option<ChildStdin>>,
    tail: Arc<Mutex<TailWriter>>,
    done: Arc<Signal<Result<(), String>>>,
}

impl Proc for RealProc {
    fn wait(&self) -> Result<(), String> {
        self.done.wait()
    }

    /// Schreibt `q` auf stdin und schließt es; ohne stdin (Spiel) wird hart beendet.
    fn quit(&self) {
        let Some(mut stdin) = lock(&self.stdin).take() else {
            self.kill();
            return;
        };
        let (written, wait) = mpsc::channel();
        self.runtime.spawn(async move {
            // Ein Fehler heißt: der Prozess ist schon weg.
            let _ = stdin.write_all(b"q\n").await;
            let _ = stdin.flush().await;
            drop(stdin);
            let _ = written.send(());
        });
        let _ = wait.recv_timeout(WAIT_DELAY);
    }

    fn kill(&self) {
        // Ein Fehler heißt: der Prozess ist schon weg.
        let _ = self.kill.kill();
    }

    fn tail(&self) -> String {
        lock(&self.tail).last.clone()
    }
}

/// Nur ffmpeg bekommt stdin (für `q`).
fn wants_stdin(program: &str) -> bool {
    let name = Path::new(program)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    name.strip_suffix(".exe").unwrap_or(name).eq_ignore_ascii_case("ffmpeg")
}

fn start_real(runtime: &Handle, args: &[String], dir: &Path, stdin: bool) -> Result<Arc<dyn Proc>, Error> {
    let mut cmd = Cmd::new(args);
    cmd.command_mut().current_dir(dir).stderr(Stdio::piped());
    if stdin {
        cmd.command_mut().stdin(Stdio::piped());
    }
    // `tokio` braucht für den Start seine Laufzeit.
    let _guard = runtime.enter();
    let mut child = cmd
        .start()
        .map_err(|e| Error::Other(format!("{}: {e}", args.first().map_or("", String::as_str))))?;
    let tail = Arc::new(Mutex::new(TailWriter::default()));
    let done = Signal::new();
    let proc = RealProc {
        runtime: runtime.clone(),
        kill: child.kill_handle(),
        stdin: Mutex::new(child.child_mut().stdin.take()),
        tail: Arc::clone(&tail),
        done: Arc::clone(&done),
    };
    let stderr = child.child_mut().stderr.take();
    let reader = runtime.spawn(async move {
        let Some(mut stderr) = stderr else { return };
        let mut buf = vec![0; 4096];
        loop {
            match stderr.read(&mut buf).await {
                Ok(0) | Err(_) => return,
                Ok(n) => lock(&tail).write(&buf[..n]),
            }
        }
    });
    runtime.spawn(async move {
        let status = child.wait().await;
        let mut reader = reader;
        if tokio::time::timeout(WAIT_DELAY, &mut reader).await.is_err() {
            reader.abort();
        }
        done.set(exit_of(status).map_err(|e| e.message));
    });
    Ok(Arc::new(proc))
}

/// Echte Prozesse in der Laufzeit `runtime`. Die Aufrufe von [`Proc`] blockieren: nie von einem ihrer Worker rufen.
pub fn exec_spawn(runtime: Handle) -> Spawn {
    Arc::new(move |args, dir| {
        let stdin = wants_stdin(args.first().map_or("", String::as_str));
        start_real(&runtime, args, dir, stdin)
    })
}

/// Ein kurzer Aufruf mit stdout und stderr in einem Strom, höchstens `limit`.
pub fn run_output(runtime: &Handle, args: &[String], limit: Duration) -> (String, Result<(), String>) {
    output_of(runtime, args, None, limit)
}

/// [`run_output`] im Ordner `dir` (nur die Tests setzen ihn).
fn output_of(runtime: &Handle, args: &[String], dir: Option<&Path>, limit: Duration) -> (String, Result<(), String>) {
    let failed = |e: std::io::Error| (String::new(), Err(e.to_string()));
    let (mut reader, writer) = match std::io::pipe() {
        Ok(pipe) => pipe,
        Err(e) => return failed(e),
    };
    let mut cmd = Cmd::new(args);
    if let Some(dir) = dir {
        cmd.command_mut().current_dir(dir);
    }
    match writer.try_clone() {
        Ok(second) => cmd.command_mut().stdout(second).stderr(writer),
        Err(e) => return failed(e),
    };
    let _guard = runtime.enter();
    // `start` gibt den Befehl und damit unsere Schreibenden frei.
    let mut child = match cmd.start() {
        Ok(child) => child,
        Err(e) => return failed(e),
    };
    let (read, output) = mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        // Ein Lesefehler lässt, was schon da ist.
        let _ = reader.read_to_end(&mut buf);
        let _ = read.send(buf);
    });
    let kill = child.kill_handle();
    let done = Signal::new();
    let set = Arc::clone(&done);
    runtime.spawn(async move { set.set(exit_of(child.wait().await).map_err(|e| e.message)) });
    let status = done.wait_timeout(limit).unwrap_or_else(|| {
        let _ = kill.kill();
        done.wait()
    });
    let bytes = output.recv_timeout(WAIT_DELAY).unwrap_or_default();
    (String::from_utf8_lossy(&bytes).into_owned(), status)
}

/// Findet die Spiel-EXE wie „Im Spiel ansehen“: erst das Release-Ergebnis, dann das Paket; Arbeitsordner wie dort.
pub fn game_finder(root: &Path) -> GameFinder {
    let root = root.to_path_buf();
    Arc::new(move || {
        let join = |parts: &str| parts.split('/').fold(root.clone(), |path, part| path.join(part));
        let release = (join("rust/target/release/k3c-game.exe"), root.clone());
        let package = (join("dist/k3c-game/k3c-game.exe"), join("dist/k3c-game"));
        [release, package]
            .into_iter()
            .find(|(exe, _)| exe.is_file())
            .map(|(exe, dir)| (exe.to_string_lossy().into_owned(), dir))
            .ok_or(Error::NoGame)
    })
}

/// Die wählbaren Geräte der Seite.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    pub mics: Vec<String>,
    /// Geprüft mit einem Probebild; leer, wenn keiner aufnehmbar ist.
    pub monitors: Vec<Monitor>,
    pub ffmpeg: bool,
    pub ffmpeg_path: String,
    /// Der Encoder, den die Aufnahme nutzen würde (echte Probe).
    pub encoder: String,
    pub message: String,
}

static QUOTED: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("\"([^\"]+)\"").ok());

/// Audio-Geräte aus `ffmpeg -list_devices true -f dshow -i dummy` (neues Format mit `(audio)` je Zeile und altes mit
/// Abschnittsüberschrift); „Alternative name“-Zeilen zählen nicht.
pub(super) fn parse_mics(out: &str) -> Vec<String> {
    let mut mics: Vec<String> = Vec::new();
    let mut audio = false;
    for line in out.split('\n') {
        if line.contains("DirectShow audio devices") {
            audio = true;
        } else if line.contains("DirectShow video devices") {
            audio = false;
        }
        if line.contains("Alternative name") {
            continue;
        }
        let Some(name) = QUOTED.as_ref().and_then(|re| re.captures(line)).and_then(|c| c.get(1)) else {
            continue;
        };
        let name = name.as_str().to_owned();
        let in_section = audio && !line.contains("(video)");
        if mics.contains(&name) || (!line.contains("(audio)") && !in_section) {
            continue;
        }
        mics.push(name);
    }
    mics
}

/// Fragt ffmpeg über `list` nach den Mikrofonen (der Aufruf endet mit Fehlercode, die Liste steht trotzdem in der
/// Ausgabe) und probiert über `probe` Monitore und Encoder durch.
pub fn list_devices(look_path: &LookPath, list: &Probe, probe: &Probe, cache: &PickCache) -> Devices {
    let ffmpeg = match look_path("ffmpeg") {
        Ok(path) => path,
        Err(e) => {
            return Devices {
                message: no_ffmpeg(e).to_string(),
                ..Devices::default()
            };
        }
    };
    let args = strings(&[
        &ffmpeg,
        "-hide_banner",
        "-list_devices",
        "true",
        "-f",
        "dshow",
        "-i",
        "dummy",
    ]);
    let (out, _) = list(&args);
    let monitors = list_monitors(probe, &ffmpeg);
    let encoder = cache.pick(probe, &ffmpeg, None);
    Devices {
        mics: parse_mics(&out),
        monitors,
        ffmpeg: true,
        ffmpeg_path: ffmpeg,
        encoder,
        message: String::new(),
    }
}

#[cfg(test)]
#[path = "exec_tests.rs"]
mod tests;
