//! Prüfen, alle Starten und Stoppen, Ports und Adressen (Port von `tools_ports.go`).

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::services::status_text;
use super::tools::Workbench;
use crate::services::{Cancel, Service};

#[derive(Deserialize, JsonSchema)]
pub(super) struct ServiceIn {
    #[schemars(description = "Name aus svc_status, z. B. Vite")]
    service: String,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct UrlsIn {
    #[serde(default)]
    #[schemars(description = "Dienst aus svc_status oder mcp; leer: alle")]
    target: String,
}

fn service_urls(svc: &Service) -> String {
    if svc.port == 0 {
        return format!("{} · ohne Port · Health process", svc.name);
    }
    format!(
        "{} · http://localhost:{}/ · Health {}",
        svc.name,
        svc.port,
        svc.health_url()
    )
}

impl Workbench {
    /// Adresse des laufenden Servers, sonst leer.
    fn url(&self) -> String {
        self.server
            .get()
            .and_then(std::sync::Weak::upgrade)
            .map_or_else(String::new, |s| s.url())
    }
}

#[tool_router(router = port_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Prüft einen Dienst sofort mit seiner Health-Prüfung, ohne ihn zu ändern; ungesund mit Grund und \
            Belegung des Ports. Nutze es bei: Dienst läuft laut svc_status, antwortet aber nicht. Statt: curl auf den \
            Port.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn svc_health(
        &self,
        Parameters(input): Parameters<ServiceIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let ctl = self.env.controller(&ws).await?;
        let (svc, health) = ctl.health(&input.service).await?;
        let (name, url) = (&svc.name, svc.health_url());
        Ok(match health {
            Ok(()) => format!("{name} · gesund · {url}"),
            Err(e) => {
                let owner = ctl.port_owner(svc.net_port()).await;
                format!("{name} · ungesund · {url} · {e} · Port: {owner}")
            }
        })
    }

    #[tool(
        description = "Startet alle Dienste, die nicht laufen, und wartet, bis sie gesund sind; antwortet mit svc_status. \
            Nutze es bei: Umgebung frisch hochfahren. Statt: mehrere task-Befehle in der Shell.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn svc_start_all(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        let ctl = self.env.controller(&ws).await?;
        let started = ctl.start_all(&Cancel::default()).await;
        let text = status_text(&ctl);
        match started {
            Ok(()) => Ok(text),
            Err(e) => Err(format!("{e}\n{text}")),
        }
    }

    #[tool(
        description = "Stoppt alle eigenen Dienste in umgekehrter Startreihenfolge; übernommene bleiben. \
            Nutze es bei: Umgebung herunterfahren. Statt: Prozesse einzeln beenden.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn svc_stop_all(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        let ctl = self.env.controller(&ws).await?;
        ctl.stop_all().await;
        Ok(status_text(&ctl))
    }

    #[tool(
        description = "Belegung der Dev-Ports (Dienste und MCP), auch durch fremde Prozesse, mit PID. \
            Nutze es bei: Start scheitert an belegtem Port. Statt: netstat -ano oder Get-NetTCPConnection.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn ports_status(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        let ctl = self.env.controller(&ws).await?;
        let mut lines = Vec::new();
        // Ohne die Prozess-Dienste, sie haben keinen Port (B-1214).
        for svc in ctl.configs().iter().filter(|s| s.port != 0) {
            let owner = ctl.port_owner(svc.net_port()).await;
            lines.push(format!("{} · {} · {owner}", svc.port, svc.name));
        }
        let url = self.url();
        if !url.is_empty() {
            let addr = url.strip_prefix("http://").unwrap_or(&url);
            let addr = addr.strip_suffix("/mcp").unwrap_or(addr);
            lines.push(format!("{addr} · MCP k3c-workbench · eigener Prozess"));
        }
        Ok(lines.join("\n"))
    }

    #[tool(
        description = "Erreichbare Adressen je Dienst und des MCP-Servers (target: Dienst oder mcp). \
            Nutze es bei: vor jedem HTTP-Aufruf und jeder Browserprüfung, gerade im Worktree mit versetzten Ports. \
            Statt: Ports raten.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn get_urls(
        &self,
        Parameters(input): Parameters<UrlsIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let ctl = self.env.controller(&ws).await?;
        let target = input.target.to_lowercase();
        let mut lines: Vec<String> = ctl
            .configs()
            .iter()
            .filter(|s| target.is_empty() || s.name.to_lowercase() == target)
            .map(service_urls)
            .collect();
        let url = self.url();
        if !url.is_empty() && (target.is_empty() || target == "mcp") {
            lines.push(format!("MCP · {url}"));
        }
        if lines.is_empty() {
            return Err(format!(
                "unbekanntes Ziel {:?} (gültig: {}, mcp)",
                input.target,
                ctl.names().join(", ")
            ));
        }
        Ok(lines.join("\n"))
    }
}
