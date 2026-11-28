//! Tool `git_commit` (Port von `tools_git_commit.go`): Index nur mit den genannten Dateien, nie auf main.

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::tools::Workbench;
use crate::gitcommit::{Git, Message};

#[cfg(test)]
#[path = "git_commit_tests.rs"]
mod tests;

/// Eingabe von `git_commit`: genau diese Dateien werden gestaged und committet.
#[derive(Deserialize, JsonSchema)]
pub(super) struct CommitIn {
    #[schemars(description = "Dateien relativ zum Repo; nur diese werden gestaged und committet")]
    paths: Vec<String>,
    #[serde(rename = "type")]
    #[schemars(description = "feat, fix, docs, refactor, test, perf, chore, ci, build oder style")]
    kind: String,
    #[serde(default)]
    #[schemars(description = "Domäne der Session kleingeschrieben, z. B. sim, cli, reg, dev, inf; leer = kein Scope")]
    scope: String,
    #[schemars(description = "Betreff, eine Zeile, höchstens 72 Zeichen")]
    subject: String,
    #[serde(default)]
    #[schemars(description = "Rumpf der Nachricht; ggf. Co-Authored-By-Zeile")]
    body: String,
}

/// Prüft Branch und Index, stagt die Pfade und committet.
async fn commit_files(git: &Git, input: CommitIn) -> Result<String, String> {
    let state = git.read().await?;
    if state.branch == "main" {
        return Err("auf main wird nicht committet: Feature-Branch anlegen".to_owned());
    }
    if let Some(file) = state.staged.iter().find(|f| !input.paths.contains(&f.path)) {
        return Err(format!(
            "fremde gestagte Datei im Index: {} (erst entfernen oder mit committen)",
            file.path
        ));
    }
    git.stage(&input.paths).await?;
    let message = Message {
        kind: input.kind,
        scope: input.scope,
        subject: input.subject,
        body: input.body,
    };
    let hash = git.commit(&message).await?;
    let message = message.normalize().map_err(|e| e.to_string())?;
    Ok(format!("Commit {hash} auf {}: {}", state.branch, message.format()))
}

#[tool_router(router = git_commit_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Committet die genannten Dateien des Repos auf dem aktuellen Feature-Branch. Staged wird nur, was \
            in paths steht; liegt anderes im Index, lehnt das Werkzeug ab. Auf main wird nie committet. Die Nachricht \
            folgt dem Format `typ(domäne): Betreff` (docs/arbeitsweise.md › Commit-Titel). Hooks laufen. Nutze es bei: \
            jedem Commit, auch bei Planungsdateien, die die Workbench angelegt hat. Statt: git add und git commit in \
            der Shell.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn git_commit(
        &self,
        Parameters(input): Parameters<CommitIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        commit_files(&Git::with_env(ws.root, self.env.git_env.clone()), input).await
    }
}
