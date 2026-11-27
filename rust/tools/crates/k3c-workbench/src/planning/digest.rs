//! Sprint-Zusammenfassungen (B-696 Teil C): je erledigtem Sprint eine kurze `ZUSAMMENFASSUNG.md`, regelbasiert aus
//! Ziel, Abnahme (oder Ergebnis), Kriterien und Tickets der Sprint-README. Die Originale bleiben; die Datei ist kein
//! Dokument nach Vorlage (`load` liest nur die Session-Tabelle, die Planungstests überspringen sie).

use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::digest_tables::{criteria_table, status_by_criterion, ticket_table};
use super::planning::{Sprint, parse_sprint};
use super::store::{ChangeSet, docs_path, exists, lock_writes, read_dir_sorted};
use super::tables::{first_of, split_lines};

/// Name der Zusammenfassung im Sprint-Ordner.
pub const DIGEST_FILE: &str = "ZUSAMMENFASSUNG.md";
pub const MAX_LINES: usize = 60;
/// Zeichen für Ziel und Ergebnis.
const GOAL: usize = 400;
const RESULT: usize = 600;

static DATE: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").ok());
static SPRINT_NUMBER: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^([A-Z]+)([0-9]+)$").ok());

/// Parameter von `plan_digest`.
#[derive(Debug, Clone, Default)]
pub struct DigestIn {
    /// Leer = alle; Sprint-ID (gleiches Präfix, kleinere Nummer) oder Datum JJJJ-MM-TT (letztes Datum der Abnahme davor).
    pub older_than: String,
    /// Nur anzeigen.
    pub dry: bool,
    /// Vorhandene Zusammenfassungen neu schreiben.
    pub renew: bool,
}

/// Schreibt die Zusammenfassungen der erledigten Sprints; ohne `renew` bleiben vorhandene unverändert.
pub fn digest(root: &Path, input: &DigestIn) -> Result<String, String> {
    let _writing = lock_writes();
    let entries = match read_dir_sorted(&docs_path(root, "sprints/erledigt")) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Ok("Zusammenfassungen: keine erledigten Sprints".to_owned());
        }
        Err(e) => return Err(format!("sprints/erledigt nicht lesbar: {e}")),
    };
    let mut c = ChangeSet::new(root);
    let (mut made, mut kept) = (Vec::new(), Vec::new());
    for entry in entries.iter().filter(|e| e.is_dir) {
        let dir = format!("sprints/erledigt/{}", entry.name);
        let readme_rel = format!("{dir}/README.md");
        let readme = c
            .read(&readme_rel)
            .map_err(|e| format!("docs/{readme_rel}: {e}, nichts geschrieben"))?;
        let sprint = parse_sprint(&readme, &entry.name, "erledigt");
        if !older_than(&sprint.id, &readme, &input.older_than) {
            continue;
        }
        let rel = format!("{dir}/{DIGEST_FILE}");
        if exists(root, &rel) && !input.renew {
            kept.push(sprint.id);
            continue;
        }
        let (body, hint) = digest_text(root, &docs_path(root, &dir), &readme, &sprint);
        c.write(&rel, body);
        let crlf = c.crlf.get(&readme_rel).copied().unwrap_or(false);
        c.crlf.insert(rel, crlf);
        made.push(sprint.id + &hint);
    }
    let mut out = format!(
        "Zusammenfassungen: {} neu, {} vorhanden (unverändert)",
        made.len(),
        kept.len()
    );
    if !made.is_empty() {
        out.push_str("\nneu: ");
        out.push_str(&made.join(", "));
    }
    if input.dry {
        return Ok(out + "\n(dry, nichts geschrieben)");
    }
    c.apply()?;
    Ok(out)
}

/// Wählt die Sprints für `älter_als` aus.
fn older_than(id: &str, readme: &str, reference: &str) -> bool {
    if reference.is_empty() {
        return true;
    }
    if reference.len() == 10 && DATE.as_ref().is_some_and(|re| re.is_match(reference)) {
        let acceptance = section_text(readme, "Abnahme");
        let latest = DATE
            .as_ref()
            .and_then(|re| re.find_iter(&acceptance).map(|m| m.as_str()).max());
        return latest.is_some_and(|date| date < reference);
    }
    let parts = |text: &str| -> Option<(String, i64)> {
        let found = SPRINT_NUMBER.as_ref()?.captures(text)?;
        // Beim Überlauf wie Go `strconv.Atoi`: die größte Zahl.
        let number = found.get(2)?.as_str().parse().unwrap_or(i64::MAX);
        Some((found.get(1)?.as_str().to_owned(), number))
    };
    matches!((parts(id), parts(reference)), (Some(a), Some(b)) if a.0 == b.0 && a.1 < b.1)
}

/// Text unter `## name` bis zur nächsten `## `-Überschrift, ohne Rand.
pub fn section_text(text: &str, name: &str) -> String {
    let wanted = format!("## {name}");
    let mut out = Vec::new();
    let mut inside = false;
    for line in split_lines(&text.replace("\r\n", "\n")) {
        if line.starts_with("## ") {
            inside = line == wanted;
        } else if inside {
            out.push(line);
        }
    }
    out.join("\n").trim().to_owned()
}

/// Fasst einen Abschnitt zu einer Zeile mit höchstens `n` Zeichen zusammen; leer wird `–`.
pub fn shorten(text: &str, n: usize) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.is_empty() {
        return "–".to_owned();
    }
    if line.chars().count() <= n {
        return line;
    }
    let cut: String = line.chars().take(n).collect();
    format!("{} …", cut.trim())
}

/// Baut die Zusammenfassung; der zweite Wert nennt fehlende Abschnitte.
fn digest_text(root: &Path, dir: &Path, readme: &str, sprint: &Sprint) -> (String, String) {
    let goal = section_text(readme, "Ziel");
    let mut result = section_text(readme, "Abnahme");
    if result.is_empty() || result.starts_with("Wird von der Review-Session") {
        result = section_text(readme, "Ergebnis");
    }
    let missing: Vec<&str> = [("Ziel", &goal), ("Ergebnis", &result)]
        .into_iter()
        .filter(|(_, text)| text.is_empty())
        .map(|(name, _)| name)
        .collect();
    let status: HashMap<String, String> = status_by_criterion(dir, readme, sprint);
    let mut lines: Vec<String> = vec![
        format!("# {} · Zusammenfassung", sprint.id),
        String::new(),
        "Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md)."
            .to_owned(),
        String::new(),
        format!("- **Titel:** {}", first_of(&[&sprint.title, "–"])),
        format!("- **Projekt:** {}", first_of(&[&sprint.project, "–"])),
        String::new(),
        "## Ziel".to_owned(),
        String::new(),
        shorten(&goal, GOAL),
        String::new(),
        "## Ergebnis".to_owned(),
        String::new(),
        shorten(&result, RESULT),
        String::new(),
    ];
    lines.extend(criteria_table(readme, &status));
    lines.extend(ticket_table(root, readme, sprint, &status));
    if lines.len() > MAX_LINES {
        lines.truncate(MAX_LINES - 1);
        lines.push("… (gekürzt)".to_owned());
    }
    let hint = if missing.is_empty() {
        String::new()
    } else {
        format!(" (ohne {})", missing.join(", "))
    };
    (lines.join("\n").trim_end_matches('\n').to_owned() + "\n", hint)
}
