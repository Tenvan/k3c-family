//! Tool `check_run` (Port von `check_run.go`): prüfen, sperren, laufen lassen, verdichten. Die volle Ausgabe steht im
//! Konsolenpuffer `check:<ziel>`.

mod compact;
mod job;
mod run;
mod state;
mod steps;
mod targets;
mod taskversion;
mod wait;

#[cfg(test)]
mod background_tests;
#[cfg(test)]
mod compact_tests;
#[cfg(test)]
mod pattern_tests;
#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod steps_tests;
#[cfg(test)]
mod targets_tests;
#[cfg(test)]
mod taskversion_tests;
#[cfg(test)]
mod testutil;
#[cfg(test)]
mod wait_tests;

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use k3c_kit::console::Store;
use k3c_kit::logger::Logger;
use k3c_kit::mcp::{Progress, Workspace};
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

pub use state::CheckState;

use super::tools::Workbench;
use job::{MAX_JOBS, Runs};
use run::{RunResult, RunSpec};
use targets::{CHECK_RUN_DESCRIPTION, find_target};
use taskversion::VersionMemo;

/// Meldet Beginn und Ende eines Prüflaufs (Quellenleiste der Oberfläche).
pub type CheckHook = Arc<dyn Fn(&CheckState) + Send + Sync>;

type Boxed<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// Nähte der Tests und die Fristen.
pub(super) struct Hooks {
    run: Arc<dyn Fn(RunSpec) -> Boxed<RunResult> + Send + Sync>,
    task_version: Arc<dyn Fn() -> Boxed<Result<String, String>> + Send + Sync>,
    /// Wie lange `check_run` ohne `progressToken` wartet.
    wait: Duration,
    /// Abstand der Fortschritts-Meldungen.
    progress_every: Duration,
}

impl Default for Hooks {
    fn default() -> Hooks {
        Hooks {
            run: Arc::new(|spec| Box::pin(run::run_process(spec))),
            task_version: Arc::new(|| Box::pin(taskversion::read_task_version())),
            wait: wait::DEFAULT_CHECK_WAIT,
            progress_every: wait::DEFAULT_PROGRESS_EVERY,
        }
    }
}

/// Die Prüfläufe eines Servers.
pub(super) struct Checks {
    log: Logger,
    console: Arc<Store>,
    on_check: Option<CheckHook>,
    hooks: Hooks,
    runs: Mutex<Runs>,
    version: VersionMemo,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct CheckIn {
    #[serde(default)]
    #[schemars(
        description = "Ziel aus dem Katalog: task:check, task:test, task:typecheck, task:lint, task:build, task:check:go, task:check:diff, task:check:diff:plan, task:check:diff:js, task:check:diff:go, task:check:race, go:test, go:lint, dev:test, task:check:dev"
    )]
    target: String,
    #[serde(default)]
    #[schemars(description = "optionales Testmuster, nur bei task:test, go:test und dev:test")]
    pattern: String,
    #[serde(default)]
    #[schemars(
        description = "Lauf-ID aus einer Antwort „läuft noch“: wartet weiter auf diesen Lauf und liefert sein Ergebnis, statt einen neuen zu starten"
    )]
    run: String,
}

impl Checks {
    pub(super) fn new(log: Logger, console: Arc<Store>, on_check: Option<CheckHook>) -> Checks {
        Checks::with_hooks(log, console, on_check, Hooks::default())
    }

    pub(super) fn with_hooks(log: Logger, console: Arc<Store>, on_check: Option<CheckHook>, hooks: Hooks) -> Checks {
        Checks {
            log,
            console,
            on_check,
            hooks,
            runs: Mutex::default(),
            version: VersionMemo::new(),
        }
    }

    fn runs(&self) -> MutexGuard<'_, Runs> {
        self.runs.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Gemerkte Läufe und Bytes ihrer Zeilen.
    pub(super) fn stats(&self) -> (usize, i64) {
        let (jobs, bytes) = self.runs().stats();
        (jobs, i64::try_from(bytes).unwrap_or(i64::MAX))
    }

    /// Warnung zur Task-Version oder `None`; Fehler, wenn Task zu alt ist.
    async fn version_warning(&self, target: &targets::CheckTarget, pattern: &str) -> Result<Option<String>, String> {
        if !taskversion::needs_version(target, pattern) {
            return Ok(None);
        }
        let read = self.version.get_or_init(|| (self.hooks.task_version)()).await;
        taskversion::check_task_version(target, read)
    }

    async fn check_run(
        self: &Arc<Self>,
        ws: &Workspace,
        input: &CheckIn,
        progress: &Progress,
    ) -> Result<String, String> {
        if !input.run.is_empty() {
            let job = self.runs().job(&input.run).ok_or_else(|| {
                format!(
                    "unbekannte Lauf-ID {:?} (gemerkt werden die letzten {MAX_JOBS} Läufe seit dem Start von \
                     k3c-workbench)",
                    input.run
                )
            })?;
            return self.await_job(&job, progress).await;
        }
        let target = find_target(&input.target)?;
        let args = target.args(&input.pattern)?;
        // Sperre, Ergebnis und Konsole je Checkout: Läufe verschiedener Worktrees stören sich nicht.
        target.preflight(&ws.root)?;
        let warn = self.version_warning(target, &input.pattern).await?;
        let key = ws.source(target.name);
        let begun = self.runs().begin(target, &key);
        let job = begun.map_err(|id| {
            format!(
                "{name} läuft bereits (Lauf-ID {id}); auf das Ende warten, Ergebnis mit check_run {{run}}, Ausgabe \
                 über console_tail check:{name}",
                name = target.name
            )
        })?;
        self.notify(&CheckState::running(&key, job.started_at));
        // Eigener Task: bricht der Client ab, läuft der Lauf zu Ende (B-372).
        tokio::spawn(Arc::clone(self).run_check(Arc::clone(&job), ws.root.clone(), args));
        let out = self.await_job(&job, progress).await?;
        Ok(match warn {
            Some(warn) => format!("{warn}\n{out}"),
            None => out,
        })
    }
}

#[tool_router(router = check_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = CHECK_RUN_DESCRIPTION,
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn check_run(
        &self,
        Parameters(input): Parameters<CheckIn>,
        Extension(ws): Extension<Workspace>,
        Extension(progress): Extension<Progress>,
    ) -> Result<String, String> {
        self.env.checks.check_run(&ws, &input, &progress).await
    }
}
