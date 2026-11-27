//! Die beiden Index-Dateien des Backlogs (B-696 Teil B). Stand RU69.9: Konstanten und Hilfen (Go `index.go`, dazu
//! `indexFiles` und `reIndexRow` aus `similar.go`); `RebuildIndex` folgt in RU69.10.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::planning::{Ticket, is_ticket_file, parse_ticket};
use super::store::{docs_path, read_dir_sorted, read_text};
use super::ticketid::ticket_cmp;

pub const MAIN_INDEX: &str = "backlog/README.md";
/// Index der erledigten und verworfenen Tickets, Links relativ zu backlog/archiv/.
pub const ARCHIVE_INDEX: &str = "backlog/archiv/README.md";
pub const INDEX_FILES: [&str; 2] = [MAIN_INDEX, ARCHIVE_INDEX];
pub const TABLE_HEAD: &str = "| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n";
pub const ARCHIVE_HEAD: &str = "# Backlog · Archiv\n\n\
    Erledigte und verworfene Tickets, eine Zeile pro Ticket (nur auf Nachfrage lesen). Die Dateien liegen in diesem Ordner\n\
    und bleiben beim Verschieben unverändert; offene und eingeplante Tickets stehen in [`../README.md`](../README.md).\n";

// `\S` wie in Go: alles außer ASCII-Leerraum.
static INDEX_ROW: LazyLock<Option<Regex>> = LazyLock::new(|| {
    Regex::new(r"^\| \[(B-[0-9]{3,})\]\([^)]*\) \|[^|]*\|[^|]*\|[^|]*\| ([^\t\n\f\r ]+) \|[^|]*\| (.*) \|$").ok()
});

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
