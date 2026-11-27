//! Felder im Kopf eines Dokuments. Stand RU69.8: die Lesehilfen, `set_field` und die Domänen; `Set`, `Section`,
//! `Sections` und die Feldprüfung folgen in RU69.9.

use std::collections::HashMap;

use super::planning::{field_line, fields};
use super::tables::{join_lines, split_lines};

pub const DOMAINS: [&str; 8] = ["REG", "SIM", "SRV", "CLI", "PLAT", "INF", "DEV", "PAP"];

/// Der Kopf ist der Teil vor der ersten `## `-Überschrift (Kopfzeile und Felder); Rückgabe ist seine Zeilenzahl.
pub fn head_end<S: AsRef<str>>(lines: &[S]) -> usize {
    lines
        .iter()
        .position(|l| l.as_ref().starts_with("## "))
        .unwrap_or(lines.len())
}

pub fn head_fields(text: &str) -> HashMap<String, String> {
    let lines = split_lines(text);
    fields(&lines[..head_end(&lines)])
}

pub fn set_field(text: &str, name: &str, value: &str) -> String {
    let mut lines = split_lines(text);
    let end = head_end(&lines);
    for line in &mut lines[..end] {
        if field_line(line).is_some_and(|(found, _)| found == name) {
            *line = format!("- **{name}:** {value}");
        }
    }
    join_lines(&lines)
}
