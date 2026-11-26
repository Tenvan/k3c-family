//! Recherche-Treffer je Asset-Slot (B-298): Ablage, Lizenz-Einordnung nach Q70 und der Recherche-Lauf.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use sha1::{Digest, Sha1};

use crate::catalog::license_flag;
use crate::error::{Error, Result};
use crate::gojson::{self, null_default};
use crate::net::parse_with_host;
use crate::register::{Slot, load, lock, write_atomic};
use crate::sources::Searcher;

/// Recherche-Treffer je Asset-Slot (B-298), getrennt vom Slot-Register.
pub const RESEARCH_PATH: &str = "data/assets-recherche.json";

/// Lizenz-Status nach Q70: übernehmbar (NC nur als markierter Lückenfüller).
pub const LICENSE_KNOWN: &str = "erkannt";
/// Lizenz-Status nach Q70: unklar, erst nach Bestätigung durch 🧑.
pub const LICENSE_CHECK: &str = "pruefen";
/// Lizenz-Status nach Q70: ND oder Quelle ohne Weitergabe der Rohdateien.
pub const LICENSE_EXCLUDED: &str = "ausgeschlossen";

/// Recherche-Treffer: Vorschlag mit Link, Quelle und Lizenz; Download nur bei direkter Datei.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Hit {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(rename = "titel", deserialize_with = "null_default")]
    pub title: String,
    #[serde(deserialize_with = "null_default")]
    pub link: String,
    #[serde(rename = "quelle", deserialize_with = "null_default")]
    pub source: String,
    #[serde(
        rename = "urheber",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub artist: String,
    #[serde(
        rename = "lizenz",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub license: String,
    #[serde(
        rename = "vorschau",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub preview: String,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub download: String,
    #[serde(rename = "lizenzStatus", deserialize_with = "null_default")]
    pub license_status: String,
    #[serde(
        rename = "kennzeichen",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub flag: String,
    #[serde(rename = "erstellt", deserialize_with = "null_default")]
    pub created: String,
    /// Nach dem Import: Datei relativ zu `public/`.
    #[serde(
        rename = "datei",
        deserialize_with = "null_default",
        skip_serializing_if = "String::is_empty"
    )]
    pub file: String,
}

/// Inhalt von `data/assets-recherche.json`: Treffer je Slot-ID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Research {
    #[serde(rename = "_comment", deserialize_with = "null_default")]
    pub comment: String,
    #[serde(rename = "treffer", deserialize_with = "hit_map")]
    pub hits: BTreeMap<String, Vec<Hit>>,
}

fn hit_map<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<BTreeMap<String, Vec<Hit>>, D::Error> {
    let map = Option::<BTreeMap<String, Option<Vec<Hit>>>>::deserialize(d)?;
    Ok(map
        .unwrap_or_default()
        .into_iter()
        .map(|(slot, hits)| (slot, hits.unwrap_or_default()))
        .collect())
}

const RESEARCH_COMMENT: &str = "Recherche-Treffer je Asset-Slot (B-298, M10.4): Vorschläge mit Link, Quelle, Urheber, Lizenz und Vorschau. \
     Schreibt k3c-workbench (Recherche, MCP assets_hit_add); ein Treffer bleibt, bis 🧑 ihn verwirft. lizenzStatus: erkannt, pruefen, ausgeschlossen (Q70), bestaetigt (🧑).";

/// Liest die Treffer; fehlt die Datei, ist die Ablage leer. Ungültiges JSON ist ein Fehler (nie überschreiben).
pub fn load_research(root: &Path) -> Result<Research> {
    let bytes = match fs::read(root.join(RESEARCH_PATH)) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok(Research {
                comment: RESEARCH_COMMENT.to_string(),
                hits: BTreeMap::new(),
            });
        }
        Err(e) => {
            return Err(Error::Io(io::Error::new(e.kind(), format!("{RESEARCH_PATH}: {e}"))));
        }
    };
    from_object(&bytes).map_err(|e| Error::Msg(format!("{RESEARCH_PATH} ungültig, nichts geschrieben: {e}")))
}

/// Liest ein JSON-Objekt (oder `null`) wie Go: serde läse auch eine Liste als Struktur und ein Speichern danach
/// überschriebe die Datei.
pub(crate) fn from_object<T: DeserializeOwned + Default>(bytes: &[u8]) -> std::result::Result<T, String> {
    if bytes.trim_ascii_start().starts_with(b"[") {
        return Err("json: cannot unmarshal array into Go value".to_string());
    }
    let value = serde_json::from_slice::<Option<T>>(bytes).map_err(|e| e.to_string())?;
    Ok(value.unwrap_or_default())
}

impl Research {
    pub(crate) fn save(&self, root: &Path) -> Result<()> {
        let mut bytes = gojson::marshal_indent(self, false).map_err(|e| Error::Msg(e.to_string()))?;
        bytes.push(b'\n');
        write_atomic(&root.join(RESEARCH_PATH), &bytes)
    }
}

/// Die gespeicherten Treffer eines Slots.
pub fn hits_of(root: &Path, slot_id: &str) -> Result<Vec<Hit>> {
    let mut research = load_research(root)?;
    Ok(research.hits.remove(slot_id).unwrap_or_default())
}

/// Legt Treffer zu einem vorhandenen Asset-Slot ab; gleicher Link wird nicht doppelt angelegt. Die Lizenz wird nach Q70
/// eingeordnet, sofern der Treffer keinen Status mitbringt. Antwort: Zahl neuer Treffer.
pub fn add_hits(root: &Path, slot_id: &str, hits: &[Hit], now: DateTime<FixedOffset>) -> Result<usize> {
    let _guard = lock();
    if load(root)?.find(slot_id).is_none() {
        return Err(Error::Msg(format!("unbekannter Asset-Slot {slot_id:?}")));
    }
    let mut research = load_research(root)?;
    let mut added = 0;
    for hit in hits {
        if hit.link.is_empty() || hit.title.is_empty() {
            return Err(Error::Msg("treffer braucht titel und link".to_string()));
        }
        let stored = research.hits.get(slot_id);
        if stored.is_some_and(|list| list.iter().any(|h| h.link == hit.link)) {
            continue;
        }
        let mut h = hit.clone();
        h.id = hit_id(&h.link);
        h.created = now.format("%Y-%m-%d").to_string();
        // Immer nach Q70 einordnen; ein vorsichtigeres „pruefen“ des Aufrufers bleibt stehen.
        let (status, flag) = classify_license(&h.license, &h.link);
        if h.license_status != LICENSE_CHECK || status != LICENSE_KNOWN {
            h.license_status = status.to_string();
        }
        h.flag = flag.to_string();
        research.hits.entry(slot_id.to_string()).or_default().push(h);
        added += 1;
    }
    if added > 0 {
        research.save(root)?;
    }
    Ok(added)
}

/// Verwirft einen Treffer (🧑 entscheidet); ein unbekannter Treffer ist ein Fehler.
pub fn drop_hit(root: &Path, slot_id: &str, id: &str) -> Result<()> {
    let _guard = lock();
    let mut research = load_research(root)?;
    if let Some(list) = research.hits.get_mut(slot_id)
        && let Some(i) = list.iter().position(|h| h.id == id)
    {
        list.remove(i);
        if list.is_empty() {
            research.hits.remove(slot_id);
        }
        return research.save(root);
    }
    Err(Error::Msg(format!("treffer {id:?} zu {slot_id} nicht gefunden")))
}

/// Stabil je Link (kurz, für Oberfläche und MCP).
fn hit_id(link: &str) -> String {
    let sum = Sha1::digest(link.as_bytes());
    sum.iter().take(4).map(|b| format!("{b:02x}")).collect()
}

/// Geben die Rohdateien nicht weiter (Q70): nie übernehmbar.
const EXCLUDED_HOSTS: &[&str] = &[
    "pixabay.com",
    "mixkit.co",
    "craftpix.net",
    "zapsplat.com",
    "sonniss.com",
];

/// Nach Q70 übernehmbar (NC nur als markierter Lückenfüller).
pub(crate) const KNOWN_LICENSES: &[&str] = &["CC0", "PUBLIC DOMAIN", "CC BY", "CC-BY", "OGA-BY", "OFL"];

fn excluded_host(link: &str) -> bool {
    let Some(url) = parse_with_host(link) else {
        return false;
    };
    let host = url.host_str().unwrap_or_default().to_lowercase();
    let host = host.strip_prefix("www.").unwrap_or(&host);
    EXCLUDED_HOSTS.iter().any(|x| {
        host.strip_suffix(x)
            .is_some_and(|rest| rest.is_empty() || rest.ends_with('.'))
    })
}

/// Ordnet eine Lizenz nach Q70 ein: Status und Kennzeichen (NC, SA, ND).
pub fn classify_license(license: &str, link: &str) -> (&'static str, &'static str) {
    if excluded_host(link) {
        return (LICENSE_EXCLUDED, "");
    }
    let flag = license_flag(license);
    if flag == "ND" {
        return (LICENSE_EXCLUDED, flag);
    }
    let upper = license.to_uppercase();
    if KNOWN_LICENSES.iter().any(|k| upper.contains(k)) {
        return (LICENSE_KNOWN, flag);
    }
    (LICENSE_CHECK, flag)
}

/// Ergebnis einer Recherche zu einem Slot: alle gespeicherten Treffer, Zahl neuer, Hinweise der Quellen.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResearchResult {
    #[serde(rename = "treffer")]
    pub hits: Vec<Hit>,
    #[serde(rename = "neu")]
    pub added: usize,
    #[serde(rename = "hinweise")]
    pub notes: Vec<String>,
}

/// Sucht für einen Asset-Slot (Suchbegriff leer = Beschreibung des Slots) und legt die Treffer ab.
pub async fn run_research(
    root: &Path,
    slot_id: &str,
    query: &str,
    s: &Searcher,
    now: DateTime<FixedOffset>,
) -> Result<ResearchResult> {
    let register = load(root)?;
    let slot = register
        .find(slot_id)
        .ok_or_else(|| Error::Msg(format!("unbekannter Asset-Slot {slot_id:?}")))?;
    let term = if query.trim().is_empty() {
        search_term(slot)
    } else {
        query
    };
    let (found, notes) = s.search(term, &slot.kind).await;
    let added = add_hits(root, slot_id, &found, now)?;
    let hits = hits_of(root, slot_id)?;
    Ok(ResearchResult { hits, added, notes })
}

/// Vorgabe der Suche: Beschreibung ohne Abschnitt („Gebäude: farm“ → „farm“).
pub(crate) fn search_term(slot: &Slot) -> &str {
    let d = slot.description.as_str();
    d.rsplit_once(':').map_or(d, |(_, term)| term).trim()
}

#[cfg(test)]
#[path = "research_tests.rs"]
mod tests;
