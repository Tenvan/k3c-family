//! Handler der Berichte-Tools (Port von `tools_gamedata.go`).

use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use crate::gamedata;

#[cfg(test)]
#[path = "tools_gamedata_tests.rs"]
mod tests;

/// Argumente von `report_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct ReportIn {
    #[schemars(description = "Dateiname aus reports_list, z. B. gamepad-2026-09-30T19-14-02-123Z.json")]
    name: String,
}

pub(super) fn reports_list(ws: &Workspace) -> Result<String, String> {
    gamedata::reports_list(&ws.root)
}

pub(super) fn report_read(ws: &Workspace, input: &ReportIn) -> Result<String, String> {
    gamedata::report_read(&ws.root, &input.name)
}

/// Der Ordner folgt `K3C_SAVES_DIR`.
pub(super) fn saves_list(ws: &Workspace) -> Result<String, String> {
    let env = std::env::var(gamedata::ENV_SAVES_DIR).unwrap_or_default();
    gamedata::saves_list(&ws.root, &gamedata::saves_dir(&ws.root, &env))
}
