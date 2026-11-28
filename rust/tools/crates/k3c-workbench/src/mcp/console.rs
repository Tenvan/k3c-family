//! Tool `console_tail` (Port von `tools_console.go`): die letzten Zeilen einer Konsolen-Quelle, stderr mit „! “.

use std::sync::LazyLock;

use k3c_kit::mcp::Workspace;
use regex::Regex;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::tools::Workbench;

const DEFAULT_TAIL: usize = 50;
const MAX_TAIL: usize = 500;

/// ANSI-Sequenzen; die Farben bleiben im Puffer für die Oberfläche.
static ANSI: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]").ok());

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub(super) struct TailIn {
    #[schemars(
        description = "Dienst oder Lauf, z. B. Vite oder check:task:test (Lauf des eigenen Checkouts, im Worktree checkout setzen)"
    )]
    pub service: String,
    #[serde(default)]
    #[schemars(description = "Zahl der letzten Zeilen, Standard 50, höchstens 500")]
    pub limit: i64,
    #[serde(default)]
    #[schemars(description = "nur Zeilen nach dieser Nummer (Nr. aus der letzten Antwort)")]
    pub since: i64,
}

fn plain(text: &str) -> String {
    ANSI.as_ref()
        .map_or_else(|| text.to_owned(), |re| re.replace_all(text, "").into_owned())
}

fn or_none(names: &[String]) -> String {
    if names.is_empty() {
        "noch keine".to_owned()
    } else {
        names.join(", ")
    }
}

impl Workbench {
    pub(super) fn tail(&self, ws: &Workspace, input: &TailIn) -> Result<String, String> {
        let n = usize::try_from(input.limit)
            .ok()
            .filter(|&n| n > 0)
            .unwrap_or(DEFAULT_TAIL)
            .min(MAX_TAIL);
        // Ab dem Cursor: alle holen, danach die ersten n nach dem Cursor.
        let want = if input.since > 0 { 0 } else { n };
        let console = &self.env.console;
        let source = &input.service;
        let lines = console.tail(&ws.console_source(source), want);
        let target = source.strip_prefix("check:");
        let Some(mut lines) = lines else {
            if target.is_some() {
                return Err(format!(
                    "kein Lauf von {source} im Checkout {} (check_run startet ihn dort)",
                    ws.label()
                ));
            }
            let own: Vec<String> = console.sources().iter().filter_map(|s| ws.own_source(s)).collect();
            return Err(format!("unbekannte Quelle {source:?}; bekannt: {}", or_none(&own)));
        };
        if input.since > 0 {
            lines.retain(|l| l.seq > input.since);
            lines.truncate(n);
        }
        let mut head = source.clone();
        // Läufe nennen Checkout und Startzeit, damit kein fremder Lauf als eigener durchgeht (B-855).
        if let Some(target) = target {
            head.push_str(&format!(" · Checkout {}", ws.label()));
            if let Some(at) = self.env.checks.start_of(&ws.source(target)) {
                head.push_str(&format!(" · Lauf ab {}", at.format("%H:%M:%S")));
            }
        }
        let Some(last) = lines.last().map(|l| l.seq) else {
            return Ok(format!("{head} · keine Ausgabe"));
        };
        let mut out = vec![format!("{head} · letzte {} Zeilen (bis Nr. {last})", lines.len())];
        out.extend(lines.iter().map(|l| {
            let mark = if l.stream == "stderr" { "! " } else { "" };
            format!("{mark}{}", plain(&l.text))
        }));
        Ok(out.join("\n"))
    }
}

#[tool_router(router = console_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Flüchtiger Konsolenpuffer eines Dienstes oder Laufs (z. B. Vite, check:task:test), stderr mit \"! \"; \
            since liefert nur neuere Zeilen. Nutze es bei: Startfehler, Rohausgabe eines Prüflaufs. Statt: Ausgabe in der \
            Shell mitlesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn console_tail(
        &self,
        Parameters(input): Parameters<TailIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.tail(&ws, &input)
    }
}
