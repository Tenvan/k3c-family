//! Der einzige Katalog aller Tools des Studios (Port von `tools.go`): Name, Beschreibung, Annotations und Handler.
//! Ein neues Tool kommt nur hier hinzu; spätere Sessions ergänzen ihre Gruppe als weiteren `tool_router`, tragen die
//! Namen in [`CATALOG`] ein und kürzen `PENDING_TOOLS` in `tests/mcp_tools_golden.rs`.

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};

use super::kit::Workspace;
use super::tools_gamedata::{self, ReportIn};

/// Empfänger der Tool-Methoden.
#[derive(Clone)]
pub struct Studio;

/// Reihenfolge der Tools in den Zählern wie in Go `register`.
pub(super) const CATALOG: &[&str] = &["reports_list", "report_read", "saves_list"];

/// Alle Routen.
pub(super) fn router() -> ToolRouter<Studio> {
    Studio::gamedata_router()
}

/// Die lesenden Tools für Berichte und Spielstände (B-063).
#[tool_router(router = gamedata_router)]
impl Studio {
    #[tool(
        description = "Xbox-Berichte der Gamepad-Testseite (reports/*.json), neueste zuerst: Datum, Gerät, Controller, FPS. Nutze es bei: neue Berichte von der Xbox finden. Statt: ls reports/.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn reports_list(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        tools_gamedata::reports_list(&ws)
    }

    #[tool(
        description = "Ein Xbox-Bericht verdichtet: Controller mit gesehenen Tasten, FPS je Sprite-Stufe, Vollbild, Zurück-Navigationen. Nutze es bei: Controller- oder FPS-Befund eines Berichts. Statt: JSON-Datei lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn report_read(
        &self,
        Parameters(input): Parameters<ReportIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        tools_gamedata::report_read(&ws, &input)
    }

    #[tool(
        description = "Spielstände (saves/ oder K3C_SAVES_DIR) mit Stufe, Tag, Spielern und Datum, Sicherungen eingeschlossen. Nutze es bei: welcher Spielstand existiert und wie weit er ist. Statt: saves/ durchsuchen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn saves_list(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        tools_gamedata::saves_list(&ws)
    }
}
