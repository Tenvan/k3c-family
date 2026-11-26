//! Recherche-Tools und Import (Port von `tools_assets_research.go`, B-298/AC-04 bis AC-06). Die Logik liegt in
//! `k3c_assetkit`; Treffer stehen in `data/assets-recherche.json`. Die Lizenz bestätigt nur 🧑 in der Oberfläche.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, FixedOffset, Local};
use k3c_assetkit as kit;
use k3c_assetkit::{Hit, Searcher};
use k3c_kit::mcp::inside_root;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use super::tools::Studio;
use super::tools_world::blocking;
use crate::kitx::go_quoted;

#[cfg(test)]
#[path = "tools_assets_research_tests.rs"]
mod tests;

/// Baut die Web-Recherche eines Checkouts (Test-Naht: lokaler Server statt Netz; Go `newSearcher`).
pub(super) type SearcherFactory = Arc<dyn Fn(&Path) -> Result<Searcher, String> + Send + Sync>;

/// Die echten Quellen.
pub(super) fn web_searcher() -> SearcherFactory {
    Arc::new(|root| kit::new_searcher(root).map_err(|e| e.to_string()))
}

/// Argumente von `assets_research`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct ResearchIn {
    #[schemars(description = "Asset-Slot, z. B. farm oder sfx:coin")]
    id: String,
    #[serde(default)]
    #[schemars(description = "Suchbegriff; leer = Beschreibung des Slots (englische Begriffe finden mehr)")]
    query: String,
}

/// Argumente von `assets_hits`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct SlotIn {
    #[schemars(description = "Asset-Slot, z. B. farm, mount:horse, sfx:coin")]
    id: String,
}

/// Argumente von `assets_hit_add`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct HitAddIn {
    #[schemars(description = "Asset-Slot, z. B. sfx:coin")]
    id: String,
    #[schemars(description = "Titel des Fundes")]
    titel: String,
    #[schemars(description = "Seite des Fundes (https)")]
    link: String,
    #[serde(default)]
    #[schemars(description = "z. B. itch.io, Kenney; Vorgabe Agent")]
    quelle: String,
    #[serde(default)]
    urheber: String,
    #[serde(default)]
    #[schemars(description = "wie auf der Seite angegeben; leer = Lizenz prüfen")]
    lizenz: String,
    #[serde(default)]
    #[schemars(description = "Bild- oder MP3-Adresse für die Vorschau")]
    vorschau: String,
    #[serde(default)]
    #[schemars(description = "direkte Datei, nur wenn die Seite sie mit maschinenlesbarer Lizenz anbietet")]
    download: String,
}

/// Argumente von `assets_hit_drop`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct HitDropIn {
    #[schemars(description = "Asset-Slot")]
    id: String,
    #[schemars(description = "ID des Treffers aus assets_hits")]
    hit: String,
}

/// Argumente von `assets_import`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct ImportIn {
    #[schemars(description = "Asset-Slot")]
    id: String,
    #[schemars(description = "ID des Treffers aus assets_hits")]
    hit: String,
    #[serde(default)]
    #[schemars(
        description = "lokale Datei (png bzw. ogg, mp3, wav); leer = direkter Download des Treffers (nur mit erkannter Lizenz)"
    )]
    path: String,
}

#[tool_router(router = assets_research_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Sucht für einen Asset-Slot im Netz (freesound mit Token, Wikimedia Commons, OpenGameArt) und speichert neue Treffer mit Link, Quelle, Urheber und Lizenz-Status nach Q70. Nutze es bei: Alternativen für einen Slot finden. Statt: Fundseiten von Hand anlegen.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true
        )
    )]
    async fn assets_research(
        &self,
        Parameters(input): Parameters<ResearchIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let searcher = (self.searcher)(&ws.root)?;
        let res = kit::run_research(&ws.root, &input.id, &input.query, &searcher, jetzt())
            .await
            .map_err(|e| e.to_string())?;
        Ok(format!(
            "{}: {} neu, {} gespeichert{}\n{}",
            input.id,
            res.added,
            res.hits.len(),
            notes_text(&res.notes),
            hits_text(&res.hits)
        ))
    }

    #[tool(
        description = "Gespeicherte Recherche-Treffer eines Asset-Slots: ID, Lizenz-Status (erkannt, pruefen, ausgeschlossen), Quelle, Titel, Link. Nutze es bei: welche Vorschläge liegen schon vor. Statt: data/assets-recherche.json lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_hits(
        &self,
        Parameters(input): Parameters<SlotIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || hits(&ws.root, &input.id)).await
    }

    #[tool(
        description = "Legt einen selbst gefundenen Treffer zu einem Asset-Slot ab (Agent hat im Netz gesucht); Lizenz wird nach Q70 eingeordnet, unklar bleibt „pruefen“. Nutze es bei: Fund von itch.io, Kenney oder anderen Seiten ohne API. Statt: Link nur im Chat nennen.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_hit_add(
        &self,
        Parameters(input): Parameters<HitAddIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || hit_add(&ws.root, input)).await
    }

    #[tool(
        description = "Verwirft einen Recherche-Treffer eines Asset-Slots. Nutze es bei: 🧑 hat einen Vorschlag abgelehnt. Statt: data/assets-recherche.json von Hand ändern.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_hit_drop(
        &self,
        Parameters(input): Parameters<HitDropIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || {
            kit::drop_hit(&ws.root, &input.id, &input.hit).map_err(|e| e.to_string())?;
            Ok(format!("{}: Treffer {} verworfen", input.id, input.hit))
        })
        .await
    }

    #[tool(
        description = "Übernimmt einen Recherche-Treffer: lokale Datei (path) oder direkter Download; legt sie unter public/…/kandidaten/ ab, trägt den Credit ein (Lizenz prüfen bis 🧑 bestätigt) und hängt sie als letzte Präferenz an. Nutze es bei: Fund übernehmen. Statt: Datei von Hand kopieren und CREDITS bearbeiten.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = true
        )
    )]
    async fn assets_import(
        &self,
        Parameters(input): Parameters<ImportIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let id = input.id.clone();
        let rel = if input.path.is_empty() {
            download(&ws.root, &input).await?
        } else {
            blocking(move || import_lokal(&ws.root, &input)).await?
        };
        Ok(format!("{id}: {rel} importiert, Credit eingetragen, letzte Präferenz"))
    }
}

/// Zeitpunkt in der Zeitzone des Rechners (Go `time.Now`).
fn jetzt() -> DateTime<FixedOffset> {
    Local::now().fixed_offset()
}

/// Direkter Download; welche Adressen erlaubt sind, prüft `k3c_assetkit`.
async fn download(root: &Path, input: &ImportIn) -> Result<String, String> {
    let client = kit::net::client().map_err(|e| e.to_string())?;
    kit::download_hit(root, &input.id, &input.hit, &client, jetzt())
        .await
        .map_err(|e| e.to_string())
}

/// Über MCP nur Dateien im eigenen Checkout (z. B. `.work/`): kein Agent kopiert fremde Dateien nach `public/`.
fn import_lokal(root: &Path, input: &ImportIn) -> Result<String, String> {
    let src: PathBuf = inside_root(root, &input.path)?;
    kit::import_file(root, &input.id, &input.hit, &src, jetzt()).map_err(|e| e.to_string())
}

fn notes_text(notes: &[String]) -> String {
    if notes.is_empty() {
        return String::new();
    }
    format!(" · Hinweise: {}", notes.join("; "))
}

fn hits_text(hits: &[Hit]) -> String {
    let mut b = String::new();
    for h in hits {
        let mut lic = if h.license.is_empty() {
            "Lizenz ?".to_owned()
        } else {
            h.license.clone()
        };
        if !h.flag.is_empty() {
            lic = format!("{lic} [{}]", h.flag);
        }
        let _ = writeln!(
            b,
            "{} {} · {} · {lic} · {} · {}",
            h.id, h.license_status, h.source, h.title, h.link
        );
    }
    b
}

fn hits(root: &Path, id: &str) -> Result<String, String> {
    let hits = kit::hits_of(root, id).map_err(|e| e.to_string())?;
    if hits.is_empty() {
        return Ok(format!("{id}: keine Treffer"));
    }
    Ok(format!("{id}: {} Treffer\n{}", hits.len(), hits_text(&hits)))
}

/// Legt einen Fund des Agenten ab; nur https-Links.
fn hit_add(root: &Path, input: HitAddIn) -> Result<String, String> {
    if !input.link.starts_with("https://") {
        return Err(format!("link muss mit https:// beginnen: {}", go_quoted(&input.link)));
    }
    let source = if input.quelle.is_empty() {
        "Agent".to_owned()
    } else {
        input.quelle
    };
    let hit = Hit {
        title: input.titel,
        link: input.link,
        source,
        artist: input.urheber,
        license: input.lizenz,
        preview: input.vorschau,
        download: input.download,
        ..Hit::default()
    };
    let n = kit::add_hits(root, &input.id, &[hit], jetzt()).map_err(|e| e.to_string())?;
    if n == 0 {
        return Ok(format!("{}: Link schon gespeichert", input.id));
    }
    Ok(format!("{}: Treffer gespeichert", input.id))
}
