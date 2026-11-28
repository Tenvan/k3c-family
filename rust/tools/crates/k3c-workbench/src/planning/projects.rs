//! Projekte (B-357): docs/projekte/<KÜRZEL>-slug.md nach vorlagen/projekt.md. Nur aktive Projekte haben einen Rang,
//! lückenlos ab 1; ABN (Abnahmen am Gerät) läuft ohne Rang neben der Rangfolge.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use super::create::{NewDoc, from_template, without_rows, write_new};
use super::edit::{head_end, head_fields, set_field, set_fields};
use super::list::archived;
use super::planning::{fields, header, load, take};
use super::projects_tables::{reorder_sprints, sync_overview};
use super::store::{ChangeSet, Ref, docs_path, entries, quoted, read_text};
use super::tables::{cells, find_table, first_of, heading, split_lines};

/// Projekt der Abnahmen am Gerät: aktiv, aber ohne Rang (arbeitsweise.md › Projekte und Rang).
pub const UNRANKED: &str = "ABN";

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

/// Legt docs/projekte/<ID>-<slug>.md an; ein aktives Projekt bekommt Rang = letzter + 1.
pub fn create_project(c: &mut ChangeSet, input: &NewDoc, template: &str) -> Result<(), String> {
    let id = &input.id;
    if !is_project_id(id) {
        return Err(format!("Projekt-Kürzel {}: drei Großbuchstaben, z. B. GRA", quoted(id)));
    }
    if find_project(&projects_in(c)?, id).is_some() {
        return Err(format!("das Projekt {id} gibt es schon"));
    }
    if input.fields.contains_key("Rang") {
        return Err("den Rang setzt plan_set nach dem Anlegen; neu: letzter + 1".to_owned());
    }
    let header = format!("{id} · {}", input.title);
    let preset = [("Status", "aktiv"), ("Rang", "–"), ("Ziel-Tickets", "–")];
    let text = from_template(&c.root, "projekt", template, &header, &preset, &input.fields)?;
    let rel = format!("projekte/{id}-{}.md", input.slug);
    write_new(c, "projekt", &rel, without_rows(&text, "## Sprints"));
    c.notes.push(format!("{id} angelegt: docs/{rel} (Übersicht ergänzt)"));
    rerank(c, id, 0)
}

/// Löscht ein Projekt ohne Sprints und ohne Sprints oder Tickets, die es nennen.
pub fn delete_project(c: &mut ChangeSet, r: &Ref) -> Result<(), String> {
    let text = c.read(&r.rel)?;
    let project = parse_project(&text, r.rel.rsplit('/').next().unwrap_or(&r.rel));
    if !project.sprints.is_empty() {
        return Err(format!(
            "{} hat Sprints ({}); erst zuordnen",
            r.id,
            project.sprints.join(", ")
        ));
    }
    let data = load(&c.root)?;
    if let Some(sprint) = data.sprints.iter().find(|s| s.project == r.id) {
        return Err(format!("Sprint {} nennt {}", sprint.id, r.id));
    }
    let done = archived(&c.root);
    if let Some(ticket) = data.tickets.iter().chain(&done).find(|t| t.project == r.id) {
        return Err(format!("Ticket {} nennt {}", ticket.nr, r.id));
    }
    c.remove.push(r.rel.clone());
    rerank(c, "", 0)
}

/// Vergibt die Ränge der aktiven Projekte lückenlos neu; `id` rückt auf Platz `pos` (1 … n), `pos` 0 lässt ein schon
/// aktives Projekt an seinem Platz und hängt ein neu aktives hinten an. Danach folgt die Übersicht.
pub fn rerank(c: &mut ChangeSet, id: &str, pos: i64) -> Result<(), String> {
    let projects = projects_in(c)?;
    let ranked = rank_order(&projects, id, pos)?;
    let want: BTreeMap<&str, String> = ranked
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), (i + 1).to_string()))
        .collect();
    for p in &projects {
        let rang = first_of(&[want.get(p.id.as_str()).map_or("", String::as_str), "–"]);
        if p.rang == rang {
            continue;
        }
        let rel = format!("projekte/{}", p.file);
        let text = c.read(&rel)?;
        c.write(&rel, set_field(&text, "Rang", rang));
    }
    sync_overview(c)
}

/// Neue Reihenfolge der aktiven Projekte mit Rang (ohne ABN); `projects` ist nach Rang sortiert, neu aktive (Rang –)
/// stehen hinten.
fn rank_order<'a>(projects: &'a [Project], id: &str, pos: i64) -> Result<Vec<&'a Project>, String> {
    let mut ranked: Vec<&Project> = projects
        .iter()
        .filter(|p| p.status == "aktiv" && p.id != UNRANKED && p.id != id)
        .collect();
    let Some(p) = find_project(projects, id).filter(|p| p.status == "aktiv" && p.id != UNRANKED) else {
        return Ok(ranked);
    };
    let count = ranked.len();
    // Ein Rang unter 1 in der Datei (Go bricht dort ab) zählt als Platz 1.
    let mut at = match p.rang.parse::<i64>() {
        Ok(old) => usize::try_from(old.saturating_sub(1).max(0)).map_or(count, |at| at.min(count)),
        Err(_) => count,
    };
    if pos > 0 {
        let wanted = usize::try_from(pos - 1).unwrap_or(usize::MAX);
        if wanted > count {
            return Err(format!("der Rang {pos} liegt außerhalb 1 … {}", count + 1));
        }
        at = wanted;
    }
    ranked.insert(at, p);
    Ok(ranked)
}

/// Ändert ein Projekt: Status und Ziel-Tickets wie die Vorlage, Rang über [`rerank`], Sprints als neue Reihenfolge der
/// Sprint-Tabelle.
pub fn set_project(c: &mut ChangeSet, r: &Ref, values: &BTreeMap<String, String>) -> Result<(), String> {
    let mut text = c.read(&r.rel)?;
    let pos = rank_value(&r.id, values)?;
    let mut rest = values.clone();
    rest.retain(|k, _| k != "Rang" && k != "Sprints");
    text = set_fields(&c.root, "projekt", &text, &rest)?;
    if let Some(order) = values.get("Sprints") {
        text = reorder_sprints(&text, &r.id, order)?;
    }
    if pos > 0 && head_fields(&text).get("Status").is_none_or(|s| s != "aktiv") {
        return Err("nur aktive Projekte haben einen Rang".to_owned());
    }
    c.write(&r.rel, text);
    c.notes.push(format!(
        "{} geändert: docs/{} (Ränge und Übersicht nachgezogen)",
        r.id, r.rel
    ));
    rerank(c, &r.id, pos)
}

/// Gewünschter Rang: eine Zahl ab 1, nie für ABN; 0 = nicht gesetzt.
fn rank_value(id: &str, values: &BTreeMap<String, String>) -> Result<i64, String> {
    let Some(value) = values.get("Rang") else {
        return Ok(0);
    };
    if id == UNRANKED {
        return Err(format!("{UNRANKED} läuft ohne Rang neben der Rangfolge"));
    }
    value
        .parse::<i64>()
        .ok()
        .filter(|n| *n >= 1)
        .ok_or_else(|| format!("der Rang {} ist keine Zahl ab 1", quoted(value)))
}

/// Der Wert des Felds `Projekt` ist `–` oder ein bestehendes Kürzel.
pub fn check_project_ref(c: &mut ChangeSet, value: &str) -> Result<(), String> {
    if value == "–" {
        return Ok(());
    }
    if find_project(&projects_in(c)?, value).is_none() {
        return Err(format!("unbekanntes Projekt {}", quoted(value)));
    }
    Ok(())
}
