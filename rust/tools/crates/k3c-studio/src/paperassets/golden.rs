//! `golden.json` einer Gruppe nachziehen (B-707, Port von `golden.go`): `gen:paper` schreibt sie für alle Gruppen,
//! `assets_set`/`create`/`render` ändern nur den Eintrag des einen Assets, im selben Format
//! (`JSON.stringify(x, null, 2)`).

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use serde_json::value::RawValue;

use super::Error;
use super::anlegen::{ersetze_text, in_index_aufnehmen};
use super::basis::{Asset, ENDUNG, gruppen_ordner};
use super::patch::objekt;
use super::set::sperre;
use crate::kitx::{go_marshal, sha256_hex};

const GOLDEN_NAME: &str = "golden.json";

#[derive(Debug, Clone, Default)]
struct Eintrag {
    id: String,
    render_key: String,
    dateien: Vec<(String, String)>,
}

fn golden_pfad(root: &Path, gruppe: &str) -> PathBuf {
    gruppen_ordner(root, gruppe).join(GOLDEN_NAME)
}

/// Trägt das gerenderte Asset in Gruppen-Index und `golden.json` ein; je Gruppe nacheinander, damit zwei Assets
/// derselben Gruppe sich nicht überschreiben.
pub(super) fn gruppe_nachziehen(root: &Path, a: &Asset) -> Result<(), Error> {
    // `#` kommt in keiner Asset-ID vor, die Sperre trifft also nie die eines Assets.
    let _sperre = sperre(root, &format!("#{}", a.gruppe));
    in_index_aufnehmen(root, a)?;
    golden_eintragen(root, a)
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Manifest {
    #[serde(rename = "renderKey")]
    render_key: String,
    animationen: Option<Box<RawValue>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Animation {
    datei: String,
}

/// Setzt den Eintrag des Assets aus seinem `manifest.json` und den PNG-Hashes.
fn golden_eintragen(root: &Path, a: &Asset) -> Result<(), Error> {
    let ordner = a.ausgabe(root);
    let raw = match fs::read(ordner.join("manifest.json")) {
        Ok(raw) => raw,
        // Ohne Ausgabe nichts einzutragen (Render übersprungen); gen:paper zieht nach.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e.into()),
    };
    let m: Manifest = serde_json::from_slice(&raw).map_err(|e| Error::Sonst(format!("{}/manifest.json: {e}", a.id)))?;
    let mut neu = Eintrag {
        id: a.id.clone(),
        render_key: m.render_key,
        dateien: Vec::new(),
    };
    for (_, roh) in schluessel_werte(m.animationen.as_deref().map_or("", RawValue::get))? {
        let x: Animation = serde_json::from_str(&roh)?;
        let datei = Path::new(&x.datei);
        if !datei.components().all(|c| matches!(c, Component::Normal(_))) {
            return Err(Error::Sonst(format!(
                "{}/manifest.json: Datei {:?} ungültig",
                a.id, x.datei
            )));
        }
        let png = fs::read(ordner.join(datei))?;
        neu.dateien.push((x.datei, sha256_hex(&png)));
    }
    golden_aendern(&golden_pfad(root, &a.gruppe), &a.id, Some(neu))
}

/// Inhalt der Datei; `None` nur, wenn sie fehlt. Jeder andere Lesefehler bricht ab (anders als Go): Sonst ersetzte der
/// Aufrufer die Datei durch eine mit nur dem einen Asset.
pub(super) fn lesen_falls_da(pfad: &Path, name: &str) -> Result<Option<Vec<u8>>, Error> {
    match fs::read(pfad) {
        Ok(raw) => Ok(Some(raw)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(Error::Sonst(format!("{name}: {e}"))),
    }
}

fn golden_aendern(pfad: &Path, id: &str, neu: Option<Eintrag>) -> Result<(), Error> {
    let mut alle = Vec::new();
    if let Some(raw) = lesen_falls_da(pfad, GOLDEN_NAME)? {
        alle = golden_lesen(&raw).map_err(|e| Error::Sonst(format!("{GOLDEN_NAME}: {e}")))?;
    }
    alle.retain(|e: &Eintrag| e.id != id);
    alle.extend(neu);
    // gen:paper sortiert nach Dateinamen <id>.paper.json („a-b“ vor „a“).
    alle.sort_by_key(|e| format!("{}{ENDUNG}", e.id));
    ersetze_text(pfad, &golden_text(&alle))
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Gelesen {
    #[serde(rename = "renderKey")]
    render_key: String,
    dateien: Option<Box<RawValue>>,
}

fn golden_lesen(raw: &[u8]) -> Result<Vec<Eintrag>, Error> {
    let text = std::str::from_utf8(raw).map_err(|_| Error::Sonst("kein UTF-8".to_owned()))?;
    schluessel_werte(text)?
        .into_iter()
        .map(|(id, roh)| {
            let x: Gelesen = serde_json::from_str(&roh)?;
            let mut dateien = Vec::new();
            if let Some(d) = x.dateien {
                for (k, v) in schluessel_werte(d.get())? {
                    dateien.push((k, serde_json::from_str(&v).unwrap_or_default()));
                }
            }
            Ok(Eintrag {
                id,
                render_key: x.render_key,
                dateien,
            })
        })
        .collect()
}

/// Die Schlüssel eines JSON-Objekts in Dateireihenfolge (Werte roh).
fn schluessel_werte(raw: &str) -> Result<Vec<(String, String)>, Error> {
    objekt(raw)?.ok_or_else(|| Error::Sonst("kein JSON-Objekt".to_owned()))
}

fn text(s: &str) -> String {
    go_marshal(&serde_json::Value::String(s.to_owned()))
}

fn golden_text(alle: &[Eintrag]) -> String {
    if alle.is_empty() {
        return "{}\n".to_owned();
    }
    let eintraege: Vec<String> = alle
        .iter()
        .map(|e| {
            let dateien: Vec<String> = e
                .dateien
                .iter()
                .map(|(k, v)| format!("      {}: {}", text(k), text(v)))
                .collect();
            let block = if dateien.is_empty() {
                "{}".to_owned()
            } else {
                format!("{{\n{}\n    }}", dateien.join(",\n"))
            };
            format!(
                "  {}: {{\n    \"renderKey\": {},\n    \"dateien\": {block}\n  }}",
                text(&e.id),
                text(&e.render_key)
            )
        })
        .collect();
    format!("{{\n{}\n}}\n", eintraege.join(",\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Review RU70.10: Nur eine fehlende Datei gilt als leer; ein Verzeichnis steht für „vorhanden, nicht lesbar“.
    #[test]
    fn unlesbare_golden_wird_nicht_ersetzt() {
        let dir = crate::test_helpers::tmp_dir();
        assert!(
            lesen_falls_da(&dir.path().join("fehlt.json"), GOLDEN_NAME)
                .unwrap()
                .is_none()
        );
        assert!(lesen_falls_da(dir.path(), GOLDEN_NAME).is_err());
        assert!(golden_aendern(dir.path(), "a", None).is_err());
    }
}
