//! IDs und Kurznamen prüfen und zur Datei auflösen (Go `store.go`): Pfade entstehen nur aus geprüften Eingaben.

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::projects::is_project_id;
use super::store::{STATES, docs_path, entries, quoted};

// Ab B-1000 vierstellig (B-863).
static TICKET_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^B-[0-9]{3,}$").ok());
static SPRINT_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^[A-Z]+[0-9]+$").ok());
static SESSION_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^([A-Z]+[0-9]+)\.([0-9]+[a-z]?)$").ok());
static SLUG: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").ok());
// Trennt doppelt vergebene Nummern (zwei Rechner, gleiche Nummer): B-1287-kurzname wählt genau die Datei.
static TICKET_SLUG: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^(B-[0-9]{3,})-([a-z0-9]+(?:-[a-z0-9]+)*)$").ok());

fn matches(re: &LazyLock<Option<Regex>>, text: &str) -> bool {
    re.as_ref().is_some_and(|re| re.is_match(text))
}

fn group(re: &LazyLock<Option<Regex>>, text: &str) -> Option<String> {
    Some(re.as_ref()?.captures(text)?.get(1)?.as_str().to_owned())
}

/// `B-123` (ohne Kurzname).
pub fn is_ticket_id(id: &str) -> bool {
    matches(&TICKET_ID, id)
}

/// Sprint-ID wie `M9`.
pub fn is_sprint_id(id: &str) -> bool {
    matches(&SPRINT_ID, id)
}

/// Kurzname eines neuen Dokuments: nur a-z, 0-9 und einzelne `-`.
pub fn is_slug(slug: &str) -> bool {
    matches(&SLUG, slug)
}

/// Sprint-ID und Nummer einer Session-ID wie `M9.1`.
pub fn session_parts(id: &str) -> Option<(String, String)> {
    let found = SESSION_ID.as_ref()?.captures(id)?;
    Some((found.get(1)?.as_str().to_owned(), found.get(2)?.as_str().to_owned()))
}

/// Ein aufgelöstes Planungs-Dokument; `rel` ist der Pfad relativ zu docs/ mit `/`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ref {
    /// ticket | sprint | session | projekt
    pub kind: &'static str,
    pub id: String,
    /// Datei (beim Sprint die README).
    pub rel: String,
    /// Sprint-Ordner relativ zu docs/ (Sprint und Session).
    pub dir: String,
    /// Ordner des Sprints oder „archiv“/leer beim Ticket.
    pub state: String,
}

/// Sucht im Ordner den Eintrag mit dem Präfix (Datei mit .md oder Ordner).
pub fn find_entry(dir: &Path, prefix: &str, want_dir: bool) -> Option<String> {
    entries(dir)
        .into_iter()
        .find(|e| e.is_dir == want_dir && e.name.starts_with(prefix) && (want_dir || e.name.ends_with(".md")))
        .map(|e| e.name)
}

pub fn find_sprint(root: &Path, id: &str) -> Option<Ref> {
    STATES.iter().find_map(|state| {
        let name = find_entry(&docs_path(root, &format!("sprints/{state}")), &format!("{id}-"), true)?;
        let dir = format!("sprints/{state}/{name}");
        Some(Ref {
            kind: "sprint",
            id: id.to_owned(),
            rel: format!("{dir}/README.md"),
            dir,
            state: (*state).to_owned(),
        })
    })
}

fn ticket_ref(id: &str, folder: &str, file: &str) -> Ref {
    let rel = if folder.is_empty() {
        format!("backlog/{file}")
    } else {
        format!("backlog/{folder}/{file}")
    };
    Ref {
        kind: "ticket",
        id: id.to_owned(),
        rel,
        state: folder.to_owned(),
        ..Ref::default()
    }
}

fn find_ticket(root: &Path, id: &str) -> Option<Ref> {
    ["", "archiv"].into_iter().find_map(|folder| {
        let file = find_entry(&docs_path(root, &format!("backlog/{folder}")), &format!("{id}-"), false)?;
        Some(ticket_ref(id, folder, &file))
    })
}

/// `B-123-kurzname`: genau die Datei `<id>.md`; die ID der Referenz ist die Nummer.
fn find_ticket_file(root: &Path, id: &str) -> Option<Ref> {
    let number = group(&TICKET_SLUG, id)?;
    ["", "archiv"].into_iter().find_map(|folder| {
        let found = ticket_ref(&number, folder, &format!("{id}.md"));
        docs_path(root, &found.rel).exists().then_some(found)
    })
}

fn find_project_file(root: &Path, id: &str) -> Option<Ref> {
    let file = find_entry(&docs_path(root, "projekte"), &format!("{id}-"), false)?;
    Some(Ref {
        kind: "projekt",
        id: id.to_owned(),
        rel: format!("projekte/{file}"),
        ..Ref::default()
    })
}

fn find_session(root: &Path, id: &str) -> Option<Ref> {
    let sprint = find_sprint(root, &group(&SESSION_ID, id)?)?;
    let file = find_entry(&docs_path(root, &sprint.dir), &format!("{id}-"), false)?;
    Some(Ref {
        kind: "session",
        id: id.to_owned(),
        rel: format!("{}/{file}", sprint.dir),
        ..sprint
    })
}

/// Findet ein vorhandenes Dokument zur ID; jede andere Form ist ein Fehler.
pub fn resolve(root: &Path, id: &str) -> Result<Ref, String> {
    let found = if matches(&TICKET_ID, id) {
        find_ticket(root, id)
    } else if matches(&TICKET_SLUG, id) {
        find_ticket_file(root, id)
    } else if matches(&SPRINT_ID, id) {
        find_sprint(root, id)
    } else if is_project_id(id) {
        find_project_file(root, id)
    } else if matches(&SESSION_ID, id) {
        find_session(root, id)
    } else {
        return Err(format!(
            "ungültige ID {} (B-123, M8, M8.1 oder Projekt GRA)",
            quoted(id)
        ));
    };
    found.ok_or_else(|| format!("{id} nicht gefunden"))
}
