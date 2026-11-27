//! B-366: Eine Fahrplan-Zeile steht nach Projekt-Rang (aktive nach Rang, ABN danach, dann ruhende und erledigte) und
//! in der Sprint-Reihenfolge der Tabelle ihres Projekts.

use super::planning::Sprint;
use super::projects::{Project, find_project, projects_in};
use super::store::ChangeSet;
use super::tables::{Table, cells};

/// Schlüssel einer Fahrplan-Zeile: Platz des Projekts, dann Platz des Sprints in seiner Projekt-Tabelle; unbekannte
/// Projekte stehen zuletzt.
fn roadmap_key_of(projects: &[Project], project: &str, id: &str) -> (usize, usize) {
    let Some(pi) = projects.iter().position(|p| p.id == project) else {
        return (projects.len(), 0);
    };
    let sprints = &projects[pi].sprints;
    let si = sprints.iter().position(|s| s == id).unwrap_or(sprints.len());
    (pi, si)
}

/// Zeile, vor der die Zeile des Sprints `sp` einzusetzen ist; ohne späteren Eintrag das Tabellenende. Zeilen ohne
/// Spalte `Projekt` zählen als Projekt `–` (zuletzt).
fn roadmap_insert_at(projects: &[Project], lines: &[String], table: &Table, sp: &Sprint) -> usize {
    let column = table.header.iter().position(|h| h == "Projekt");
    let mine = roadmap_key_of(projects, &sp.project, &sp.id);
    (table.first..table.end)
        .find(|&i| {
            let row = cells(&lines[i]);
            let project = column.and_then(|col| row.get(col)).map_or("–", String::as_str);
            let id = row.first().map_or("", String::as_str);
            roadmap_key_of(projects, project, id) > mine
        })
        .unwrap_or(table.end)
}

/// Zeile für den Sprint; Tabellen ohne Spalte `Projekt` und Sprints ohne bekanntes Projekt bekommen das
/// Tabellenende, Letzteres mit Hinweis (B-366).
pub fn roadmap_place(c: &mut ChangeSet, lines: &[String], table: &Table, sp: &Sprint) -> Result<usize, String> {
    if !table.header.iter().any(|h| h == "Projekt") {
        return Ok(table.end);
    }
    let projects = projects_in(c)?;
    if find_project(&projects, &sp.project).is_none() {
        c.notes.push(format!(
            "{}: Fahrplan-Zeile am Tabellenende, kein bekanntes Projekt ({})",
            sp.id, sp.project
        ));
        return Ok(table.end);
    }
    Ok(roadmap_insert_at(&projects, lines, table, sp))
}
