//! Liste ohne Fließtexte und einzelne Dokumente je ID (B-909).

use std::collections::HashSet;
use std::path::Path;

use serde::Serialize;

use super::create::get_ref;
use super::planning::{Data, load};
use super::store::resolve;

/// Grenze eines Aufrufs von [`items`]; mehr IDs sind ein Fehler der Oberfläche (sie lädt dann die Liste neu).
pub const MAX_ITEMS: usize = 100;

/// Ein Planungs-Dokument mit Fließtext: Ticket, Sprint-README, Session oder Projekt.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Item {
    pub id: String,
    /// ticket | sprint | session | projekt
    pub kind: String,
    /// Datei relativ zum Repo (docs/…).
    pub path: String,
    /// Markdown der Datei mit `\n`, beim Ticket um die Rückverweise ergänzt wie `get`.
    pub text: String,
    /// Unbekannte oder ungültige ID; Text und Pfad dann leer.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
}

/// Liest die Planung wie `load`, aber ohne die Fließtexte der Session-Dateien (den Löwenanteil der Daten). Titel,
/// Umgebung, Domäne und Abhängigkeiten bleiben: die Listen brauchen sie.
pub fn load_list(root: &Path) -> Result<Data, String> {
    let mut data = load(root)?;
    for session in data.sprints.iter_mut().flat_map(|s| &mut s.sessions) {
        session.text = String::new();
    }
    Ok(data)
}

fn item(root: &Path, id: &str) -> Item {
    let base = Item {
        id: id.to_owned(),
        ..Item::default()
    };
    let found = match resolve(root, id) {
        Ok(found) => found,
        Err(error) => return Item { error, ..base },
    };
    let kind = found.kind.to_owned();
    match get_ref(root, &found) {
        Err(error) => Item { kind, error, ..base },
        Ok(text) => Item {
            kind,
            path: format!("docs/{}", found.rel),
            // `get` setzt den Pfad und eine Leerzeile davor.
            text: text
                .split_once("\n\n")
                .map_or_else(String::new, |(_, rest)| rest.to_owned()),
            ..base
        },
    }
}

/// Pfad und Fließtext zu den IDs (B-123, WZ61, WZ61.1, GRA) in der Reihenfolge der Anfrage; Doppelte zählen einmal.
/// Eine unbekannte ID ergibt einen Eintrag mit Fehler, keinen Fehler des ganzen Aufrufs.
pub fn items(root: &Path, ids: &[String]) -> Result<Vec<Item>, String> {
    if ids.len() > MAX_ITEMS {
        return Err(format!("höchstens {MAX_ITEMS} IDs je Aufruf, gefragt {}", ids.len()));
    }
    let mut seen = HashSet::new();
    Ok(ids
        .iter()
        .filter(|id| seen.insert(id.as_str()))
        .map(|id| item(root, id))
        .collect())
}
