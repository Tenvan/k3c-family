//! `assets_preview` (Port von `tools_assets_preview.go`): ein Frame als PNG mit den Teil-Boxen, aus dem Arbeitsstand
//! über die Render-CLI oder mit `basis` aus dem Review.

use std::fs;
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use k3c_kit::mcp::image_result;
use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use super::tools::Studio;
use super::tools_assets_review::ReviewQuelle;
use super::tools_assets_set::fehler_zeilen;
use super::tools_world::blocking;
use crate::paperassets as pa;
use crate::review::Teile;

#[cfg(test)]
#[path = "tools_assets_preview_tests.rs"]
mod tests;

/// Argumente von `assets_preview`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct PreviewIn {
    #[schemars(description = "Asset-ID, z. B. monarch (bei doppelter ID <gruppe>/<id>, z. B. gegner/wolf)")]
    id: String,
    #[serde(default)]
    #[schemars(description = "Animation; leer = erste der Spec")]
    animation: String,
    #[serde(default)]
    #[schemars(description = "Frame-Nummer, Standard 0")]
    frame: i64,
    #[serde(default)]
    #[schemars(description = "Variante (Material-Tausch); leer = keine")]
    variante: String,
    #[serde(default)]
    #[schemars(description = "alter Stand: HEAD, HEAD~1, Commit-SHA oder origin/main; leer = Arbeitsstand")]
    basis: String,
}

/// Ein Frame als PNG mit den Teil-Boxen.
struct Bild {
    png: Vec<u8>,
    teile: Teile,
}

#[tool_router(router = assets_preview_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Ein Frame eines Assets als Bild (image/png) plus Text: Animation, Frame, Basis, Größe, Teil-Boxen x,y,b,h; mit basis der alte Stand. Nutze es bei: Ergebnis eines assets_set ansehen, Teile finden. Statt: PNG-Dateien öffnen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_preview(
        &self,
        Parameters(input): Parameters<PreviewIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<CallToolResult, String> {
        let review = self.review.clone();
        blocking(move || {
            let (png, text) = preview(&ws.root, &input, || review(&ws.root))?;
            Ok(image_result(&png, &text))
        })
        .await
    }
}

fn preview(
    root: &Path,
    input: &PreviewIn,
    review: impl FnOnce() -> Box<dyn ReviewQuelle>,
) -> Result<(Vec<u8>, String), String> {
    if !input.basis.is_empty() {
        pa::check_basis(&input.basis).map_err(|e| e.to_string())?;
    }
    let a = pa::finde(root, &input.id).map_err(|e| e.to_string())?;
    let spec = fs::read_to_string(a.abs(root)).map_err(|e| e.to_string())?;
    let anims = pa::schluessel(&spec, &["animationen"]);
    let anim = match anims.first() {
        Some(erste) if input.animation.is_empty() => erste.clone(),
        _ => input.animation.clone(),
    };
    if !anims.contains(&anim) {
        return Err(format!("Animation „{anim}“ fehlt ({})", anims.join(", ")));
    }
    let bild = if input.basis.is_empty() {
        frame_des_arbeitsstands(root, &a, input, &anim)?
    } else {
        frame_der_basis(&*review(), input, &anim)?
    };
    let text = preview_text(&a, input, &anim, &spec, &bild);
    Ok((bild.png, text))
}

fn frame_des_arbeitsstands(root: &Path, a: &pa::Asset, input: &PreviewIn, anim: &str) -> Result<Bild, String> {
    let tmp = pa::temp_ordner(root).map_err(|e| e.to_string())?;
    let ergebnis = rendere_frame(root, a, input, anim, &tmp.join("frame.png"));
    // Der Temp-Ordner liegt unter .cache; ein Rest dort stört nicht.
    let _ = fs::remove_dir_all(&tmp);
    ergebnis
}

fn rendere_frame(root: &Path, a: &pa::Asset, input: &PreviewIn, anim: &str, out: &Path) -> Result<Bild, String> {
    let args = pa::FrameArgs {
        animation: anim.to_owned(),
        frame: input.frame,
        variante: input.variante.clone(),
        ..pa::FrameArgs::default()
    };
    let f = pa::Cli::new(root)
        .frame(&a.abs(root), &args, out)
        .map_err(|e| e.abgelehnt().map_or_else(|| e.to_string(), fehler_zeilen))?;
    Ok(Bild {
        png: fs::read(out).map_err(|e| e.to_string())?,
        teile: f.teile,
    })
}

fn frame_der_basis(review: &dyn ReviewQuelle, input: &PreviewIn, anim: &str) -> Result<Bild, String> {
    let f = review.frame(&input.id, anim, input.frame, &input.variante, &input.basis)?;
    Ok(Bild {
        png: data_url(&f.bild)?,
        teile: f.teile,
    })
}

/// Dekodiert `data:image/png;base64,…` (Bilder der Oberflächen-Methoden).
pub(super) fn data_url(s: &str) -> Result<Vec<u8>, String> {
    let (_, roh) = s
        .split_once(";base64,")
        .ok_or_else(|| "Bild ist keine Data-URL".to_owned())?;
    STANDARD.decode(roh).map_err(|e| e.to_string())
}

/// Breite und Höhe aus dem `IHDR` am Anfang eines PNG.
fn png_groesse(b: &[u8]) -> Option<(u32, u32)> {
    let kopf = b.get(..24)?;
    if !kopf.starts_with(b"\x89PNG\r\n\x1a\n") || &kopf[12..16] != b"IHDR" {
        return None;
    }
    let zahl = |i: usize| kopf[i..i + 4].try_into().ok().map(u32::from_be_bytes);
    Some((zahl(16)?, zahl(20)?))
}

fn preview_text(a: &pa::Asset, input: &PreviewIn, anim: &str, spec: &str, b: &Bild) -> String {
    let basis = if input.basis.is_empty() { "–" } else { &input.basis };
    let groesse = png_groesse(&b.png).map_or_else(|| "?".to_owned(), |(w, h)| format!("{w}×{h}"));
    let mut kopf = format!(
        "{} · {anim} · Frame {} · Basis {basis} · {groesse} px",
        a.id, input.frame
    );
    if !input.variante.is_empty() {
        kopf = format!("{kopf} · Variante {}", input.variante);
    }
    let teile: Vec<String> = box_reihenfolge(spec, &b.teile)
        .iter()
        .filter_map(|id| match b.teile.get(id)?[..] {
            [x, y, w, h] => Some(format!("{id} {x},{y},{w},{h}")),
            _ => None,
        })
        .collect();
    format!("{kopf}\nTeile (x,y,b,h px): {}", teile.join(" · "))
}

/// Erst die Teile der Spec in Zeichenreihenfolge, dann die Boxen des Reittiers (`reittier:<teil>`, sortiert; B-709).
/// Andere Schlüssel der CLI bleiben weg.
fn box_reihenfolge(spec: &str, boxen: &Teile) -> Vec<String> {
    let mut aus: Vec<String> = teil_ids(spec).into_iter().filter(|id| boxen.contains_key(id)).collect();
    // `Teile` ist nach Schlüssel sortiert.
    aus.extend(boxen.keys().filter(|id| id.starts_with("reittier:")).cloned());
    aus
}

/// Die Teil-IDs der Spec in Zeichenreihenfolge.
fn teil_ids(spec: &str) -> Vec<String> {
    #[derive(Default, Deserialize)]
    #[serde(default)]
    struct Spec {
        teile: Vec<Teil>,
    }
    #[derive(Default, Deserialize)]
    #[serde(default)]
    struct Teil {
        id: String,
    }
    let s: Spec = serde_json::from_str(spec).unwrap_or_default();
    s.teile.into_iter().map(|t| t.id).collect()
}
