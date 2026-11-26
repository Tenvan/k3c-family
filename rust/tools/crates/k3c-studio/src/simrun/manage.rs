//! Berichte verwalten (Port von `manage.go`, WZ28.13, B-612): Bewertung je Bericht in `reports/ratings.json`, Löschen
//! einzelner Berichte.
//!
//! Aus RU70.19 vorgezogen (V11) ist nur das Lesen der Bewertungen (für die Liste); Bewerten und Löschen sind Stubs.

use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Mutex;

use serde::Deserialize;

use super::run::lock;
use super::stamp::read_object;

/// Erlaubte Bewertungen; leer = keine.
pub const RATINGS: [&str; 4] = ["gut", "schlecht", "unklar", "referenz"];

/// Liegt neben den Berichten; Schlüssel ist der Berichtspfad relativ zum Checkout (mit `/`).
const RATINGS_FILE: &str = "ratings.json";

#[derive(Default, Deserialize)]
#[serde(default)]
struct RatingsDoc {
    ratings: BTreeMap<String, String>,
}

/// Schützt Lesen und Schreiben von `ratings.json` (Seite und mehrere Aufrufe gleichzeitig).
static RATINGS_LOCK: Mutex<()> = Mutex::new(());

/// Liest `reports/ratings.json`; fehlt die Datei, ist die Liste leer.
pub(super) fn load_ratings(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let _guard = lock(&RATINGS_LOCK);
    match fs::read(root.join("reports").join(RATINGS_FILE)) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(e) => Err(e.to_string()),
        Ok(raw) => read_object::<RatingsDoc>(&raw)
            .map(|doc| doc.ratings)
            .map_err(|e| format!("{RATINGS_FILE} unlesbar: {e}")),
    }
}

/// Setzt die Bewertung eines vorhandenen Berichts; leer löscht sie.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn rate_report(_root: &Path, _rel: &str, _rating: &str) -> Result<(), String> {
    unimplemented!("RU70.19")
}

/// Setzt die Bewertung mehrerer Berichte in einem Schritt (B-662); ein ungültiger Pfad lehnt alle ab.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn rate_reports(_root: &Path, _rels: &[String], _rating: &str) -> Result<(), String> {
    unimplemented!("RU70.19")
}

/// Löscht einen Bericht samt gleichnamigem `.md` und seiner Bewertung.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn delete_report(_root: &Path, _rel: &str) -> Result<(), String> {
    unimplemented!("RU70.19")
}

/// Löscht mehrere Berichte wie [`delete_report`]; erst werden alle Pfade geprüft.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn delete_reports(_root: &Path, _rels: &[String]) -> Result<(), String> {
    unimplemented!("RU70.19")
}
