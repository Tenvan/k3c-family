//! Felder der Sessions, die `Set` prüft und nachträgt (Go `next_lanes.go`). Stand RU69.9: `Modell`-Werte,
//! `check_lane` und `insert_field`; die Lane-Auswahl von `plan_next` folgt in RU69.11.

use std::sync::LazyLock;

use regex::Regex;

use super::edit::head_end;
use super::planning::field_line;
use super::relations::{FIELD_DUP, FIELD_REPLACE};
use super::store::quoted;
use super::tables::{join_lines, split_lines};

/// Werte des Session-Felds `Modell`.
pub const MODELS: [&str; 5] = ["–", "haiku", "sonnet", "opus", "fable"];

static LANE: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^(–|[a-z0-9-]+)$").ok());

/// Das Session-Feld `Lane`: `–` oder Kurzname aus a-z, 0-9 und `-`.
pub fn check_lane(value: &str) -> Result<(), String> {
    if LANE.as_ref().is_some_and(|re| re.is_match(value)) {
        return Ok(());
    }
    Err(format!(
        "feld Lane = {} ungültig (– oder Kurzname aus a-z, 0-9, -)",
        quoted(value)
    ))
}

/// Felder, die in älteren Dateien fehlen dürfen, und die Felder, hinter die sie beim Setzen kommen.
fn optional_after(kind: &str, name: &str) -> &'static [&'static str] {
    match (kind, name) {
        ("session", "Lane") => &["Domäne"],
        ("session", "Modell") => &["Lane", "Domäne"],
        ("ticket", FIELD_DUP) => &["Freigabe", "Status"],
        ("ticket", FIELD_REPLACE) => &[FIELD_DUP, "Freigabe", "Status"],
        _ => &[],
    }
}

/// Fügt ein fehlendes optionales Feld hinter dem ersten vorhandenen Vorgänger ein; sonst `None`.
pub fn insert_field(kind: &str, text: &str, name: &str, value: &str) -> Option<String> {
    let mut lines = split_lines(text);
    let end = head_end(&lines);
    let at = optional_after(kind, name).iter().find_map(|previous| {
        lines[..end]
            .iter()
            .position(|l| field_line(l).is_some_and(|(found, _)| found == *previous))
    })?;
    lines.insert(at + 1, format!("- **{name}:** {value}"));
    Some(join_lines(&lines))
}
