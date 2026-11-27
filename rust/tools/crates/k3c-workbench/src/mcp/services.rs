//! Dienste-Tools mit Zustandswechsel und `svc_status` (Port von `tools_services.go`).

use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{Local, TimeDelta};
use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::status::{format_ms, format_uptime};
use super::tools::Workbench;
use crate::services::{Cancel, Controller, LevelCounts, Outcome, State, Status};

/// Höchstes `waitSeconds` (wie das Start-Timeout der Dienste).
const MAX_WAIT: i64 = 60;
/// Zeilen der Konsole, die eine gescheiterte Aktion mitliefert.
const CONSOLE_ON_ERROR: usize = 10;

#[derive(Deserialize, JsonSchema)]
pub(super) struct StartIn {
    #[schemars(description = "Name aus svc_status, z. B. Vite")]
    service: String,
    #[serde(default, rename = "waitSeconds")]
    #[schemars(description = "höchstens so lange auf Health warten (1–60, Standard 60)")]
    wait_seconds: i64,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct StopIn {
    #[schemars(description = "Name aus svc_status, z. B. Vite")]
    service: String,
    #[serde(default)]
    #[schemars(description = "nötig für übernommene Dienste (vor k3c-workbench gestartet): beendet deren Prozessbaum")]
    force: bool,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct RestartIn {
    #[schemars(description = "Name aus svc_status, z. B. Vite")]
    service: String,
    #[serde(default, rename = "waitSeconds")]
    #[schemars(description = "höchstens so lange auf Health warten (1–60, Standard 60)")]
    wait_seconds: i64,
    #[serde(default)]
    #[schemars(description = "muss true sein: der Dienst ist während des Neustarts weg")]
    confirm: bool,
}

/// Signal, das nach `seconds` abläuft; 0 heißt Vorgabe des Controllers.
fn wait_signal(seconds: i64) -> Result<Cancel, String> {
    let signal = Cancel::default();
    if seconds == 0 {
        return Ok(signal);
    }
    let Some(secs) = u64::try_from(seconds)
        .ok()
        .filter(|_| (1..=MAX_WAIT).contains(&seconds))
    else {
        return Err(format!("waitSeconds {seconds} ungültig: erlaubt 1–{MAX_WAIT}"));
    };
    let timer = signal.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(secs)).await;
        timer.expire();
    });
    Ok(signal)
}

/// Größe deutsch: „512 B“, „1,5 KB“, „480,0 MB“.
fn format_bytes(n: u64) -> String {
    if n < 1024 {
        return format!("{n} B");
    }
    let (value, unit) = if n < 1 << 20 {
        (n as f64 / 1024.0, "KB")
    } else {
        (n as f64 / f64::from(1 << 20), "MB")
    };
    format!("{value:.1} {unit}").replacen('.', ",", 1)
}

/// WARN und ERROR immer, die übrigen Level nur, wenn es Einträge gibt.
fn level_line(c: &LevelCounts) -> String {
    let mut parts: Vec<String> = ["DEBUG", "INFO", "WARN", "ERROR"]
        .into_iter()
        .filter_map(|level| {
            let n = c.counts.get(level).copied().unwrap_or_default();
            (n > 0 || level == "WARN" || level == "ERROR").then(|| format!("{level} {n}"))
        })
        .collect();
    if c.budget_hit {
        parts.push("(Budget erreicht)".to_owned());
    }
    parts.join(" ")
}

/// Port für Antworten; Prozess-Dienste wie das Spiel haben keinen (B-1214).
fn port_text(port: i64) -> String {
    if port == 0 {
        "ohne Port".to_owned()
    } else {
        format!("Port {port}")
    }
}

/// Ein Dienst in einer Zeile.
fn service_line(ctl: &Controller, st: &Status) -> String {
    let mut parts = vec![st.name.clone(), st.state.to_string(), port_text(st.port)];
    if matches!(st.state, State::Running | State::Adopted) {
        parts.push(format!("PID {}", st.pid));
        // Die erste Messung kommt bis zu 2 s nach dem Start.
        if st.memory > 0 {
            parts.push(format!("CPU {}", format!("{:.1} %", st.cpu).replacen('.', ",", 1)));
            parts.push(format_bytes(st.memory));
        }
        let up = st.started_at.map_or_else(TimeDelta::zero, |at| Local::now() - at);
        parts.push(format_uptime(up));
    }
    if st.restarts > 0 {
        parts.push(format!("Neustarts {}", st.restarts));
    }
    if let Ok(Some(counts)) = ctl.log_levels(&st.name) {
        parts.push(format!("Log 60 min: {}", level_line(&counts)));
    }
    if !st.last_error.is_empty() {
        parts.push(format!("letzter Fehler: {}", st.last_error));
    }
    parts.join(" · ")
}

pub(super) fn status_text(ctl: &Controller) -> String {
    let lines: Vec<String> = ctl.statuses().iter().map(|st| service_line(ctl, st)).collect();
    lines.join("\n")
}

impl Workbench {
    /// Zeile für `workbench_status`.
    pub(super) async fn service_summary(&self, ws: &Workspace) -> String {
        let ctl = match self.env.controller(ws).await {
            Ok(ctl) => ctl,
            Err(e) => return format!("Dienste: {e}"),
        };
        let parts: Vec<String> = ctl
            .statuses()
            .iter()
            .map(|st| {
                let port = port_text(st.port);
                let port = port.strip_prefix("Port ").unwrap_or(&port);
                format!("{} {} ({port})", st.name, st.state)
            })
            .collect();
        format!("Dienste: {}", parts.join(" · "))
    }

    fn console_excerpt(&self, source: &str) -> String {
        match self.env.console.tail(source, CONSOLE_ON_ERROR) {
            Some(lines) if !lines.is_empty() => {
                let text: Vec<&str> = lines.iter().map(|l| l.text.as_str()).collect();
                format!("\nletzte Zeilen:\n{}", text.join("\n"))
            }
            _ => String::new(),
        }
    }

    /// Führt einen Befehl aus und antwortet mit dem neuen Zustand und der Dauer; bei einem Fehler mit dem Grund und den
    /// letzten Konsolenzeilen des Dienstes.
    async fn action<F>(
        &self,
        ws: &Workspace,
        name: &str,
        act: impl FnOnce(Arc<Controller>) -> F,
    ) -> Result<String, String>
    where
        F: Future<Output = Outcome>,
    {
        let ctl = self.env.controller(ws).await?;
        let began = Instant::now();
        match act(Arc::clone(&ctl)).await {
            Ok(st) => {
                let ms = began.elapsed().as_millis() as f64;
                Ok(format!("{} · {}", service_line(&ctl, &st), format_ms(ms)))
            }
            Err(e) => Err(format!("{}{}", e.message, self.console_excerpt(&ws.source(name)))),
        }
    }
}

#[tool_router(router = service_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Alle Dienste aus services.json mit Zustand, Port, PID, CPU, Speicher, Laufzeit, Neustarts, \
            Log-Level der letzten 60 min und letztem Fehler. Nutze es bei: jeder Arbeit an der laufenden Umgebung, als \
            ersten Aufruf. Statt: tasklist, netstat oder Raten.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn svc_status(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        Ok(status_text(&*self.env.controller(&ws).await?))
    }

    #[tool(
        description = "Startet einen Dienst und wartet, bis er gesund ist (waitSeconds, höchstens 60 s). \
            Nutze es bei: Dienst gestoppt oder fehlgeschlagen. Statt: task dev oder task start in der Shell; Ausgabe \
            über console_tail <Dienst>.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn svc_start(
        &self,
        Parameters(input): Parameters<StartIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let signal = wait_signal(input.wait_seconds)?;
        let name = input.service;
        self.action(&ws, &name, async |ctl| ctl.start(&signal, &name).await)
            .await
    }

    #[tool(
        description = "Stoppt einen Dienst samt Prozessbaum; übernommene (vor k3c-workbench gestartete) nur mit force. \
            Nutze es bei: Port freigeben, Dienst beenden. Statt: taskkill oder Stop-Process.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn svc_stop(
        &self,
        Parameters(input): Parameters<StopIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let (name, force) = (input.service, input.force);
        self.action(&ws, &name, async |ctl| ctl.stop(&name, force).await).await
    }

    #[tool(
        description = "Startet einen eigenen Dienst neu und wartet, bis er gesund ist; verlangt confirm=true. \
            Nutze es bei: Dienst hängt oder braucht neue Konfiguration. Statt: Stopp und Start von Hand in der Shell.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn svc_restart(
        &self,
        Parameters(input): Parameters<RestartIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        if !input.confirm {
            return Err(
                "svc_restart braucht confirm=true (der Dienst ist während des Neustarts nicht erreichbar)".to_owned(),
            );
        }
        let signal = wait_signal(input.wait_seconds)?;
        let name = input.service;
        self.action(&ws, &name, async |ctl| ctl.restart(&signal, &name).await)
            .await
    }
}
