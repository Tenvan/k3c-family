//! Fahrplan aus den Dateien (B-842): Nach jedem schreibenden `plan_*`-Aufruf an Sprints schreibt [`refresh_roadmap`]
//! die Teile von docs/sprints/README.md, die sonst kein Werkzeug pflegt: den Abschnitt „Offen am Gerät“, Zeilen mit
//! fehlendem Ordner fliegen heraus, und ein Thema aus lauter Domänen wird wieder der Sprint-Titel.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::edit::DOMAINS;
use super::planning::{Sprint, load_sprint, parse_sprint};
use super::queue::{device_order, human_items};
use super::store::{docs_path, entries, read_lossy, write_atomic};
use super::tables::{cells, header_at, join_lines, split_lines, table_row};

pub const ROADMAP_REL: &str = "sprints/README.md";

/// Ordner-Zelle einer Fahrplan-Zeile: `aktiv/NG15-…/`.
static ROADMAP_DIR: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new(r"^`((?:aktiv|geplant|erledigt)/[^/`]+)/`$").ok());

/// Schreibt den Fahrplan neu, wenn sich etwas ändert; ohne Fahrplan oder Sprint-Ordner tut es nichts.
pub fn refresh_roadmap(root: &Path) -> Result<(), String> {
    // Ungültiges UTF-8 bleibt unangetastet: Zurückschreiben würde die Bytes verändern.
    let Some(raw) = fs::read(docs_path(root, ROADMAP_REL))
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
    else {
        return Ok(());
    };
    if !docs_path(root, "sprints/aktiv").exists() {
        return Ok(());
    }
    let text = raw.replace("\r\n", "\n");
    let lines = heal_roadmap(root, &split_lines(&text));
    let lines = device_section(lines, &device_groups(&active_sprints(root)));
    let out = join_lines(&lines);
    if out == text {
        return Ok(());
    }
    let path = docs_path(root, ROADMAP_REL);
    write_atomic(&path, &out, raw.contains("\r\n")).map_err(|e| format!("{}: {e}", path.display()))
}

/// Entfernt Zeilen, deren Ordner fehlt, und setzt ein Thema aus lauter Domänen auf den Sprint-Titel.
fn heal_roadmap(root: &Path, lines: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        let mut row = cells(line);
        let folder = row
            .last()
            .filter(|_| line.starts_with("| ") && row.len() >= 2)
            .and_then(|last| Some(ROADMAP_DIR.as_ref()?.captures(last)?.get(1)?.as_str().to_owned()));
        let Some(folder) = folder else {
            out.push(line.clone());
            continue;
        };
        let dir = docs_path(root, &format!("sprints/{folder}"));
        if !dir.join("README.md").exists() {
            continue;
        }
        let column = header_at(lines, i)
            .filter(|header| header.len() == row.len())
            .and_then(|header| header.iter().position(|h| h == "Thema"))
            .filter(|&k| only_domains(&row[k]));
        match column.map(|k| (k, sprint_title(&dir))) {
            Some((k, title)) if !title.is_empty() => {
                row[k] = title;
                out.push(table_row(&row));
            }
            _ => out.push(line.clone()),
        }
    }
    out
}

/// Der Wert besteht nur aus Domänen-Kürzeln (`SIM, REG`), ist also kein Thema.
fn only_domains(value: &str) -> bool {
    value.split(',').all(|p| DOMAINS.contains(&p.trim()))
}

/// Titel aus der README eines Sprint-Ordners; leer, wenn sie nicht lesbar ist.
fn sprint_title(dir: &Path) -> String {
    let name = dir.file_name().map(|n| n.to_string_lossy().into_owned());
    read_lossy(&dir.join("README.md")).map_or_else(
        |_| String::new(),
        |text| parse_sprint(&text, &name.unwrap_or_default(), "").title,
    )
}

/// Nur die aktiven Sprints samt Session-Dateien (ohne Backlog und git, schnell genug je Aufruf).
fn active_sprints(root: &Path) -> Vec<Sprint> {
    let dir = docs_path(root, "sprints/aktiv");
    entries(&dir)
        .into_iter()
        .filter(|e| e.is_dir)
        .filter_map(|e| load_sprint(&dir.join(&e.name), &e.name, "aktiv"))
        .collect()
}

/// Offene Mensch-Sessions aktiver Sprints nach Gerät (wie `plan_queue`).
fn device_groups(sprints: &[Sprint]) -> BTreeMap<String, Vec<String>> {
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for item in sprints.iter().flat_map(human_items) {
        groups.entry(item.device).or_default().push(item.line);
    }
    groups
}

/// Ersetzt den Inhalt von `## Offen am Gerät` bis zur nächsten `## `-Überschrift; fehlt der Abschnitt, bleibt der
/// Fahrplan unverändert.
fn device_section(lines: Vec<String>, groups: &BTreeMap<String, Vec<String>>) -> Vec<String> {
    let Some(start) = lines.iter().position(|l| l == "## Offen am Gerät") else {
        return lines;
    };
    let end = (start + 1..lines.len())
        .find(|&i| lines[i].starts_with("## "))
        .unwrap_or(lines.len());
    let intro = "Offene Mensch-Sessions aktiver Sprints nach Gerät (Zeile `Gerät:` im Kontext der Session). Die Workbench \
                 schreibt den Abschnitt bei jedem schreibenden `plan_*`-Aufruf neu; mit Freigaben und Entscheidungen: \
                 `plan_queue`.";
    let mut body = vec![String::new(), intro.to_owned(), String::new()];
    let order = device_order(groups);
    if order.is_empty() {
        body.extend(["keine".to_owned(), String::new()]);
    }
    for device in order {
        body.extend([format!("### {device}"), String::new()]);
        body.extend(groups.get(&device).into_iter().flatten().cloned());
        body.push(String::new());
    }
    let mut out = lines;
    out.splice(start + 1..end, body);
    out
}
