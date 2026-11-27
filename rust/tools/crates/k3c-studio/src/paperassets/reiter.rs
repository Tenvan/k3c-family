//! Abhängigkeit Reiter → Reittier (B-706, Port von `reiter.go`): Eine Spec, deren Animationen ein Reittier nutzen,
//! hängt beim Prüfen und Rendern von dessen Spec ab (Regel reittier-unbekannt, tools/paper/rig/reiten.ts).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::basis::{Asset, assets};
use super::gueltig::datei_stand;
use crate::review::reittiere;

/// Die IDs der Assets, die `id` als Reittier nutzen (sortiert); ihr Render ist nach einer Änderung am Reittier
/// veraltet.
pub fn reiter_von(root: &Path, id: &str) -> Vec<String> {
    let Ok(alle) = assets(root) else {
        return Vec::new();
    };
    let mut out: Vec<String> = alle
        .into_iter()
        .filter(|a| fs::read(a.abs(root)).is_ok_and(|b| reittiere(&b).iter().any(|r| r == id)))
        .map(|a| a.id)
        .collect();
    out.sort();
    out
}

/// Der Datei-Stand der Reittier-Specs von `pfad` (leer ohne Reittier); ein geändertes oder gelöschtes Reittier macht den
/// Gültigkeits-Eintrag des Reiters ungültig. Ein leerer Pfad in `nach_id` heißt „mehrdeutige ID“.
pub(super) fn reittier_stand(root: &Path, pfad: &Path, nach_id: &BTreeMap<String, Asset>) -> String {
    let Ok(spec) = fs::read(pfad) else {
        return String::new();
    };
    reittiere(&spec)
        .into_iter()
        .map(|id| match nach_id.get(&id) {
            None => format!("|{id}:–"),
            Some(r) if r.pfad.is_empty() => format!("|{id}:mehrdeutig"),
            Some(r) => format!("|{id}:{}", datei_stand(&r.abs(root))),
        })
        .collect()
}
