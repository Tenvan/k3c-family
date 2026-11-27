//! `assets_render` (Port von `tools_assets_render.go`): ein Asset synchron, alle als Job, Stand und Warten. Die Jobs
//! teilt sich das Tool mit `assets_style_set`.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use k3c_kit::mcp::Progress;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use super::tools::Studio;
use super::tools_assets::mehrzahl;
use super::tools_assets_set::ASSET_JOBS;
use super::tools_world::blocking;
use crate::kitx::go_quoted;
use crate::paperassets as pa;

#[cfg(test)]
#[path = "tools_assets_render_tests.rs"]
mod tests;

/// Begrenzt `{job, wait: true}` ohne `progressToken` (wie `check_run`); mit Fortschritt wird bis zum Ende gewartet.
const RENDER_WARTEN: Duration = Duration::from_secs(4 * 60);
const MIT_FORTSCHRITT: Duration = Duration::from_secs(24 * 60 * 60);
/// Abstand der Lebenszeichen beim Warten.
const TAKT: Duration = Duration::from_secs(5);

/// Argumente von `assets_render`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RenderIn {
    #[serde(default)]
    #[schemars(description = "ein Asset synchron rendern")]
    id: String,
    #[serde(default)]
    #[schemars(description = "true startet einen Job für alle Assets")]
    alle: bool,
    #[serde(default)]
    #[schemars(description = "Job-ID j-<n>: Stand abfragen")]
    job: String,
    #[serde(default)]
    #[schemars(description = "mit job: bis zum Ende warten")]
    wait: bool,
}

#[tool_router(router = assets_render_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Rendert Assets über die Render-CLI: {id} synchron, {alle: true} als Job (sofort zurück), {job} Stand, {job, wait: true} bis zum Ende. Nutze es bei: Ausgabe neu erzeugen, nach assets_style_set warten. Statt: CLI in der Shell.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_render(
        &self,
        Parameters(input): Parameters<RenderIn>,
        Extension(ws): Extension<Workspace>,
        Extension(progress): Extension<Progress>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, String> {
        pruefe_form(&input)?;
        if !input.wait {
            return blocking(move || render(&ws.root, &input)).await;
        }
        // Ein abgebrochener Aufruf beendet nur das Warten; der Job läuft weiter.
        tokio::select! {
            text = warte(&ASSET_JOBS, input.job, progress, RENDER_WARTEN, TAKT) => text,
            () = context.ct.cancelled() => Err("context canceled".to_owned()),
        }
    }
}

/// Genau eine der Formen `{id}`, `{alle}`, `{job}`; `wait` nur mit `job`.
fn pruefe_form(input: &RenderIn) -> Result<(), String> {
    let formen = [!input.id.is_empty(), input.alle, !input.job.is_empty()];
    if formen.into_iter().filter(|gesetzt| *gesetzt).count() != 1 || (input.wait && input.job.is_empty()) {
        return Err("assets_render braucht genau eins: {id}, {alle: true} oder {job[, wait: true]}".to_owned());
    }
    Ok(())
}

/// Alle Formen ohne Warten.
fn render(root: &Path, input: &RenderIn) -> Result<String, String> {
    let cli = pa::Cli::new(root);
    if !input.id.is_empty() {
        let a = pa::finde(root, &input.id).map_err(|e| e.to_string())?;
        return render_eins_text(root, &pa::render_eins(&cli, &a));
    }
    if input.alle {
        let assets = pa::assets(root).map_err(|e| e.to_string())?;
        let st = ASSET_JOBS.start(&cli, &assets);
        let job = go_quoted(&st.id);
        return Ok(format!(
            "Job {} gestartet: {} {} (alle)\nWeiter: assets_render {{job: {job}}} oder {{job: {job}, wait: true}}",
            st.id,
            st.n,
            mehrzahl(st.n, "Asset", "Assets"),
        ));
    }
    let st = ASSET_JOBS.status(&input.job).map_err(|e| e.to_string())?;
    Ok(job_text(&st))
}

/// Wartet auf den Job: mit `progressToken` bis zum Ende und Lebenszeichen, sonst höchstens `grenze`.
async fn warte(
    jobs: &'static pa::Jobs,
    id: String,
    progress: Progress,
    grenze: Duration,
    takt: Duration,
) -> Result<String, String> {
    let grenze = if progress.is_active() { MIT_FORTSCHRITT } else { grenze };
    let start = Instant::now();
    loop {
        let scheibe = grenze.saturating_sub(start.elapsed()).min(takt);
        let job = id.clone();
        let st = blocking(move || jobs.wait(&job, Some(scheibe)).map_err(|e| e.to_string())).await?;
        if st.fertig || start.elapsed() >= grenze {
            return Ok(job_text(&st));
        }
        let msg = format!("Job {id} {}/{} ({})", st.erledigt, st.n, st.aktuell);
        progress.report(&msg, start.elapsed());
    }
}

/// Laufend eine Zeile, fertig das Ergebnis je Asset.
fn job_text(st: &pa::Stand) -> String {
    if !st.fertig {
        return format!("Job {} läuft {}/{} ({})", st.id, st.erledigt, st.n, st.aktuell);
    }
    let (mut pngs, mut fehler) = (0, 0);
    let mut zeilen = Vec::with_capacity(st.ergebnisse.len());
    for e in &st.ergebnisse {
        pngs += e.pngs;
        if !e.fehler.is_empty() {
            fehler += 1;
            zeilen.push(format!("{} · Fehler: {}", e.id, e.fehler));
        } else if !e.uebersprungen.is_empty() {
            zeilen.push(format!("{} · übersprungen (ungültig: {})", e.id, e.uebersprungen));
        } else {
            let mut z = format!("{} · {} {}", e.id, e.pngs, mehrzahl(e.pngs, "PNG", "PNGs"));
            if !e.manifest.is_empty() {
                z = format!("{z} · {}", e.manifest);
            }
            zeilen.push(z);
        }
    }
    let dauer = format!("{:.1}", st.dauer.as_secs_f64()).replacen('.', ",", 1);
    format!(
        "Job {} fertig: {}/{} Assets, {pngs} {}, {fehler} Fehler, {dauer} s\n{}",
        st.id,
        st.erledigt,
        st.n,
        mehrzahl(pngs, "PNG", "PNGs"),
        zeilen.join("\n")
    )
}

fn render_eins_text(root: &Path, e: &pa::AssetErgebnis) -> Result<String, String> {
    if !e.fehler.is_empty() {
        return Err(e.fehler.clone());
    }
    if !e.uebersprungen.is_empty() {
        return Err(format!(
            "{} nicht gerendert: ungültig ({}); assets_list zeigt die Fehler",
            e.id, e.uebersprungen
        ));
    }
    Ok(format!(
        "{} gerendert: {} {}, manifest.json · renderKey {}\n{}",
        e.id,
        e.pngs,
        mehrzahl(e.pngs, "PNG", "PNGs"),
        render_key(root, &e.manifest),
        e.manifest
    ))
}

/// Der `renderKey` aus dem Manifest (Pfad relativ zum Checkout); unbekannt = –.
fn render_key(root: &Path, manifest: &str) -> String {
    #[derive(Deserialize)]
    struct Manifest {
        #[serde(rename = "renderKey", default)]
        render_key: String,
    }
    fs::read(root.join(manifest))
        .ok()
        .and_then(|b| serde_json::from_slice::<Manifest>(&b).ok())
        .map(|m| m.render_key)
        .filter(|k| !k.is_empty())
        .unwrap_or_else(|| "–".to_owned())
}
