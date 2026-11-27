//! Der einzige Katalog aller Tools der Workbench (Port von `tools.go`): Name, Beschreibung, Annotations und Handler.
//! Ein neues Tool kommt hier hinzu (oder als weiterer `tool_router` seiner Gruppe) und mit seinem Namen in [`CATALOG`].

use std::sync::{Arc, OnceLock, Weak};

use k3c_kit::mcp::{Server as KitServer, Workspace};
use k3c_kit::usage::Tracker;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};

use super::Env;
use super::notify::{self, Notice, NotifyHook};
use super::status;

/// Empfänger der Tool-Methoden.
#[derive(Clone)]
pub(super) struct Workbench {
    /// Der eigene Server (Zähler, Adresse, Laufzeit); schwach, weil er diesen Empfänger besitzt.
    pub(super) server: Arc<OnceLock<Weak<KitServer<Workbench>>>>,
    pub(super) usage: Option<Arc<Tracker>>,
    pub(super) on_notify: Option<NotifyHook>,
    pub(super) env: Arc<Env>,
}

/// Reihenfolge der Tools in den Zählern wie in Go `register`.
pub(super) const CATALOG: &[&str] = &[
    "workbench_status",
    "notify_ui",
    "check_run",
    "console_tail",
    "svc_status",
    "svc_health",
    "svc_start",
    "svc_stop",
    "svc_restart",
    "svc_start_all",
    "svc_stop_all",
    "ports_status",
    "get_urls",
    "task_list",
    "task_start",
    "task_stop",
    "task_status",
    "task_output",
];

#[tool_router]
impl Workbench {
    #[tool(
        description = "Zustand von k3c-workbench: Adresse, Laufzeit, Aufrufe, Fehler, Clients und parallele Aufrufe. \
            Nutze es bei: Werkzeug antwortet seltsam, welcher Checkout gilt, Versionen. Statt: Prozesse und Ports von \
            Hand prüfen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn workbench_status(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        status::workbench_status(self, &ws).await
    }

    #[tool(
        description = "Zeigt dem Nutzer einen Hinweis in der Oberfläche von k3c-workbench (level info, success, warn oder \
            error). Nutze es bei: lange Arbeit fertig, Entscheidung oder Freigabe nötig. Statt: auf den Chat hoffen.",
        annotations(read_only_hint = false, idempotent_hint = false, open_world_hint = false)
    )]
    async fn notify_ui(&self, Parameters(notice): Parameters<Notice>) -> Result<String, String> {
        notify::notify_ui(self.on_notify.as_ref(), &notice)
    }
}

/// Alle Tools der Workbench.
pub(super) fn router() -> ToolRouter<Workbench> {
    Workbench::tool_router()
        + Workbench::check_router()
        + Workbench::console_router()
        + Workbench::service_router()
        + Workbench::port_router()
        + Workbench::task_router()
}
