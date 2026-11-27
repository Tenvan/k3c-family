//! Berichte verwalten (Port von `manage.go`, WZ28.13, B-612): Bewertung je Bericht in `reports/ratings.json`, Löschen
//! einzelner Berichte. Beide Studios teilen sich die Datei (gleiches Format).
//!
//! Geschrieben und gelöscht wird nur über den Schreibschutz des Balancings (`balance::writeguard`): nur unter `reports/`
//! und nie über einen Link oder eine Junction. Härtung gegenüber Go: alle Pfade werden auch damit vorab geprüft, und
//! gesicherte Baselines (`reports/baseline/`) lassen sich nicht löschen.

use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use super::reports::{clean_parts, read_report_file, report_path};
use super::run::lock;
use super::stamp::read_object;
use crate::balance::baselines::BASELINE_DIR;
use crate::balance::writeguard;
use crate::kitx::{go_marshal_indent, go_quoted};

/// Erlaubte Bewertungen; leer = keine.
pub const RATINGS: [&str; 4] = ["gut", "schlecht", "unklar", "referenz"];

/// Liegt neben den Berichten; Schlüssel ist der Berichtspfad relativ zum Checkout (mit `/`).
const RATINGS_FILE: &str = "ratings.json";

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
struct RatingsDoc {
    version: i64,
    ratings: BTreeMap<String, String>,
}

/// Schützt Lesen und Schreiben von `ratings.json` (Seite und mehrere Aufrufe gleichzeitig).
static RATINGS_LOCK: Mutex<()> = Mutex::new(());

fn ratings_path(root: &Path) -> PathBuf {
    root.join("reports").join(RATINGS_FILE)
}

/// Liest `reports/ratings.json`; fehlt die Datei, ist die Liste leer.
pub(super) fn load_ratings(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let _guard = lock(&RATINGS_LOCK);
    read_ratings(root)
}

/// Wie [`load_ratings`], der Aufrufer hält [`RATINGS_LOCK`].
fn read_ratings(root: &Path) -> Result<BTreeMap<String, String>, String> {
    match fs::read(ratings_path(root)) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(e) => Err(e.to_string()),
        Ok(raw) => read_object::<RatingsDoc>(&raw)
            .map(|doc| doc.ratings)
            .map_err(|e| format!("{RATINGS_FILE} unlesbar: {e}")),
    }
}

/// Schreibt atomar (Temp-Datei und Umbenennen).
fn save_ratings(root: &Path, ratings: BTreeMap<String, String>) -> Result<(), String> {
    let mut text = go_marshal_indent(&RatingsDoc { version: 1, ratings })?;
    text.push('\n');
    writeguard::mkdir_all(root, root.join("reports")).map_err(|e| e.to_string())?;
    writeguard::write_file(root, ratings_path(root), text.as_bytes()).map_err(|e| e.to_string())
}

fn update_ratings(root: &Path, change: impl FnOnce(&mut BTreeMap<String, String>)) -> Result<(), String> {
    let _guard = lock(&RATINGS_LOCK);
    let mut ratings = read_ratings(root)?;
    change(&mut ratings);
    save_ratings(root, ratings)
}

/// Setzt die Bewertung eines vorhandenen Berichts; leer löscht sie.
pub fn rate_report(root: &Path, rel: &str, rating: &str) -> Result<(), String> {
    rate_reports(root, &[rel.to_owned()], rating)
}

/// Setzt die Bewertung mehrerer Berichte in einem Schritt (B-662); ein ungültiger Pfad lehnt alle ab.
pub fn rate_reports(root: &Path, rels: &[String], rating: &str) -> Result<(), String> {
    if !rating.is_empty() && !RATINGS.contains(&rating) {
        return Err(format!(
            "bewertung {}: erlaubt sind {} oder leer",
            go_quoted(rating),
            RATINGS.join(", ")
        ));
    }
    let keys = report_keys(root, rels)?;
    update_ratings(root, |ratings| {
        for key in keys {
            if rating.is_empty() {
                ratings.remove(&key);
            } else {
                ratings.insert(key, rating.to_owned());
            }
        }
    })
}

/// Löscht einen Bericht (`balance-*.json` oder `tuning-*.json` unter `reports/`) samt gleichnamigem `.md` und seiner
/// Bewertung; der Lauf-Ordner verschwindet nur, wenn er danach leer ist.
pub fn delete_report(root: &Path, rel: &str) -> Result<(), String> {
    delete_reports(root, &[rel.to_owned()])
}

/// Löscht mehrere Berichte wie [`delete_report`] (B-662). Erst werden alle Pfade geprüft: ein ungültiger lehnt den
/// ganzen Aufruf ab, nichts ist gelöscht.
pub fn delete_reports(root: &Path, rels: &[String]) -> Result<(), String> {
    let keys = report_keys(root, rels)?;
    for key in &keys {
        deletable(root, key)?;
    }
    let mut done = Vec::new();
    let result = keys.iter().try_for_each(|key| {
        remove_report(root, key)?;
        done.push(key);
        Ok(())
    });
    // Gelöschte Berichte verlieren ihre Bewertung; eine unlesbare ratings.json hält das Löschen nicht auf (die Liste
    // zeigt sie ohne Bewertungen).
    let _ = update_ratings(root, |ratings| {
        for key in done {
            ratings.remove(key);
        }
    });
    result
}

/// Darf der geprüfte Bericht `key` weg? Weder aus den Baselines noch über einen Link.
fn deletable(root: &Path, key: &str) -> Result<(), String> {
    let path = report_path(root, key)?;
    // über das Dateisystem verglichen: Groß-/Kleinschreibung, Punkte am Ende und Kurznamen zählen nicht
    let baselines = fs::canonicalize(root.join(BASELINE_DIR));
    let folder = path.parent().map(fs::canonicalize);
    if let (Ok(baselines), Some(Ok(folder))) = (baselines, folder)
        && folder.starts_with(baselines)
    {
        return Err(format!(
            "bericht {}: Baselines unter {BASELINE_DIR}/ werden nicht gelöscht",
            go_quoted(key)
        ));
    }
    for file in [&path, &path.with_extension("md")] {
        writeguard::guard().check(root, &file.to_string_lossy())?;
    }
    Ok(())
}

/// Löscht `.json` und `.md` eines geprüften Berichts und den Lauf-Ordner, wenn er danach leer ist.
fn remove_report(root: &Path, key: &str) -> Result<(), String> {
    let path = report_path(root, key)?;
    writeguard::remove(root, &path).map_err(|e| e.to_string())?;
    match writeguard::remove(root, path.with_extension("md")) {
        Err(e) if e.kind() != ErrorKind::NotFound => return Err(e.to_string()),
        _ => {}
    }
    if let Some(folder) = path.parent()
        && folder != root.join("reports")
    {
        let _ = writeguard::remove(root, folder); // nur ein leerer Ordner lässt sich entfernen
    }
    Ok(())
}

/// Prüft alle Pfade mit [`report_key`], bevor etwas geschieht; doppelte zählen einmal.
fn report_keys(root: &Path, rels: &[String]) -> Result<Vec<String>, String> {
    if rels.is_empty() {
        return Err("keine Berichte gewählt".to_owned());
    }
    let mut keys = Vec::new();
    for rel in rels {
        let key = report_key(root, rel)?;
        if !keys.contains(&key) {
            keys.push(key);
        }
    }
    Ok(keys)
}

/// Prüft einen vorhandenen Bericht (`balance-*.json`, `tuning-*.json` unter `reports/`) und liefert seinen Pfad in der
/// Form der Liste (relativ, mit `/`).
fn report_key(root: &Path, rel: &str) -> Result<String, String> {
    let name = rel.rsplit(['/', '\\']).find(|p| !p.is_empty()).unwrap_or_default();
    if !name.starts_with("balance-") && !name.starts_with("tuning-") {
        return Err(format!(
            "bericht {}: nur balance-*.json und tuning-*.json",
            go_quoted(rel)
        ));
    }
    read_report_file(root, rel)?;
    Ok(clean_parts(rel).join("/"))
}
