//! Asset-Tools auf Spec-Basis (Port von `tools_assets.go`; Vertrag docs/standards/paper-mcp.md › MCP-Tools): guide,
//! list, get und create. Sie rufen die Render-CLI über `paperassets`; die ändernden Tools stehen in
//! `tools_assets_set.rs`, die Prompts in `tools_assets_prompts.rs`.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use rmcp::handler::server::common::Extension;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;

use super::kit::Workspace;
use super::tools::Studio;
use super::tools_assets_set::fehler_zeilen;
use super::tools_world::blocking;
use crate::kitx::{compact, go_quoted};
use crate::paperassets as pa;

#[cfg(test)]
#[path = "tools_assets_slots_tests.rs"]
mod slots_tests;
#[cfg(test)]
#[path = "tools_assets_tests.rs"]
mod tests;

/// Merkt sich `validate` je Spec, solange Spec und Stil-Datei gleich bleiben (`assets_list`).
static GUELTIG: LazyLock<pa::Gueltigkeit> = LazyLock::new(pa::Gueltigkeit::new);

/// Argumente von `assets_guide`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct GuideIn {
    #[serde(default)]
    #[schemars(description = "Rig-Typ: figur, vierbeiner, gebaeude, requisite, hintergrund; leer = alle")]
    rig: String,
}

/// Argumente von `assets_list`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct ListIn {
    #[serde(default)]
    #[schemars(description = "nur dieser Rig-Typ")]
    rig: String,
    #[serde(default)]
    #[schemars(description = "nur diese Gruppe (Ordner unter public/grafik, z. B. figuren)")]
    gruppe: String,
    #[serde(default)]
    #[schemars(description = "nur Assets mit Änderung gegen HEAD")]
    geaendert: bool,
}

/// Argumente von `assets_get`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct GetIn {
    #[schemars(
        description = "Asset-ID (Dateiname ohne .paper.json), z. B. monarch; bei gleicher ID in zwei Gruppen <gruppe>/<id>, z. B. gegner/wolf"
    )]
    id: String,
    #[serde(default)]
    #[schemars(description = "nur diese obersten Schlüssel, z. B. [umhang, ausgabe]")]
    felder: Option<Vec<String>>,
}

/// Argumente von `assets_create`.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct CreateIn {
    #[schemars(description = "neue Asset-ID: a-z, 0-9, - (eindeutig über alle Gruppen)")]
    id: String,
    #[schemars(description = "figur, vierbeiner, gebaeude, requisite oder hintergrund")]
    rig: String,
    #[schemars(description = "was das Asset zeigt")]
    beschreibung: String,
    #[schemars(description = "Ordner unter public/grafik, z. B. requisiten")]
    gruppe: String,
}

#[tool_router(router = assets_router, vis = "pub(super)")]
impl Studio {
    #[tool(
        description = "Einstieg jeder Asset-Arbeit: Stil-Regeln, Papier-Materialien und Einzeltöne der Stil-Datei, Pflicht-Teile und Pivots des Rig-Typs, Proportion, Lagen-Arten, Vorlage, Ablauf. Nutze es bei: bevor du ein Asset anlegst oder änderst. Statt: paper-spec.md und Stil-Datei lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_guide(
        &self,
        Parameters(input): Parameters<GuideIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || guide(&ws.root, &input.rig)).await
    }

    #[tool(
        description = "Alle Assets (Paper-Specs unter public/grafik) je Zeile: id · rig · pfad · gültig|ungültig[ · geändert]. Nutze es bei: Überblick, ungültige oder geänderte Assets finden. Statt: public/grafik durchsuchen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_list(
        &self,
        Parameters(input): Parameters<ListIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || list(&ws.root, &input)).await
    }

    #[tool(
        description = "Pfad und Paper-Spec eines Assets als JSON; felder kürzt auf oberste Schlüssel. Nutze es bei: vor einem assets_set. Statt: die .paper.json-Datei lesen.",
        annotations(read_only_hint = true, idempotent_hint = true, open_world_hint = false)
    )]
    async fn assets_get(
        &self,
        Parameters(input): Parameters<GetIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || get(&ws.root, &input)).await
    }

    #[tool(
        description = "Legt public/grafik/<gruppe>/<id>.paper.json aus der Rig-Vorlage an (Pflicht-Teile mit leeren Lagen), prüft und rendert. Nutze es bei: neues Asset. Statt: Spec-Datei von Hand anlegen.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn assets_create(
        &self,
        Parameters(input): Parameters<CreateIn>,
        Extension(ws): Extension<Workspace>,
    ) -> Result<String, String> {
        blocking(move || create(&ws.root, &input)).await
    }
}

/// Die Teile der Stil-Datei, die guide und `style_get` zeigen; Fehlendes ist null bzw. leer wie in Go.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(super) struct StilKurz {
    pub papier: Papier,
    pub licht: String,
    proportionen: Proportionen,
}

/// `papier` der Stil-Datei.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(super) struct Papier {
    pub lage: Schatten,
    pub auflage: Schatten,
    pub struktur: Struktur,
    #[serde(rename = "falz_deckkraft")]
    pub falz: f64,
}

/// Schatten eines Papierstücks (`lage` mit Farbe, `auflage` ohne).
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(super) struct Schatten {
    pub dx: f64,
    pub dy: f64,
    pub unschaerfe: f64,
    pub deckkraft: f64,
    pub farbe: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(super) struct Struktur {
    pub frequenz: f64,
    pub oktaven: f64,
    pub staerke: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Proportionen {
    figur: Figur,
    vierbeiner: Vierbeiner,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Figur {
    kopf_anteil: f64,
    toleranz: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Vierbeiner {
    rumpfhoehe_kopflaengen: f64,
    toleranz: f64,
}

/// Zahl wie Go `FormatFloat(f, 'f', -1, 64)`.
pub(super) fn zahl(f: f64) -> String {
    f.to_string()
}

/// Anteil 0..1 als Prozent ohne Gleitkomma-Rest (0.55 → 55, nicht 55.00000000000001; B-709).
fn anteil_prozent(f: f64) -> String {
    zahl((f * 1e6).round() / 1e4)
}

/// Liest die Stil-Datei roh und verdichtet.
pub(super) fn lese_stil(root: &Path) -> Result<(String, StilKurz), String> {
    let roh =
        fs::read_to_string(root.join(pa::STIL_PFAD)).map_err(|_| format!("Stil-Datei {} fehlt", pa::STIL_PFAD))?;
    let kurz = serde_json::from_str(&roh).map_err(|e| format!("Stil-Datei: {e}"))?;
    Ok((roh, kurz))
}

fn guide(root: &Path, rig: &str) -> Result<String, String> {
    if !rig.is_empty() {
        pa::finde_rig(rig).map_err(|e| e.to_string())?;
    }
    let (roh, k) = lese_stil(root)?;
    let titel = if rig.is_empty() {
        "alle Rigs".to_owned()
    } else {
        format!("Rig {rig}")
    };
    let mut b = format!("Papierschnitt · {titel} · Standard docs/standards/paper-spec.md\n");
    stil_regeln(&mut b, &k);
    let materialien = pa::schluessel(&roh, &["materialien"]).join(", ");
    let toene = pa::schluessel(&roh, &["einzeltoene"]).join(", ");
    let _ = writeln!(b, "Papier-Materialien (licht/basis/schatten): {materialien}");
    let _ = writeln!(b, "Einzeltöne (ohne Ton): {toene}");
    let rigs = if rig.is_empty() { pa::rig_namen() } else { vec![rig] };
    for name in rigs {
        guide_rig(&mut b, name, !rig.is_empty(), &k)?;
    }
    b.push_str(
        "Lagen: flaeche {form, material, ton?, auflage?} · falz {form, breite, material, ton?} · punkt {mitte, radius, material, ton?}\n",
    );
    if !rig.is_empty() {
        let vorlage = pa::vorlage(rig, "beispiel", "…").map_err(|e| e.to_string())?;
        let _ = writeln!(b, "Vorlage {rig}: {}", compact(&vorlage));
    }
    b.push_str("Beispiele (vollständig): tools/paper/fixtures/specs/monarch.paper.json (figur), pferd.paper.json (vierbeiner), mauer.paper.json (gebaeude)\n");
    b.push_str("Ablauf: assets_get → assets_set (Merge-Patch) → assets_preview → assets_review");
    Ok(b)
}

fn stil_regeln(b: &mut String, k: &StilKurz) {
    let (lage, auflage) = (&k.papier.lage, &k.papier.auflage);
    let _ = writeln!(
        b,
        "Stil: keine Konturen; jede Fläche ein Papierstück mit Schatten papier.lage ({}/{}, Unschärfe {}, {} %); \
         Schattierungen als aufgeklebte Lage (auflage: true, {}/{}/{}/{} %); Innenlinien nur als Falz (Deckkraft {}); Licht von {}; \
         keine Farben in der Spec, nur Papier-Materialien.",
        zahl(lage.dx),
        zahl(lage.dy),
        zahl(lage.unschaerfe),
        anteil_prozent(lage.deckkraft),
        zahl(auflage.dx),
        zahl(auflage.dy),
        zahl(auflage.unschaerfe),
        anteil_prozent(auflage.deckkraft),
        zahl(k.papier.falz),
        k.licht.replace('-', " "),
    );
}

fn guide_rig(b: &mut String, name: &str, einzeln: bool, k: &StilKurz) -> Result<(), String> {
    let r = pa::finde_rig(name).map_err(|e| e.to_string())?;
    let mut pflicht = r.pflicht.join(", ");
    if pflicht.is_empty() {
        pflicht = format!("mindestens ein Teil (Vorlage: {})", r.platzhalter);
    }
    let _ = writeln!(b, "Pflicht-Teile {name}: {pflicht}");
    if einzeln {
        let _ = writeln!(b, "Pivots: {}", r.pivot_text);
    } else {
        let _ = writeln!(b, "Pivots {name}: {}", r.pivot_text);
    }
    let p = &k.proportionen;
    let _ = match name {
        "figur" => writeln!(
            b,
            "Proportion: Kopfhöhe / Rahmenhöhe = {} ± {}",
            zahl(p.figur.kopf_anteil),
            zahl(p.figur.toleranz)
        ),
        "vierbeiner" => writeln!(
            b,
            "Proportion: Rumpfhöhe = {} Kopflängen ± {}",
            zahl(p.vierbeiner.rumpfhoehe_kopflaengen),
            zahl(p.vierbeiner.toleranz)
        ),
        _ => Ok(()),
    };
    Ok(())
}

pub(super) fn mehrzahl<'a>(n: usize, eins: &'a str, viele: &'a str) -> &'a str {
    if n == 1 { eins } else { viele }
}

fn list(root: &Path, input: &ListIn) -> Result<String, String> {
    let alle = pa::assets(root).map_err(|e| e.to_string())?;
    let geaendert = pa::geaendert(root).map_err(|e| e.to_string())?;
    let ist_geaendert = |id: &str| geaendert.get(id).copied().unwrap_or(false);
    let liste: Vec<pa::Asset> = alle
        .into_iter()
        .filter(|a| input.rig.is_empty() || a.rig == input.rig)
        .filter(|a| input.gruppe.is_empty() || a.gruppe == input.gruppe)
        .filter(|a| !input.geaendert || ist_geaendert(&a.id))
        .collect();
    if liste.is_empty() {
        return Ok("keine Assets für diesen Filter".to_owned());
    }
    let fehler = GUELTIG.pruefe(&pa::Cli::new(root), &liste);
    let mut zeilen = Vec::with_capacity(liste.len());
    let (mut ungueltig, mut n_geaendert) = (0, 0);
    for a in &liste {
        let mut zustand = "gültig".to_owned();
        if fehler.get(&a.id).is_some_and(|f| !f.is_empty()) {
            zustand = "ungültig".to_owned();
            ungueltig += 1;
        }
        if ist_geaendert(&a.id) {
            zustand.push_str(" · geändert");
            n_geaendert += 1;
        }
        zeilen.push(format!("{} · {} · {} · {zustand}", a.id, a.rig, a.pfad));
    }
    Ok(format!(
        "{} {} · {ungueltig} ungültig · {n_geaendert} geändert gegen HEAD\n{}",
        liste.len(),
        mehrzahl(liste.len(), "Asset", "Assets"),
        zeilen.join("\n")
    ))
}

fn get(root: &Path, input: &GetIn) -> Result<String, String> {
    let a = pa::finde(root, &input.id).map_err(|e| e.to_string())?;
    let roh = fs::read_to_string(a.abs(root)).map_err(|e| e.to_string())?;
    let felder: Vec<&str> = input.felder.iter().flatten().map(String::as_str).collect();
    let auswahl = pa::felder(&roh, &felder).map_err(|e| e.to_string())?;
    let text = pa::format(&auswahl).map_err(|e| e.to_string())?;
    let mut kopf = a.pfad.clone();
    if !felder.is_empty() {
        let _ = write!(kopf, " · Felder {}", felder.join(", "));
    }
    Ok(format!("{kopf}\n{}", text.strip_suffix('\n').unwrap_or(&text)))
}

fn create(root: &Path, input: &CreateIn) -> Result<String, String> {
    let cli = pa::Cli::new(root);
    let r = pa::anlegen(&cli, &input.id, &input.rig, &input.beschreibung, &input.gruppe).map_err(|e| e.to_string())?;
    let mut b = format!(
        "{} angelegt: {} (Rig {}, Vorlage mit {} {})\n",
        r.asset.id,
        r.asset.pfad,
        r.asset.rig,
        mehrzahl(r.teile.len(), "Teil", "Teilen"),
        r.teile.join(", ")
    );
    if r.fehler.is_empty() {
        let _ = writeln!(b, "validate: ok\n{}", render_zeile(&r.render));
    } else {
        let _ = writeln!(
            b,
            "validate: {} Fehler\n{}\nRender: übersprungen (ungültig)",
            r.fehler.len(),
            fehler_zeilen(&r.fehler)
        );
    }
    let _ = write!(
        b,
        "Nächster Schritt: assets_set {{id: {}, patch: …}}, dann assets_preview",
        go_quoted(&r.asset.id)
    );
    Ok(b)
}

/// Das Render-Ergebnis eines Assets in einer Zeile.
pub(super) fn render_zeile(e: &pa::AssetErgebnis) -> String {
    if !e.fehler.is_empty() {
        return format!("Render: Fehler · {}", e.fehler);
    }
    if !e.uebersprungen.is_empty() {
        return format!("Render: übersprungen (ungültig: {})", e.uebersprungen);
    }
    let mut t = format!("Render: {} {}", e.pngs, mehrzahl(e.pngs, "PNG", "PNGs"));
    if !e.manifest.is_empty() {
        let _ = write!(t, " · {}", e.manifest);
    } else if let Some(datei) = e.dateien.first() {
        let _ = write!(t, " · {datei}");
    }
    t
}
