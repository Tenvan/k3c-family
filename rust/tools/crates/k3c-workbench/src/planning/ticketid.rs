//! Ticket-Nummern ab B-1000 (B-863): Nummern haben keine feste Länge, Schlüssel und Sortierung gehen über die Zahl.

use std::cmp::Ordering;

/// Zahl einer Ticket-ID (B-1000 → 1000).
fn ticket_num(id: &str) -> Option<i64> {
    id.strip_prefix("B-")?.parse().ok().filter(|n| *n >= 0)
}

/// Ordnet Ticket-IDs nach Nummer (B-999 vor B-1000), sonst nach Text.
pub fn ticket_cmp(a: &str, b: &str) -> Ordering {
    match (ticket_num(a), ticket_num(b)) {
        (Some(na), Some(nb)) if na != nb => na.cmp(&nb),
        _ => a.cmp(b),
    }
}

/// ID aus einem Dateinamen wie `B-1000-titel.md`, sonst leer.
pub fn ticket_file_id(name: &str) -> String {
    name.strip_prefix("B-")
        .and_then(|rest| rest.split_once('-'))
        .map(|(number, _)| format!("B-{number}"))
        .unwrap_or_default()
}
