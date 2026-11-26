//! Tickets nach `docs/vorlagen/ticket.md` (B-595): Nummer, Vorlage, Index-Zeile. `plan_create` der Workbench nutzt
//! dieselben Bausteine, das Studio ruft [`create`] direkt (offline, ohne Workbench).

mod draft;
mod files;
mod rename;

#[cfg(test)]
mod draft_tests;
#[cfg(test)]
mod ticket_tests;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::gojson::null_default;
use draft::{set_section, situation};
use files::{read_text, write_atomic};

pub use draft::{OPTIONAL, Row, add_to_index, defaults, draft, index_row, slug};
pub use files::{highest, local_highest, recent};
pub use rename::rename;

/// Wartezeit auf die Backlog-Sperre.
const LOCK_WAIT: Duration = Duration::from_secs(10);
/// Alter, ab dem eine Sperrdatei als verwaist gilt.
const LOCK_STALE: Duration = Duration::from_secs(30);

/// Auswahlliste Typ: die Teilmenge der Vorlage, die ein Mensch im Dialog wählt.
pub const TYPES: &[&str] = &["Idee", "Problem", "Frage"];
/// Auswahlliste Prio.
pub const PRIOS: &[&str] = &["hoch", "mittel", "niedrig"];
/// Auswahlliste Domäne.
pub const DOMAINS: &[&str] = &["REG", "SIM", "SRV", "CLI", "PLAT", "INF", "DEV"];

/// Die Seite, von der das Ticket kommt; leere Felder fehlen im Ticket.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Context {
    #[serde(deserialize_with = "null_default")]
    pub app: String,
    #[serde(deserialize_with = "null_default")]
    pub seite: String,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub auswahl: String,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub version: String,
    #[serde(deserialize_with = "null_default")]
    pub zeit: String,
}

/// Eingabe des Ticket-Dialogs. Projekt ist optional (Kürzel aus `docs/projekte/`, sonst `–`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Input {
    #[serde(deserialize_with = "null_default")]
    pub titel: String,
    #[serde(deserialize_with = "null_default")]
    pub beschreibung: String,
    #[serde(deserialize_with = "null_default")]
    pub typ: String,
    #[serde(deserialize_with = "null_default")]
    pub prio: String,
    #[serde(deserialize_with = "null_default")]
    pub domaene: String,
    #[serde(deserialize_with = "null_default", skip_serializing_if = "String::is_empty")]
    pub projekt: String,
    #[serde(deserialize_with = "null_default")]
    pub kontext: Context,
}

/// Angelegtes Ticket; Pfad relativ zur Repo-Wurzel mit `/`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Created {
    #[serde(deserialize_with = "null_default")]
    pub id: String,
    #[serde(deserialize_with = "null_default")]
    pub pfad: String,
    #[serde(deserialize_with = "null_default")]
    pub titel: String,
}

fn context(text: &str, e: impl std::fmt::Display) -> Error {
    Error::Msg(format!("{text}: {e}"))
}

/// Legt das Ticket mit der nächsten freien Nummer (Backlog und Archiv) an und trägt es in den Index ein.
pub fn create(root: &Path, mut input: Input) -> Result<Created> {
    input.titel = input.titel.trim().to_string();
    validate(root, &input)?;
    let _lock = lock_backlog(root)?;
    let top = local_highest(root).map_err(|e| context("backlog nicht lesbar", e))?;
    let id = format!("B-{:03}", top.wrapping_add(1));
    let (tpl, tpl_crlf) =
        read_text(&root.join("docs").join("vorlagen").join("ticket.md")).map_err(|e| context("vorlage fehlt", e))?;
    let text = fill(&tpl, &id, &input)?;
    let index_path = root.join("docs").join("backlog").join("README.md");
    let (index, index_crlf) = read_text(&index_path).map_err(|e| context("index nicht lesbar", e))?;
    let file = format!("{id}-{}.md", slug(&input.titel));
    let row = Row {
        nr: id.clone(),
        link: file.clone(),
        domain: input.domaene.clone(),
        typ: input.typ.clone(),
        prio: input.prio.clone(),
        status: "offen".to_string(),
        sprint: "–".to_string(),
        title: input.titel.clone(),
    };
    let index = add_to_index(&index, &row)?;
    store(root, &file, &text, tpl_crlf, &index, index_crlf)?;
    Ok(Created {
        id,
        pfad: format!("docs/backlog/{file}"),
        titel: input.titel,
    })
}

/// Sperrdatei des Backlogs; beim Fallenlassen wird nur die eigene gelöscht.
struct BacklogLock {
    path: PathBuf,
    token: String,
}

impl Drop for BacklogLock {
    fn drop(&mut self) {
        if fs::read(&self.path).is_ok_and(|b| b == self.token.as_bytes()) {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// Sperrt das Anlegen über Prozesse hinweg (B-615): exklusives Anlegen einer Sperrdatei im Backlog, solange Nummer
/// vergeben und Ticket samt Index geschrieben werden. Eine Sperre älter als [`LOCK_STALE`] gilt als verwaist.
fn lock_backlog(root: &Path) -> Result<BacklogLock> {
    let path = root.join("docs").join("backlog").join(".ticket.lock");
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let token = format!("{}-{nanos}", std::process::id());
    let deadline = Instant::now() + LOCK_WAIT;
    loop {
        let err = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                let _ = file.write_all(token.as_bytes());
                return Ok(BacklogLock { path, token });
            }
            Err(e) => e,
        };
        // Windows meldet eine gerade zum Löschen markierte Datei als PermissionDenied: wie belegt behandeln.
        if !matches!(
            err.kind(),
            io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied
        ) {
            return Err(context("backlog nicht beschreibbar", err));
        }
        if is_stale(&path) {
            // Übernahme per Umbenennen: nur ein Prozess gewinnt, eine frische Sperre eines anderen bleibt unberührt.
            let mut grave = path.clone().into_os_string();
            grave.push(format!(".{token}"));
            if fs::rename(&path, &grave).is_ok() {
                let _ = fs::remove_file(&grave);
            }
            continue;
        }
        if Instant::now() > deadline {
            return Err(Error::Msg("backlog gesperrt, bitte erneut senden".to_string()));
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn is_stale(path: &Path) -> bool {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.elapsed().ok())
        .is_some_and(|age| age > LOCK_STALE)
}

/// Füllt die Vorlage: Kopf wie `plan_create`, Felder der Eingabe, Ausgangslage mit Beschreibung und Kontext.
fn fill(tpl: &str, id: &str, input: &Input) -> Result<String> {
    let mut values = defaults(chrono::Local::now().fixed_offset());
    for (key, value) in [("Domäne", &input.domaene), ("Typ", &input.typ), ("Prio", &input.prio)] {
        values.insert(key.to_string(), value.clone());
    }
    if !input.projekt.is_empty() {
        values.insert("Projekt".to_string(), input.projekt.clone());
    }
    let text = draft(tpl, id, &input.titel, &values)?;
    set_section(&text, "Ausgangslage", &situation(input))
}

/// Schreibt erst das Ticket, dann den Index; scheitert der Index, verschwindet das Ticket wieder.
fn store(root: &Path, file: &str, text: &str, text_crlf: bool, index: &str, index_crlf: bool) -> Result<()> {
    let dir = root.join("docs").join("backlog");
    let ticket = dir.join(file);
    if ticket.exists() {
        return Err(Error::Msg(format!("{file} gibt es schon, bitte erneut senden")));
    }
    write_atomic(&ticket, text, text_crlf).map_err(|e| context("backlog nicht beschreibbar", e))?;
    if let Err(e) = write_atomic(&dir.join("README.md"), index, index_crlf) {
        let mut message = format!("index nicht beschreibbar: {e}");
        if let Err(removed) = crate::fsutil::remove(&ticket) {
            message = format!("{message}\n{removed}");
        }
        return Err(Error::Msg(message));
    }
    Ok(())
}

/// Prüft die Eingabe wie [`create`], ohne zu schreiben (Vorabprüfung der Chat-Aktion `ticket`, B-679).
pub fn validate(root: &Path, input: &Input) -> Result<()> {
    if input.titel.is_empty() || input.titel.contains(['|', '\r', '\n']) {
        return Err(Error::Msg(
            "titel fehlt oder enthält `|` bzw. einen Zeilenumbruch".to_string(),
        ));
    }
    for (name, value, list) in [
        ("Typ", &input.typ, TYPES),
        ("Prio", &input.prio, PRIOS),
        ("Domäne", &input.domaene, DOMAINS),
    ] {
        if !list.contains(&value.as_str()) {
            return Err(Error::Msg(format!("{name} {value:?} ungültig ({})", list.join(", "))));
        }
    }
    if input.projekt.is_empty() || known_project(root, &input.projekt) {
        return Ok(());
    }
    Err(Error::Msg(format!("unbekanntes Projekt {:?}", input.projekt)))
}

/// Ein Kürzel aus 2 bis 4 Großbuchstaben mit einer Datei `docs/projekte/<Kürzel>-*.md`.
fn known_project(root: &Path, project: &str) -> bool {
    if !(2..=4).contains(&project.len()) || !project.bytes().all(|b| b.is_ascii_uppercase()) {
        return false;
    }
    let prefix = format!("{project}-");
    fs::read_dir(root.join("docs").join("projekte")).is_ok_and(|entries| {
        entries.flatten().any(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.strip_prefix(&prefix).is_some_and(|rest| rest.ends_with(".md"))
        })
    })
}
