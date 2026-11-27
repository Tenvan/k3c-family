//! Projekte (B-357): docs/projekte/<KÜRZEL>-slug.md nach vorlagen/projekt.md. Nur aktive Projekte haben einen Rang,
//! lückenlos ab 1; ABN (Abnahmen am Gerät) läuft ohne Rang neben der Rangfolge.
//!
//! Stand RU69.8: Lesen. Anlegen, Ändern und Löschen folgen in RU69.9 und RU69.10.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use super::edit::head_end;
use super::planning::{fields, header, take};
use super::store::{ChangeSet, docs_path, entries, read_text};
use super::tables::{cells, find_table, heading, split_lines};

/// Kopf einer Projekt-Datei und die Sprints ihrer Tabelle in Reihenfolge.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    /// aktiv | ruht | erledigt
    pub status: String,
    /// Zahl ab 1 oder –
    pub rang: String,
    pub sprints: Vec<String>,
    /// Dateiname in docs/projekte.
    pub file: String,
}

static PROJECT_ID: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"^[A-Z]{3}$").ok());

/// Drei Großbuchstaben.
pub fn is_project_id(id: &str) -> bool {
    PROJECT_ID.as_ref().is_some_and(|re| re.is_match(id))
}

/// Liest eine Projekt-Datei; `file` ist ihr Dateiname.
pub fn parse_project(text: &str, file: &str) -> Project {
    let lines = split_lines(&text.replace("\r\n", "\n"));
    let f = fields(&lines[..head_end(&lines)]);
    let mut project = Project {
        id: file.split('-').next().unwrap_or_default().to_owned(),
        title: file.to_owned(),
        status: take(&f, "Status"),
        rang: take(&f, "Rang"),
        sprints: project_sprints(&lines),
        file: file.to_owned(),
    };
    if let [id, title @ ..] = header(&lines).as_slice()
        && !title.is_empty()
    {
        project.id.clone_from(id);
        project.title = title.join(" · ");
    }
    project
}

/// Sprint-IDs der Tabelle unter `## Sprints`.
fn project_sprints(lines: &[String]) -> Vec<String> {
    let Some(table) = find_table(lines, heading("## Sprints")) else {
        return Vec::new();
    };
    lines[table.first..table.end]
        .iter()
        .filter_map(|l| cells(l).into_iter().next())
        .filter(|c| !c.is_empty())
        .collect()
}

fn is_project_file(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.len() > 4 && bytes[..3].iter().all(u8::is_ascii_uppercase) && bytes[3] == b'-' && name.ends_with(".md")
}

/// Alle Projekte unter `root/docs/projekte`; ohne Ordner eine leere Liste.
pub fn load_projects(root: &Path) -> Vec<Project> {
    let mut out: Vec<Project> = entries(&docs_path(root, "projekte"))
        .iter()
        .filter(|e| !e.is_dir && is_project_file(&e.name))
        .filter_map(|e| {
            let (text, _) = read_text(root, &format!("projekte/{}", e.name)).ok()?;
            Some(parse_project(&text, &e.name))
        })
        .collect();
    sort_projects(&mut out);
    out
}

/// Aktive nach Rang (ABN danach), dann ruhende, dann erledigte; sonst nach Kürzel.
fn sort_projects(projects: &mut [Project]) {
    let group = |status: &str| match status {
        "ruht" => 1,
        "erledigt" => 2,
        _ => 0,
    };
    let rank = |p: &Project| match p.rang.parse::<i64>() {
        Ok(n) if p.status == "aktiv" => n,
        _ => 1 << 20,
    };
    projects.sort_by(|a, b| (group(&a.status), rank(a), &a.id).cmp(&(group(&b.status), rank(b), &b.id)));
}

/// Die Projekte mit den schon geplanten Änderungen des `ChangeSet`.
pub fn projects_in(c: &mut ChangeSet) -> Result<Vec<Project>, String> {
    let mut names: BTreeMap<String, bool> = entries(&docs_path(&c.root, "projekte"))
        .into_iter()
        .map(|e| (e.name, !e.is_dir))
        .collect();
    for rel in c.writes.keys() {
        if let Some(name) = rel.strip_prefix("projekte/").filter(|n| !n.contains('/')) {
            names.insert(name.to_owned(), true);
        }
    }
    for rel in &c.remove {
        names.remove(rel.rsplit('/').next().unwrap_or(rel));
    }
    let mut out = Vec::new();
    for (name, is_file) in names {
        if is_file && is_project_file(&name) {
            let text = c.read(&format!("projekte/{name}"))?;
            out.push(parse_project(&text, &name));
        }
    }
    sort_projects(&mut out);
    Ok(out)
}

pub fn find_project<'a>(projects: &'a [Project], id: &str) -> Option<&'a Project> {
    projects.iter().find(|p| p.id == id)
}
