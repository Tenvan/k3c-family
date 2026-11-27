//! Teil-Patch (B-710, Port von `teile.go`).

use serde_json::Value;
use serde_json::value::RawValue;

use super::patch::{index, ist_null, merge_patch, objekt, schreibe_objekt};
use super::{Error, Fehler};
use crate::kitx::marshal_raw;

/// Ist `teile` im Patch ein Objekt nach Teil-ID, wird je genanntem Teil ein Merge-Patch auf dessen Eintrag in der Liste
/// der Spec angewandt (`null` entfernt das Teil); Reihenfolge und übrige Teile bleiben. Eine Liste ersetzt wie bisher
/// alles (RFC 7396). Eine unbekannte ID lehnt den Patch ab, mit Feld `teile.<id>` und den vorhandenen IDs.
pub(super) fn teile_patch(spec: &str, patch: &str) -> Result<String, Error> {
    // Kaputtes JSON meldet `merge_patch`.
    let unveraendert = || Ok(patch.to_owned());
    if serde_json::from_str::<&RawValue>(patch).is_err() {
        return unveraendert();
    }
    let Ok(Some(mut felder)) = objekt(patch) else {
        return unveraendert();
    };
    let Some(i) = index(&felder, "teile") else {
        return unveraendert();
    };
    let Ok(Some(je_id)) = objekt(&felder[i].1) else {
        return unveraendert();
    };
    let (mut liste, ids) = spec_teile(spec);
    let mut fehler = Vec::new();
    let mut weg = vec![false; liste.len()];
    for (id, wert) in je_id {
        match ids.iter().position(|x| *x == id) {
            None => fehler.push(Fehler {
                feld: format!("teile.{id}"),
                regel: "teil-unbekannt".to_owned(),
                text: format!("Teil „{id}“ fehlt; vorhanden: {}", ids.join(", ")),
            }),
            Some(k) if ist_null(&wert) => weg[k] = true,
            Some(k) => liste[k] = merge_patch(&liste[k], &wert)?,
        }
    }
    if !fehler.is_empty() {
        return Err(Error::Abgelehnt(fehler));
    }
    let rest: Vec<String> = liste
        .iter()
        .zip(weg)
        .filter(|(_, weg)| !weg)
        .map(|(t, _)| marshal_raw(t))
        .collect();
    felder[i].1 = format!("[{}]", rest.join(","));
    schreibe_objekt(&felder)
}

/// Die Liste `teile` der Spec roh und die ID je Eintrag.
pub(super) fn spec_teile(spec: &str) -> (Vec<String>, Vec<String>) {
    let felder = match serde_json::from_str::<&RawValue>(spec) {
        Ok(_) => objekt(spec).ok().flatten().unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    let roh = felder.iter().rev().find(|(k, _)| k == "teile");
    let liste: Vec<String> = roh
        .and_then(|(_, v)| serde_json::from_str::<Vec<Box<RawValue>>>(v).ok())
        .map(|l| l.iter().map(|t| t.get().to_owned()).collect())
        .unwrap_or_default();
    let ids = liste.iter().map(|t| teil_id(t)).collect();
    (liste, ids)
}

fn teil_id(teil: &str) -> String {
    let wert: Value = serde_json::from_str(teil).unwrap_or_default();
    wert.get("id").and_then(Value::as_str).unwrap_or_default().to_owned()
}

#[cfg(test)]
#[path = "teile_tests.rs"]
mod tests;
