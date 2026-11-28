//! Tool `plan_digest` (Port von `tools_planning_digest.go`): Zusammenfassungen erledigter Sprints (B-696 Teil C).

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::planning::in_workspace;
use super::tools::Workbench;
use crate::planning::{self, DigestIn};

/// Die Eigenschaft `älter_als` heißt wie in Go, auch wenn die Anthropic-API den Namen ablehnt.
#[derive(Deserialize, JsonSchema)]
pub(super) struct ToolIn {
    #[serde(default, rename = "älter_als")]
    #[schemars(
        description = "nur Sprints vor dieser Sprint-ID (gleiches Präfix, z. B. SP10) oder mit letzter Abnahme vor dem \
            Datum JJJJ-MM-TT; leer = alle erledigten"
    )]
    older_than: String,
    #[serde(default)]
    #[schemars(description = "nur anzeigen, nichts schreiben (Standard true)")]
    dry: Option<bool>,
    #[serde(default, rename = "neu")]
    #[schemars(description = "vorhandene ZUSAMMENFASSUNG.md neu schreiben (Standard: unverändert lassen)")]
    renew: bool,
}

#[tool_router(router = planning_digest_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Schreibt je erledigtem Sprint docs/sprints/erledigt/<Sprint>/ZUSAMMENFASSUNG.md (höchstens 60 \
            Zeilen): Ziel und Abnahme gekürzt, Kriterien mit Status aus den Session-Ergebnissen, Tickets mit Status; \
            regelbasiert, ohne KI. Originale bleiben, vorhandene Zusammenfassungen nur mit neu. dry (Standard true) zeigt \
            nur die Sprints. Nutze es bei: erledigte Sprints verdichten, bevor eine Session sie liest. Statt: \
            Sprint-Ordner ganz lesen oder Zusammenfassungen von Hand.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn plan_digest(
        &self,
        Parameters(input): Parameters<ToolIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        // `DigestIn::default` schreibt (dry = false); ohne Angabe zeigt das Tool nur an.
        let digest = DigestIn {
            older_than: input.older_than,
            dry: input.dry.unwrap_or(true),
            renew: input.renew,
        };
        in_workspace(ws, move |root| planning::digest(&root, &digest)).await
    }
}
