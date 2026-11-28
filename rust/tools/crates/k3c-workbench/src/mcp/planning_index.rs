//! Tool `plan_index` (Port von `tools_planning_index.go`): Haupt- und Archiv-Index neu aufbauen (B-864).

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::planning::in_workspace;
use super::tools::Workbench;
use crate::planning;

#[derive(Deserialize, JsonSchema)]
pub(super) struct IndexIn {
    #[serde(default)]
    #[schemars(description = "nur die Zahlen zeigen, nichts schreiben (Standard true)")]
    dry: Option<bool>,
}

#[tool_router(router = planning_index_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Baut docs/backlog/README.md (offene und eingeplante Tickets) und docs/backlog/archiv/README.md \
            (erledigte und verworfene) aus den Ticket-Dateien neu auf: Zeilen gelöschter oder verschobener Dateien \
            verschwinden, fehlende kommen nach Nummer dazu, die Reihenfolge bisheriger Zeilen bleibt. dry (Standard true) \
            zeigt nur die Zahlen. Nutze es bei: Index passt nicht mehr zu den Dateien (z. B. nach git rm oder \
            Umnummerieren). Statt: Index von Hand bearbeiten.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_index(
        &self,
        Parameters(input): Parameters<IndexIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        // Ohne Angabe zeigt das Tool nur an.
        let dry = input.dry.unwrap_or(true);
        in_workspace(ws, move |root| planning::rebuild_index(&root, dry)).await
    }
}
