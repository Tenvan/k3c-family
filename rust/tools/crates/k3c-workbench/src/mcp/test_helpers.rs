//! Hilfen der Server-Tests: Attrappen für Tools, die erst spätere Sessions bringen (Checkout-Regeln, Zähler), und
//! Checkouts im Temp-Ordner des Repos.

use std::path::Path;

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::tools::{self, Workbench};
use super::{Config, Server};

#[derive(Deserialize, JsonSchema)]
pub(super) struct EchoIn {
    text: String,
    #[serde(default)]
    times: u32,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct SectionIn {
    text: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct IdIn {
    id: String,
}

/// Datei, die die Attrappe `plan_section` im Checkout des Aufrufs schreibt.
pub(super) const WRITTEN: &str = "geschrieben.md";

#[tool_router(router = stubs)]
impl Workbench {
    #[tool(description = "Test")]
    async fn echo(&self, Parameters(input): Parameters<EchoIn>) -> String {
        input.text.repeat(1 + input.times as usize)
    }

    #[tool(description = "Test")]
    async fn boom(&self) -> String {
        panic!("kaputt")
    }

    #[tool(description = "Test")]
    async fn plan_section(
        &self,
        Parameters(input): Parameters<SectionIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        std::fs::write(ws.root.join(WRITTEN), input.text).map_err(|e| e.to_string())?;
        Ok("ersetzt".to_owned())
    }

    #[tool(description = "Test")]
    async fn plan_get(&self, Parameters(input): Parameters<IdIn>) -> String {
        input.id
    }
}

/// Gestarteter Server auf einem freien Port mit den echten Tools und den Attrappen.
pub(super) async fn started(cfg: Config) -> Server {
    let server = Server::with_router(cfg, tools::router() + Workbench::stubs());
    server.start().await.unwrap();
    server
}

pub(super) fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}
