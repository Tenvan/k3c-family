//! Asset-Katalog: alle Bilder und Sounds unter `public/` mit Pack, Gruppe, Maßen und Credit; Suche und Lizenz-Kennzeichen.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::error::Result;
use crate::gojson::null_default;
use crate::register::{KIND_GRAPHIC, KIND_SOUND, SOUND_EXT};
use crate::{fsutil, pngsize};

/// Datei des Asset-Katalogs mit Pack, Gruppe (Tag), Maßen und Credit.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Asset {
    /// Relativ zu `public/`.
    #[serde(rename = "datei")]
    pub file: String,
    #[serde(rename = "art")]
    pub kind: String,
    pub pack: String,
    #[serde(rename = "gruppe", skip_serializing_if = "String::is_empty")]
    pub group: String,
    #[serde(skip_serializing_if = "is_zero")]
    pub w: i64,
    #[serde(skip_serializing_if = "is_zero")]
    pub h: i64,
    #[serde(rename = "titel", skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(rename = "urheber", skip_serializing_if = "String::is_empty")]
    pub artist: String,
    #[serde(rename = "lizenz")]
    pub license: String,
    #[serde(rename = "quelle", skip_serializing_if = "String::is_empty")]
    pub source: String,
    #[serde(rename = "kennzeichen", skip_serializing_if = "String::is_empty")]
    pub flag: String,
}

fn is_zero(v: &i64) -> bool {
    *v == 0
}

/// Ordner unter `public/` mit ihrer Credit-Datei.
const CATALOG_DIRS: &[(&str, &str)] = &[
    ("grafik", "grafik/CREDITS.md"),
    ("sprites", "sprites/CREDITS.md"),
    ("audio", "audio/README.md"),
    ("audio-test", "audio-test/README.md"),
];

/// Gilt für Sounds ohne Credit-Zeile: die Atlas- und Probe-Töne sind selbst erzeugt (`public/audio/README.md`).
// Ordner-Annahme wie in Go; ein fremder Sound braucht eine Zeile in der Credit-Tabelle (Import M10.5).
pub(crate) const SELF_MADE: &str = "selbst erzeugt (kein Credit nötig)";

/// Gilt für eigene Gebäude-Grafik aus `task gen:gebaeude` unter `public/grafik/gebaeude/` (B-437).
pub(crate) const OWN_BUILDING: &str = "eigene Grafik (tools/buildings, kein Credit nötig)";

#[derive(Debug, Clone)]
struct Credit {
    title: String,
    artist: String,
    license: String,
    source: String,
}

/// Liest alle Bilder und Sounds unter `public/` mit Pack, Gruppe, Maßen und Credit, nach Datei sortiert.
pub fn catalog(root: &Path) -> Result<Vec<Asset>> {
    let public = root.join("public");
    let groups = graphic_groups(&public);
    let mut list = Vec::new();
    for (dir, credit_file) in CATALOG_DIRS {
        let credits = read_credits(&fsutil::join_below(&public, credit_file));
        // Ein fehlender Ordner ist kein Fehler des Katalogs.
        for entry in WalkDir::new(public.join(dir)).sort_by_file_name() {
            let Ok(entry) = entry else { continue };
            if entry.file_type().is_dir() {
                continue;
            }
            let Ok(rel) = entry.path().strip_prefix(&public) else {
                continue;
            };
            let parts: Vec<_> = rel.iter().map(|p| p.to_string_lossy()).collect();
            if let Some(mut a) = asset(&parts.join("/"), entry.path(), &credits) {
                a.group = groups.get(&a.file).cloned().unwrap_or_default();
                list.push(a);
            }
        }
    }
    list.sort_by(|a, b| a.file.cmp(&b.file));
    Ok(list)
}

fn asset(rel: &str, full: &Path, credits: &BTreeMap<String, Credit>) -> Option<Asset> {
    let name = rel.rsplit('/').next().unwrap_or_default();
    let ext = name.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default();
    let parts: Vec<&str> = rel.split('/').collect();
    let mut a = Asset {
        file: rel.to_string(),
        kind: KIND_GRAPHIC.to_string(),
        ..Asset::default()
    };
    if SOUND_EXT.contains(&ext.as_str()) {
        a.kind = KIND_SOUND.to_string();
        a.pack = parts[0].to_string();
    } else if ext == "png" && parts.len() > 2 {
        a.pack = parts[1].to_string();
        (a.w, a.h) = pngsize::size(full).unwrap_or_default();
    } else {
        return None;
    }
    // Import: Zeile je Datei (kandidaten/<datei>).
    let below = parts.get(1..).unwrap_or_default().join("/");
    match credits.get(&below).or_else(|| credits.get(&a.pack)) {
        Some(c) => {
            a.title = c.title.clone();
            a.artist = c.artist.clone();
            a.license = c.license.clone();
            a.source = c.source.clone();
        }
        None if a.kind == KIND_SOUND => a.license = SELF_MADE.to_string(),
        None if a.pack == "gebaeude" => a.license = OWN_BUILDING.to_string(),
        None => a.license = "unbekannt".to_string(),
    }
    a.flag = license_flag(&a.license).to_string();
    Some(a)
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct GraphicItem {
    #[serde(deserialize_with = "null_default")]
    pack: String,
    #[serde(deserialize_with = "null_default")]
    group: String,
    #[serde(deserialize_with = "null_default")]
    file: String,
}

/// Ordnet jeder Datei unter `grafik/` ihre Gruppe aus `public/grafik/index.json` zu (Tag der Suche).
fn graphic_groups(public: &Path) -> BTreeMap<String, String> {
    let Ok(bytes) = fs::read(public.join("grafik").join("index.json")) else {
        return BTreeMap::new();
    };
    let Ok(items) = serde_json::from_slice::<Option<Vec<GraphicItem>>>(&bytes) else {
        return BTreeMap::new();
    };
    items
        .unwrap_or_default()
        .into_iter()
        .map(|it| (format!("grafik/{}/{}", it.pack, it.file), it.group))
        .collect()
}

/// Liest die Tabellen `| Ordner | Pack | Urheber | Lizenz | Quelle |` einer Credit-Datei; eine Zelle Ordner darf mehrere
/// Ordner durch Komma nennen.
fn read_credits(file: &Path) -> BTreeMap<String, Credit> {
    let mut out = BTreeMap::new();
    let Ok(bytes) = fs::read(file) else {
        return out;
    };
    let mut in_table = false;
    for line in String::from_utf8_lossy(&bytes).replace("\r\n", "\n").split('\n') {
        if !line.starts_with('|') {
            in_table = false;
        } else if line.starts_with("| Ordner |") {
            in_table = true;
        } else if in_table && !line.starts_with("|---") {
            let cells: Vec<&str> = line.trim_matches('|').split('|').collect();
            if cells.len() < 5 {
                continue;
            }
            let credit = Credit {
                title: cells[1].trim().to_string(),
                artist: cells[2].trim().to_string(),
                license: cells[3].trim().to_string(),
                source: cells[4].trim().to_string(),
            };
            for dir in cells[0].split(',') {
                out.insert(dir.trim().trim_matches('`').to_string(), credit.clone());
            }
        }
    }
    out
}

/// Kennzeichnet eine Lizenz nach Q70: `ND` (nicht übernehmbar), `NC` (nur markierter Lückenfüller), `SA` (Ausschnitt
/// bleibt unter CC-BY-SA), leer = übernehmbar oder unbekannt.
pub fn license_flag(license: &str) -> &'static str {
    // Elemente einzeln (CC BY-NC-SA 4.0), damit Wörter wie „Sample“ nicht zählen.
    let upper = license.to_uppercase();
    let found: HashSet<&str> = upper.split([' ', '-', '/']).collect();
    ["ND", "NC", "SA"].into_iter().find(|f| found.contains(f)).unwrap_or("")
}

/// Filtert den Katalog: jedes Wort muss in Datei, Pack, Gruppe, Titel oder Lizenz vorkommen (ohne Groß/Klein).
/// `kind` leer = beide Arten; `limit` ≤ 0 = alle.
pub fn search(list: &[Asset], query: &str, kind: &str, limit: i64) -> Vec<Asset> {
    let query = query.to_lowercase();
    let words: Vec<&str> = query.split_whitespace().collect();
    let limit = usize::try_from(limit).ok().filter(|&n| n > 0).unwrap_or(usize::MAX);
    list.iter()
        .filter(|a| kind.is_empty() || a.kind == kind)
        .filter(|a| {
            let hay = [&a.file, &a.pack, &a.group, &a.title, &a.license]
                .map(String::as_str)
                .join(" ")
                .to_lowercase();
            words.iter().all(|w| hay.contains(w))
        })
        .take(limit)
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;
