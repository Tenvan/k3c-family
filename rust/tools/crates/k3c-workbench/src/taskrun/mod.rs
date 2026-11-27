//! Läufe von Taskfile-Tasks (Port von `internal/taskrun`): Start, Stopp, Zustand und Exit-Code je Task, höchstens ein
//! Lauf je Task. Kein Supervisor: ein Task endet mit einem Exit-Code und wird nie neu gestartet. Der Prozessbaum hängt
//! über `k3c_kit::proc` in einem Job, Stopp beendet ihn ganz.

mod args;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use chrono::{DateTime, Local};
use k3c_kit::proc::Cmd;
use serde::Serialize;

pub use args::{FORBIDDEN_ARG_CHARS, MAX_ARG_LEN, validate_arg, validate_args};

use crate::services::{Cancel, Exit, OutFn, Spawned, executable, go_time, spawn};

/// Zustand eines Laufs. Ein Task, der nie lief, hat keinen Eintrag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl State {
    pub fn as_str(self) -> &'static str {
        match self {
            State::Running => "running",
            State::Succeeded => "succeeded",
            State::Failed => "failed",
            State::Cancelled => "cancelled",
        }
    }
}

/// Aktueller oder letzter Lauf eines Tasks (`TaskRun` in `types.ts`).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub name: String,
    pub state: State,
    pub pid: u32,
    pub args: Vec<String>,
    #[serde(serialize_with = "go_time")]
    pub started_at: Option<DateTime<Local>>,
    /// Fehlt, solange er läuft.
    #[serde(serialize_with = "go_time")]
    pub ended_at: Option<DateTime<Local>>,
    pub exit_code: i64,
    pub duration_ms: i64,
    /// Grund eines Fehlschlags ohne Exit-Code (Prozess ließ sich nicht starten).
    pub reason: String,
}

/// Eine Ausgabezeile: Task, Strom, Text.
pub type Out = Arc<dyn Fn(&str, &str, &str) + Send + Sync>;

/// Teile eines Runners.
#[derive(Clone)]
pub struct Options {
    /// Arbeitsverzeichnis von `task`: die Repo-Wurzel.
    pub root: PathBuf,
    pub out: Out,
    /// Vor einem neuen Lauf: Konsole leeren.
    pub reset: Arc<dyn Fn(&str) + Send + Sync>,
    /// Jeder Zustandswechsel, nie unter der Sperre.
    pub on_change: Arc<dyn Fn(&Run) + Send + Sync>,
    /// Test-Naht: macht aus der Befehlszeile die, die wirklich startet.
    pub argv: Arc<dyn Fn(Vec<String>) -> Vec<String> + Send + Sync>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            root: PathBuf::new(),
            out: Arc::new(|_, _, _| {}),
            reset: Arc::new(|_| {}),
            on_change: Arc::new(|_| {}),
            argv: Arc::new(|argv| argv),
        }
    }
}

/// Beendet den Prozessbaum eines Laufs.
type Kill = Arc<dyn Fn() -> Result<(), String> + Send + Sync>;

struct Entry {
    run: Run,
    kill: Option<Kill>,
    /// Angeforderter Stopp: das Ende zählt als Cancelled, gleich mit welchem Exit-Code.
    stopping: bool,
    /// Wird ausgelöst, sobald der Lauf abgerechnet ist.
    done: Cancel,
}

struct Inner {
    opts: Options,
    runs: Mutex<BTreeMap<String, Entry>>,
}

/// Besitzt die gestarteten Läufe; Klone teilen sie.
#[derive(Clone)]
pub struct Runner(Arc<Inner>);

impl Runner {
    pub fn new(opts: Options) -> Runner {
        Runner(Arc::new(Inner {
            opts,
            runs: Mutex::default(),
        }))
    }

    fn runs(&self) -> MutexGuard<'_, BTreeMap<String, Entry>> {
        self.0.runs.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn emit(&self, run: &Run) {
        (self.0.opts.on_change)(run);
    }

    /// Führt `task <name> [-- args...]` aus. Ein Startfehler steht als Lauf im Zustand Failed mit `reason`, damit die
    /// Oberfläche ihn in der Zeile zeigt. Braucht eine tokio-Laufzeit.
    pub fn start(&self, name: &str, args: Vec<String>) -> Result<Run, String> {
        validate_args(&args)?;
        let mut runs = self.runs();
        if let Some(e) = runs.get(name).filter(|e| e.run.state == State::Running) {
            return Err(format!("task läuft bereits: {name} (PID {})", e.run.pid));
        }
        (self.0.opts.reset)(name);
        let spawned = self.spawn(name, &args);
        let mut entry = Entry {
            run: Run {
                name: name.to_owned(),
                state: State::Running,
                pid: 0,
                args,
                started_at: Some(Local::now()),
                ended_at: None,
                exit_code: 0,
                duration_ms: 0,
                reason: String::new(),
            },
            kill: None,
            stopping: false,
            done: Cancel::default(),
        };
        let spawned = match spawned {
            Ok(spawned) => spawned,
            Err(reason) => {
                entry.run.state = State::Failed;
                entry.run.ended_at = entry.run.started_at;
                entry.run.reason.clone_from(&reason);
                let run = entry.run.clone();
                runs.insert(name.to_owned(), entry);
                drop(runs);
                self.emit(&run);
                return Err(reason);
            }
        };
        let Spawned { pid, kill, wait } = spawned;
        entry.run.pid = pid;
        entry.kill = Some(Arc::from(kill));
        let (run, done) = (entry.run.clone(), entry.done.clone());
        runs.insert(name.to_owned(), entry);
        drop(runs);
        self.emit(&run);
        let (runner, name) = (self.clone(), name.to_owned());
        tokio::spawn(async move {
            let exit = wait.await;
            runner.finish(&name, exit, &done);
        });
        Ok(run)
    }

    fn spawn(&self, name: &str, args: &[String]) -> Result<Spawned, String> {
        let mut argv = vec!["task".to_owned(), name.to_owned()];
        if !args.is_empty() {
            argv.push("--".to_owned());
            argv.extend_from_slice(args);
        }
        let argv = (self.0.opts.argv)(argv);
        let root = &self.0.opts.root;
        let first = argv.first().ok_or_else(|| "command fehlt".to_owned())?;
        let mut line = vec![executable(first, root).into_os_string()];
        line.extend(argv.iter().skip(1).map(Into::into));
        let mut cmd = Cmd::new(&line);
        cmd.command_mut()
            .current_dir(root)
            .env("PW_TEST_HTML_REPORT_OPEN", "never");
        let (out, task) = (Arc::clone(&self.0.opts.out), name.to_owned());
        let lines: OutFn = Arc::new(move |stream: &str, text: &str| out(&task, stream, text));
        spawn(cmd, &lines)
    }

    /// Rechnet den Lauf ab, auf den `done` gehört.
    fn finish(&self, name: &str, exit: Exit, done: &Cancel) {
        let run = {
            let mut runs = self.runs();
            let Some(e) = runs.get_mut(name) else { return };
            let now = Local::now();
            e.run.ended_at = Some(now);
            e.run.duration_ms = e.run.started_at.map_or(0, |at| (now - at).num_milliseconds());
            e.run.exit_code = exit.as_ref().err().map_or(0, |w| w.code);
            e.run.state = match &exit {
                _ if e.stopping => State::Cancelled,
                Ok(()) => State::Succeeded,
                Err(w) => {
                    // Ohne Exit-Code ist das Warten selbst gescheitert.
                    if !w.message.starts_with("exit status") {
                        e.run.reason.clone_from(&w.message);
                    }
                    State::Failed
                }
            };
            e.kill = None;
            e.run.clone()
        };
        done.cancel();
        self.emit(&run);
    }

    /// Beendet den Prozessbaum und wartet höchstens `limit`, bis der Lauf abgerechnet ist.
    pub async fn stop(&self, name: &str, limit: Duration) -> Result<Run, String> {
        let (kill, done) = {
            let mut runs = self.runs();
            let running = runs.get_mut(name).filter(|e| e.run.state == State::Running);
            let Some(e) = running else {
                return Err(format!("task läuft nicht: {name}"));
            };
            e.stopping = true;
            (e.kill.clone(), e.done.clone())
        };
        if let Some(kill) = kill {
            kill().map_err(|e| format!("task {name}: {e}"))?;
        }
        // Nach Ablauf gilt der Stand, wie er ist.
        let _ = tokio::time::timeout(limit, done.cancelled()).await;
        let runs = self.runs();
        runs.get(name)
            .map(|e| e.run.clone())
            .ok_or_else(|| format!("task läuft nicht: {name}"))
    }

    /// Beendet alle laufenden Tasks (Beenden der Anwendung); Fehler werden gesammelt.
    pub async fn stop_all(&self, limit: Duration) -> Result<(), String> {
        let deadline = tokio::time::Instant::now() + limit;
        let mut errors = Vec::new();
        for run in self.all().into_iter().filter(|r| r.state == State::Running) {
            let left = deadline.saturating_duration_since(tokio::time::Instant::now());
            match self.stop(&run.name, left).await {
                Err(e) if !e.starts_with("task läuft nicht") => errors.push(e),
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    /// Alle bekannten Läufe nach Name.
    pub fn all(&self) -> Vec<Run> {
        self.runs().values().map(|e| e.run.clone()).collect()
    }
}
