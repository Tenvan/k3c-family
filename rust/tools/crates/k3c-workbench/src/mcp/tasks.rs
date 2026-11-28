//! Tools `task_*` (Port von `tools_tasks.go`): Katalog und Läufe der Tasks-Seite. Agenten dürfen alle Tasks starten.

use std::sync::Arc;

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::console::TailIn;
use super::status::format_ms;
use super::tools::Workbench;
use crate::taskcat::{self, Task};
use crate::taskrun::{self, Run, State};

/// Die Tasks-Seite für die Tools; die Methoden blockieren.
pub trait TaskHost: Send + Sync {
    fn task_list(&self) -> Result<Vec<Task>, String>;
    fn task_start(&self, name: &str, args: Vec<String>) -> Result<Run, String>;
    fn task_stop(&self, name: &str) -> Result<Run, String>;
    fn task_runs(&self) -> Vec<Run>;
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct ListIn {
    #[serde(default)]
    #[schemars(description = "nur dieser Namensraum, z. B. golden")]
    namespace: String,
    #[serde(default)]
    #[schemars(description = "Freitext auf Name und Beschreibung")]
    query: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct StartIn {
    #[schemars(description = "Name aus task_list, z. B. rust:game:exe")]
    task: String,
    #[serde(default)]
    #[schemars(description = "Zusatzargumente, angehängt als task <name> -- <args>; keine Shell-Metazeichen")]
    args: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct TaskIn {
    #[schemars(description = "Name aus task_list")]
    task: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct StatusIn {
    #[serde(default)]
    #[schemars(description = "nur dieser Task; leer: alle Läufe dieser Sitzung")]
    task: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct OutputIn {
    #[schemars(description = "Name aus task_list")]
    task: String,
    #[serde(default)]
    #[schemars(description = "Zahl der letzten Zeilen, Standard 50, höchstens 500")]
    limit: i64,
    #[serde(default)]
    #[schemars(description = "nur Zeilen nach dieser Nummer")]
    since: i64,
}

/// Ein Lauf in einer Zeile, z. B. „rust:game:exe · succeeded · Exit 0 · 4,2 s · -- --dry“.
fn run_line(r: &Run) -> String {
    let mut parts = vec![r.name.clone(), r.state.as_str().to_owned()];
    if r.state == State::Running {
        parts.push(format!("PID {}", r.pid));
        let since = r.started_at.map(|at| at.format("%H:%M:%S").to_string());
        parts.push(format!("seit {}", since.unwrap_or_default()));
    } else {
        parts.push(format!("Exit {}", r.exit_code));
        parts.push(format_ms(r.duration_ms as f64));
    }
    if !r.reason.is_empty() {
        parts.push(r.reason.clone());
    }
    if !r.args.is_empty() {
        parts.push(format!("-- {}", r.args.join(" ")));
    }
    parts.join(" · ")
}

impl Workbench {
    /// `task_*` bedient nur die Repo-Wurzel.
    fn task_host(&self, ws: &Workspace) -> Result<Arc<dyn TaskHost>, String> {
        let Some(host) = &self.env.tasks else {
            return Err("keine Tasks: k3c-workbench läuft ohne Tasks-Seite".to_owned());
        };
        if !ws.name.is_empty() {
            return Err(format!(
                "task_* bedient nur die Repo-Wurzel, nicht den Worktree {}; im Worktree check_run nutzen",
                ws.name
            ));
        }
        Ok(Arc::clone(host))
    }

    /// Ruft den Host in einem blockierenden Faden.
    async fn with_host<T: Send + 'static>(
        &self,
        ws: &Workspace,
        call: impl FnOnce(&dyn TaskHost) -> T + Send + 'static,
    ) -> Result<T, String> {
        let host = self.task_host(ws)?;
        tokio::task::spawn_blocking(move || call(&*host))
            .await
            .map_err(|e| e.to_string())
    }
}

#[tool_router(router = task_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Taskfile-Baum nach Namensraum; jeder Task ist mit task_start startbar (kein Freigabe-Schloss). \
            Nutze es bei: welcher Task existiert. Statt: task --list-all.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn task_list(
        &self,
        Parameters(input): Parameters<ListIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let tasks = self.with_host(&ws, |h| h.task_list()).await??;
        let wanted = input.namespace.to_lowercase();
        let mut out = Vec::new();
        for ns in taskcat::group(&taskcat::filter(tasks, &input.query)) {
            if !wanted.is_empty() && ns.name.to_lowercase() != wanted {
                continue;
            }
            out.push(ns.name);
            out.extend(ns.tasks.iter().map(|t| format!("  {} · {}", t.name, t.desc)));
        }
        if out.is_empty() {
            return Ok("keine Tasks gefunden".to_owned());
        }
        Ok(out.join("\n"))
    }

    #[tool(
        description = "Startet einen beliebigen Task aus task_list und kehrt sofort zurück (kein Freigabe-Schloss). \
            Nutze es bei: Werkzeug-Tasks ohne eigenes Tool. Statt: task <name> in der Shell. Auf das Ende warten: \
            check_run.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn task_start(
        &self,
        Parameters(input): Parameters<StartIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.task_host(&ws)?;
        let args: Vec<String> = input.args.split_whitespace().map(str::to_owned).collect();
        taskrun::validate_args(&args)?;
        let name = input.task.clone();
        let run = self.with_host(&ws, move |h| h.task_start(&name, args)).await??;
        Ok(format!("{} · Ausgabe: task_output {}", run_line(&run), input.task))
    }

    #[tool(
        description = "Stoppt einen laufenden Task samt Prozessbaum. Nutze es bei: Task hängt oder läuft ungewollt. \
            Statt: taskkill.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn task_stop(
        &self,
        Parameters(input): Parameters<TaskIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let run = self.with_host(&ws, move |h| h.task_stop(&input.task)).await??;
        Ok(run_line(&run))
    }

    #[tool(
        description = "Läufe dieser Sitzung mit Zustand, Exit-Code, Dauer und Argumenten. \
            Nutze es bei: ist mein Task fertig. Statt: Prozessliste.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn task_status(
        &self,
        Parameters(input): Parameters<StatusIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let runs = self.with_host(&ws, |h| h.task_runs()).await?;
        let out: Vec<String> = runs
            .iter()
            .filter(|r| input.task.is_empty() || r.name == input.task)
            .map(run_line)
            .collect();
        if out.is_empty() {
            return Ok("keine Läufe in dieser Sitzung".to_owned());
        }
        Ok(out.join("\n"))
    }

    #[tool(
        description = "Konsole eines Task-Laufs (wie die rechte Hälfte der Tasks-Seite), since für neuere Zeilen. \
            Nutze es bei: Ausgabe eines laufenden oder beendeten Tasks. Statt: Ausgabe in der Shell mitlesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn task_output(
        &self,
        Parameters(input): Parameters<OutputIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.task_host(&ws)?;
        let tail = TailIn {
            service: format!("task:{}", input.task),
            limit: input.limit,
            since: input.since,
        };
        self.tail(&ws, &tail)
    }
}
