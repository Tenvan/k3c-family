//! Die beiden Index-Dateien des Backlogs (B-696 Teil B): docs/backlog/README.md führt nur offene und eingeplante
//! Tickets, die Zeilen der erledigten und verworfenen stehen in docs/backlog/archiv/README.md. [`rebuild_index`]
//! erzeugt beide aus den Ticket-Dateien neu; Ticket-Dateien ändert es nie. Dazu `indexFiles` und `reIndexRow` aus
//! Go `similar.go`.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use k3c_assetkit::ticket::{Row, index_row};
use regex::Regex;

use super::planning::{Ticket, is_ticket_file, parse_ticket};
use super::store::{ChangeSet, docs_path, exists, lock_writes, read_dir_sorted, read_text};
use super::ticketid::ticket_cmp;

pub const MAIN_INDEX: &str = "backlog/README.md";
/// Index der erledigten und verworfenen Tickets, Links relativ zu backlog/archiv/.
pub const ARCHIVE_INDEX: &str = "backlog/archiv/README.md";
pub const INDEX_FILES: [&str; 2] = [MAIN_INDEX, ARCHIVE_INDEX];
pub const TABLE_HEAD: &str = "| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n";
const MAIN_HEAD: &str = "# Backlog\n\n\
    Offene und eingeplante Tickets, eine Zeile pro Ticket; erledigte und verworfene stehen in [`archiv/README.md`](archiv/README.md).\n\
    Jedes Ticket ist eine eigene Datei nach [`../vorlagen/ticket.md`](../vorlagen/ticket.md) und eine Spec (SDD, siehe\n\
    [`../arbeitsweise.md`](../arbeitsweise.md)). Anlegen und ändern nur über k3c-workbench (`plan_create`, `plan_set`);\n\
    `task test` prüft Dateien und beide Index-Dateien.\n";
pub const MAIN_ARCHIVE: &str = "## Archiv\n\n\
    Erledigte und verworfene Tickets liegen in [`archiv/`](archiv/), ihre Zeilen in\n\
    [`archiv/README.md`](archiv/README.md) (nur auf Nachfrage lesen). Ändert ein Ticket seinen Status auf `erledigt` oder\n\
    `verworfen`, verschiebt k3c-workbench Datei und Zeile dorthin, beim Zurücksetzen zurück.\n";
pub const ARCHIVE_HEAD: &str = "# Backlog · Archiv\n\n\
    Erledigte und verworfene Tickets, eine Zeile pro Ticket (nur auf Nachfrage lesen). Die Dateien liegen in diesem Ordner\n\
    und bleiben beim Verschieben unverändert; offene und eingeplante Tickets stehen in [`../README.md`](../README.md).\n";

// `\S` wie in Go: alles außer ASCII-Leerraum.
static INDEX_ROW: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"^\| \[(B-[0-9]{3,})\]\([^)]*\) \|[^|]*\|[^|]*\|[^|]*\| ([^\t\n\f\r ]+) \|[^|]*\| (.*) \|$").ok()
});

/// Nummer, Status und Titel einer Index-Zeile.
pub fn index_entry(line: &str) -> Option<(&str, &str, &str)> {
    let found = INDEX_ROW.as_ref()?.captures(line)?;
    Some((found.get(1)?.as_str(), found.get(2)?.as_str(), found.get(3)?.as_str()))
}

/// Schreibt beide Index-Dateien aus den Ticket-Dateien neu. Die Reihenfolge bisheriger Zeilen bleibt, neue kommen nach
/// Nummer ans Ende. `dry` zeigt nur die Zahlen.
pub fn rebuild_index(root: &Path, dry: bool) -> Result<String, String> {
    let _writing = lock_writes();
    let mut c = ChangeSet::new(root);
    let mut old = c.read(MAIN_INDEX)?;
    match c.read(ARCHIVE_INDEX) {
        Ok(archive) => {
            old.push('\n');
            old.push_str(&archive);
        }
        Err(e) if exists(root, ARCHIVE_INDEX) => return Err(format!("{ARCHIVE_INDEX}: {e}")),
        Err(_) => {}
    }
    let mut tickets = index_tickets(root)?;
    let order = row_order(&old);
    tickets.sort_by(|a, b| {
        let (a, b) = (&a.ticket.nr, &b.ticket.nr);
        match (less(&order, a, b), less(&order, b, a)) {
            (true, _) => Ordering::Less,
            (_, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    });
    let (done, open): (Vec<IndexTicket>, Vec<IndexTicket>) = tickets.into_iter().partition(|x| x.archived);
    let (open_rows, done_rows) = (rows(&open), rows(&done));
    let main = format!("{MAIN_HEAD}\n## Offen\n\n{TABLE_HEAD}{open_rows}\n{MAIN_ARCHIVE}");
    c.write(MAIN_INDEX, main);
    c.write(
        ARCHIVE_INDEX,
        format!("{ARCHIVE_HEAD}\n## Archiv\n\n{TABLE_HEAD}{done_rows}"),
    );
    let crlf = c.crlf.get(MAIN_INDEX).copied().unwrap_or(false);
    c.crlf.insert(ARCHIVE_INDEX.to_owned(), crlf);
    let note = format!(
        "Index neu: {} offen/eingeplant in docs/{MAIN_INDEX}, {} erledigt/verworfen in docs/{ARCHIVE_INDEX}",
        open.len(),
        done.len()
    );
    if dry {
        return Ok(note + " (dry, nichts geschrieben)");
    }
    c.apply()?;
    Ok(note)
}

fn rows(tickets: &[IndexTicket]) -> String {
    let rows: Vec<String> = tickets
        .iter()
        .map(|x| {
            let t = &x.ticket;
            index_row(&Row {
                nr: t.nr.clone(),
                link: x.file.clone(),
                domain: t.domain.clone(),
                typ: t.typ.clone(),
                prio: t.prio.clone(),
                status: t.status.clone(),
                sprint: t.sprint.clone(),
                title: t.title.clone(),
            })
        })
        .collect();
    join_rows(&rows)
}

/// Ein Ticket mit seinem Dateinamen und Ordner für den Neuaufbau.
#[derive(Debug, Clone)]
pub struct IndexTicket {
    pub ticket: Ticket,
    pub file: String,
    pub archived: bool,
}

/// Zeilen mit abschließendem Zeilenende; ohne Zeilen leer.
pub fn join_rows(rows: &[String]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    rows.join("\n") + "\n"
}

/// Köpfe aller Tickets; der Ordner muss zum Status passen, sonst bricht der Neuaufbau ab.
pub fn index_tickets(root: &Path) -> Result<Vec<IndexTicket>, String> {
    let mut out = Vec::new();
    for folder in ["backlog", "backlog/archiv"] {
        let dir = docs_path(root, folder);
        let entries = match read_dir_sorted(&dir) {
            Ok(entries) => entries,
            Err(e) if folder == "backlog" => return Err(format!("open {}: {e}", dir.display())),
            Err(_) => continue,
        };
        for entry in entries.into_iter().filter(|e| !e.is_dir && is_ticket_file(&e.name)) {
            let (text, _) = read_text(root, &format!("{folder}/{}", entry.name))?;
            let ticket = parse_ticket(&text);
            let archived = folder != "backlog";
            if archived != (ticket.status == "erledigt" || ticket.status == "verworfen") {
                return Err(format!(
                    "{}: Status {} passt nicht zum Ordner docs/{folder}, nichts geschrieben",
                    entry.name, ticket.status
                ));
            }
            out.push(IndexTicket {
                ticket,
                file: entry.name,
                archived,
            });
        }
    }
    Ok(out)
}

/// Position jeder Ticket-Nummer in den bisherigen Index-Zeilen.
pub fn row_order(text: &str) -> HashMap<String, usize> {
    let mut order = HashMap::new();
    for line in text.split('\n') {
        if let Some(nr) = INDEX_ROW.as_ref().and_then(|re| re.captures(line)?.get(1)) {
            let next = order.len();
            order.entry(nr.as_str().to_owned()).or_insert(next);
        }
    }
    order
}

/// Bekannte Zeilen in alter Reihenfolge, unbekannte danach nach Nummer.
pub fn less(order: &HashMap<String, usize>, a: &str, b: &str) -> bool {
    match (order.get(a), order.get(b)) {
        (Some(ia), Some(ib)) => ia < ib,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => ticket_cmp(a, b) == Ordering::Less, // B-999 vor B-1000 (B-863)
    }
}
