//! Prozesslauf mit Zeitlimit und der Ablauf eines Laufs (Port von `runProcess`, `runCheck` und `logRun` aus
//! `check_run.go`).

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Local};
use k3c_kit::proc::Cmd;
use serde_json::json;

use super::Checks;
use super::job::Job;
use super::state::CheckState;
use super::steps::{StepLog, acquire_heavy};
use crate::services::{OutFn, Spawned, executable, spawn};

/// Ein Prozesslauf; `out` bekommt jede Zeile mit ihrem Strom.
#[derive(Clone)]
pub(super) struct RunSpec {
    pub dir: PathBuf,
    pub args: Vec<String>,
    pub timeout: Duration,
    pub out: OutFn,
    /// Zusätzlich zur Umgebung der Workbench.
    pub env: Vec<(String, String)>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct RunResult {
    pub exit: i64,
    pub dur: Duration,
    pub timed_out: bool,
    /// Start gescheitert.
    pub err: Option<String>,
    pub at: Option<DateTime<Local>>,
}

impl RunResult {
    pub(super) fn ms(&self) -> f64 {
        self.dur.as_micros() as f64 / 1000.0
    }

    fn failed(err: String) -> RunResult {
        RunResult {
            err: Some(err),
            ..RunResult::default()
        }
    }
}

/// Temp des Kindprozesses im Checkout (CLAUDE.md › Schreiben nur im Repo), wie das `env` der Wurzel-Taskfile.
fn temp_env(root: &Path) -> Result<Vec<(String, String)>, String> {
    let dir = root.join(".work").join("tmp");
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let dir = dir.display().to_string();
    Ok(["TMP", "TEMP"]
        .into_iter()
        .map(|name| (name.to_owned(), dir.clone()))
        .collect())
}

fn start_process(spec: &RunSpec) -> Result<Spawned, String> {
    let first = spec.args.first().ok_or_else(|| "command fehlt".to_owned())?;
    let mut line = vec![executable(first, &spec.dir).into_os_string()];
    line.extend(spec.args.iter().skip(1).map(Into::into));
    let mut cmd = Cmd::new(&line);
    cmd.command_mut()
        .current_dir(&spec.dir)
        .envs(spec.env.iter().map(|(k, v)| (k, v)));
    spawn(cmd, &spec.out)
}

/// Startet einen Prozess mit Zeitlimit; das Zeitlimit beendet den ganzen Prozessbaum. Ein Startfehler kommt vor dem
/// Zeitlimit, weil erst danach gewartet wird (B-702).
pub(super) async fn run_process(spec: RunSpec) -> RunResult {
    let (at, start) = (Local::now(), Instant::now());
    let mut res = RunResult::default();
    match start_process(&spec) {
        Err(e) => res = RunResult::failed(e),
        Ok(Spawned { kill, mut wait, .. }) => match tokio::time::timeout(spec.timeout, &mut wait).await {
            Ok(exit) => res.exit = exit.err().map_or(0, |e| e.code),
            Err(_) => {
                // Scheitert das Beenden, liefe `wait` ohne Ende; das Ergebnis steht trotzdem fest.
                if kill().is_ok() {
                    let _ = wait.await;
                }
                res.timed_out = true;
                res.exit = -1;
            }
        },
    }
    res.dur = start.elapsed();
    res.at = Some(at);
    res
}

fn panic_text(error: tokio::task::JoinError) -> String {
    let Ok(payload) = error.try_into_panic() else {
        return "abgebrochen".to_owned();
    };
    let text = payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default();
    format!("abgestürzt: {text}")
}

impl Checks {
    /// Führt den Lauf aus und meldet Ende, Ergebnis und Freigabe der Sperre, auch nach einer Panik.
    pub(super) async fn run_check(self: Arc<Self>, job: Arc<Job>, root: PathBuf, args: Vec<String>) {
        let source = format!("check:{}", job.key);
        self.console.reset(&source);
        let mut res = {
            // Ein schwerer Lauf zugleich (B-739); der Slot ist vor den Meldungen wieder frei.
            let _slot = acquire_heavy(&self.log, job.target.name, &job.key).await;
            self.execute(&job, root, args, source).await
        };
        // Startzeit für die Kopfzeile von `console_tail`, auch nach einer Panik (B-855).
        res.at = res.at.or(Some(job.started_at));
        job.set_result(res.clone());
        self.log_run(&job.key, &res);
        // Ende melden, solange das Ziel noch gesperrt ist: sonst käme „läuft“ des nächsten Laufs davor an.
        self.notify(&CheckState::finished(&job.key, &res));
        self.runs().end(&job.key, res);
        job.finish();
    }

    async fn execute(&self, job: &Arc<Job>, root: PathBuf, args: Vec<String>, source: String) -> RunResult {
        let env = match temp_env(&root) {
            Ok(env) => env,
            Err(e) => return RunResult::failed(e),
        };
        let console = Arc::clone(&self.console);
        let steps = StepLog::new(self.log.clone(), &job.key);
        let lines = Arc::clone(job);
        let spec = RunSpec {
            dir: root,
            args,
            timeout: job.target.timeout,
            out: Arc::new(move |stream: &str, text: &str| {
                console.add(&source, stream, text);
                lines.add(text);
                steps.line(text);
            }),
            env,
        };
        // Eigener Task, damit eine Panik als Ergebnis ankommt und die Sperre nicht hängen bleibt.
        let run = Arc::clone(&self.hooks.run);
        match tokio::spawn(async move { run(spec).await }).await {
            Ok(res) => res,
            Err(e) => RunResult::failed(panic_text(e)),
        }
    }

    fn log_run(&self, name: &str, res: &RunResult) {
        let mut attrs = vec![
            ("ns", json!("check")),
            ("target", json!(name)),
            ("exit", json!(res.exit)),
            ("ms", json!(res.ms() as u64)),
        ];
        if let Some(e) = &res.err {
            attrs.push(("error", json!(e)));
            self.log.error("💥 lauf nicht gestartet", &attrs);
        } else if res.timed_out {
            self.log.warn("⏳ lauf abgebrochen: zeitlimit", &attrs);
        } else if res.exit != 0 {
            self.log.warn("❌ lauf beendet", &attrs);
        } else {
            self.log.info("✅ lauf beendet", &attrs);
        }
    }
}
