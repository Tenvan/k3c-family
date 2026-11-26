//! Credit-Zeilen importierter Kandidaten (B-165, B-298) und die Lizenz-Bestätigung durch 🧑.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::{DateTime, FixedOffset};

use crate::error::{Error, Result};
use crate::fsutil;
use crate::register::{KIND_GRAPHIC, KIND_SOUND, load, lock, write_atomic};
use crate::research::{Hit, LICENSE_KNOWN, load_research};

/// Lizenz-Status, den nur 🧑 in der Oberfläche setzt (B-298/AC-05), nie ein Agent.
// Steht in Go in `import.go`; hier, bei den Credit-Zeilen.
pub const LICENSE_CONFIRMED: &str = "bestaetigt";

/// Abschnitt der Credit-Datei für Importe; je Datei eine Zeile (Ordner = `kandidaten/<datei>`).
pub(crate) const IMPORT_SECTION: &str = "## Importe (k3c-workbench)";

/// Tabelle des Import-Abschnitts; Ordner nennt die Datei, Status die Lizenz-Prüfung.
const CREDIT_HEADER: &str = "| Ordner | Pack | Urheber | Lizenz | Quelle | Status |\n|---|---|---|---|---|---|\n";

const IMPORT_INTRO: &str = "Mit k3c-workbench importierte Kandidaten (B-298): je Datei eine Zeile. „Lizenz prüfen“ gilt, \
     bis 🧑 die Lizenz in k3c-workbench bestätigt.\n\n";

/// Credit-Datei je Art, relativ zu `public/`.
pub(crate) fn credit_file(kind: &str) -> &'static str {
    match kind {
        KIND_GRAPHIC => "grafik/CREDITS.md",
        KIND_SOUND => "audio/README.md",
        _ => "",
    }
}

fn credit_path(root: &Path, kind: &str) -> PathBuf {
    fsutil::join_below(&root.join("public"), credit_file(kind))
}

/// Macht Text tabellentauglich (kein `|`, kein Zeilenumbruch).
fn cell(s: &str, fallback: &str) -> String {
    let s = s.replace('|', "/").replace(['\r', '\n'], " ");
    match s.trim() {
        "" => fallback.to_string(),
        text => text.to_string(),
    }
}

fn confirmed_status(now: DateTime<FixedOffset>) -> String {
    format!("bestätigt 🧑 {}", now.format("%Y-%m-%d"))
}

fn credit_status(h: &Hit, now: DateTime<FixedOffset>) -> String {
    match h.license_status.as_str() {
        LICENSE_CONFIRMED => confirmed_status(now),
        LICENSE_KNOWN => format!("Lizenz erkannt ({})", cell(&h.source, "Quelle")),
        _ => "Lizenz prüfen".to_string(),
    }
}

/// Text der Credit-Datei mit LF. Strenger als Go: Ungültiges UTF-8 (z. B. Windows-1252) wird abgelehnt, weil das
/// Zurückschreiben sonst die Umlaute der ganzen Datei zerstörte.
fn utf8_text(file: &Path, bytes: Vec<u8>) -> Result<String> {
    match String::from_utf8(bytes) {
        Ok(text) => Ok(text.replace("\r\n", "\n")),
        Err(_) => Err(Error::Msg(format!(
            "{} ist kein gültiges UTF-8, nichts geschrieben",
            file.display()
        ))),
    }
}

/// Trägt die Datei sofort in die Credit-Datei ein (B-165), im Abschnitt „Importe (k3c-workbench)“.
pub(crate) fn add_credit(root: &Path, kind: &str, rel: &str, h: &Hit, now: DateTime<FixedOffset>) -> Result<()> {
    let file = credit_path(root, kind);
    let bytes = match fs::read(&file) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(Error::Io(e)),
    };
    let mut text = utf8_text(&file, bytes)?;
    if !text.contains(IMPORT_SECTION) {
        let head = text.trim_end_matches('\n');
        text = format!("{head}\n\n{IMPORT_SECTION}\n\n{IMPORT_INTRO}{CREDIT_HEADER}");
    }
    let row = format!(
        "| {} | {} | {} | {} | {} | {} |\n",
        credit_folder(kind, rel),
        cell(&h.title, "ohne Titel"),
        cell(&h.artist, "unbekannt"),
        cell(&h.license, "unbekannt"),
        cell(&h.link, "–"),
        credit_status(h, now)
    );
    let text = format!("{}\n{row}", text.trim_end_matches('\n'));
    write_atomic(&file, text.as_bytes())
}

/// Pfad relativ zum Ordner der Credit-Datei (`grafik/kandidaten/x.png` → `kandidaten/x.png`).
pub(crate) fn credit_folder(kind: &str, rel: &str) -> String {
    let dir = credit_file(kind).rsplit_once('/').map_or(".", |(dir, _)| dir);
    rel.strip_prefix(&format!("{dir}/")).unwrap_or(rel).to_string()
}

/// Vermerkt am Treffer die importierte Datei.
pub(crate) fn mark_imported(root: &Path, slot_id: &str, hit_id: &str, rel: &str) -> Result<()> {
    update_hit(root, slot_id, hit_id, |h| h.file = rel.to_string())
}

pub(crate) fn update_hit(root: &Path, slot_id: &str, hit_id: &str, change: impl FnOnce(&mut Hit)) -> Result<()> {
    let _guard = lock();
    let mut research = load_research(root)?;
    let hit = research
        .hits
        .get_mut(slot_id)
        .and_then(|list| list.iter_mut().find(|h| h.id == hit_id));
    let Some(hit) = hit else {
        return Err(Error::Msg(format!("treffer {hit_id:?} zu {slot_id} nicht gefunden")));
    };
    change(hit);
    research.save(root)
}

/// Bestätigt die Lizenz eines Treffers (nur 🧑 über die Oberfläche): Status am Treffer und in der Credit-Zeile.
pub fn confirm_license(root: &Path, slot_id: &str, hit_id: &str, now: DateTime<FixedOffset>) -> Result<()> {
    let register = load(root)?;
    let slot = register
        .find(slot_id)
        .ok_or_else(|| Error::Msg(format!("unbekannter Asset-Slot {slot_id:?}")))?;
    let mut file = String::new();
    update_hit(root, slot_id, hit_id, |h| {
        h.license_status = LICENSE_CONFIRMED.to_string();
        file = h.file.clone();
    })?;
    if file.is_empty() {
        return Ok(()); // noch nicht importiert: die Credit-Zeile entsteht beim Import mit „bestätigt“
    }
    set_credit_status(
        root,
        &slot.kind,
        &credit_folder(&slot.kind, &file),
        &confirmed_status(now),
    )
}

pub(crate) fn set_credit_status(root: &Path, kind: &str, folder: &str, status: &str) -> Result<()> {
    let file = credit_path(root, kind);
    let text = utf8_text(&file, fs::read(&file)?)?;
    let mut lines: Vec<String> = text.split('\n').map(str::to_string).collect();
    let prefix = format!("| {folder} |");
    let Some(line) = lines.iter_mut().find(|l| l.starts_with(&prefix)) else {
        return Err(Error::Msg(format!(
            "credit-zeile {folder} fehlt in {}",
            credit_file(kind)
        )));
    };
    let status = format!(" {status} ");
    let mut cells: Vec<&str> = line.trim_matches('|').split('|').collect();
    if let Some(last) = cells.last_mut() {
        *last = &status;
    }
    let joined = format!("|{}|", cells.join("|"));
    *line = joined;
    write_atomic(&file, lines.join("\n").as_bytes())
}

#[cfg(test)]
#[path = "credits_tests.rs"]
mod tests;
