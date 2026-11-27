//! Tickets aus angehakten Befunden (Port von `tickets.go`, B-1224). Die Anlage läuft über die Planungslogik
//! ([`Planner`]): Pflichtfelder Domäne, Typ, Prio, Projekt und Sprint. Nur angehakte Befunde ohne Ticket kommen dran;
//! ein Befund mit `duplicate_of` legt nichts an, das vorhandene Ticket bekommt einen Hinweis. Scheitert ein Befund, steht
//! der Grund an ihm und die anderen laufen weiter.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::findings::{FINDINGS, Finding, read_findings, write_findings};
use super::{Error, Store, lock};
use crate::planning::{self, NewDoc};

/// Zugang zur Planungslogik (Tests setzen Attrappen ein).
pub trait Planner {
    /// Legt ein Ticket an und liefert die Meldung der Planung.
    fn create(&self, slug: &str, title: &str, fields: BTreeMap<String, String>) -> Result<String, String>;
    fn get(&self, id: &str) -> Result<String, String>;
    fn sections(&self, id: &str, bodies: BTreeMap<String, String>) -> Result<(), String>;
}

/// Die Planung der Workbench auf einer Wurzel; sie sperrt ihre Schreibwege selbst.
pub struct RepoPlanner<'a>(pub &'a Path);

impl Planner for RepoPlanner<'_> {
    fn create(&self, slug: &str, title: &str, fields: BTreeMap<String, String>) -> Result<String, String> {
        let input = NewDoc {
            kind: "ticket".to_owned(),
            slug: slug.to_owned(),
            title: title.to_owned(),
            fields,
            ..NewDoc::default()
        };
        planning::create(self.0, &input)
    }

    fn get(&self, id: &str) -> Result<String, String> {
        planning::get(self.0, id)
    }

    fn sections(&self, id: &str, bodies: BTreeMap<String, String>) -> Result<(), String> {
        planning::sections(self.0, id, &bodies).map(|_| ())
    }
}

/// Werte für Befunde ohne eigene Angabe.
#[derive(Clone, Debug, Default)]
pub struct TicketDefaults {
    pub domain: String,
    pub kind: String,
    pub prio: String,
    pub project: String,
    pub sprint: String,
}

/// Zusammenfassung eines Laufs.
#[derive(Clone, Debug, Default)]
pub struct TicketResult {
    /// `f1 → B-1300`
    pub created: Vec<String>,
    /// `f2 → Hinweis in B-1100`
    pub noted: Vec<String>,
    /// `f3: Grund`
    pub failed: Vec<String>,
    /// Nicht angehakt oder schon mit Ticket.
    pub skipped: usize,
    /// Hinweise der Planung auf ähnliche Titel.
    pub similar: Vec<String>,
}

static NEW_TICKET: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?m)^(B-[0-9]+) angelegt").ok());
static SLUG_BAD: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("[^a-z0-9]+").ok());

/// Kurzname aus dem Titel (a-z, 0-9, `-`; höchstens 40 Zeichen).
fn slug_of(title: &str) -> String {
    let plain = [("ä", "ae"), ("ö", "oe"), ("ü", "ue"), ("ß", "ss")]
        .iter()
        .fold(title.to_lowercase(), |text, (from, to)| text.replace(from, to));
    let mut slug = SLUG_BAD
        .as_ref()
        .map_or_else(String::new, |re| re.replace_all(&plain, "-").into_owned());
    slug.truncate(40); // nur noch ASCII
    match slug.trim_matches('-') {
        "" => "befund".to_owned(),
        trimmed => trimmed.to_owned(),
    }
}

/// Der erste nichtleere Wert, ohne Rand.
fn pick(values: &[&str]) -> String {
    let found = values.iter().map(|v| v.trim()).find(|v| !v.is_empty());
    found.unwrap_or_default().to_owned()
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Titel des Befunds, sonst die ersten Worte des Zitats.
fn title_of(finding: &Finding) -> String {
    let title = finding.title.trim();
    if !title.is_empty() {
        return title.to_owned();
    }
    let mut quote = one_line(&finding.quote);
    if quote.chars().count() > 70 {
        quote = quote.chars().take(70).collect::<String>() + " …";
    }
    format!("Spieltest: {quote}")
}

/// Abschnitte des neuen Tickets: Zitat, Zeitpunkt, Bild und Spielkontext.
fn ticket_text(session: &str, finding: &Finding) -> BTreeMap<String, String> {
    let quote = finding.quote.trim().replace('\n', "\n> ");
    let mut text = format!(
        "Aus dem Spieltest {session}, Befund {} (Zeitpunkt {} ms Unix).\n\n> {quote}\n",
        finding.id, finding.ts
    );
    // Schreiben in eine Zeichenkette scheitert nicht.
    if !finding.context.is_empty() {
        let _ = write!(text, "\n- **Spielkontext:** {}\n", finding.context);
    }
    if !finding.frame.is_empty() {
        let frame = finding.frame.split('/').filter(|part| !matches!(*part, "" | "."));
        let path: Vec<&str> = ["playtests", session].into_iter().chain(frame).collect();
        let _ = writeln!(text, "- **Bild:** `{}` (lokal, nicht eingecheckt)", path.join("/"));
    }
    BTreeMap::from([("Ausgangslage".to_owned(), text)])
}

/// Ein angelegtes Ticket; `error`, wenn nur das Füllen der Abschnitte scheiterte.
struct Created {
    id: String,
    similar: Vec<String>,
    error: Option<String>,
}

fn create_one(
    session: &str,
    finding: &Finding,
    def: &TicketDefaults,
    planner: &dyn Planner,
) -> Result<Created, String> {
    let fields = [
        ("Domäne", pick(&[&finding.domain, &def.domain])),
        ("Typ", pick(&[&finding.kind, &def.kind])),
        ("Prio", pick(&[&finding.prio, &def.prio, "mittel"])),
        ("Projekt", pick(&[&finding.project, &def.project])),
        ("Sprint", pick(&[&finding.sprint, &def.sprint])),
    ];
    let missing: Vec<&str> = fields.iter().filter(|(_, v)| v.is_empty()).map(|(k, _)| *k).collect();
    if !missing.is_empty() {
        let list = missing.join(", ");
        return Err(format!("{list} fehlt (am Befund oder in der Auswahl setzen)"));
    }
    let fields = fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
    let title = title_of(finding);
    let out = planner.create(&slug_of(&title), &title, fields)?;
    let Some(id) = NEW_TICKET
        .as_ref()
        .and_then(|re| re.captures(&out))
        .and_then(|c| c.get(1))
    else {
        return Err(format!("Ticket-ID nicht erkannt: {out}"));
    };
    let id = id.as_str().to_owned();
    let similar = out
        .split('\n')
        .filter(|line| line.to_lowercase().contains("ähnlich"))
        .map(str::to_owned)
        .collect();
    let error = planner
        .sections(&id, ticket_text(session, finding))
        .err()
        .map(|e| format!("{id} angelegt, Abschnitt nicht gefüllt: {e}"));
    Ok(Created { id, similar, error })
}

/// Trägt einen Hinweis in die Notizen des vorhandenen Tickets ein (einmal je Befund).
fn note_duplicate(session: &str, finding: &Finding, planner: &dyn Planner) -> Result<String, String> {
    let ticket = &finding.duplicate_of;
    let markdown = planner.get(ticket).map_err(|e| format!("duplicate_of {ticket}: {e}"))?;
    let tag = format!("Spieltest {session}, Befund {}", finding.id);
    let body = section_body(&markdown, "Notizen");
    if body.contains(&tag) {
        return Ok(format!("Hinweis schon in {ticket}"));
    }
    let mut line = format!("- {tag}: „{}“", one_line(&finding.quote));
    if !finding.context.is_empty() {
        line = format!("{line} ({})", finding.context);
    }
    let body = if matches!(body.trim(), "" | "–" | "-") {
        line
    } else {
        format!("{}\n{line}", body.trim_end_matches(['\n', ' ']))
    };
    planner
        .sections(ticket, BTreeMap::from([("Notizen".to_owned(), body)]))
        .map_err(|e| format!("Hinweis in {ticket}: {e}"))?;
    Ok(format!("Hinweis in {ticket}"))
}

/// Text unter `## name` bis zur nächsten `## `-Überschrift (leer, wenn es sie nicht gibt).
fn section_body(markdown: &str, name: &str) -> String {
    let markdown = markdown.replace("\r\n", "\n");
    let mut out = Vec::new();
    let mut inside = false;
    for line in markdown.split('\n') {
        if let Some(heading) = line.strip_prefix("## ") {
            if inside {
                break;
            }
            inside = heading.trim() == name;
        } else if inside {
            out.push(line);
        }
    }
    out.join("\n").trim().to_owned()
}

/// Bearbeitet einen angehakten Befund und trägt Ticket bzw. Fehler an ihm ein.
fn process(session: &str, finding: &mut Finding, def: &TicketDefaults, planner: &dyn Planner, res: &mut TicketResult) {
    finding.error = String::new();
    let id = finding.id.clone();
    let error = if finding.duplicate_of.is_empty() {
        match create_one(session, finding, def, planner) {
            Ok(created) => {
                res.created.push(format!("{id} → {}", created.id));
                res.similar.extend(created.similar);
                finding.ticket = created.id;
                created.error
            }
            Err(e) => Some(e),
        }
    } else {
        match note_duplicate(session, finding, planner) {
            Ok(message) => {
                res.noted.push(format!("{id} → {message}"));
                None
            }
            Err(e) => Some(e),
        }
    };
    if let Some(error) = error {
        res.failed.push(format!("{id}: {error}"));
        finding.error = error;
    }
}

impl Store {
    /// Legt Tickets für alle angehakten Befunde ohne Ticket an und trägt die IDs in `findings.json` ein. Blockiert.
    pub fn create_tickets(&self, id: &str, def: &TicketDefaults, planner: &dyn Planner) -> Result<TicketResult, Error> {
        let mut result = TicketResult::default();
        let dir = self.session_dir(id)?;
        let _writing = lock(&FINDINGS);
        let mut list = read_findings(&dir)?;
        for i in 0..list.len() {
            let Some(finding) = list.get_mut(i) else { break };
            if !finding.selected || !finding.ticket.is_empty() {
                result.skipped += 1;
                continue;
            }
            process(id, finding, def, planner, &mut result);
            write_findings(&dir, &list)?;
        }
        Ok(result)
    }
}

#[cfg(test)]
#[path = "tickets_tests.rs"]
mod tests;
