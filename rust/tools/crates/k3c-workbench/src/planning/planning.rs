//! Modell und Parser: Sprint-READMEs, Session-Dateien, Ticket-Köpfe, die lesbaren Dokumente.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use super::priority::rank;
use super::projects::{Project, load_projects};
use super::relations::{FIELD_DUP, FIELD_REPLACE, rel_ids};
use super::store::{read_dir_sorted, read_lossy};
use super::ticketid::ticket_cmp;
use super::worktree::{mark_worktrees, worktrees};

/// Eine Zeile der Session-Tabelle eines Sprints. Ein Entwurf hat nur Stichpunkte: Status „entwurf“.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Session {
    pub nr: String,
    pub typ: String,
    pub agent: String,
    pub status: String,
    pub titel: String,
    /// Spalte „Datei“ der Tabelle, relativ zum Sprint-Ordner.
    #[serde(skip)]
    pub file: String,
    /// Inhalt der Session-Datei (Markdown) für das Detail-Panel.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    /// Session-IDs aus „Abhängig von“.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deps: Vec<String>,
    /// Feld „Umgebung“ der Session-Datei: offline | live | ?
    #[serde(skip_serializing_if = "String::is_empty")]
    pub env: String,
    /// Feld „Domäne“ der Session-Datei.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub domain: String,
}

/// Kopfzeile und Session-Tabelle einer Sprint-README.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Sprint {
    pub id: String,
    pub title: String,
    pub domain: String,
    /// Feld „Projekt“: Kürzel oder –.
    pub project: String,
    /// Sprints, auf deren Sessions seine Sessions warten.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deps: Vec<String>,
    /// aktiv | geplant | erledigt
    pub status: String,
    pub reife: String,
    pub spec: String,
    pub tickets: Vec<String>,
    pub sessions: Vec<Session>,
    /// Branch eines Worktrees, der gerade an diesem Sprint arbeitet.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub worktree: String,
    /// Alle solchen Branches (mehrere Agenten, `claude/…`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub worktrees: Vec<String>,
}

/// Kopf einer Backlog-Datei.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Ticket {
    pub nr: String,
    pub title: String,
    pub domain: String,
    pub typ: String,
    pub prio: String,
    /// offline | live | ?
    pub env: String,
    pub status: String,
    pub sprint: String,
    /// Feld „Projekt“: Kürzel oder –.
    pub project: String,
    pub spec: String,
    /// Relationen (B-696), leer bei `–` oder fehlendem Feld.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub duplicate: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<String>,
}

/// Stand für die Oberfläche.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Data {
    /// Aktive nach Rang (ABN danach), dann ruhende, dann erledigte.
    pub projects: Vec<Project>,
    pub sprints: Vec<Sprint>,
    pub tickets: Vec<Ticket>,
    /// Zahl der erledigten Sprints.
    pub done: usize,
}

/// Lesbare Dokumente (Name der Oberfläche, Datei unter docs/) in der Reihenfolge des Umschalters; das Glossar
/// erscheint nur, wenn es die Datei gibt (B-211).
pub const DOCS: [(&str, &str); 3] = [
    ("plan", "plan-weiterentwicklung.md"),
    ("fragen", "fragenkatalog.md"),
    ("glossar", "glossar.md"),
];

// Muster wie Go: `\s`, `\d` und `\b` meinen dort nur ASCII.
static FIELD: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^- \*\*([^:*]+):\*\*[\t\n\f\r ]*(.*)$").ok());
static SESSION: LazyLock<Option<Regex>> = LazyLock::new(|| {
    let s = r"[\t\n\f\r ]*";
    let cell = format!(r"\|{s}([^|]*?){s}");
    Regex::new(&format!(
        r"^\|{s}([A-Za-z0-9]+\.[0-9]+[a-z]?){s}\|{s}`?([^|`]*?)`?{s}{cell}{cell}{cell}\|"
    ))
    .ok()
});
static BULLET: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^- ([A-Za-z0-9]+\.[0-9]+[a-z]?)[\t\n\f\r ]+(.*)$").ok());
static TICKET_FILE: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^B-[0-9]+-.*\.md$").ok());

/// Dateiname eines Tickets (`B-123-kurzname.md`).
pub fn is_ticket_file(name: &str) -> bool {
    TICKET_FILE.as_ref().is_some_and(|re| re.is_match(name))
}

/// Feldzeile `- **Name:** Wert` als (Name, Wert).
pub fn field_line(line: &str) -> Option<(&str, &str)> {
    let found = FIELD.as_ref()?.captures(line)?;
    Some((found.get(1)?.as_str(), found.get(2)?.as_str()))
}

/// Text an `\n` getrennt, CRLF zählt als LF.
fn lf_lines(text: &str) -> Vec<String> {
    text.replace("\r\n", "\n").split('\n').map(str::to_owned).collect()
}

/// Zerlegt `# SP11 · SRV · Titel` oder `# B-090 · Titel` am Mittelpunkt; ohne Überschrift leer.
pub fn header<S: AsRef<str>>(lines: &[S]) -> Vec<String> {
    lines
        .iter()
        .find_map(|l| l.as_ref().strip_prefix("# "))
        .map(|rest| rest.split(" · ").map(str::to_owned).collect())
        .unwrap_or_default()
}

/// Alle Feldzeilen; eine spätere gleichen Namens gewinnt.
pub fn fields<S: AsRef<str>>(lines: &[S]) -> HashMap<String, String> {
    lines
        .iter()
        .filter_map(|l| field_line(l.as_ref()))
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect()
}

pub(super) fn take(f: &HashMap<String, String>, name: &str) -> String {
    f.get(name).cloned().unwrap_or_default()
}

/// Liest eine Sprint-README; `dir` (Ordnername) ersetzt eine fehlende Kopfzeile.
pub fn parse_sprint(text: &str, dir: &str, status: &str) -> Sprint {
    let lines = lf_lines(text);
    let f = fields(&lines);
    let mut sprint = Sprint {
        id: dir.split('-').next().unwrap_or_default().to_owned(),
        title: dir.to_owned(),
        domain: take(&f, "Domäne"),
        project: take(&f, "Projekt"),
        status: status.to_owned(),
        reife: take(&f, "Reife"),
        spec: take(&f, "Spec"),
        tickets: take(&f, "Tickets")
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(str::to_owned)
            .collect(),
        sessions: parse_sessions(&lines),
        ..Sprint::default()
    };
    if let [id, domain, title @ ..] = header(&lines).as_slice()
        && !title.is_empty()
    {
        sprint.id.clone_from(id);
        sprint.domain.clone_from(domain);
        sprint.title = title.join(" · ");
    }
    sprint
}

/// Sagt, ob die Datei-Zelle einer Session-Zeile eine Datei nennt; Platzhalter wie `–` oder `– (Entwurf)` (B-436) sind
/// keine Datei.
pub fn is_session_file(file: &str) -> bool {
    // `:` auch: `join` ersetzte bei `C:x.md` den ganzen Pfad durch den laufwerksrelativen (Go hängt nur an).
    file.ends_with(".md") && !file.contains(['/', '\\', ':'])
}

fn table_session(line: &str) -> Option<Session> {
    let m = SESSION.as_ref()?.captures(line)?;
    let cell = |i: usize| m.get(i).map_or("", |g| g.as_str()).to_owned();
    Some(Session {
        nr: cell(1),
        file: cell(2),
        typ: cell(3),
        agent: cell(4),
        status: cell(5),
        ..Session::default()
    })
}

fn bullet_session(line: &str) -> Option<Session> {
    let m = BULLET.as_ref()?.captures(line)?;
    Some(Session {
        nr: m.get(1)?.as_str().to_owned(),
        status: "entwurf".to_owned(),
        titel: m.get(2)?.as_str().to_owned(),
        ..Session::default()
    })
}

/// Liest den Abschnitt „## Sessions“: Tabellenzeilen, dazu Stichpunkte (Entwurf), deren Nr. nicht schon in der Tabelle
/// steht. Erledigte Sprints führen beides nebeneinander; sonst zählte jede Session doppelt.
fn parse_sessions(lines: &[String]) -> Vec<Session> {
    let (mut table, mut bullets) = (Vec::new(), Vec::new());
    let mut in_sessions = false;
    for line in lines {
        if line.starts_with("## ") {
            in_sessions = line == "## Sessions";
        } else if !in_sessions {
        } else if let Some(s) = table_session(line) {
            table.push(s);
        } else if let Some(s) = bullet_session(line) {
            bullets.push(s);
        }
    }
    let known: HashSet<String> = table.iter().map(|s| s.nr.clone()).collect();
    table.extend(bullets.into_iter().filter(|s| !known.contains(&s.nr)));
    table
}

/// Liest den Kopf einer Backlog-Datei.
pub fn parse_ticket(text: &str) -> Ticket {
    let lines = lf_lines(text);
    let f = fields(&lines);
    let mut ticket = Ticket {
        domain: take(&f, "Domäne"),
        typ: take(&f, "Typ"),
        prio: take(&f, "Prio"),
        env: take(&f, "Umgebung"),
        status: take(&f, "Status"),
        sprint: take(&f, "Sprint"),
        project: take(&f, "Projekt"),
        spec: take(&f, "Spec"),
        duplicate: rel_ids(&take(&f, FIELD_DUP)).into_iter().next().unwrap_or_default(),
        replaces: rel_ids(&take(&f, FIELD_REPLACE)),
        ..Ticket::default()
    };
    if let [nr, title @ ..] = header(&lines).as_slice()
        && !title.is_empty()
    {
        ticket.nr.clone_from(nr);
        ticket.title = title.join(" · ");
    }
    ticket
}

/// Liest die README eines Sprint-Ordners und die Session-Dateien aus der Spalte „Datei“ (nur Dateinamen im Ordner).
pub(super) fn load_sprint(dir: &Path, name: &str, status: &str) -> Option<Sprint> {
    let readme = read_lossy(&dir.join("README.md")).ok()?;
    let mut sprint = parse_sprint(&readme, name, status);
    for session in &mut sprint.sessions {
        if !is_session_file(&session.file) {
            continue;
        }
        let Ok(text) = read_lossy(&dir.join(&session.file)) else {
            continue;
        };
        // B-402: Titel aus `# WZ2.1 · Titel`; ein Stichpunkt-Titel bleibt.
        if let [_, title @ ..] = header(&lf_lines(&text)).as_slice()
            && !title.is_empty()
            && session.titel.is_empty()
        {
            session.titel = title.join(" · ");
        }
        session.text = text;
    }
    Some(sprint)
}

fn load_sprints(root: &Path, data: &mut Data) -> Result<(), String> {
    for status in ["aktiv", "geplant", "erledigt"] {
        let dir = root.join("docs").join("sprints").join(status);
        let entries = match read_dir_sorted(&dir) {
            Ok(entries) => entries,
            Err(_) if status == "erledigt" => continue, // ohne erledigte Sprints geht es auch
            Err(e) => return Err(format!("sprints nicht lesbar: {e}")),
        };
        for entry in entries.iter().filter(|e| e.is_dir) {
            if status == "erledigt" {
                data.done += 1;
            }
            if let Some(sprint) = load_sprint(&dir.join(&entry.name), &entry.name, status) {
                data.sprints.push(sprint);
            }
        }
    }
    Ok(())
}

/// Liest die Planung unter `root/docs`. Eine unlesbare Datei überspringt nur ihren Eintrag; fehlt docs/sprints oder
/// docs/backlog ganz, kommt ein Fehler (falsche Wurzel).
pub fn load(root: &Path) -> Result<Data, String> {
    let mut data = Data {
        projects: load_projects(root),
        ..Data::default()
    };
    load_sprints(root, &mut data)?;
    mark_worktrees(&mut data.sprints, &worktrees(root));
    let backlog = root.join("docs").join("backlog");
    let files = read_dir_sorted(&backlog).map_err(|e| format!("backlog nicht lesbar: {e}"))?;
    for file in files.iter().filter(|f| !f.is_dir && is_ticket_file(&f.name)) {
        if let Ok(text) = read_lossy(&backlog.join(&file.name)) {
            data.tickets.push(parse_ticket(&text));
        }
    }
    data.tickets.sort_by(|a, b| ticket_cmp(&a.nr, &b.nr));
    rank(&mut data);
    Ok(data)
}

/// Liefert eines der Dokumente aus [`DOCS`].
pub fn doc(root: &Path, name: &str) -> Result<String, String> {
    let Some((_, file)) = DOCS.iter().find(|(n, _)| *n == name) else {
        return Err(format!("unbekanntes Dokument {}", super::store::quoted(name)));
    };
    let path = root.join("docs").join(file);
    read_lossy(&path).map_err(|e| format!("open {}: {e}", path.display()))
}

/// Nennt die Dokumente aus [`DOCS`], deren Datei existiert.
pub fn available(root: &Path) -> Vec<String> {
    DOCS.iter()
        .filter(|(_, file)| root.join("docs").join(file).exists())
        .map(|(name, _)| (*name).to_owned())
        .collect()
}
