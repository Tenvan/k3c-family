//! Tool `gh_status` (Port von `registerGitHub`): Stand von PR und CI je Sprint (B-212). Gilt wie in Go immer dem Repo
//! der Workbench, nicht dem Checkout des Aufrufs.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::tools::Workbench;
use crate::github;

#[derive(Deserialize, JsonSchema)]
pub(super) struct StatusIn {
    #[serde(default)]
    #[schemars(description = "Zwischenspeicher (60 s) umgehen")]
    force: bool,
}

#[tool_router(router = github_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "GitHub-Stand je Sprint über die gh-CLI: PR (offen, Entwurf, gemergt), CI (grün, rot, läuft), \
            Merge-Konflikt, dazu der letzte CI-Lauf auf main. 60 s zwischengespeichert, force holt neu. \
            Nutze es bei: Stand von PR und CI eines Sprints. Statt: gh pr list und gh run list in der Shell.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn gh_status(&self, Parameters(input): Parameters<StatusIn>) -> String {
        github::text(&self.env.github.status(input.force).await)
    }
}
