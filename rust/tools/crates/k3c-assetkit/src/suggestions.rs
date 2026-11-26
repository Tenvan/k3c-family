//! Download-Vorschlagsliste für Ton und Grafik (B-601, WZ29.5): lesen und prüfen.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::catalog::license_flag;
use crate::error::{Error, Result};
use crate::gojson::null_default;
use crate::net::parse_with_host;
use crate::register::{KIND_GRAPHIC, KIND_SOUND, Register};
use crate::research::from_object;

/// Vorschlagsliste; das Studio lädt daraus in den Vorrat `.work/vorrat/` (WZ29.6).
pub const SUGGESTIONS_PATH: &str = "data/assets-vorschlaege.json";

/// Mindestzahl Kandidaten je Lücke und je Skill-Linie (B-601/AC-01).
pub const MIN_SUGGESTIONS: usize = 3;

/// Begrenzt einen Download (Zip-Packs eingeschlossen); größere Einträge lehnt die Prüfung ab.
pub const MAX_SUGGESTION_BYTES: i64 = 50 << 20;

/// Quellen der Liste, zugleich die Schlüssel der Quellen-Übersicht im Studio.
pub const SUGGESTION_SOURCES: &[&str] = &["OpenGameArt", "Kenney", "itch.io", "freesound"];

/// Status eines Eintrags; „gesperrt“ setzt das Backend, wenn die Quelle eine andere Lizenz nennt.
pub const SUGGESTION_STATUS: &[&str] = &["offen", "geladen", "verworfen", "gesperrt"];

/// Endungen, die der Vorrat lädt (Pakete nur zum Auspacken von Hand).
// Steht in Go in `vorrat.go`; hier, weil die Prüfung sie vor dem Vorrat (RU67.3) braucht.
pub(crate) const VORRAT_EXT: &[&str] = &["png", "ogg", "mp3", "wav", "zip", "7z"];

/// Eintrag der Vorschlagsliste (Felder wie `SourceProposal` im Studio-Frontend, ohne Laufzeitfelder).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Suggestion {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    /// Slot-ID oder Gruppe (`skill-effekt:<linie>`).
    #[serde(deserialize_with = "null_default")]
    pub slot: String,
    #[serde(rename = "art", deserialize_with = "null_default")]
    pub kind: String,
    #[serde(rename = "titel", deserialize_with = "null_default")]
    pub title: String,
    #[serde(rename = "quelle", deserialize_with = "null_default")]
    pub source: String,
    #[serde(rename = "seite", deserialize_with = "null_default")]
    pub page: String,
    #[serde(rename = "dateiUrl", deserialize_with = "null_default")]
    pub file_url: String,
    #[serde(
        rename = "urheber",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub artist: String,
    #[serde(rename = "lizenz", deserialize_with = "null_default")]
    pub license: String,
    #[serde(rename = "dateiname", deserialize_with = "null_default")]
    pub file_name: String,
    #[serde(rename = "groesse", deserialize_with = "null_default")]
    pub size: i64,
    #[serde(rename = "prioritaet", deserialize_with = "null_default")]
    pub priority: i64,
    #[serde(
        rename = "hinweis",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub note: String,
    #[serde(deserialize_with = "null_default")]
    pub status: String,
}

/// Inhalt von `data/assets-vorschlaege.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Suggestions {
    #[serde(rename = "_comment", deserialize_with = "null_default")]
    pub comment: String,
    #[serde(rename = "vorschlaege", deserialize_with = "null_default")]
    pub list: Vec<Suggestion>,
}

/// Liest die Liste; fehlt die Datei, ist sie leer. Ungültiges JSON ist ein Fehler.
pub fn load_suggestions(root: &Path) -> Result<Suggestions> {
    let bytes = match fs::read(root.join(SUGGESTIONS_PATH)) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Suggestions::default()),
        Err(e) => return Err(Error::Io(e)),
    };
    from_object(&bytes).map_err(|e| Error::Msg(format!("{SUGGESTIONS_PATH}: {e}")))
}

/// Lizenz nach Q70 übernehmbar (CC0, CC-BY, OGA-BY, CC-BY-SA); NC und ND nie, Sampling+ nie.
pub fn suggestion_license_allowed(license: &str) -> bool {
    let upper = license.to_uppercase();
    if upper.contains("SAMPLING") || matches!(license_flag(license), "ND" | "NC") {
        return false;
    }
    ["CC0", "CC-BY", "OGA-BY"].iter().any(|p| upper.starts_with(p))
}

/// Ziele, die je [`MIN_SUGGESTIONS`] Kandidaten brauchen: jede Lücke des Registers (ihre Art) und je Skill-Linie
/// (`sfx:skill:<linie>`) ein Ton für den Slot und eine Grafik für die Gruppe `skill-effekt:<linie>`. Schlüssel ist das
/// Ziel, Wert die Art.
pub fn suggestion_targets(reg: &Register) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for s in &reg.slots {
        if s.choices().is_empty() {
            out.insert(s.id.clone(), s.kind.clone());
        }
        if let Some(line) = s.id.strip_prefix("sfx:skill:") {
            out.insert(s.id.clone(), KIND_SOUND.to_string());
            out.insert(format!("skill-effekt:{line}"), KIND_GRAPHIC.to_string());
        }
    }
    out
}

/// Prüft Pflichtfelder, Lizenz, Größe und eindeutige IDs und Datei-URLs je Eintrag sowie die Mindestzahl je Ziel
/// (B-601/AC-01); leer = in Ordnung.
pub fn check_suggestions(list: &[Suggestion], targets: &BTreeMap<String, String>) -> Vec<Error> {
    let mut errors = Vec::new();
    let (mut ids, mut urls) = (HashSet::new(), HashSet::new());
    let mut count: HashMap<&str, usize> = HashMap::new();
    for s in list {
        if let Err(e) = check_suggestion(s) {
            errors.push(Error::Msg(format!("{}: {e}", s.id)));
        }
        if ids.contains(&s.id) || urls.contains(&s.file_url) {
            errors.push(Error::Msg(format!("{}: ID oder Datei-URL doppelt", s.id)));
        }
        ids.insert(&s.id);
        urls.insert(&s.file_url);
        if targets.get(&s.slot) == Some(&s.kind) && s.status != "verworfen" {
            *count.entry(&s.slot).or_default() += 1;
        }
    }
    for (target, kind) in targets {
        let n = count.get(target.as_str()).copied().unwrap_or_default();
        if n < MIN_SUGGESTIONS {
            errors.push(Error::Msg(format!(
                "{target} ({kind}): {n} Kandidaten, mindestens {MIN_SUGGESTIONS}"
            )));
        }
    }
    errors
}

/// Endung mit Punkt entfernt, wie Go `filepath.Ext` auch bei Namen wie `.png`; ohne Punkt leer.
fn extension(file_name: &str) -> String {
    let name = file_name.rsplit(std::path::is_separator).next().unwrap_or_default();
    name.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default()
}

pub(crate) fn check_suggestion(s: &Suggestion) -> Result<()> {
    if s.id.is_empty() || s.slot.is_empty() || s.title.is_empty() || s.file_name.is_empty() {
        return Err(Error::Msg("id, slot, titel und dateiname sind Pflicht".to_string()));
    }
    if !VORRAT_EXT.contains(&extension(&s.file_name).as_str()) {
        return Err(Error::Msg(format!(
            "endung von {:?} lädt der Vorrat nicht",
            s.file_name
        )));
    }
    check_suggestion_values(s)?;
    for u in [&s.page, &s.file_url] {
        if !parse_with_host(u).is_some_and(|p| p.scheme() == "https") {
            return Err(Error::Msg(format!("adresse {u:?} ist keine https-URL")));
        }
    }
    Ok(())
}

/// Prüft Art, Quelle, Lizenz, Größe, Priorität und Status gegen ihre Wertebereiche.
fn check_suggestion_values(s: &Suggestion) -> Result<()> {
    let text = if s.kind != KIND_GRAPHIC && s.kind != KIND_SOUND {
        format!("art {:?} unbekannt", s.kind)
    } else if !SUGGESTION_SOURCES.contains(&s.source.as_str()) {
        format!("quelle {:?} unbekannt", s.source)
    } else if !suggestion_license_allowed(&s.license) {
        format!("lizenz {:?} nicht erlaubt (Q70)", s.license)
    } else if s.size <= 0 || s.size > MAX_SUGGESTION_BYTES {
        format!("groesse {} außerhalb 1 … {MAX_SUGGESTION_BYTES}", s.size)
    } else if !(1..=3).contains(&s.priority) {
        format!("prioritaet {} außerhalb 1 … 3", s.priority)
    } else if !SUGGESTION_STATUS.contains(&s.status.as_str()) {
        format!("status {:?} unbekannt", s.status)
    } else {
        return Ok(());
    };
    Err(Error::Msg(text))
}

#[cfg(test)]
#[path = "suggestions_tests.rs"]
mod tests;
