//! `assets_review` (Port von `tools_assets_review.go`). Den Vergleich rechnen `review` und `assetsui`; hier wird nur
//! formatiert und das Bild je Asset nach `.cache/paper/mcp/` geschrieben.

use std::fs;
use std::path::Path;
use std::sync::Arc;

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use super::tools::Studio;
use super::tools_assets::mehrzahl;
use super::tools_assets_preview::data_url;
use super::tools_world::blocking;
use crate::assetsui::{Dienst, Fehler, FeldAenderung, Frame, PixelDiff, Review, ReviewAsset};
use crate::paperassets as pa;

#[cfg(test)]
#[path = "tools_assets_review_tests.rs"]
mod tests;

/// Der Vergleich mit einer Basis (Go `assetReview`).
pub(super) trait ReviewQuelle {
    fn review(&self, basis: &str) -> Result<Review, String>;
    fn frame(&self, id: &str, animation: &str, frame: i64, variante: &str, basis: &str) -> Result<Frame, String>;
    fn pixel_diff(&self, id: &str, animation: &str, frame: i64, basis: &str) -> Result<PixelDiff, String>;
}

impl ReviewQuelle for Dienst {
    fn review(&self, basis: &str) -> Result<Review, String> {
        self.assets_review(basis)
    }

    fn frame(&self, id: &str, animation: &str, frame: i64, variante: &str, basis: &str) -> Result<Frame, String> {
        self.assets_frame(id, animation, frame, variante, basis)
    }

    fn pixel_diff(&self, id: &str, animation: &str, frame: i64, basis: &str) -> Result<PixelDiff, String> {
        self.assets_pixel_diff(id, animation, frame, basis)
    }
}

/// Liefert die Review-Quelle eines Checkouts (Test-Naht; Go `newAssetReview`).
pub(super) type ReviewFactory = Arc<dyn Fn(&Path) -> Box<dyn ReviewQuelle> + Send + Sync>;

/// `assetsui::Dienst` (RV1, Vergleich über `review`).
pub(super) fn dienst() -> ReviewFactory {
    Arc::new(|root| Box::new(Dienst::new(root)))
}

/// Argumente von `assets_review`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct ReviewIn {
    #[serde(default)]
    #[schemars(description = "HEAD (Standard), HEAD~1, Commit-SHA oder origin/main")]
    basis: String,
}

#[tool_router(router = assets_review_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Review des Arbeitsstands gegen eine Basis: je Asset status · id · Felder · Pixel-Anteil · Bilder (PNG unter .cache/paper/mcp/); Stiländerungen mit stil: true. Nutze es bei: zum Schluss jeder Asset-Änderung. Statt: git diff auf Specs und PNGs.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_review(
        &self,
        Parameters(input): Parameters<ReviewIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        let quelle = self.review.clone();
        blocking(move || review_text(&ws.root, &input.basis, &*quelle(&ws.root))).await
    }
}

/// Die Kopfzeile wie `fixtures/mcp/assets_review[-lokal].txt`.
fn review_kopf(basis: &str, commit: &str, stand: &str, n: usize) -> String {
    let assets = mehrzahl(n, "Asset", "Assets");
    if stand == "lokal" {
        return format!(
            "Review gegen {basis} ({commit}) · Basis-Stand: lokal (ohne vorheriges Fetch; origin/main kann veraltet sein): {n} {assets}"
        );
    }
    format!("Review gegen {basis} ({commit}, Basis-Stand {stand}): {n} {assets}")
}

fn review_text(root: &Path, basis: &str, quelle: &dyn ReviewQuelle) -> Result<String, String> {
    let basis = if basis.is_empty() { "HEAD" } else { basis };
    pa::check_basis(basis).map_err(|e| e.to_string())?;
    let r = quelle.review(basis)?;
    let mut zeilen = vec![review_kopf(&r.basis, &r.basis_commit, &r.basis_stand, r.assets.len())];
    for a in &r.assets {
        zeilen.push(review_zeile(root, quelle, &r.basis, a));
    }
    Ok(zeilen.join("\n"))
}

fn review_zeile(root: &Path, quelle: &dyn ReviewQuelle, basis: &str, a: &ReviewAsset) -> String {
    let mut teile = vec![a.status.clone(), a.id.clone()];
    match a.status.as_str() {
        "ungueltig" => teile.push(fehler_zeilen_kurz(&a.fehler)),
        "neu" | "geloescht" => teile.push(a.pfad.clone()),
        _ => {
            if !a.felder.is_empty() {
                teile.push(felder_text(&a.felder));
            }
            if a.stil {
                teile.push("stil: true".to_owned());
            }
            teile.push(format!("Pixel {}", prozent(a.pixel_anteil)));
        }
    }
    let bild = review_bild(root, quelle, basis, a);
    if !bild.is_empty() {
        teile.push(bild);
    }
    teile.join(" · ")
}

fn fehler_zeilen_kurz(fehler: &[Fehler]) -> String {
    let zeilen: Vec<String> = fehler
        .iter()
        .map(|f| format!("{} · {} · {}", f.feld, f.regel, f.text))
        .collect();
    zeilen.join("; ")
}

/// Wie in `assets_set`: je Feld `pfad alt → neu`.
fn felder_text(felder: &[FeldAenderung]) -> String {
    let teile: Vec<String> = felder
        .iter()
        .map(|f| format!("{} {} → {}", f.pfad, pa::wert(&f.alt), pa::wert(&f.neu)))
        .collect();
    teile.join("; ")
}

/// Anteil 0..1 als Prozent mit Komma und zwei Nachkommastellen.
fn prozent(f: f64) -> String {
    format!("{:.2} %", f * 100.0).replacen('.', ",", 1)
}

/// Schreibt Overlay (geändert) oder Bild (neu) nach `.cache/paper/mcp/` und nennt den Pfad; ohne Bild leer.
fn review_bild(root: &Path, quelle: &dyn ReviewQuelle, basis: &str, a: &ReviewAsset) -> String {
    let anim = erste_animation(root, &a.id);
    let (data, name, praefix) = match a.status.as_str() {
        "geaendert" => match quelle.pixel_diff(&a.id, &anim, 0, basis) {
            Ok(d) => (d.overlay, cache_name(&[&a.id, &anim, "0", "diff"]), "Overlay "),
            Err(_) => return String::new(),
        },
        "neu" => match quelle.frame(&a.id, &anim, 0, "", "") {
            Ok(f) => (f.bild, cache_name(&[&a.id, &anim, "0"]), ""),
            Err(_) => return String::new(),
        },
        _ => return String::new(),
    };
    let Ok(bild) = data_url(&data) else {
        return String::new();
    };
    let dir = root.join(".cache").join("paper").join("mcp");
    if fs::create_dir_all(&dir).is_err() || fs::write(dir.join(&name), bild).is_err() {
        return String::new();
    }
    format!("{praefix}.cache/paper/mcp/{name}")
}

/// Dateiname aus Teilen (ID, Animation …), der in `.cache/paper/mcp/` bleibt.
fn cache_name(teile: &[&str]) -> String {
    let sicher = |t: &&str| -> String {
        t.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    };
    let teile: Vec<String> = teile.iter().map(sicher).collect();
    format!("{}.png", teile.join("-"))
}

/// Die erste Animation der Spec im Arbeitsstand (leer, wenn es sie nicht gibt).
fn erste_animation(root: &Path, id: &str) -> String {
    let Ok(a) = pa::finde(root, id) else {
        return String::new();
    };
    let Ok(spec) = fs::read_to_string(a.abs(root)) else {
        return String::new();
    };
    pa::schluessel(&spec, &["animationen"])
        .into_iter()
        .next()
        .unwrap_or_default()
}
