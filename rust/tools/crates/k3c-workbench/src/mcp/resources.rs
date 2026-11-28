//! `workbench_resources` und die Puffer-Messwerte des Servers (Port von `resources.go`). `/debug/pprof/` gibt es hier
//! nicht.

use std::sync::{Arc, Weak, mpsc};
use std::time::Duration;

use k3c_kit::mcp::Server as KitServer;
use rmcp::{tool, tool_router};
use tokio::runtime::Handle;

use super::Env;
use super::tools::Workbench;
use crate::resmon::Monitor;

#[cfg(test)]
#[path = "resources_tests.rs"]
mod tests;

const OFF: &str = "das Ressourcen-Monitoring ist in dieser Instanz nicht eingeschaltet";
/// So lange wartet ein Messwert auf die Laufzeit des Servers.
const SESSIONS_WAIT: Duration = Duration::from_millis(500);

/// Zahl der MCP-Sessions aus einem Thread heraus, der nicht zur Laufzeit gehört oder dort blockieren darf.
fn sessions(server: &Weak<KitServer<Workbench>>, runtime: &Handle) -> i64 {
    let Some(server) = server.upgrade() else {
        return 0;
    };
    let (send, receive) = mpsc::channel();
    runtime.spawn(async move {
        let _ = send.send(server.clients().await);
    });
    let count = receive.recv_timeout(SESSIONS_WAIT).unwrap_or_default();
    i64::try_from(count).unwrap_or(i64::MAX)
}

fn count(n: usize) -> i64 {
    i64::try_from(n).unwrap_or(i64::MAX)
}

/// Meldet die Puffer des Servers an: Konsolenpuffer, gehaltene Prüfläufe, Sessions.
pub(super) fn register_gauges(monitor: &Monitor, env: &Arc<Env>, server: Weak<KitServer<Workbench>>) {
    let console = Arc::clone(&env.console);
    monitor.gauge("Konsole-Quellen", false, move || count(console.stats().0));
    let console = Arc::clone(&env.console);
    monitor.gauge("Konsole-Zeilen", false, move || count(console.stats().1));
    let console = Arc::clone(&env.console);
    monitor.gauge("Konsole-Text", true, move || console.stats().2);
    let checks = Arc::clone(&env.checks);
    monitor.gauge("Prüfläufe", false, move || count(checks.stats().0));
    let checks = Arc::clone(&env.checks);
    monitor.gauge("Prüflauf-Text", true, move || checks.stats().1);
    // Der Server entsteht nur in einer Laufzeit mit Start; ohne sie fehlt der Messwert.
    let runtime = Handle::try_current().ok();
    monitor.gauge("MCP-Sessions", false, move || {
        runtime.as_ref().map_or(0, |runtime| sessions(&server, runtime))
    });
}

/// Der Bericht des Messgeräts.
async fn report(monitor: Option<Arc<Monitor>>) -> Result<String, String> {
    let monitor = monitor.ok_or_else(|| OFF.to_owned())?;
    // Ohne frühere Probe misst der Bericht selbst und blockiert dabei.
    tokio::task::spawn_blocking(move || monitor.report())
        .await
        .map_err(|e| e.to_string())
}

#[tool_router(router = resources_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Ressourcenverbrauch von k3c-workbench: Live-Heap, Prozess-RSS, Kindprozesse, Threads, Handles, Trend \
            je Stunde, Größe der Puffer (Konsole, Prüfläufe) und die Vorgänge mit der größten Allokation. Nutze es bei: \
            Workbench wird langsam oder frisst Speicher, Verdacht auf ein Leck. Statt: Task-Manager und Raten.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn workbench_resources(&self) -> Result<String, String> {
        report(self.env.resources.clone()).await
    }
}
