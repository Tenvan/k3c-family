//! Die beiden Tabellen einer Zusammenfassung (Go `digest.go`): Kriterien mit Status und Tickets des Sprints.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::digest::{section_text, shorten};
use super::planning::{Sprint, is_session_file, parse_ticket};
use super::relations::read_head;
use super::store::{docs_path, is_ticket_id, resolve};
use super::tables::first_of;

/// Höchstens so viele Kriterien und Tickets.
const MAX_ROWS: usize = 15;

// `\d`, `\s`, `\w` und `\b` meinen in Go nur ASCII.
static CRITERION: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^- \*\*(AC-[0-9]{2})\*\*[\t\n\f\r ]*(.*)$").ok());
static STATUS_WORD: LazyLock<Option<Regex>> = LazyLock::new(|| {
    let s = r"[\t\n\f\r ]";
    Regex::new(&format!(
        r"(?:^|[^/0-9A-Za-z_-])(AC-[0-9]{{2}})(?:{s}*(?:–|-|bis){s}*(AC-[0-9]{{2}}))?\**:?{s}+(geprüft|erfüllt|bestanden|grün|teilweise|verschoben|offen|entfällt|verworfen)"
    ))
    .ok()
});
static TICKET_REF: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"(?-u:\b)B-[0-9]+(?-u:\b)").ok());

fn number(criterion: &str) -> u32 {
    criterion.strip_prefix("AC-").and_then(|n| n.parse().ok()).unwrap_or(0)
}

/// Nummer und Text jeder Zeile unter „Akzeptanzkriterien“.
fn criteria(readme: &str) -> Vec<(String, String)> {
    section_text(readme, "Akzeptanzkriterien")
        .split('\n')
        .filter_map(|line| {
            let found = CRITERION.as_ref()?.captures(line)?;
            Some((found.get(1)?.as_str().to_owned(), found.get(2)?.as_str().to_owned()))
        })
        .collect()
}

/// Je Sprint-Kriterium das erste Statuswort aus der Abnahme und den Ergebnissen der Sessions.
pub fn status_by_criterion(dir: &Path, readme: &str, sprint: &Sprint) -> HashMap<String, String> {
    let mut texts = vec![section_text(readme, "Abnahme")];
    for session in sprint.sessions.iter().filter(|s| is_session_file(&s.file)) {
        if let Ok(bytes) = fs::read(dir.join(&session.file)) {
            texts.push(section_text(&String::from_utf8_lossy(&bytes), "Ergebnis"));
        }
    }
    let mut out = HashMap::new();
    let Some(word) = STATUS_WORD.as_ref() else {
        return out;
    };
    for text in &texts {
        for found in word.captures_iter(text) {
            let (Some(from), Some(status)) = (found.get(1), found.get(3)) else {
                continue;
            };
            let from = number(from.as_str());
            let to = found.get(2).map_or(from, |m| number(m.as_str()));
            for n in (from..=to).take(50) {
                out.entry(format!("AC-{n:02}"))
                    .or_insert_with(|| status.as_str().to_owned());
            }
        }
    }
    out
}

fn status_of<'a>(status: &'a HashMap<String, String>, id: &str) -> &'a str {
    first_of(&[status.get(id).map_or("", String::as_str), "–"])
}

fn head(title: &str, columns: &str, rule: &str) -> Vec<String> {
    vec![title.to_owned(), String::new(), columns.to_owned(), rule.to_owned()]
}

/// Kriterien der Sprint-README mit Status.
pub fn criteria_table(readme: &str, status: &HashMap<String, String>) -> Vec<String> {
    let mut out = head("## Kriterien", "| Kriterium | Status | Inhalt |", "|---|---|---|");
    for (i, (id, text)) in criteria(readme).iter().enumerate() {
        if i == MAX_ROWS {
            out.push("| … | | weitere in README.md |".to_owned());
            break;
        }
        let text = shorten(text, 90).replace('|', "/");
        out.push(format!("| {id} | {} | {text} |", status_of(status, id)));
    }
    out.push(String::new());
    out
}

/// Je Ticket die Sprint-Kriterien, die es nennen, mit Status.
fn criteria_by_ticket(readme: &str, status: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for (criterion, text) in criteria(readme) {
        let Some(re) = TICKET_REF.as_ref() else {
            break;
        };
        for ticket in re.find_iter(&text) {
            let entry = format!("{criterion} {}", status_of(status, &criterion));
            let list = out.entry(ticket.as_str().to_owned()).or_default();
            if !list.contains(&entry) {
                list.push(entry);
            }
        }
    }
    out
}

/// Tickets des Sprints mit Status und den Sprint-Kriterien, die sie nennen.
pub fn ticket_table(root: &Path, readme: &str, sprint: &Sprint, status: &HashMap<String, String>) -> Vec<String> {
    let mut out = head(
        "## Tickets",
        "| Ticket | Status | Kriterien | Titel |",
        "|---|---|---|---|",
    );
    let by_ticket = criteria_by_ticket(readme, status);
    for (i, id) in sprint.tickets.iter().filter(|id| is_ticket_id(id)).enumerate() {
        if i == MAX_ROWS {
            out.push("| … | | | weitere in README.md |".to_owned());
            break;
        }
        let (state, title) = match resolve(root, id) {
            Ok(found) => {
                let ticket = parse_ticket(&read_head(&docs_path(root, &found.rel)));
                (ticket.status, shorten(&ticket.title, 80).replace('|', "/"))
            }
            Err(_) => ("fehlt".to_owned(), "–".to_owned()),
        };
        let named = by_ticket.get(id).map_or_else(String::new, |list| list.join(", "));
        out.push(format!("| {id} | {state} | {} | {title} |", first_of(&[&named, "–"])));
    }
    out
}
