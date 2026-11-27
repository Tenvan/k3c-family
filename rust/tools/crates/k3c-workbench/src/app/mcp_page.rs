//! MCP-Seite der Oberfläche (Port von `app_mcp.go`). Die Methoden blockieren und dürfen nur aus einem blockierenden
//! Thread (`spawn_blocking`, `block_in_place`) oder einem Thread ohne Laufzeit gerufen werden.

use std::sync::Arc;

use k3c_kit::mcp::{Call, Snapshot};
use k3c_kit::usage::{BASELINE_CALLS, OUTLIER_FACTOR, OUTLIER_FLOOR_MS, PERCENTILE_ERROR_PCT, Tracker, Usage};
use serde::Serialize;
use tokio::runtime::Handle;

use super::{App, McpState, lock};
use crate::mcp::{self, INSTRUCTIONS};

/// Meldung, wenn eine Methode nach dem Beenden kommt.
const STOPPED: &str = "backend beendet";

/// Band 1 und 2 der MCP-Seite (B-065): Zustand des Servers und seine Zähler.
#[derive(Clone, Debug, Serialize)]
pub struct McpOverview {
    pub mcp: McpState,
    pub stats: Snapshot,
}

/// Die Regeln der Statistik für die Erklärzeile; sie stehen nur in `k3c_kit::usage`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageRules {
    pub outlier_factor: f64,
    pub outlier_floor_ms: f64,
    pub baseline_calls: i64,
    pub percentile_error_pct: i64,
}

/// Nutzungsstatistik (Sitzung, Gesamtzeit, Minuten-Zeitreihe) mit ihren Regeln.
#[derive(Clone, Debug, Serialize)]
pub struct McpUsageView {
    #[serde(flatten)]
    pub usage: Usage,
    pub rules: UsageRules,
}

impl App {
    fn parts(&self) -> Result<(Arc<mcp::Server>, Arc<Tracker>, Handle), String> {
        self.wait();
        lock(&self.running)
            .as_ref()
            .map(|r| (Arc::clone(&r.server), Arc::clone(&r.tracker), r.runtime.clone()))
            .ok_or_else(|| STOPPED.to_owned())
    }

    /// Zustand und Zähler des MCP-Servers.
    pub fn mcp_overview(&self) -> Result<McpOverview, String> {
        let (server, _, runtime) = self.parts()?;
        let mcp = lock(&self.mcp).clone();
        let stats = runtime.block_on(server.stats());
        Ok(McpOverview { mcp, stats })
    }

    /// Startet den HTTP-Teil neu; Zähler und Katalog bleiben. Ein Fehler steht im Zustand, damit das Badge ihn zeigt.
    pub fn mcp_restart(&self) -> Result<McpState, String> {
        let (server, _, runtime) = self.parts()?;
        let result = runtime.block_on(server.restart());
        self.set_mcp(result.err());
        Ok(lock(&self.mcp).clone())
    }

    /// Der Text, den jeder Client beim Verbinden bekommt.
    pub fn mcp_instructions(&self) -> String {
        INSTRUCTIONS.to_owned()
    }

    /// Aufruf-Log, neueste zuerst, laufende eingeschlossen.
    pub fn mcp_calls(&self) -> Result<Vec<Call>, String> {
        let (server, _, _) = self.parts()?;
        Ok(server.calls())
    }

    /// Nutzungsstatistik mit ihren Regeln.
    pub fn mcp_usage(&self) -> Result<McpUsageView, String> {
        let (_, tracker, _) = self.parts()?;
        Ok(McpUsageView {
            usage: tracker.snapshot(),
            rules: UsageRules {
                outlier_factor: OUTLIER_FACTOR,
                outlier_floor_ms: OUTLIER_FLOOR_MS,
                baseline_calls: BASELINE_CALLS,
                percentile_error_pct: PERCENTILE_ERROR_PCT,
            },
        })
    }
}
