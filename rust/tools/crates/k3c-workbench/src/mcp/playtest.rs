//! Tools `playtest_list`, `playtest_get`, `playtest_frame`, `playtest_findings_set` und `playtest_tickets_create` (Port
//! von `tools_playtest.go`, B-1224): Claude liest Session, Transkript, Marker und Bilder und schlägt Befunde vor; die
//! Workbench ruft selbst keine KI auf. Angehakte Befunde werden über die Planungslogik zu Tickets. Die Antworttexte
//! stehen in `playtest_text.rs`.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use k3c_kit::mcp::Workspace;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::runtime::Handle;

use super::planning::in_workspace;
use super::playtest_text as text;
use super::tools::Workbench;
use crate::playtest::{self, Active, FindingIn, LookPath, RepoPlanner, Spawn, Store, TicketDefaults};

#[cfg(test)]
#[path = "playtest_tests.rs"]
mod tests;

/// Was die Tools der App mitbenutzen.
#[derive(Clone, Default)]
pub struct PlaytestHooks {
    /// Kennung der laufenden Aufnahme; gilt für die Ablage der Repo-Wurzel.
    pub active: Option<Active>,
    /// Suche und Start von ffmpeg für `playtest_frame` (Tests); ohne sie die echten.
    pub ffmpeg: Option<(LookPath, Spawn)>,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct GetIn {
    #[schemars(description = "Session-Kennung, z. B. 2026-11-22-1 (aus playtest_list)")]
    id: String,
    #[serde(default)]
    #[schemars(description = "Zeilen der Zeitleiste je Aufruf (Standard 60, höchstens 500)")]
    limit: i64,
    #[serde(default)]
    #[schemars(description = "ab welcher Zeile der Zeitleiste (aus der letzten Antwort)")]
    cursor: i64,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct FrameIn {
    #[schemars(description = "Session-Kennung")]
    id: String,
    #[schemars(description = "Zeitpunkt in Unix-ms (ts eines Markers oder einer Zeile von playtest_get)")]
    t: i64,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct FindingsIn {
    #[schemars(description = "Session-Kennung")]
    id: String,
    #[schemars(description = "Befunde; vorhandene (gleiche id) werden aktualisiert, Haken und Ticket bleiben")]
    findings: Vec<FindingIn>,
}

#[derive(Deserialize, JsonSchema)]
pub(super) struct TicketsIn {
    #[schemars(description = "Session-Kennung")]
    id: String,
    #[serde(default)]
    #[schemars(description = "Domäne für Befunde ohne eigene (REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP)")]
    domain: String,
    #[serde(default, rename = "type")]
    #[schemars(description = "Typ für Befunde ohne eigenen (Idee, Problem, Schuld, Frage)")]
    kind: String,
    #[serde(default)]
    #[schemars(description = "Prio für Befunde ohne eigene, sonst mittel")]
    prio: String,
    #[serde(default)]
    #[schemars(description = "Projekt-Kürzel für Befunde ohne eigenes")]
    project: String,
    #[serde(default)]
    #[schemars(description = "Sprint für Befunde ohne eigenen (fehlt er, zuerst plan_create kind sprint)")]
    sprint: String,
}

/// Die Ablage des Checkouts; die laufende Aufnahme der App gehört zur Repo-Wurzel.
fn store(root: &Path, repo: &Path, hooks: &PlaytestHooks) -> Store {
    Store {
        root: root.join("playtests"),
        active: hooks.active.clone().filter(|_| root == repo),
    }
}

fn create_tickets(store: &Store, root: &Path, input: TicketsIn) -> Result<String, String> {
    let defaults = TicketDefaults {
        domain: input.domain,
        kind: input.kind,
        prio: input.prio,
        project: input.project,
        sprint: input.sprint,
    };
    let result = store
        .create_tickets(&input.id, &defaults, &RepoPlanner(root))
        .map_err(|e| e.to_string())?;
    let report = text::ticket_report(&result);
    Ok(format!("{report}\nCheckout: {}", root.display()))
}

impl Workbench {
    /// Ruft `call` mit der Ablage des Checkouts in einem blockierenden Faden.
    async fn with_store(
        &self,
        ws: Workspace,
        call: impl FnOnce(Store, PathBuf) -> Result<String, String> + Send + 'static,
    ) -> Result<String, String> {
        let (repo, hooks) = (self.env.root.clone(), self.env.playtest.clone());
        in_workspace(ws, move |root| call(store(&root, &repo, &hooks), root)).await
    }
}

#[tool_router(router = playtest_router, vis = "pub(super)")]
impl Workbench {
    #[tool(
        description = "Aufgenommene Spieltests (playtests/) mit Stand, Dauer, Markern und Befunden, neueste zuerst. \
            Nutze es bei: Auswertung eines Spieltests beginnen. Statt: playtests/ durchsuchen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn playtest_list(&self, Extension(ws): Extension<Workspace>) -> Result<String, String> {
        self.with_store(ws, |store, _| text::list(&store)).await
    }

    #[tool(
        description = "Eine Session: Kopf, Fragebogen, Befunde und die Zeitleiste aus Transkript-Sätzen und Markern \
            (Spielkontext, Bildpfad, Zeitpunkt ts in Unix-ms), seitenweise mit limit und cursor. Nutze es bei: Transkript \
            und Marker lesen, um Befunde zu bilden. Statt: transcript.json und markers.jsonl von Hand öffnen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn playtest_get(
        &self,
        Parameters(input): Parameters<GetIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.with_store(ws, move |store, _| {
            text::get(&store, &input.id, input.limit, input.cursor)
        })
        .await
    }

    #[tool(
        description = "Zieht ein Bild an Zeitpunkt t (Unix-ms) aus video.mkv nach frames/t-<t>.jpg und nennt den Pfad \
            (dann mit Read ansehen). Nutze es bei: ein Bild zu einer Transkript-Stelle ohne Marker. Statt: ffmpeg von \
            Hand aufrufen.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn playtest_frame(
        &self,
        Parameters(input): Parameters<FrameIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let (look_path, spawn) = self.env.playtest.ffmpeg.clone().unwrap_or_else(|| {
            let look_path: LookPath = Arc::new(playtest::find_ffmpeg);
            (look_path, playtest::exec_spawn(Handle::current()))
        });
        self.with_store(ws, move |store, root| {
            let path = playtest::frame_at(&store, &look_path, &spawn, &input.id, input.t).map_err(|e| e.to_string())?;
            Ok(format!("Bild: {}\nCheckout: {}", text::clean(&path), root.display()))
        })
        .await
    }

    #[tool(
        description = "Schreibt oder aktualisiert Befunde (findings.json) einer Session; je Befund id, quote, ts, Vorschlag \
            für Typ, Prio, Projekt, Sprint und duplicate_of (offenes Ticket, vorher mit plan_list geprüft). Haken \
            (selected) und Ticket setzt nur Ralf bzw. playtest_tickets_create. Nutze es bei: Befunde aus Transkript und \
            Markern vorschlagen. Statt: findings.json von Hand schreiben.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn playtest_findings_set(
        &self,
        Parameters(input): Parameters<FindingsIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.with_store(ws, move |store, root| {
            let (added, updated) = store
                .set_findings(&input.id, &input.findings)
                .map_err(|e| e.to_string())?;
            Ok(format!(
                "Befunde geschrieben: {added} neu, {updated} aktualisiert (Session {}). Ralf hakt sie auf der Seite \
                 Spieltest an.\nCheckout: {}",
                input.id,
                root.display()
            ))
        })
        .await
    }

    #[tool(
        description = "Legt für alle angehakten Befunde ohne Ticket per Planungslogik ein Ticket an (Zitat, Bild, \
            Spielkontext; Domäne, Typ, Prio, Projekt, Sprint vom Befund oder aus den Argumenten) und trägt die ID ein; \
            Befunde mit duplicate_of bekommen stattdessen einen Hinweis im vorhandenen Ticket. Nutze es bei: Ralf hat \
            Befunde angehakt und Tickets angefordert. Statt: je Befund plan_create und plan_sections von Hand.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn playtest_tickets_create(
        &self,
        Parameters(input): Parameters<TicketsIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        self.with_store(ws, move |store, root| create_tickets(&store, &root, input))
            .await
    }
}
