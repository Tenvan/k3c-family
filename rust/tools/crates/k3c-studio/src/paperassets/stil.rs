//! Die Stil-Datei (Port von `stil.go`): Hash, Feld-Diff, Materialien, Merge-Patch mit Schema-Prüfung und die betroffenen
//! Assets.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::Value;

use super::basis::{Asset, ENDUNG, STIL_PFAD, assets};
use super::cli::Cli;
use super::patch::{format as formatiere, merge_patch};
use super::set::{lies_text, sperre, wie_zeilenenden};
use super::{Error, Fehler, FeldAenderung};
use crate::kitx::go_marshal;
use crate::review::reittiere;

/// Das JSON-Schema der Stil-Datei relativ zum Checkout (PV1.3).
const STIL_SCHEMA: &str = "tools/paper/schema/stil.schema.json";

/// Ein Feldwert kurz: Text ohne Anführungszeichen, fehlend als –, sonst kompaktes JSON.
pub fn wert(v: &Value) -> String {
    match v {
        Value::Null => "–".to_owned(),
        Value::String(s) => s.clone(),
        v => go_marshal(v),
    }
}

/// Der stilHash laut paper-spec.md › Hash (SHA-256 der kanonischen Form ohne `$schema`).
pub fn stil_hash(raw: &[u8]) -> Result<String, Error> {
    Ok(crate::review::spec_hash(raw)?)
}

/// Gleichheit wie Gos `reflect.DeepEqual` nach `any`: Zahlen als `float64`.
fn gleich(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_f64() == b.as_f64(),
        (Value::Array(a), Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| gleich(a, b)),
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len() && a.iter().all(|(k, v)| b.get(k).is_some_and(|w| gleich(v, w)))
        }
        (a, b) => a == b,
    }
}

/// Vergleicht zwei JSON-Dokumente je Schlüssel (sortiert); Listen und Werte als Ganzes.
pub fn feld_diff(alt: &str, neu: &str) -> Result<Vec<FeldAenderung>, Error> {
    let a: Value = serde_json::from_str(alt)?;
    let n: Value = serde_json::from_str(neu)?;
    let mut out = Vec::new();
    vergleiche(&a, &n, "", &mut out);
    Ok(out)
}

fn vergleiche(a: &Value, n: &Value, pfad: &str, out: &mut Vec<FeldAenderung>) {
    if let (Value::Object(ma), Value::Object(mn)) = (a, n) {
        let keys: BTreeSet<&String> = ma.keys().chain(mn.keys()).collect();
        for k in keys.into_iter().filter(|k| *k != "$schema") {
            let p = if pfad.is_empty() {
                k.clone()
            } else {
                format!("{pfad}.{k}")
            };
            vergleiche(
                ma.get(k).unwrap_or(&Value::Null),
                mn.get(k).unwrap_or(&Value::Null),
                &p,
                out,
            );
        }
        return;
    }
    if !gleich(a, n) {
        out.push(FeldAenderung {
            pfad: pfad.to_owned(),
            alt: a.clone(),
            neu: n.clone(),
        });
    }
}

/// Alle Materialien und Einzeltöne, die eine Spec nennt (Lagen, Umhang, tausch).
pub fn materialien(spec: &[u8]) -> BTreeSet<String> {
    let v: Value = serde_json::from_str(&String::from_utf8_lossy(spec)).unwrap_or_default();
    let mut out = BTreeSet::new();
    sammle(&v, &mut out);
    out
}

/// Die Tiefe ist durch das Lesen begrenzt (`serde_json` bricht nach 128 Ebenen ab).
fn sammle(v: &Value, out: &mut BTreeSet<String>) {
    match v {
        Value::Object(map) => {
            for (k, e) in map {
                sammle_feld(k, e, out);
                sammle(e, out);
            }
        }
        Value::Array(liste) => liste.iter().for_each(|e| sammle(e, out)),
        _ => {}
    }
}

/// `"material": "<name>"` und die Paare einer `tausch`-Tabelle.
fn sammle_feld(k: &str, e: &Value, out: &mut BTreeSet<String>) {
    if let Value::String(s) = e
        && k == "material"
    {
        out.insert(s.clone());
    }
    let (Value::Object(tausch), "tausch") = (e, k) else {
        return;
    };
    for (von, nach) in tausch {
        out.insert(von.clone());
        if let Value::String(s) = nach {
            out.insert(s.clone());
        }
    }
}

/// Die geänderten Felder der Stil-Datei und die Assets, die sie nutzen.
#[derive(Debug, Clone, Default)]
pub struct StilErgebnis {
    pub felder: Vec<FeldAenderung>,
    pub betroffen: Vec<Asset>,
}

/// Wendet einen Merge-Patch auf die Stil-Datei an: Schema, material-doppelt und material-unbekannt (Specs, die ein
/// entferntes Material nutzen) prüfen, dann per Temp-Datei und Umbenennen ersetzen. Abgelehnt ändert nichts.
pub fn stil_set(cli: &Cli, patch: &str) -> Result<StilErgebnis, Error> {
    let _sperre = sperre(&cli.root, STIL_PFAD);
    let pfad = cli.root.join("public").join("grafik").join("stil.paper.json");
    let alt = lies_text(&pfad)?;
    let text = formatiere(&merge_patch(&alt, patch)?)?;
    let liste = assets(&cli.root)?;
    let fehler = pruefe_stil(&cli.root, &text, &liste);
    if !fehler.is_empty() {
        return Err(Error::Abgelehnt(fehler));
    }
    let felder = feld_diff(&alt, &text)?;
    let betroffen = betroffen(&cli.root, &felder, &liste);
    let tmp = pfad.with_file_name("stil.paper.json.tmp");
    let geschrieben = fs::write(&tmp, wie_zeilenenden(&alt, text)).and_then(|()| fs::rename(&tmp, &pfad));
    if let Err(e) = geschrieben {
        let _ = fs::remove_file(&tmp);
        return Err(e.into());
    }
    Ok(StilErgebnis { felder, betroffen })
}

fn regel(feld: String, regel: &str, text: String) -> Fehler {
    Fehler {
        feld,
        regel: regel.to_owned(),
        text,
    }
}

fn pruefe_stil(root: &Path, text: &str, liste: &[Asset]) -> Vec<Fehler> {
    let inst: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => return vec![regel("stil".into(), "json", e.to_string())],
    };
    if let Err(e) = validiere_schema(root, &inst) {
        return vec![regel("stil".into(), "schema", e)];
    }
    let namen = |key: &str| -> BTreeMap<&String, &Value> {
        let map = inst.get(key).and_then(Value::as_object);
        map.into_iter().flatten().collect()
    };
    let (materialien_da, toene) = (namen("materialien"), namen("einzeltoene"));
    let mut fehler: Vec<Fehler> = toene
        .keys()
        .filter(|name| materialien_da.contains_key(*name))
        .map(|name| {
            let text = format!("„{name}“ ist zugleich Material und Einzelton");
            regel(format!("einzeltoene.{name}"), "material-doppelt", text)
        })
        .collect();
    for a in liste {
        let Ok(spec) = fs::read(a.abs(root)) else {
            continue;
        };
        for m in materialien(&spec) {
            if !materialien_da.contains_key(&m) && !toene.contains_key(&m) {
                let text = format!("Material „{m}“ fehlt in der Stil-Datei");
                fehler.push(regel(format!("{}{ENDUNG}", a.id), "material-unbekannt", text));
            }
        }
    }
    fehler
}

fn validiere_schema(root: &Path, inst: &Value) -> Result<(), String> {
    let mut pfad = root.to_path_buf();
    pfad.extend(STIL_SCHEMA.split('/'));
    let raw = fs::read(pfad).map_err(|e| format!("Schema {STIL_SCHEMA} fehlt: {e}"))?;
    let schema: Value = serde_json::from_slice(&raw).map_err(|e| e.to_string())?;
    let pruefer = jsonschema::validator_for(&schema).map_err(|e| e.to_string())?;
    pruefer.validate(inst).map_err(|e| {
        let ort = e.instance_path().to_string();
        format!("{ort}: {e}")
    })
}

/// Änderungen nur an Materialien oder Einzeltönen betreffen die Assets, die sie nutzen; alles andere alle.
fn betroffen(root: &Path, felder: &[FeldAenderung], liste: &[Asset]) -> Vec<Asset> {
    let mut namen = BTreeSet::new();
    for f in felder {
        let mut teile = f.pfad.splitn(3, '.');
        match (teile.next(), teile.next()) {
            (Some("materialien" | "einzeltoene"), Some(name)) => namen.insert(name.to_owned()),
            _ => return liste.to_vec(),
        };
    }
    let out = liste
        .iter()
        .filter(|a| fs::read(a.abs(root)).is_ok_and(|s| !materialien(&s).is_disjoint(&namen)))
        .cloned()
        .collect();
    mit_reitern(root, out, liste)
}

/// Ergänzt die Reiter der betroffenen Reittiere: ihr Render zeigt das Reittier mit (B-706).
pub(super) fn mit_reitern(root: &Path, mut betroffen: Vec<Asset>, liste: &[Asset]) -> Vec<Asset> {
    let drin: BTreeSet<String> = betroffen.iter().map(|a| a.id.clone()).collect();
    for a in liste.iter().filter(|a| !drin.contains(&a.id)) {
        if fs::read(a.abs(root)).is_ok_and(|spec| reittiere(&spec).iter().any(|r| drin.contains(r))) {
            betroffen.push(a.clone());
        }
    }
    betroffen
}

#[cfg(test)]
#[path = "stil_tests.rs"]
mod tests;
