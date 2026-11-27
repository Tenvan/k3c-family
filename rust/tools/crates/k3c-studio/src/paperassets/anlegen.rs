//! Ein Asset anlegen und löschen, PNGs zählen und die Pflege des Gruppen-Index (Port von `anlegen.go`).

use std::fs;
use std::io;
use std::path::Path;

use serde::Deserialize;
use serde_json::value::RawValue;

use super::basis::{Asset, ENDUNG, GRAFIK, INDEX_NAME, finde, gruppen_ordner};
use super::cli::Cli;
use super::golden::{gruppe_entfernen, lesen_falls_da, pruefe_golden};
use super::job::{AssetErgebnis, render_eins};
use super::patch::format as formatiere;
use super::rigs::{finde_rig, vorlage};
use super::set::{ist_name, name_fehler, schreibe_datei, sperre};
use super::{Error, Fehler};
use crate::kitx::{go_marshal, marshal_raw};

/// Ein neu angelegtes Asset: Vorlagen-Teile, Fehler von validate, Render-Ergebnis.
#[derive(Debug, Clone, Default)]
pub struct AnlegenErgebnis {
    pub asset: Asset,
    pub teile: Vec<String>,
    pub fehler: Vec<Fehler>,
    pub render: AssetErgebnis,
}

/// Legt `public/grafik/<gruppe>/<id>.paper.json` aus der Rig-Vorlage an, prüft und rendert sie. Eine Vorlage, die
/// validate nicht besteht (Figur mit leeren Lagen: proportion), bleibt liegen; Fehler stehen im Ergebnis.
pub fn anlegen(cli: &Cli, id: &str, rig: &str, beschreibung: &str, gruppe: &str) -> Result<AnlegenErgebnis, Error> {
    if !ist_name(id) || !ist_name(gruppe) {
        return Err(name_fehler(id, gruppe));
    }
    if id == "index" {
        return Err(Error::Sonst(
            "id „index“ ist für den Gruppen-Index reserviert".to_owned(),
        ));
    }
    if beschreibung.trim().is_empty() {
        return Err(Error::Sonst("beschreibung fehlt".to_owned()));
    }
    let r = finde_rig(rig)?;
    let text = vorlage(rig, id, beschreibung)?;
    let asset = Asset {
        id: id.to_owned(),
        rig: rig.to_owned(),
        gruppe: gruppe.to_owned(),
        pfad: format!("{GRAFIK}/{gruppe}/{id}{ENDUNG}"),
    };
    lege_an(&cli.root, &asset, &text)?;
    let mut out = AnlegenErgebnis {
        asset,
        teile: r.vorlage_teile().into_iter().map(str::to_owned).collect(),
        ..AnlegenErgebnis::default()
    };
    match cli.validate(&out.asset.abs(&cli.root)) {
        Ok(()) => out.render = render_eins(cli, &out.asset),
        Err(Error::Abgelehnt(fehler)) => out.fehler = fehler,
        Err(e) => return Err(e),
    }
    Ok(out)
}

/// Schreibt die neue Spec; Prüfung „gibt es schon“ und Schreiben laufen unter der Sperre der ID (nicht der Gruppe),
/// damit zwei parallele Anlagen derselben ID in verschiedenen Gruppen nicht beide gelingen (B-706).
fn lege_an(root: &Path, a: &Asset, text: &str) -> Result<(), Error> {
    let _sperre = sperre(root, &a.id);
    let gibt_es = |wo: &str| Err(Error::Sonst(format!("{} gibt es schon ({wo})", a.id)));
    match finde(root, &a.id) {
        Err(Error::Mehrdeutig(m)) => return gibt_es(&m.treffer.join(", ")),
        Ok(vorhanden) => return gibt_es(&vorhanden.pfad),
        Err(_) => {}
    }
    schreibe_neu(&a.abs(root), text)
}

/// Legt eine Datei über Temp-Datei und Umbenennen an; eine vorhandene Datei bleibt (Fehler).
fn schreibe_neu(ziel: &Path, text: &str) -> Result<(), Error> {
    if ziel.metadata().is_ok() {
        let name = ziel.file_name().unwrap_or_default().to_string_lossy();
        return Err(Error::Sonst(format!("{name} gibt es schon")));
    }
    let mut tmp = ziel.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = Path::new(&tmp);
    schreibe_datei(tmp, text.as_bytes())?;
    Ok(fs::rename(tmp, ziel)?)
}

/// Die PNGs im Ausgabe-Ordner eines Assets.
pub fn ausgabe_pngs(root: &Path, id: &str) -> Result<usize, Error> {
    Ok(zaehle_pngs(&finde(root, id)?.ausgabe(root)))
}

/// Ein fehlender Ordner hat keine PNGs (wie `filepath.Glob`).
fn zaehle_pngs(ordner: &Path) -> usize {
    let eintraege = fs::read_dir(ordner).into_iter().flatten().flatten();
    eintraege
        .filter(|e| e.file_name().to_string_lossy().ends_with(".png"))
        .count()
}

/// Ein gelöschtes Asset; `index` = Gruppen-Index nachgezogen.
#[derive(Debug, Clone, Default)]
pub struct LoeschErgebnis {
    pub asset: Asset,
    pub pngs: usize,
    pub index: bool,
}

/// Entfernt Spec, Ausgabe-Ordner und die Einträge im Gruppen-Index und in `golden.json`. Ob andere Assets das
/// gelöschte nutzen (Reittier), prüft wie in Go niemand.
pub fn loesche(root: &Path, id: &str) -> Result<LoeschErgebnis, Error> {
    // Gesperrt wird die bloße ID wie bei allen Schreibwegen (Go sperrt auch `<gruppe>/<id>` als Schlüssel).
    let _sperre = sperre(root, &finde(root, id)?.id);
    let asset = finde(root, id)?;
    pruefe_loeschbar(root, &asset)?;
    let ausgabe = asset.ausgabe(root);
    let pngs = zaehle_pngs(&ausgabe);
    // Anders als Go: erst Index und golden.json lesen, dann löschen, die Spec nach der Ausgabe. So bleibt bei einem
    // Fehler das Asset bestehen und der Aufruf lässt sich wiederholen.
    lies_index(&gruppen_ordner(root, &asset.gruppe).join(INDEX_NAME))?;
    pruefe_golden(root, &asset.gruppe)?;
    entferne_ausgabe(&ausgabe)?;
    fs::remove_file(asset.abs(root))?;
    let index = gruppe_entfernen(root, &asset)?;
    Ok(LoeschErgebnis { asset, pngs, index })
}

/// Vor dem ersten Löschen (strenger als Go): ID und Gruppe sind gültige Namen (eine Datei `...paper.json` ergäbe sonst
/// den Ausgabe-Ordner `<gruppe>/..`), und der Gruppen-Ordner liegt wirklich unter `public/grafik`.
fn pruefe_loeschbar(root: &Path, a: &Asset) -> Result<(), Error> {
    if !ist_name(&a.id) || !ist_name(&a.gruppe) {
        return Err(name_fehler(&a.id, &a.gruppe));
    }
    let erwartet = fs::canonicalize(root)?.join("public").join("grafik").join(&a.gruppe);
    if fs::canonicalize(gruppen_ordner(root, &a.gruppe))? != erwartet {
        return Err(Error::Sonst(format!(
            "{}: Gruppen-Ordner führt aus {GRAFIK} hinaus, nichts gelöscht",
            a.gruppe
        )));
    }
    Ok(())
}

/// Wie `os.RemoveAll`: Ein Link oder eine Junction verschwindet selbst, das Ziel bleibt (`remove_dir_all` folgt nicht).
fn entferne_ausgabe(ordner: &Path) -> Result<(), Error> {
    match fs::symlink_metadata(ordner) {
        Ok(m) if m.is_dir() || m.is_symlink() => Ok(fs::remove_dir_all(ordner)?),
        Ok(_) => Ok(fs::remove_file(ordner)?),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Kopf {
    id: String,
}

fn kopf_id(eintrag: &RawValue) -> String {
    serde_json::from_str::<Kopf>(eintrag.get()).unwrap_or_default().id
}

/// Trägt das Asset in den Gruppen-Index ein (`{id, rig, manifest}`, nach id sortiert wie die CLI bei
/// `render <ordner>`): `render <spec>` schreibt den Index nicht, das Spiel liest aber nur die Gruppen-Indizes.
pub(super) fn in_index_aufnehmen(root: &Path, a: &Asset) -> Result<(), Error> {
    let pfad = gruppen_ordner(root, &a.gruppe).join(INDEX_NAME);
    let mut ids = Vec::new();
    let mut eintraege = Vec::new();
    for e in lies_index(&pfad)?.unwrap_or_default() {
        ids.push(kopf_id(&e));
        eintraege.push(e.get().to_owned());
    }
    let text = |s: &str| go_marshal(&serde_json::Value::String(s.to_owned()));
    let neu = format!(
        r#"{{"id":{},"rig":{},"manifest":{}}}"#,
        text(&a.id),
        marshal_raw(&text(&a.rig)),
        text(&format!("{}/manifest.json", a.id))
    );
    let mut ersetzt = false;
    for (e, id) in eintraege.iter_mut().zip(&ids) {
        if *id == a.id {
            e.clone_from(&neu);
            ersetzt = true;
        }
    }
    if !ersetzt {
        let i = ids.partition_point(|id| id.as_str() < a.id.as_str());
        eintraege.insert(i, neu);
    }
    ersetze_text(&pfad, &formatiere(&format!("[{}]", eintraege.join(",")))?)
}

/// Die Einträge eines Gruppen-Index; `None` nur, wenn die Datei fehlt.
fn lies_index(pfad: &Path) -> Result<Option<Vec<Box<RawValue>>>, Error> {
    let Some(raw) = lesen_falls_da(pfad, INDEX_NAME)? else {
        return Ok(None);
    };
    let gelesen: Option<Vec<Box<RawValue>>> =
        serde_json::from_slice(&raw).map_err(|e| Error::Sonst(format!("{INDEX_NAME}: {e}")))?;
    Ok(Some(gelesen.unwrap_or_default()))
}

/// Schreibt den Gruppen-Index ohne den Eintrag `id`; `false`, wenn es keinen Index gibt.
pub(super) fn aus_index_entfernen(pfad: &Path, id: &str) -> Result<bool, Error> {
    let Some(eintraege) = lies_index(pfad)? else {
        return Ok(false);
    };
    let rest: Vec<&str> = eintraege.iter().filter(|e| kopf_id(e) != id).map(|e| e.get()).collect();
    ersetze_text(pfad, &formatiere(&format!("[{}]", rest.join(",")))?)?;
    Ok(true)
}

pub(super) fn ersetze_text(pfad: &Path, text: &str) -> Result<(), Error> {
    let mut tmp = pfad.as_os_str().to_owned();
    tmp.push(".tmp");
    fs::write(&tmp, text)?;
    Ok(fs::rename(&tmp, pfad)?)
}

#[cfg(test)]
#[path = "anlegen_tests.rs"]
mod tests;
