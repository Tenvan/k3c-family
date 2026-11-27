//! Die ändernden Asset-Tools und `assets_style_get` (Port von `tools_assets_set.go`): `assets_set`, `assets_style_set`,
//! `assets_delete`. Dazu die Render-Jobs, die auch `assets_render` (RU70.13) nutzt.

use std::fmt::Write as _;
use std::path::Path;
use std::sync::LazyLock;

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Map, Value};

use super::kit::Workspace;
use super::tools::Studio;
use super::tools_assets::{lese_stil, mehrzahl, zahl};
use super::tools_world::blocking;
use crate::kitx::{go_marshal, go_quoted};
use crate::paperassets as pa;

#[cfg(test)]
#[path = "tools_assets_set_tests.rs"]
mod tests;

/// Die Render-Jobs des Servers (Go `assetJobs`).
pub(super) static ASSET_JOBS: LazyLock<pa::Jobs> = LazyLock::new(pa::Jobs::new);

/// Argumente von `assets_set`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct SetIn {
    #[schemars(description = "Asset-ID, z. B. monarch")]
    id: String,
    #[schemars(
        description = "JSON-Merge-Patch (RFC 7396) auf die Spec: null löscht, Objekte werden gemischt, Listen ersetzt; teile auch als Objekt nach Teil-ID ({teile: {kopf: {...}}}): ändert nur die genannten Teile"
    )]
    patch: Map<String, Value>,
}

/// Argumente von `assets_style_set`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct StyleSetIn {
    #[schemars(description = "JSON-Merge-Patch (RFC 7396) auf public/grafik/stil.paper.json")]
    patch: Map<String, Value>,
}

/// Argumente von `assets_delete`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct DeleteIn {
    #[schemars(description = "Asset-ID")]
    id: String,
    #[serde(default)]
    #[schemars(description = "true löscht wirklich; ohne nur Hinweis")]
    confirm: bool,
}

/// Keine Argumente (außer `checkout`).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct LeerIn {}

#[tool_router(router = assets_set_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Ändert eine Paper-Spec per JSON-Merge-Patch atomar (validate, render, dann ersetzen); ungültig = nichts geändert, Antwort je Fehler feld · regel · text. Nutze es bei: jede Änderung an einem Asset. Statt: .paper.json von Hand ändern.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_set(
        &self,
        Parameters(input): Parameters<SetIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || set(&ws.root, input)).await
    }

    #[tool(
        description = "Stil-Datei verdichtet: Pfad, stilHash, Materialien, Einzeltöne, Papier-Werte. Nutze es bei: welche Materialien gibt es. Statt: public/grafik/stil.paper.json lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_style_get(
        &self,
        Parameters(LeerIn {}): Parameters<LeerIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || style_get(&ws.root)).await
    }

    #[tool(
        description = "Merge-Patch auf die Stil-Datei (nur Sessions mit Lane: stil); prüft Schema und ob Specs entfernte Materialien nutzen, startet den Render der betroffenen Assets als Job. Nutze es bei: Farbton oder Papier-Wert ändern. Statt: Stil-Datei von Hand ändern.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_style_set(
        &self,
        Parameters(input): Parameters<StyleSetIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || style_set(&ws.root, input)).await
    }

    #[tool(
        description = "Löscht Spec, Ausgabe-Ordner und den Eintrag im Gruppen-Index; ohne confirm=true nur Hinweis. Nutze es bei: Asset entfernen. Statt: Dateien von Hand löschen.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_delete(
        &self,
        Parameters(input): Parameters<DeleteIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || delete(&ws.root, &input)).await
    }
}

/// Je Fehler `feld · regel · text`.
pub(super) fn fehler_zeilen(fehler: &[pa::Fehler]) -> String {
    let zeilen: Vec<String> = fehler
        .iter()
        .map(|f| format!("{} · {} · {}", f.feld, f.regel, f.text))
        .collect();
    zeilen.join("\n")
}

/// Aus einer Ablehnung wird der Vertragstext; andere Fehler bleiben.
fn ablehnung(err: &pa::Error) -> String {
    let Some(fehler) = err.abgelehnt() else {
        return err.to_string();
    };
    let text = format!(
        "abgelehnt, nichts geändert: {} Fehler\n{}",
        fehler.len(),
        fehler_zeilen(fehler)
    );
    if fehler.iter().any(|f| f.regel == "material-unbekannt") {
        return text + "\nErlaubt: Papier-Materialien und Einzeltöne aus assets_style_get";
    }
    text + "\nRegeln: docs/standards/paper-spec.md › Regeln"
}

fn feld_text(f: &pa::FeldAenderung) -> String {
    format!("{} {} → {}", f.pfad, pa::wert(&f.alt), pa::wert(&f.neu))
}

fn felder_text(felder: &[pa::FeldAenderung]) -> String {
    let teile: Vec<String> = felder.iter().map(feld_text).collect();
    teile.join("; ")
}

/// Anteil 0..1 als Prozent mit Komma und zwei Nachkommastellen.
fn prozent(f: f64) -> String {
    format!("{:.2} %", f * 100.0).replacen('.', ",", 1)
}

fn set(root: &Path, input: SetIn) -> Result<String, String> {
    // Wie Go: Der Patch geht als neu geschriebenes JSON weiter (Schlüssel sortiert).
    let patch = go_marshal(&Value::Object(input.patch));
    let r = pa::set(&pa::Cli::new(root), &input.id, &patch).map_err(|e| set_abbruch(&e.stand, &e.fehler))?;
    Ok(set_text(&r))
}

fn set_text(r: &pa::SetErgebnis) -> String {
    let mut b = if r.felder.is_empty() {
        format!("{} geändert: keine Feldänderung\n", r.asset.id)
    } else {
        format!(
            "{} geändert: {} {} · {}\n",
            r.asset.id,
            r.felder.len(),
            mehrzahl(r.felder.len(), "Feld", "Felder"),
            felder_text(&r.felder)
        )
    };
    let _ = write!(b, "Render: {} {}", r.pngs, mehrzahl(r.pngs, "PNG", "PNGs"));
    if !r.animationen.is_empty() {
        let _ = write!(b, " ({})", r.animationen.join(", "));
    }
    b.push_str(&pixel_text(&r.pixel));
    let mut vorschau = Vec::new();
    if let Some(datei) = r.dateien.first() {
        vorschau.push(format!("Vorschau: {datei}"));
    }
    if let Some(o) = groesste_abweichung(&r.pixel).filter(|o| !o.overlay.is_empty()) {
        vorschau.push(format!("Overlay: {}", o.overlay));
    }
    if !vorschau.is_empty() {
        let _ = write!(b, "\n{}", vorschau.join(" · "));
    }
    if !r.reiter.is_empty() {
        let _ = write!(
            b,
            "\nReiter veraltet: {} · Weiter: assets_render {{id}} je Reiter",
            r.reiter.join(", ")
        );
    }
    b
}

/// Die Fehlerantwort, wenn Prüfen und Rendern abbrach (B-870): nichts geändert, Grund, die bis dahin bekannten Felder
/// und Pixel; sonst wie [`ablehnung`].
fn set_abbruch(r: &pa::SetErgebnis, err: &pa::Error) -> String {
    if r.abbruch.is_empty() {
        return ablehnung(err);
    }
    let mut b = format!("{}: Render abgebrochen, nichts geändert · {}", r.asset.id, r.abbruch);
    if !r.felder.is_empty() {
        let _ = write!(
            b,
            "\nBekannt vor dem Abbruch: {} {} · {}",
            r.felder.len(),
            mehrzahl(r.felder.len(), "Feld", "Felder"),
            felder_text(&r.felder)
        );
    }
    let pixel = pixel_text(&r.pixel);
    if !pixel.is_empty() {
        let _ = write!(
            b,
            "\nBekannt vor dem Abbruch:{}",
            pixel.strip_prefix(" ·").unwrap_or(&pixel)
        );
    }
    b
}

fn pixel_text(px: &[pa::Pixel]) -> String {
    if px.is_empty() {
        return String::new();
    }
    let teile: Vec<String> = px
        .iter()
        .map(|p| match p.fehler.as_str() {
            "neu" => format!("{} neu", p.animation),
            "" => format!("{} {}", p.animation, prozent(p.anteil)),
            _ => format!("{} –", p.animation),
        })
        .collect();
    let mut t = format!(" · Pixel-Anteil {}", teile.join(", "));
    if let Some(g) = groesste_abweichung(px)
        && let Some(bx) = g.boxen.first()
        && let [x, y, w, h] = bx[..]
    {
        let _ = write!(t, " (Box {} {x},{y} {w}×{h})", g.animation);
    }
    t
}

/// Die erste Animation mit dem größten Pixel-Anteil (> 0).
fn groesste_abweichung(px: &[pa::Pixel]) -> Option<&pa::Pixel> {
    let mut best: Option<&pa::Pixel> = None;
    for p in px {
        if p.fehler.is_empty() && p.anteil > 0.0 && best.is_none_or(|b| p.anteil > b.anteil) {
            best = Some(p);
        }
    }
    best
}

fn style_get(root: &Path) -> Result<String, String> {
    let (roh, k) = lese_stil(root)?;
    let hash = pa::stil_hash(roh.as_bytes()).map_err(|e| e.to_string())?;
    let materialien = pa::schluessel(&roh, &["materialien"]);
    let toene = pa::schluessel(&roh, &["einzeltoene"]);
    let (lage, auflage, struktur) = (&k.papier.lage, &k.papier.auflage, &k.papier.struktur);
    let lage_text = [lage.dx, lage.dy, lage.unschaerfe, lage.deckkraft].map(zahl).join("/");
    let auflage_text = [auflage.dx, auflage.dy, auflage.unschaerfe, auflage.deckkraft]
        .map(zahl)
        .join("/");
    let struktur_text = [struktur.frequenz, struktur.oktaven, struktur.staerke]
        .map(zahl)
        .join("/");
    Ok(format!(
        "{pfad} · stilHash {hash}\nMaterialien ({}): {}\nEinzeltöne ({}): {}\n\
         papier: lage {lage_text} {} · auflage {auflage_text} · struktur {struktur_text} · falz {} · licht {}\n\
         Vollständig: {pfad} (ändern nur mit assets_style_set)",
        materialien.len(),
        materialien.join(", "),
        toene.len(),
        toene.join(", "),
        lage.farbe,
        zahl(k.papier.falz),
        k.licht,
        pfad = pa::STIL_PFAD,
    ))
}

fn style_set(root: &Path, input: StyleSetIn) -> Result<String, String> {
    let patch = go_marshal(&Value::Object(input.patch));
    let cli = pa::Cli::new(root);
    let r = pa::stil_set(&cli, &patch).map_err(|e| ablehnung(&e))?;
    if r.felder.is_empty() {
        return Ok("Stil unverändert: der Patch ändert kein Feld".to_owned());
    }
    let felder: Vec<String> = r.felder.iter().map(feld_text).collect();
    let mut b = format!("Stil geändert: {}\n", felder.join(", "));
    if r.betroffen.is_empty() {
        b.push_str("Betroffen: keine Assets");
        return Ok(b);
    }
    let mut ids: Vec<&str> = r.betroffen.iter().map(|a| a.id.as_str()).collect();
    ids.sort_unstable();
    let job = ASSET_JOBS.start(&cli, &r.betroffen);
    let _ = write!(
        b,
        "Betroffen: {} {} ({}) · Render als Job {} gestartet\nWeiter: assets_render {{job: {}, wait: true}}",
        ids.len(),
        mehrzahl(ids.len(), "Asset", "Assets"),
        ids.join(", "),
        job.id,
        go_quoted(&job.id)
    );
    Ok(b)
}

fn delete(root: &Path, input: &DeleteIn) -> Result<String, String> {
    if !input.confirm {
        let n = pa::ausgabe_pngs(root, &input.id).map_err(|e| e.to_string())?;
        return Ok(format!(
            "nicht gelöscht: assets_delete braucht confirm=true ({}, {n} {})",
            input.id,
            mehrzahl(n, "PNG", "PNGs")
        ));
    }
    let r = pa::loesche(root, &input.id).map_err(|e| e.to_string())?;
    let index = if r.index {
        format!("Gruppen-Index {} nachgezogen", r.asset.gruppe)
    } else {
        "kein Gruppen-Index".to_owned()
    };
    Ok(format!(
        "{id} gelöscht: {} und {}/{}/{id}/ ({} {}); {index}",
        r.asset.pfad,
        pa::GRAFIK,
        r.asset.gruppe,
        r.pngs,
        mehrzahl(r.pngs, "PNG", "PNGs"),
        id = r.asset.id,
    ))
}
